use std::ops;

struct MyType {
    id: i32,
}

// Generic function that requires that the type implements the Add-trait.
fn add<T: ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

// Same as above, but the generic bound is expressed in a where clause.
fn multiply<T>(x: T, y: T) -> T
where
    T: ops::Mul<Output = T>
{
    x * y
}

fn main() {
    let x: i32 = add(5, 5);

    println!("{}", x);
}
