/// The `HelloMacro` trait defines a single function `hello_macro` which
/// prints a greeting message including the name of the type implementing it.
///
/// # Example
///
/// ```rust
/// struct Pancakes;
///
/// impl HelloMacro for Pancakes {
///     fn hello_macro() {
///         println!("Hello, Macro! My name is Pancakes!");
///     }
/// }
///
/// Pancakes::hello_macro();
/// ```
pub trait HelloMacro {
    fn hello_macro();
}
