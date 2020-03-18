#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

async fn hello() -> u8 {
    println!("hello");
    1
}

struct TraitStruct;

trait MyTrait {
    // type FutureReturnTypeTestFn<'a>: core::future::Future<Output = u8>;

    // fn test_fn(&self) -> Self::FutureReturnTypeTestFn<'_>;
    type FutureReturnTypeTestFnDefaultMyTrait<'a>: core::future::Future<Output = u8>;

    fn test_fn_default(&self) -> Self::FutureReturnTypeTestFnDefaultMyTrait<'_> {
        async move { 1 }
    }
}

impl MyTrait for TraitStruct {
    type FutureReturnTypeTestFnDefaultMyTrait<'a> = impl core::future::Future<Output = u8>;

    // fn test_fn(&self) -> Self::FutureReturnTypeTestFn<'_> {
    // async move { hello().await }
    // }
    // type FutureReturnTypeTestFn<'a> = impl core::future::Future<Output = u8>;
}

fn main() {
    let s = TraitStruct;
    // s.test_fn();
    // s.test_fn_default();
}
