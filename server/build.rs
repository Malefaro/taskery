use std::fs::{File};


fn main() -> Result<(), std::io::Error>{
    // here we can create schema file(for graphql-client) from query.


    // let f= File::create("./test.txt")?; // Thats works!

    println!("Before exec"); // prints does not work
    Ok(())
}