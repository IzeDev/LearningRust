mod ownership_and_rules {
    // This function takes ownership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // `c` is destroyed and the memory freed
    }

    pub fn main() {
        // _Stack_ allocated integer
        let x = 5u32;

        // *Copy* `x` into `y` - no resources are moved
        let y = x;

        // Both values can be independently used
        println!("x is {}, and y is {}", x, y);

        // `a` is a pointer to a _heap_ allocated integer
        let a = Box::new(5i32);

        println!("a contains: {}", a);

        // *Move* `a` into `b`
        let b = a;
        // The pointer address of `a` is copied (not the data) into `b`.
        // Both are now pointers to the same heap allocated data, but
        // `b` now owns it.

        // Error! `a` can no longer access the data, because it no longer owns the
        // heap memory
        //println!("a contains: {}", a);
        // TODO ^ Try uncommenting this line

        // This function takes ownership of the heap allocated memory from `b`
        destroy_box(b);

        // Since the heap memory has been freed at this point, this action would
        // result in dereferencing freed memory, but it's forbidden by the compiler
        // Error! Same reason as the previous Error
        //println!("b contains: {}", b);
        // TODO ^ Try uncommenting this line
    }
}

mod partial_moves {
    pub fn main() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        let person = Person {
            name: String::from("Alice"),
            age: 20,
        };
        // `name` is moved out of person, but `age` is referenced
        let Person { name, ref age } = person;
        println!("The person's age is {}", age);
        println!("The person's name is {}", name);
        // Error! borrow of partially moved value: `person` partial move occurs
        //println!("The person struct is {:?}", person);
        // `person` cannot be used but `person.age` can be used as it is not moved
        println!("The person's age from person struct is {}", person.age);
    }
}

mod borrowing {
    // This function takes ownership of a box and destroys it
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // This function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    pub fn main() {
        // Create a boxed i32, and a stacked i32
        let boxed_i32 = Box::new(5_i32);
        let stacked_i32 = 6_i32;

        // Borrow the contents of the box. Ownership is not taken,
        // so the contents can be borrowed again.
        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            // Take a reference to the data contained inside the box
            let _ref_to_i32: &i32 = &boxed_i32;

            // Error!
            // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
            // eat_box_i32(boxed_i32);
            // FIXME ^ Comment out this line

            // Attempt to borrow `_ref_to_i32` after inner value is destroyed
            borrow_i32(_ref_to_i32);
            // `_ref_to_i32` goes out of scope and is no longer borrowed.
            println!("");
        }

        // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
        eat_box_i32(boxed_i32);
    }
}

fn main() {
    ownership_and_rules::main();
    partial_moves::main();
    borrowing::main();
    println!("Hello, world!");
}
