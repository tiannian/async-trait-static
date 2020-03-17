#![feature(type_alias_impl_trait)]
use async_trait_static::async_trait;

async fn hello() -> u8 {
    println!("hello");
    1
}

#[async_trait]
trait MyTrait {
    async fn test_fn(&self) -> u8;

    async fn test_fn_default(&self) -> u8 {
        self.test_fn()
    }
}

struct TraitStruct;

#[async_trait]
impl MyTrait for TraitStruct {
    async fn test_fn(&self) -> u8 {
        hello().await
    }
}

fn main() {
    let s = TraitStruct;
    s.test_fn();
    s.test_fn_default();
}
