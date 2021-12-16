fn main() {
    let my_number = 15;
    let mut my_mutable_number = my_number;
    let my_unbound_variable;
    my_mutable_number = my_mutable_number + 1;

    println!("My number is {0} and my other number is {1}", my_number, my_mutable_number);

    // A block, creating its own scope
    {
        let my_number = 18; // Only viable in this scope.
        my_mutable_number = my_number; // Will presist outside of this scope.
        my_unbound_variable = 13; // Will presist outside of this scope.
        
        println!("My number is {0} and my other number is {1}", my_number, my_mutable_number);
    }
    println!("My number is {0} and my other number is {1}", my_number, my_mutable_number);
    println!("My secret number is {0}.", my_unbound_variable);
}