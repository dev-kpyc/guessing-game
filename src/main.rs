extern crate multimap;
extern crate rand;

use multimap::MultiMap;
use rand::Rng;
use std::io;

fn main() {

    let fruits = ["Apple", "Orange", "Banana"];

    let mut fruit_properties = MultiMap::new();
    fruit_properties.insert("Apple", "Red");
    fruit_properties.insert("Apple", "Round");
    fruit_properties.insert("Orange", "Round");
    fruit_properties.insert("Orange", "Orange");

    let random_fruit = rand::thread_rng().choose(&fruits).unwrap();

    match fruit_properties.get_vec(random_fruit) {
        Some(properties) => {
            for property in properties {
                println!("It's {}", property);
            }
        },
        None => println!("I can't describe this item.")
    }
    println!("What am I thinking of?");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    if &guess.trim() == random_fruit {
       println!("You guessed it!"); 
    }

    //println!("I picked {}", random_fruit);
}
