use std::ops::Add;

struct Person {
    name: String,
    age: i32
}

fn copy_and_add_five(n : i32) -> i32 {
    n + 5
}

fn copy_and_add_five_with_decimal(n : f32) -> f32 {
    n + 5f32
}

fn copy_and_shout(s: String) -> String {
    s.add("!")
}

fn mutate_add_five (n: i32) -> i32 {
    n + 5
}

fn age_one_year(person: &mut Person) -> &mut Person{
    person.age = 30;
    person
}

fn main() {
    let my_number = 0;
    let my_other_number = copy_and_add_five(my_number);

    let my_number = 0f32;
    let my_other_number = copy_and_add_five_with_decimal(my_number);

    let hello = String::from("hello");
    let hello_angry = copy_and_shout(hello);

    let mut my_num = 0;
    my_num = mutate_add_five(my_num);

    let mut me = Person{name: String::from("Jimmy"), age: 29};
    let other_me = age_one_year(&mut me);
    other_me.age = 31;



    println!("Hello, world!");
}