use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::fs;

fn hello_world() {
    println!("Hello, world!");
}

fn guess_value() {
    let mut user_input: String = String::new();

    let random_num: i32 = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please enter a guess.");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read input!");

        println!("This is your input! -> {user_input}");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(user_input) => user_input,
            Err(_) => continue,
        };

        match user_input.cmp(&random_num) {
            Ordering::Less => println!("Your guess is too low."),
            Ordering::Equal => {
                println!("Your guess is correct!");
                break;
            }
            Ordering::Greater => println!("Your guess is too high."),
        }
        println!("Your guess was incorrect!");
    }
}

fn original_owner() {
    let b_string: String = String::from("= ownership =");

    transfer_ownership(b_string); // Ownership transferred to transfer_ownership function, no longer valid here;

    // println!("Old var (ownership): {b_string}"); Does not work;

    let c_string: String = String::from("= ownership 2 =");

    let d_string: String = transfer_return(c_string);

    // println!("{c_string}"); Does not work; (moved)
    println!("{d_string}"); // Works because value assigned to new returned variable;
}

fn transfer_ownership(string: String) {
    println!("String value: {string}");
}

fn transfer_return(string: String) -> String {
    println!("String value: {string}");

    string
}

fn modify_borrowed(string: &mut String) -> usize {
    let string_length: usize = string.len();
    string.push_str(" -> new text");

    string_length
}

fn original_borrow() {
    let mut a_string: String = String::from("not fixed");
    let length: usize = modify_borrowed(&mut a_string);

    println!("{a_string}, Length: {length}");

    println!("Old variable (borrow): {a_string}");
}

fn enum_test() {
    enum Cookie {
        HasFruit(String),
        HasChocolate
    }

    let my_cookie: Cookie = Cookie::HasFruit(String::from("orange"));

    if let Cookie::HasChocolate = my_cookie {
        println!("Your other cookie is a chocolate cookie.");
    }
    else if let Cookie::HasFruit(fruit) = my_cookie {
        if fruit.as_str() == "Strawberry" {
            println!("Your cookie is a strawberry cookie.");
        }
        else {
            println!("Your cookie is not a strawberry cookie. It is a {} cookie.", fruit);
        }
    }
}

fn file_read_test() {
    let contents: String = fs::read_to_string("src/test.txt").expect("Could not read file.");
    println!("{contents}");
}

fn main() {
    // hello_world();
    // guess_value();
    // original_borrow();
    // original_owner();
    // enum_test();
    // file_read_test();   
}
