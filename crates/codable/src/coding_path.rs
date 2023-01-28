use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodingKey {
    Root,
    Int(usize),
    String(Cow<'static, str>),
}

impl CodingKey {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            CodingKey::Root => Cow::Borrowed(""),
            CodingKey::String(x) => x.clone(),
            CodingKey::Int(x) => Cow::Owned(format!("{}", x)),
        }
    }
}

pub trait ToCodingKey {
    fn to_coding_key(&self) -> CodingKey;
    fn as_str(&self) -> Cow<'static, str> {
        self.to_coding_key().as_str()
    }
}

impl ToCodingKey for String {
    fn to_coding_key(&self) -> CodingKey {
        CodingKey::String(Cow::Owned(self.clone()))
    }
}

impl<'a> ToCodingKey for &'a str {
    fn to_coding_key(&self) -> CodingKey {
        CodingKey::String(Cow::Owned(self.to_string()))
    }
}

impl Display for CodingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodingKey::Root => f.write_str("<root>"),
            CodingKey::String(x) => Display::fmt(&x, f),
            CodingKey::Int(x) => Display::fmt(&x, f),
        }
    }
}

#[derive(Clone)]
pub struct CodingPath<'a, T>(*const CodingPath<'a, T>, T, PhantomData<&'a ()>);

impl<T: Clone + Debug> Debug for CodingPath<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_vec().fmt(f)
    }
}

impl<'a, T: Clone> CodingPath<'a, T> {
    pub fn root(value: T) -> CodingPath<'static, T> {
        CodingPath(std::ptr::null(), value, PhantomData)
    }

    pub fn join(&self, item: T) -> CodingPath<'a, T> {
        CodingPath(self, item, PhantomData)
    }

    pub fn iter(&self) -> CodingPathIter<'_, T> {
        CodingPathIter { current: self }
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = self.iter().cloned().collect::<Vec<_>>();
        vec.reverse();
        vec
    }
}

pub struct CodingPathIter<'a, T> {
    current: *const CodingPath<'a, T>,
}

impl<'a, T> Iterator for CodingPathIter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        let data = unsafe { &*self.current };
        self.current = data.0;
        Some(&data.1)
    }
}
