use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::{iter::FromIterator, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertRes {
    Ok,
    Full,
}

#[derive(Debug, Clone, Copy)]
/// Basic stack-based circular buffer
pub struct ArrDeque<T, const N: usize> {
    head: usize,
    len: usize,
    // head and len should always stay in sync with arr
    // Some(T) if within array bounds and None if not
    arr: [Option<T>; N],
}
impl<T, const N: usize> ArrDeque<T, N> {
    pub fn new() -> Self {
        // Work around so that T does not need to be Copy
        // Initializes the array with None
        // Slow but at least doesn't require unsafe
        let arr = [(); N].map(|_| None);
        ArrDeque {
            head: 0,
            len: 0,
            arr,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity(&self) -> usize {
        N
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn push_back(&mut self, item: T) -> InsertRes {
        if self.len == N {
            return InsertRes::Full;
        }
        let i = (self.head + self.len) % N;
        self.arr[i] = Some(item);
        self.len += 1;
        InsertRes::Ok
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let res = self.arr[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;
        res
    }
    pub fn clear(&mut self) {
        for i in 0..self.len {
            let idx = (self.head + i) % N;
            self.arr[idx].take();
        }
        self.head = 0;
        self.len = 0;
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let start = self.head;
        let (end, wrap_end) = if self.head + self.len > N {
            (N, self.head + self.len - N)
        } else {
            (self.head + self.len, 0)
        };

        self.arr[start..end]
            .iter()
            .chain(self.arr[..wrap_end].iter())
            .map(|x| x.as_ref().unwrap())
    }
}
impl<T, const N: usize> Extend<T> for ArrDeque<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter.into_iter() {
            if let InsertRes::Full = self.push_back(item) {
                return;
            }
        }
    }
}
impl<'a, T: 'a + Copy, const N: usize> Extend<&'a T> for ArrDeque<T, N> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied())
    }
}
impl<T, const N: usize> Index<usize> for ArrDeque<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let i = (index + self.head) % N;
        self.arr[i].as_ref().unwrap()
    }
}
impl<T, const N: usize> PartialEq<Self> for ArrDeque<T, N>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.len() {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}
impl<T, const N: usize> Eq for ArrDeque<T, N> where T: Eq {}
impl<T, const N: usize> Hash for ArrDeque<T, N>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..self.len() {
            self[i].hash(state);
        }
    }
}
impl<T, const N: usize> FromIterator<T> for ArrDeque<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arr = ArrDeque::new();
        arr.extend(iter);
        arr
    }
}
impl<'a, T: 'a, const N: usize> FromIterator<&'a T> for ArrDeque<T, N>
where
    T: Copy,
{
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut arr = ArrDeque::new();
        for &item in iter {
            if let InsertRes::Full = arr.push_back(item) {
                return arr;
            }
        }
        arr
    }
}

// Manually implement ser/de (because derive doesn't seem to work)
impl<T, const N: usize> Serialize for ArrDeque<T, N>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.len()))?;
        for val in self.iter() {
            state.serialize_element(val)?;
        }
        state.end()
    }
}
struct ArrDequeVisitor<T, const N: usize> {
    marker: PhantomData<fn() -> ArrDeque<T, N>>,
}
impl<T, const N: usize> ArrDequeVisitor<T, N> {
    fn new() -> Self {
        ArrDequeVisitor {
            marker: PhantomData,
        }
    }
}
impl<'de, T, const N: usize> Visitor<'de> for ArrDequeVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = ArrDeque<T, N>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ring-vector with max capacity {}", N)
    }
    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut arr = ArrDeque::<T, N>::new();
        while let Some(val) = access.next_element::<T>()? {
            if let InsertRes::Full = arr.push_back(val) {
                return Err(S::Error::custom(format!(
                    "supplied value longer than max capacity: {}",
                    N
                )));
            }
        }
        Ok(arr)
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for ArrDeque<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ArrDequeVisitor::new())
    }
}
