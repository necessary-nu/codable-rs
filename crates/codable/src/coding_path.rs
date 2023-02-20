use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodingKey<'a> {
    Root,
    Int(usize),
    String(Cow<'a, str>),
}

impl<'a> CodingKey<'a> {
    pub fn as_str(&self) -> Cow<'a, str> {
        match self {
            CodingKey::Root => Cow::Borrowed(""),
            CodingKey::String(x) => x.clone(),
            CodingKey::Int(x) => Cow::Owned(format!("{}", x)),
        }
    }
}

pub trait ToCodingKey {
    fn to_coding_key(&self) -> CodingKey<'_>;
    fn as_str(&self) -> Cow<'_, str> {
        self.to_coding_key().as_str()
    }
}

impl ToCodingKey for String {
    fn to_coding_key(&self) -> CodingKey<'_> {
        CodingKey::String(Cow::Owned(self.clone()))
    }
}

impl<'a> ToCodingKey for &'a str {
    fn to_coding_key(&self) -> CodingKey<'_> {
        CodingKey::String(Cow::Owned(self.to_string()))
    }
}

impl Display for CodingKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodingKey::Root => f.write_str("<root>"),
            CodingKey::String(x) => Display::fmt(&x, f),
            CodingKey::Int(x) => Display::fmt(&x, f),
        }
    }
}

#[derive(Clone)]
pub struct CodingPath<'a>(*const CodingPath<'a>, CodingKey<'a>);

impl Debug for CodingPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_vec().fmt(f)
    }
}

impl Display for CodingPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = self.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        v.pop(); // pop the root element
        v.reverse();
        f.write_str(&v.join("."))
    }
}

impl<'a> CodingPath<'a> {
    pub fn root() -> CodingPath<'a> {
        CodingPath(std::ptr::null(), CodingKey::Root)
    }

    pub fn replace(&'a mut self, item: CodingKey<'a>) {
        self.1 = item;
    }

    pub fn join(&self, item: CodingKey<'a>) -> CodingPath<'a> {
        CodingPath(self, item)
    }

    pub fn iter(&'a self) -> CodingPathIter<'a> {
        CodingPathIter { current: self }
    }

    pub fn to_vec(&'a self) -> Vec<CodingKey<'_>> {
        let mut vec = self.iter().cloned().collect::<Vec<_>>();
        vec.reverse();
        vec
    }
}

pub struct CodingPathIter<'a> {
    current: *const CodingPath<'a>,
}

impl<'a> Iterator for CodingPathIter<'a> {
    type Item = &'a CodingKey<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        let data = unsafe { &*self.current };
        self.current = data.0;
        Some(&data.1)
    }
}
