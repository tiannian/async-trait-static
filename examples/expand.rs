#![feature(prelude_import)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

async fn hello() -> u8 {
    1
}

trait Er {}

struct Eri {}

impl Er for Eri {}

#[warn(incomplete_features)]
trait MyTrait {
    type O;

    fn test_fn(&self, u: u8) -> Self::ReturnTypeFutureReturnTypeTestFn<'_>;
    fn test_fn_no_async(&self) -> u8;
    fn test_fn_by_mut_reference(&self) -> Self::ReturnTypeFutureReturnTypeTestFnByMutReference<'_>;
    type ReturnTypeFutureReturnTypeTestFn<'a>: core::future::Future<Output = u8>;
    type ReturnTypeFutureReturnTypeTestFnByMutReference<T>: core::future::Future<Output = T>;
}
/* struct TraitStruct; */
// impl MyTrait for TraitStruct {
//     type O = u8;
//
//     fn test_fn(&self, u: u8) -> Self::ReturnTypeFutureReturnTypeTestFn<'_> {
//         async move { hello().await }
//     }
//     fn test_fn_no_async(&self) -> u8 {
//         1
//     }
//     fn test_fn_by_mut_reference(&self) -> Self::ReturnTypeFutureReturnTypeTestFnByMutReference<'_> {
//         async move { 1 }
//     }
//     type ReturnTypeFutureReturnTypeTestFn<'a> = impl core::future::Future<Output = u8>;
//     type ReturnTypeFutureReturnTypeTestFnByMutReference<'a> =
//         impl core::future::Future<Output = Self::O>;
/* } */
fn main() {
    let s = TraitStruct;
}
