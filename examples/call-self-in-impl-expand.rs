#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

async fn hello() -> u8 {
    println!("hello");
    1
}

struct TraitStruct;

impl TraitStruct {
    async fn hello(&self) -> u8 {
        println!("hello");
        1
    }
}

trait MyTrait {
    fn test_fn(&self) -> Self::FutureReturnTypeTestFn<'_>;

    type FutureReturnTypeTestFn<'a>: core::future::Future<Output = u8>;
}

impl MyTrait for TraitStruct {
    type FutureReturnTypeTestFn<'a> = impl core::future::Future<Output = u8>;

    fn test_fn(&self) -> Self::FutureReturnTypeTestFn<'_> {
        async move { self.hello().await }
    }
}

#[async_std::main]
async fn main() {
    let s = TraitStruct;
    s.test_fn().await;
}
