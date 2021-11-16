#![allow(dead_code)]
#![allow(unused_variables)]

fn primitives() {
    println!("Primitives!");
    
    // Variables can be type annotated.
    let logical: bool = true;
    
    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    
    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut _inferred_type = 12; // Type i64 is inferred from another line
    _inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let _mutable = true;
}

fn tuples() {
    let mut t1 = ("Jimmy", 29);
    println!("{0} is {1} years old.", t1.0, t1.1);

    t1.0 = "Adam";
    println!("{0} is {1} years old.", t1.0, t1.1);

    t1.0 = "Elise";
    t1.1 = 28;
    let (name, age) = t1;
    println!("{0} is {1} years old.", name, age);
}

fn arrays() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());
}

fn main() {
    primitives();
    tuples();
    arrays();
}