//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

trait TestTrait {}

#[async_trait]
trait AsyncFnTrait {
    async fn run(self, t: impl TestTrait);
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run(self, t: impl TestTrait) {
        self.hello().await;
    }
}

fn main() {}
