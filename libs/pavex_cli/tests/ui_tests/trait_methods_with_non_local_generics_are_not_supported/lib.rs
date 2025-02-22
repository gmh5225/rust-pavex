use pavex_builder::{f, AppBlueprint, Lifecycle};

pub struct A;

pub struct B;

pub fn b_constructor() -> B {
    todo!()
}

pub trait GenericTrait<T> {
    fn a_method_using_the_trait_generic_param() -> T;
}

impl<T> GenericTrait<T> for B {
    fn a_method_using_the_trait_generic_param() -> T {
        todo!()
    }
}

pub fn handler(_a: A, _b: B) -> pavex_runtime::response::Response {
    todo!()
}

pub fn blueprint() -> AppBlueprint {
    let mut bp = AppBlueprint::new();
    bp.constructor(f!(crate::b_constructor), Lifecycle::RequestScoped);
    bp.constructor(
        f!(<crate::B as crate::GenericTrait<crate::A>>::a_method_using_the_trait_generic_param),
        Lifecycle::RequestScoped,
    );
    bp.route(f!(crate::handler), "/home");
    bp
}
