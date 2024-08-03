# HelloMacro

`HelloMacro` is a Rust procedural macro that automatically implements the `HelloMacro` trait for any struct or enum. This trait includes a single method, `hello_macro`, which prints a greeting message including the name of the type.

## Usage

To use the `HelloMacro` derive macro, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
hello_macro = "0.1.0"
```

Add the following to your crate root:

```
extern crate bogdan_hello_macro;
use bogdan_hello_macro::HelloMacro;
```

Then, you can use the HelloMacro derive macro on any struct or enum:

```
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro(); // Prints: "Hello, Macro! My name is Pancakes!"
}
```

##Example

Here’s a complete example demonstrating how to use the HelloMacro derive macro:
use hello_macro::HelloMacro;

```
#[derive(HelloMacro)]
struct Waffles;

#[derive(HelloMacro)]
enum Breakfast {
    Eggs,
    Bacon,
}

fn main() {
    Waffles::hello_macro(); // Prints: "Hello, Macro! My name is Waffles!"
    Breakfast::Eggs.hello_macro(); // Prints: "Hello, Macro! My name is Eggs!"
    Breakfast::Bacon.hello_macro(); // Prints: "Hello, Macro! My name is Bacon!"
}
```
