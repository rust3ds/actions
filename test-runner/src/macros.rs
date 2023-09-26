//! Macros for working with test runners.

// We use a little trick with cfg(doctest) to make code fences appear in
// rustdoc output, but compile without them when doctesting. This raises warnings
// for invalid code, though, so silence that lint here.
#[cfg_attr(not(doctest), allow(rustdoc::invalid_rust_codeblocks))]
/// Helper macro for writing doctests using this runner. Call this macro at the
/// beginning of a doctest enables output from failing tests using this crate's
/// [`GdbRunner`](crate::GdbRunner). Without `setup_doctest!()`, doctests will
/// still fail on panic, but they won't display anything written to `stdout` or
/// `stderr`.
///
/// # Examples
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// test_runner::setup_doctest!();
/// assert_eq!(2 + 2, 4);
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```should_panic
/// test_runner::setup_doctest!();
/// assert_eq!(2 + 2, 5);
/// ```
#[cfg_attr(not(doctest), doc = "````")]
#[macro_export]
macro_rules! setup_doctest {
    () => {
        use $crate::TestRunner as _;

        let mut runner = $crate::GdbRunner::default();
        runner.setup();

        // We don't bother with cleanup here, since the macro is meant to be used
        // in a doctest context (i.e. `fn main()`, not used as a test runner)
    };
}
