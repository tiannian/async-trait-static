//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

#[async_trait]
trait AsyncFnTrait {
    type A;
    async fn run(self, a: Self::A) -> Self::A;
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    type A = u8;
    async fn run(self, a: Self::A) -> Self::A {
        self.hello().await;
        a
    }
}

fn main() {}
