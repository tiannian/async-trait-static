//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

#[async_trait]
trait AsyncFnTrait {
    async fn run<'a>(&self, i: &'a u8) -> u8;

    async fn deff(&self) -> u8 {
        1
    }

    // async fn ret_value(&self) -> &[u8];
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run<'a>(&self, i: &'a u8) -> u8 {
        self.hello().await;
        0
    }
}

fn main() {}
