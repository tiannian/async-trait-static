#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

#[async_trait]
trait MyTrait {
    // Self by reference
    async fn test_fn(self) -> u8;

    async fn test_fn_by_mut_reference();
}

struct TraitStruct;

impl TraitStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[async_trait]
impl MyTrait for TraitStruct {
    async fn test_fn(self) -> u8 {
        self.hello().await
    }

    async fn test_fn_by_mut_reference() {}
}

fn main() {
    let s = TraitStruct;
}
