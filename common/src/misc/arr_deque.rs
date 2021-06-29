use std::{convert::TryInto, iter::FromIterator, ops::Index, slice::Iter};

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
        let arr = (0..N)
            .map(|_| None)
            .collect::<Vec<Option<T>>>()
            .try_into()
            .ok()
            .unwrap();
        ArrDeque {
            head: 0,
            len: 0,
            arr,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn push_back(&mut self, item: T) -> InsertRes {
        if self.len == N {
            return InsertRes::Full;
        }
        self.len += 1;
        let i = (self.head + self.len) % N;
        self.arr[i] = Some(item);
        InsertRes::Ok
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let i = (self.head + self.len) % N;
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
        if self.head + self.len > N {
            self.arr.iter().map(|x| x.unwrap())
        }
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
        self.arr == other.arr
    }
}
impl<T, const N: usize> Eq for ArrDeque<T, N> where T: Eq {}
impl<T, const N: usize> FromIterator<T> for ArrDeque<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arr = ArrDeque::new();
        arr.extend(iter);
        arr
    }
}
