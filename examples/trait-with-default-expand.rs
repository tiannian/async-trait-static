#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

async fn hello() -> u8 {
    println!("hello");
    1
}

struct TraitStruct;

type FutureReturnTypeTestFnDefaultMyTrait<'a> = impl core::future::Future<Output = u8>;

trait MyTrait {
    type FutureReturnTypeTestFn<'a>: core::future::Future<Output = u8>;

    fn test_fn(&self) -> Self::FutureReturnTypeTestFn<'_>;

    fn test_fn_default(&self) -> FutureReturnTypeTestFnDefaultMyTrait<'_>
    where
        Self: Sync,
    {
        async fn _inner<AsyncTrait: ?Sized + MyTrait>(_self: &AsyncTrait) -> u8 {
            _self.test_fn().await
        }
        _inner(self)
    }
}

/* impl MyTrait for TraitStruct { */
// fn test_fn<'async_trait>(&self) -> Self::FutureReturnTypeTestFn {
//     async move { hello().await }
// }
// type FutureReturnTypeTestFn = impl core::future::Future<Output = u8>;
/* } */

fn main() {
    let s = TraitStruct;
    // s.test_fn();
    // s.test_fn_default();
}
