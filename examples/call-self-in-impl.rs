#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;
use std::pin::Pin;

async fn hello() -> u8 {
    println!("hello");
    1
}

#[async_trait]
trait MyTrait {
    async fn test_fn(&self) -> u8;

    async fn test_call(self: Pin<Self>) -> u8;
}

struct TraitStruct;

#[async_trait]
impl MyTrait for TraitStruct {
    async fn test_fn(&self) -> u8 {
        hello().await
    }

    async fn test_call(self: Pin<Self>) -> u8 {
        self.test_fn().await
    }
}

fn main() {
    let s = TraitStruct;
    s.test_call();
}
