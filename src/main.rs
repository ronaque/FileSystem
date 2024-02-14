use std::io::{self, stdin, stdout};

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    println!("O valor de entrada é: {index}.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("O valor processado é: {index}.");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
