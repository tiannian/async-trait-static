//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;
use core::fmt::Display;

#[async_trait]
trait AsyncFnTrait {
    async fn run<T: Display>(&self, t: T) -> T;
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run<T: Display>(&self, t: T) -> T {
        self.hello().await;
        t
    }
}

fn main() {}
