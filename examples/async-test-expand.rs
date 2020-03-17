#![feature(type_alias_impl_trait)]

struct TraitStruct {}

async fn hello() -> u8 {
    println!("hello");
    1
}

trait MyTrait: Sized {
    fn test_fn(&self) -> Self::FutureReturnTypeTestFn;

    type FutureReturnTypeTestFn: core::future::Future<Output = u8>;
}

impl MyTrait for TraitStruct {
    fn test_fn(&self) -> Self::FutureReturnTypeTestFn {
        async move { hello().await }
    }
    type FutureReturnTypeTestFn = impl core::future::Future<Output = u8>;
}

fn main() {
    println!("hello");
}
