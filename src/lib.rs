pub mod ext_traits;
pub mod lending_iterator;

pub use ext_traits::BufReadExt;
pub use lending_iterator::LendingIterator;

#[macro_export]
macro_rules! lazily {
    ($expr:expr) => {{
        static LAZY: ::once_cell::sync::Lazy<
            ::std::boxed::Box<
                dyn ::std::any::Any + ::std::marker::Sync + ::std::marker::Send + 'static,
            >,
        > = ::once_cell::sync::Lazy::new(|| Box::new($expr));

        fn infer_type<T>(_: fn() -> T) -> &'static T {
            LAZY.downcast_ref().unwrap()
        }
        infer_type(|| $expr)
    }};
}