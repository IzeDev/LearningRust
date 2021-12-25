mod domain;
use domain::invoice as invoice;

mod operations {
    fn private_function(x :i32, y: i32) -> i32 {
        if y == 0 {
            0
        } else {
            x / y
        }
    }

    pub fn divide(x :i32, y: i32) -> i32 {
        private_function(x, y)
    }

    pub struct api_key {
        pub external_key: String, // Accessible outside the module due to the struct being public.
        internal_key: String // Only accessible inside the module
    }
}

fn main() {
    let x = operations::divide(35, 2);
    println!("X is: {}", x);

    let h = invoice::Head::new(10000, "10 000 dollars!");

    println!("X is: {}", x);
}
