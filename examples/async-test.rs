#![feature(type_alias_impl_trait)]
use async_trait_static::async_trait;

#[async_trait]
trait MyTrait {
    async fn test_fn() -> u8;

    async fn test_fn_default() -> u8 {
        1
    }
}

struct TraitStruct {}

#[async_trait]
impl MyTrait for TraitStruct {
    async fn test_fn() -> u8 {
        1
    }
}

fn main() {
    println!("hello");
}

