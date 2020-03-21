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
    async fn test_fn<'a>(&self, u: &'a u8) -> u8;

    // no async
    fn test_fn_no_async(&self) -> u8;
}

struct TraitStruct;

#[async_trait]
impl MyTrait for TraitStruct {
    async fn test_fn<'a>(&self, u: &'a u8) -> &'a u8 {
        u
    }

    fn test_fn_no_async(&self) -> u8 {
        1
    }
}

fn main() {
    let s = TraitStruct;
}
