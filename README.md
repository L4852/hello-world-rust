# hello_world_rust

### A collection of practice programs written in Rust ###
https://github.com/L4852/hello-world-rust

## Resource ##
https://doc.rust-lang.org/stable/book/

### Current Concept ###

`Looping using 'loop'`

```rs
loop{
    // Code here
}
```

### Previous Concepts ###

**Parsing a String type to Integer type using .parse()**

```rs
let mut var = "2";

var.trim().parse().expect("");

// Equivalent:

var.trim(); // Removes whitespaces
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

let n = rand::thread_rng().gen_range(a..=b);
```

**User Input**

```rs
io::stdin().read_line(&mut string).expect("");
```

**Variable Assignment**

```rs
let mut a = String::new();
let b = 2;
```
