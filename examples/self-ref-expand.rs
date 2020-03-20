trait Trait {
    type ReturnTypeTestFn: core::future::Future;

    fn test_fn(&self) -> Self::ReturnTypeTestFn;

    type ReturnTypeTestFnMut: core::future::Future;

    fn test_fn_mut(&mut self) -> Self::ReturnTypeTestFn;
}

fn main() {}
