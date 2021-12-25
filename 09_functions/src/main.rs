fn methods() {
    struct Bomb {
        id: String,
    }
    impl Bomb {
        fn new(id: String) -> Bomb {
            Bomb { id: id }
        }
        // Takes a reference to the Bomb, borrowing ownership for the scope.
        fn copy(&self) -> Bomb {
            Bomb {
                id: self.id.to_string() + "-2",
            }
        }
        fn change_id(&mut self, id: String) {
            self.id = id;
        }
        // Moves the struct into this scope and "destroys" it once it goes out of scope.
        fn explode(self) {
            println!("The bomb {} went... booom!", self.id);
        }
    }
    let mut b_1 = Bomb::new(String::from("1337"));
    let b_2 = &b_1.copy();
    b_1.change_id(String::from("1338"));

    println!("b_1 ID: {}", b_1.id);
    println!("b_2 ID: {}", b_2.id);
    b_1.explode();
    // b_1.explode(); CAN'T EXPLODE TWICE !
}

fn closures() {
    // Closures are functions that can capture the enclosing environment.
    // Anonymous function
    fn function(i: i32) -> i32 {
        i + 1
    }
    let x = function(0);
    let increment = |i: i32| -> i32 { i + 1 };
    let increment_inferred = |i| i + 1;
    let add = |x, y| x + y;

    let y = increment(x);
    let z = increment_inferred(y);

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    fn apply<F, X>(f: F, x: X) -> X
    where
        F: Fn(X) -> X,
    {
        f(x)
    }

    let q = apply(increment, 12);
    println!("{}", q);
    let my_str = add("abc".to_owned(), "def");

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }
    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }
    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

fn high_order_functions() {
    // Higher order functions
    let add = |x: i32,y: i32|x+y;

    let x =
        (0..100).map(|n|n * n)
            .fold(0, |acc, ele| add(acc, ele));
    println!("{}", x)
}

fn main() {
    methods();
    closures();
    high_order_functions();  
}
