use proc_macro::TokenStream;
use quote::quote;

/// The `HelloMacro` derive macro. This macro generates an implementation
/// of the `HelloMacro` trait for the annotated struct or enum.
///
/// # Example
///
/// ```rust
/// use hello_macro::HelloMacro;
///
/// #[derive(HelloMacro)]
/// struct Pancakes;
///
/// Pancakes::hello_macro(); // Prints: "Hello, Macro! My name is Pancakes!"
/// ```
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

/// Generates the implementation of the `HelloMacro` trait for the given
/// struct or enum.
///
/// # Arguments
///
/// * `ast` - The abstract syntax tree of the input code.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
