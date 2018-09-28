use std::env;

fn help() {
   println!("usage:
match_args <string>
   Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
   Increase or decrease given integer by one.");
}

fn roll(dice: String) -> String{
   dice
}

fn main() {
   let args: Vec<String> = env::args().collect();
   match args.len() {
       // no arguments passed
       1 => {
           println!("My name is ‘match_args’. Try passing some arguments!");
       },
       // one argument passed
       2 => {
           match args[1].as_ref() {
               "d20" => println!("40"),
               _ => println!("This is not the answer."),
           }
       },
       // two argument passed
       3 => {
           match args[1].as_ref() {
               "roll" => println!("{}", roll(args[2].to_string())),
               _ => println!("This is not the answer."),
           }
       },
       // all the other cases
       _ => {
           // show a help message
           help();
       }
   }

}