use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn hello_world() {
    println!("Hello, world!");
}

fn guess_value() {
    let mut user_input = String::new();

    let random_num = rand::thread_rng().gen_range(1..100);

    println!("Please enter a guess.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read input!");

    println!("This is your input! -> {user_input}");

    let user_input: i32 = user_input.trim().parse().expect("Could not parse input.");

    match user_input.cmp(&random_num){
        Ordering::Less => println!("Your guess is too low."),
        Ordering::Equal => println!("Your guess is correct!"),
        Ordering::Greater => println!("Your guess is too high.")
    }
    

}

fn main() {
    // hello_world();
    guess_value();    
}
