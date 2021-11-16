#[allow(dead_code)]
fn structs() {
    /*
        There are three types of structures that can be created using the struct keyword:
            1. Tuple structs, which are, basically, named tuples.
            2. The classic C structs
            3. Unit structs, which are field-less, are useful for generics.
    */
    // A tuple struct
    struct Pair(i32, f32);
    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }
    // A unit struct
    struct Unit;

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let left_edge = point.x;
    let top_edge = point.y;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn enums() {
    enum StateMessage {
        Success,
        Warning(String),
        Error {
            error_code: i64,
            error_message: String,
        },
    }

    // The use declaration can be used so manual scoping isn't needed
    use StateMessage::{Error, Success, Warning};

    fn get_state(state_message: StateMessage) -> String {
        return match state_message {
            Success => String::from("Success!"),
            Warning(e) => format!("Warning...{}", e),
            Error {
                error_code,
                error_message,
            } => {
                format!("Error {0}! Msg:{1}", error_code, error_message)
            }
        };
    }

    let s = get_state(Success);
    let w = get_state(Warning(String::from("Weird code!")));
    let e = get_state(Error {
        error_code: 500,
        error_message: String::from("Server said: \"Kaboom!\"!"),
    });
    println!("{}", s);
    println!("{}", w);
    println!("{}", e);

    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    //  common use for enums is to create a linked-list
    enum List {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }

    use List::*;

    // Methods can be attached to an enum
    impl List {
        // Create an empty List
        fn new() -> List {
            //'Nil' has type 'List'
            Nil
        }

        // Consume a list, and return the same list with a new element at its front
        fn prepend(self, elem: u32) -> List {
            // `Cons` also has type List
            Cons(elem, Box::new(self))
        }

        // Return the length of the list
        fn len(&self) -> u32 {
            // `self` has to be matched, because the behavior of this method
            // depends on the variant of `self`
            // `self` has type `&List`, and `*self` has type `List`, matching on a
            // concrete type `T` is preferred over a match on a reference `&T`
            // after Rust 2018 you can use self here and tail (with no ref) below as well,
            // rust will infer &s and ref tail.
            // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
            match *self {
                // Can't take ownership of the tail, because `self` is borrowed;
                // instead take a reference to the tail
                Cons(_, ref tail) => 1 + tail.len(),
                // Base Case: An empty list has zero length
                Nil => 0,
            }
        }

        // Return representation of the list as a (heap allocated) string
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // `format!` is similar to `print!`, but returns a heap
                    // allocated string instead of printing to the console
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn constants() {
    /*
        Rust has two different types of constants which can be declared in any scope
        including global. Both require explicit type annotation:println!
            -   const: An unchangeable value (the common case).
            -   static: A possibly mutable variable with 'static lifetime. The static
                lifetime is inferred and does not have to be specified. Accessing or
                modifying a mutable static variable is unsafe.
    */
    
    // Globals are declared outside all other scopes.
    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }    

    let n = 16;
    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

fn main() {
    structs();
    enums();
    constants();
}
