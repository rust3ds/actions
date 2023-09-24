//! Macros for working with test runners.

// Use a neat little trick with cfg(doctest) to make code fences appear in
// rustdoc output, but still compile normally when doctesting. This raises warnings
// for invalid code though, so we also silence that lint here.
#[cfg_attr(not(doctest), allow(rustdoc::invalid_rust_codeblocks))]
/// Helper macro for writing doctests using this runner. Wrap this macro around
/// your normal doctest to enable running it with this crate's
/// [`GdbRunner`](crate::GdbRunner).
///
/// You may use any of the various
/// [`fn main()`](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#using--in-doc-tests)
/// signatures allowed by documentation tests.
///
/// # Examples
///
/// ## Basic usage
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// test_runner::doctest! {
///     let two = 2;
///     let four = 4;
///     assert_eq!(two + two, four);
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
/// ## `should_panic`
///
/// Using `no_run` or `ignore` makes this macro somewhat irrelevant, but
/// `should_panic` is still supported:
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```should_panic
/// test_runner::doctest! {
///     assert_eq!(2 + 2, 5);
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
/// ## Custom `fn main`, crate attribute
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// #![allow(unused)]
///
/// use std::error::Error;
///
/// test_runner::doctest! {
///     // imports can be added outside or inside the macro
///     use std::ops::Add;
///
///     fn main() -> Result<(), Box<dyn Error>> {
///         let two = 2;
///         let four = 4;
///         assert_eq!(Add::add(two, two), four);
///         Ok(())
///     }
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
/// ## Implicit return type
///
/// Note that for the rustdoc preprocessor to understand the return type, the
/// `Ok(())` expression must be written _outside_ the `doctest!` invocation.
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// test_runner::doctest! {
///     assert_eq!(2 + 2, 4);
/// }
/// Ok::<(), std::io::Error>(())
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```should_panic
/// test_runner::doctest! {
///     assert_eq!(2 + 2, 4);
///     Err::<(), &str>("uh oh")
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
#[macro_export]
macro_rules! doctest {
    (@_@ $($body:tt)*) => {
        fn main() -> impl std::process::Termination {
            #[allow(unused_imports)]
            use $crate::TestRunner as _;

            let mut _runner = $crate::GdbRunner::default();
            _runner.setup();

            // Luckily, Rust allows $body to define an inner shadowing main()
            // and call it, without resulting in infinite recursion.
            let _result = { $($body)* };

            _runner.cleanup(_result)
        }
    };
    ( $($body:tt)* ) => {
        $crate::doctest! { @_@ $($body)* }
    };
    ( $($attrs:meta)* $($items:item)* ) => {
        $(attrs)*
        $crate::doctest! { @_@
            $($items)*
            main()
        }
    };
}
