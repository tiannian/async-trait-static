#![feature(generic_associated_types)]

use std::pin::Pin;
use std::task::{Context, Poll};

struct ReturnTypeTestFnImpl<'a, S, T> {
    _self: &'a S,
    t: T,
}

impl<'a, S, T> ReturnTypeTestFnImpl<'a, S, T> {
    pub fn new(_self: &'a S, t: T) -> Self {
        ReturnTypeTestFnImpl { _self, t }
    }
}

impl<'a, S, T> core::future::Future for ReturnTypeTestFnImpl<'a, S, T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        async fn _inner<IS, IT>(_self: &IS, t: IT) {
            ()
        }
        let mut r = _inner::<S, T>(self._self, self.t);
        let f = unsafe { Pin::new_unchecked(&mut r) };
        f.poll(cx)
    }
}

trait Trait: Sized {
    fn test_fn<T>(&self, t: T) -> ReturnTypeTestFnImpl<'_, Self, T>;
}

struct Struct;

impl Trait for Struct {
    fn test_fn<T>(&self, t: T) -> ReturnTypeTestFnImpl<'_, Self, T> {
        ReturnTypeTestFnImpl::new(self, t)
    }
}

fn main() {}
