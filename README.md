# hello-world-rust

### A collection of practice programs written in Rust ###
https://github.com/L4852/hello-world-rust

## Resource ##
https://doc.rust-lang.org/stable/book/

### Current Concepts ###

`Creating and Using Structs and Enums, Using impl`

```rs
struct Demo {
    num: i32,
    string: String,
    points: f32
}

let mut demo_struct: Demo = Demo {
    num: 5,
    string: String::from("Hello World!"),
    points: 8.1
};
    
demo_struct.points = 8.9;

// If params are the same as their struct names, you can use the Field Init shorthand.

fn my_function(num: i32, string: String){
    struct Demo {
        num: i32,
        string: String,
        points: f32
    }

    let test = Demo {
        num,
        string,
        points: 8.1
    };
}
```
```rs
enum Settings {
    Low, // CamelCase
    Medium,
    High
}

let my_setting: Settings = Settings::Medium;
```
```
Reading from Enums:
```
```rs
enum Word {
    FourLetter(char, char, char, char),
    FiveLetter(char, char, char, char, char)
}

let word_one: Word = Word::FourLetter('A', 'B', 'C', 'D');
let word_two: Word = Word::FiveLetter('A', 'B', 'C', 'D', 'E');

if let Word::FourLetter(a, b, c, d) = word_one {
    let chars: [char; 4] = [a, b, c, d];
}
```

`Reading Text from a File`

```rs
use std::fs;
```
```rs
let text: String = fs::read_to_string(filepath).expect("Unable to read file.");
```


`Command Line Arguments`

```rs
use std::env;
```
```rs
let args: Vec<String> = env::args().collect();
// CLI args saved to Vector;
```

`Creating and Modifying Arrays / Tuples, Using Vectors`

```rs
let immutable_array: [i32; 5] = [1, 2, 3, 4, 5];
// [data type; number of items]
// or populate with value (values are not initialized in array by default):

let immutable_array: [0; 10]; // > [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

println!(immutable_array[0]);

let immutable_tuple: (i32, str) = (5, "abc");
```
```
For dynamically resizeable equivalents of arrays ("Lists") use Vectors:
```
```rs
let vector: Vec<i32> = Vec::new();
// or use the macro:
let vector = vec![1, 2, 3];

// Use a mutable vector if you plan on changing the value.

let mut m_vector = Vec::new();

// Push to vector using push();

m_vector.push(2);

// Read value with get() or using index + reference;

let m_vector.get(0); // <- .get(index);
let number: &i32 = &m_vector[0];
```


### Previous Concepts ###

**Ownership in Rust / References**

```rs
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
    string.push_str(" -> extra text");

    string_length
}

fn original_borrow() {
    let mut a_string: String = String::from("= borrowing =");
    let length: usize = modify_borrowed(&mut a_string); 

    println!("{a_string}, length: {length}"); // > = borrowing = -> extra text
}
```

**Catching an 'Err' and 'Ok' Result**

```rs
let val: &str = "NaN";

let number: i32 = match val.trim().parse() {
    Ok(parsed_num) => parsed_num, // Successful result;
    Err(_) => continue // Value to be set to variable if 'Err' result
};
```

**Looping using 'loop'**

```rs
loop {
    // Code here
}
```

**Parsing a String type to Integer type using .parse()**

```rs
let mut var: &str = "2";

var.trim().parse().expect("");

// Equivalent:

var.trim(); // Removes whitespaces and return characters
var.parse(); // Converts string to type specified
var.expect(""); // Handle error

```

**Match Statement, Comparing Values using 'Ordering' type**

```rs
use std::cmp::Ordering;
```
```rs
match a.cmp(&b){
        Ordering::Less => fn(),
        Ordering::Equal => fn(),
        Ordering::Greater => fn()
}
```

**Including Dependencies / Random Number Generation`**

```rs
use rand::Rng;

let n: i32 = rand::thread_rng().gen_range(a..=b);
```

**User Input**

```rs
io::stdin().read_line(&mut string).expect("");
```

**Variable Assignment**

```rs
let mut a: String = String::new();
let b = 2;

let c: str = "ABC";
let d: char = 'A';
let e: f32 = 5.3;
let f: i32 = 10;
```
