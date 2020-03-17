trait Trait {
    fn run<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
    fn run1<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: ::core::marker::Sync + 'async_trait,
    {
        async fn __run1<AsyncTrait: ?Sized + Trait + ::core::marker::Sync>(_self: &AsyncTrait) {
            _self.run1().await;
        }
        Box::pin(__run1::<Self>(self))
    }
}
impl Trait for Struct {
    fn run<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        #[allow(
            clippy::missing_docs_in_private_items,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        async fn __run(_self: &Struct) {
            _self.hello();
        }
        Box::pin(__run(self))
    }
}
