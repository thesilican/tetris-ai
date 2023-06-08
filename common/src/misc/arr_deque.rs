use anyhow::{bail, Result};
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Index;

#[derive(Clone, Copy)]
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
    pub fn push_back(&mut self, item: T) -> Result<()> {
        if self.len == N {
            bail!("insufficient capacity for ArrDeque");
        }
        let i = (self.head + self.len) % N;
        self.arr[i] = Some(item);
        self.len += 1;
        Ok(())
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let i = self.head;
        self.head = (self.head + 1) % N;
        self.len -= 1;
        self.arr[i].take()
    }
    pub fn clear(&mut self) {
        for i in 0..self.len {
            let idx = (self.head + i) % N;
            self.arr[idx].take();
        }
        self.head = 0;
        self.len = 0;
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            idx: self.head,
            count: self.len,
            arr: &self.arr,
        }
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
impl<T, const N: usize> Default for ArrDeque<T, N>
where
    T: Default,
{
    fn default() -> Self {
        let arr = [(); N].map(|_| None);
        Self {
            head: 0,
            len: 0,
            arr,
        }
    }
}
impl<T, const N: usize> Extend<T> for ArrDeque<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            if let Err(_) = self.push_back(item) {
                break;
            }
        }
    }
}
impl<T, const N: usize> FromIterator<T> for ArrDeque<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arr = ArrDeque::<T, N>::new();
        arr.extend(iter);
        arr
    }
}
impl<T, const N: usize> Debug for ArrDeque<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a, T> {
    idx: usize,
    count: usize,
    arr: &'a [Option<T>],
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let i = self.idx;
            self.idx += 1;
            if self.idx == self.arr.len() {
                self.idx = 0;
            }
            self.count -= 1;
            Some(self.arr[i].as_ref().unwrap())
        }
    }
}

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
        write!(f, "ring-vector with max capacity {N}")
    }
    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut arr = ArrDeque::<T, N>::new();
        while let Some(val) = access.next_element::<T>()? {
            match arr.push_back(val) {
                Ok(_) => {}
                Err(_) => {
                    return Err(S::Error::custom(format!(
                        "supplied value longer than max capacity: {N}"
                    )))
                }
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

mod test {
    #[test]
    fn test_arr_deque() {
        use crate::ArrDeque;
        let mut arr = ArrDeque::<i32, 6>::new();
        for i in 0..6 {
            arr.push_back(i).unwrap();
        }
        for _ in 0..4 {
            arr.pop_front();
        }
        for i in 0..4 {
            arr.push_back(i).unwrap();
        }
        let expected = "[4, 5, 0, 1, 2, 3]";
        assert_eq!(expected, &format!("{arr:?}"))
    }
}
