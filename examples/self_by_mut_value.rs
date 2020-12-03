// Can't compile.
// Due to `#[deny(patterns_in_fns_without_body)]`

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

#[async_trait]
trait AsyncFnTrait {
    async fn run(mut self);
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(self) -> u8 {
        1
    }
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run(mut self) {
        self.hello().await;
    }
}

fn main() {}
