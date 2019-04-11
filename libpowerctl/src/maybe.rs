pub enum Maybe<T, E> {
    Some(T),
    None,
    Err(E),
}
use std::{convert::TryInto, fmt::Debug, ops::Try};

impl<T, E> Maybe<T, E>
where
    E: Debug,
{
    pub fn is_some(&self) -> bool {
        match *self {
            Maybe::Some(_) => true,
            Maybe::None => false,
            Maybe::Err(_) => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match *self {
            Maybe::Some(_) => false,
            Maybe::None => true,
            Maybe::Err(_) => false,
        }
    }

    pub fn is_ok(&self) -> bool {
        match *self {
            Maybe::Some(_) => true,
            Maybe::None => true,
            Maybe::Err(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match *self {
            Maybe::Some(_) => false,
            Maybe::None => false,
            Maybe::Err(_) => true,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Maybe::Some(e) => e,
            Maybe::None => panic!("Called unwrap on a Maybe::None"),
            Maybe::Err(e) => panic!("Called unwrap on a Maybe::Err: {:?}", e),
        }
    }
}

impl<T, E> Into<Result<Option<T>, E>> for Maybe<T, E> {
    fn into(self) -> Result<Option<T>, E> {
        match self {
            Maybe::Some(t) => Ok(Some(t)),
            Maybe::None => Ok(None),
            Maybe::Err(e) => Err(e),
        }
    }
}

impl<T, E> TryInto<Option<T>> for Maybe<T, E> {
    type Error = E;

    fn try_into(self) -> Result<Option<T>, Self::Error> {
        match self {
            Maybe::Some(t) => Ok(Some(t)),
            Maybe::None => Ok(None),
            Maybe::Err(e) => Err(e),
        }
    }
}

impl<T, E> Try for Maybe<T, E> {
    type Error = E;
    type Ok = Option<T>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Maybe::Some(t) => Ok(Some(t)),
            Maybe::None => Ok(None),
            Maybe::Err(e) => Err(e),
        }
    }

    fn from_error(v: Self::Error) -> Self { Maybe::Err(v) }

    fn from_ok(v: Self::Ok) -> Self {
        match v {
            Some(t) => Maybe::Some(t),
            None => Maybe::None,
        }
    }
}
