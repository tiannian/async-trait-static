#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

async fn hello() -> u8 {
    1
}

trait Test {}

#[async_trait]
trait MyTrait {
    // Self by reference
    async fn test_fn(&self, u: u8) -> u8;

    // no async
    fn test_fn_no_async(&self) -> u8;

    async fn test_fn_by_mut_reference(&mut self, s: impl Test);
}

struct TraitStruct;

#[async_trait]
impl MyTrait for TraitStruct {
    async fn test_fn(&self, u: u8) -> u8 {
        hello().await
    }

    fn test_fn_no_async(&self) -> u8 {
        1
    }

    async fn test_fn_by_mut_reference(&mut self, s: impl Test) {}
}

fn main() {
    let s = TraitStruct;
}
