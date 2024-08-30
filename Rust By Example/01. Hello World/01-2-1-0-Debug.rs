// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);



// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
struct Deep(Structure);


fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    println!("[WITH {{0:?}}] \t\t {1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
            actor="actor's");

    println!("[WITH {{}}] \t\t {1} {0} is the {actor} name.",
            "Slater",
            "Christian",
           actor="actor's");

    // 'Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I wnat this to just show a '7'?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // Error if we try
    // println!("Now {} will print!", Deep(Structure(7)));

}