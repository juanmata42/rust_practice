#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

/* use rand::Rng; */
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked,
}

pub fn combination_lock() {
    let combination = String::from("9562");
    let mut state = State::Locked;
    let mut entry = String::new();
    /*this works too, the match is after the if logic
    loop {
         entry = String::new();
         println!("Enter your combination: ");
         stdin().read_line(&mut entry).expect("Failed to read line");
         entry = entry.trim().to_string();
         if entry == combination {
             state = State::Unlocked;
         } else {
             state = State::Failed;
         }
         match state {
             State::Locked => println!("Locked!"),
             State::Failed => println!("Failed!"),
             State::Unlocked => {println!("Unlocked!"); break;},
         }
     } */
    //here the if is inside the locked loop. i like more the commented one
    loop {
        match state {
            State::Locked => {
                entry = String::new();
                println!("Enter your combination: ");
                stdin().read_line(&mut entry).expect("Failed to read line");
                entry = entry.trim().to_string();
                if entry == combination {
                    state = State::Unlocked;
                    continue;
                } else {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Try again!");
                state = State::Locked;
            }
            State::Unlocked => {
                println!("Unlocked!");
                break;
            }
        }
    }
}
