extern crate rand;
use std::env;
use rand::prelude::*;


fn help() {
   println!("TODO: Put help text");
}

fn roll (dice: String) -> Vec<i32>{
   
   let mut rng = thread_rng();

    let split = dice.split("d");
    let vec: Vec<&str> = split.collect::<Vec<&str>>();
    let mut num_rolls: i32 = 1;
    if vec[0] != "" {
        num_rolls = vec[0].parse::<i32>().unwrap();
    }
    let sides = vec[1].parse::<i32>().unwrap();
    let mut rolls = Vec::new();

    for _ in 0..num_rolls {
        rolls.push(rng.gen_range(1, sides+1));
    }
    rolls

}

fn print_roll(dice: String) {
    let rolls: Vec<i32> = roll(dice);
    for roll in rolls {
        print!("{}", roll.to_string()+" ");
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
       // no arguments passed
        1 => {
            println!("Try passing two arguments!");
        },
        // one argument passed
        2 => {
            println!("Try passing two arguments!");
            // match args[1].as_ref() {
            //     "d20" => println!("40"),
            //     _ => println!("Error"),
            // }
        },
        // two argument passed
        3 => {
            match args[1].as_ref() {
                "roll" => print_roll(args[2].to_string()),
                _ => println!("Error"),
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}