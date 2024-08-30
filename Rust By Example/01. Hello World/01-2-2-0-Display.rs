// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a stucture for which `fmt::Display` will be implemented. 
//This is a tuple struct named `Structure` that contians an `i32`.
struct Structure(i32);

/*
This code defines a way to customize how a Structure is formatted when using the println! macro or similar formatting macros in Rust. It does this by implementing the fmt::Display trait for the Structure type.
*/


// To use the `{}` marker, the trait `fmt::Display` must be implemented manually for the type.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: `f`.
        // Return `fmt::Result` which indicates whether the operation succeeded or failed. Note that `write!` uses syntax which is very similar to println!.
        //The write! macro is similar to println!, but instead of printing to the console, it writes the formatted string to the specified output stream (f).
        write!(f, "{}", self.0) 
    }
}

// A structure holding two numbers. `Debug` will be derived so the results  be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positonal data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are mameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similaryly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("This big range is {big} and the smalll is {small}",
                small = small_range,
                big = big_range);
    
    let point = Point2D { x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

/*
### Macros in Rust

Macros in Rust are a way to write code that writes other code (often referred to as **metaprogramming**). Macros allow you to define patterns that are expanded into code at compile time, which can be useful for reducing boilerplate, creating domain-specific languages, or performing complex operations that are cumbersome to write out manually.

Rust has several types of macros:

1. **Declarative Macros** (also known as "macro_rules!"):
   - These are the most common type of macros in Rust. They allow you to match patterns and generate code based on those patterns.
   - Example:
     ```rust
     macro_rules! my_macro {
         ($val:expr) => {
             println!("The value is: {}", $val);
         };
     }

     my_macro!(42); // Expands to println!("The value is: {}", 42);
     ```

2. **Procedural Macros**:
   - These macros are more powerful and complex, as they allow you to manipulate Rust code directly. Procedural macros can generate code from input tokens.
   - There are three kinds of procedural macros:
     - **Function-like macros**: Similar to functions but are invoked as macros.
     - **Derive macros**: Automatically implement traits for types.
     - **Attribute macros**: Allow you to modify the behavior of items (e.g., functions, structs).
   - Example (derive macro):
     ```rust
     #[derive(Debug)]
     struct MyStruct {
         value: i32,
     }
     ```
     This `derive` macro generates an implementation of the `Debug` trait for `MyStruct`.

### Keywords and Concepts in the Code Snippet

Let's break down each part of the Rust code snippet you provided:

```rust
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

1. **`impl`**:
   - `impl` stands for "implementation" and is used to implement traits or methods for a specific type.
   - In this context, `impl fmt::Display for Structure` means that we are implementing the `fmt::Display` trait for the `Structure` type.

2. **`fmt::Display`**:
   - `fmt::Display` is a trait in the `std::fmt` module that dictates how a type should be formatted when using the `{}` placeholder in Rust's formatting macros (like `println!`).
   - Implementing this trait allows instances of the type to be printed in a user-friendly way.

3. **`for Structure`**:
   - This specifies that the implementation is for the `Structure` type. In this case, `Structure` is a user-defined type (e.g., a struct or enum).

4. **`fn`**:
   - `fn` is the keyword used to define a function in Rust.
   - In this context, `fn fmt` is defining the `fmt` function, which is required by the `fmt::Display` trait.

5. **`&self`**:
   - `self` represents the instance of the type that the method is being called on.
   - The `&` indicates that this is a reference to `self`, meaning the method borrows the instance rather than taking ownership of it.

6. **`f: &mut fmt::Formatter`**:
   - `f` is a parameter of the `fmt` function. It's a mutable reference (`&mut`) to a `fmt::Formatter`.
   - `fmt::Formatter` is a struct that contains information about how to format the output (e.g., alignment, width, precision).
   - The `&mut` keyword indicates that `f` is mutable, meaning the function can modify the `Formatter`.

7. **`fmt::Result`**:
   - `fmt::Result` is a type alias for `Result<(), fmt::Error>`.
   - It is the return type of the `fmt` function and indicates whether the formatting was successful (`Ok(())`) or if an error occurred (`Err(fmt::Error)`).

8. **`write!`**:
   - `write!` is a macro that writes formatted data to a given output stream (`f` in this case).
   - The syntax is similar to `println!`, but instead of printing to the console, it writes the output to the stream provided.
   - In this context, `write!(f, "{}", self.0)` writes the first field of `Structure` (accessed as `self.0`) to the output stream.

9. **`self.0`**:
   - `self.0` refers to the first field of the tuple struct `Structure`. If `Structure` were defined as `struct Structure(T);`, `self.0` would refer to that field.
   - It is a shorthand for accessing fields in a tuple struct.

### Summary:

- **Macros**: In Rust, macros are powerful tools for metaprogramming, allowing you to write code that generates other code at compile time.
- **Keywords**: The code snippet provided shows how to implement the `fmt::Display` trait for a custom type (`Structure`), allowing instances of that type to be printed in a formatted way using Rust's formatting macros. The keywords `impl`, `fn`, `self`, `mut`, `Result`, and others play crucial roles in defining the behavior of this trait implementation.
*/