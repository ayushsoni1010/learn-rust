// Variables hold primitve data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ayush";
    let mut age = 22;

    println!("My name is {} and I am {}", name, age);

    age = age + 1;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 123;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ayush",22);
    println!("{} is {}", my_name, my_age);
}
