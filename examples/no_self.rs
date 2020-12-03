//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

#[async_trait]
trait AsyncFnTrait {
    async fn run();
}

struct AsyncStruct;

async fn hello() -> u8 {
    1
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run() {
        hello().await;
    }
}

fn main() {}
