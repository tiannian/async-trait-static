#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

async fn hello() -> u8 {
    1
}

trait Test {}

#[async_trait]
trait MyTrait {
    type O;

    // Self by reference
    async fn test_fn(&self, u: u8) -> Self::O;
}

struct TraitStruct;

#[async_trait]
impl MyTrait for TraitStruct {
    type O = u8;

    async fn test_fn(&self, u: u8) -> Self::O {
        hello().await
    }
}

fn main() {
    let s = TraitStruct;
}
