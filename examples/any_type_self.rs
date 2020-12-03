//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use std::boxed::Box;
use async_trait_static::async_trait;

#[async_trait]
trait AsyncFnTrait {
    async fn run(s: Box<Self>);
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run(s: Box<Self>) {
        s.hello().await;
    }
}

fn main() {}
