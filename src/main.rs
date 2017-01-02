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
    fruit_properties.insert("Apple", "Sweet");
    fruit_properties.insert("Orange", "Round");
    fruit_properties.insert("Orange", "Orange");

    let mut rng = rand::thread_rng();

    let random_fruit = rng.choose(&fruits).unwrap();
    let mut remaining_hints;
    if let Some(properties) = fruit_properties.get_vec(random_fruit) {
        remaining_hints = properties.clone();
        rng.shuffle(&mut remaining_hints);
    } else {
        remaining_hints = Vec::new();
    }

    println!("What fruit am I thinking of?");
    println!("Type \"Hint\" for a hint!");
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if &guess.trim() == random_fruit {
            println!("You guessed it!");
            break;
        } else if guess.trim() == "Hint" {
            if let Some(hint) = remaining_hints.pop() {
                println!("It's {}", hint);
            }
            else {
                println!("I have no more hints to give you!");
            }
        } else {
            println!("Nope! Guess again.");
        }
    }
    //println!("I picked {}", random_fruit);
}
