//! Macros for working with test runners.

// Use a neat little trick with cfg(doctest) to make code fences appear in
// rustdoc output, but still compile normally when doctesting. This raises warnings
// for invalid code though, so we also silence that lint here.
#[cfg_attr(not(doctest), allow(rustdoc::invalid_rust_codeblocks))]
/// Helper macro for writing doctests using this runner. Wrap this macro around
/// your normal doctest to enable running it with the test runners in this crate.
///
/// You may optionally specify a runner before the test body, and may use any of
/// the various [`fn main()`](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#using--in-doc-tests)
/// signatures allowed by documentation tests.
///
/// # Examples
///
/// ## Basic usage
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// test_runner::doctest! {
///     assert_eq!(2 + 2, 4);
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
/// ## Custom runner
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```no_run
/// test_runner::doctest! { SocketRunner,
///     assert_eq!(2 + 2, 4);
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
/// ## `should_panic`
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```should_panic
/// test_runner::doctest! {
///     assert_eq!(2 + 2, 5);
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
/// ## Custom `fn main`
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// test_runner::doctest! {
///     fn main() {
///         assert_eq!(2 + 2, 4);
///     }
/// }
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// test_runner::doctest! {
///     fn main() -> Result<(), Box<dyn std::error::Error>> {
///         assert_eq!(2 + 2, 4);
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
#[macro_export]
macro_rules! doctest {
    ($runner:ident, fn main() $(-> $ret:ty)? { $($body:tt)* } ) => {
        fn main() $(-> $ret)? {
            $crate::doctest!{ $runner, $($body)* }
        }
    };
    ($runner:ident, $($body:tt)*) => {
        use $crate::TestRunner as _;
        let mut _runner = $crate::$runner::default();
        _runner.setup();
        let _result = { $($body)* };
        _runner.cleanup(_result)
    };
    ($($body:tt)*) => {
        $crate::doctest!{ GdbRunner,
            $($body)*
        }
    };
}
