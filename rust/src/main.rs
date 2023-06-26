/*
Guessing game example

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // 1..=100 defines a range from start..=finish
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // string placholder of a var
    println!("the secret number is {secret_num}");

    // example of a loop
    loop{
        println!("Please input your guess.");
    
        // mut keyword shows that this variable is mutable
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        // converting guess to an i32
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_num){
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
*/

/* 
example of variables and scope
fn main(){
    let x = 5;

    println!("The value of x is: {x}"); // prints 5

    let x = x + 1; // allows redeclarations even if not mut

    println!("The value of x is: {x}"); // prints 6

    // scope definition
    {
        let x = x * 2; // get x from outer scope and makes copy

        println!("The value of x is: {x}"); // prints 12
    }

    println!("the final value is: {x}"); // prints 6
}
*/

/*
data types example
fn main(){
    // tuples 
    let tup: (i32, f64, u8) = (5000, 6.4, 1);
    
    let (x, y, z) = tup; // assigns the values from tup
    
    let five_thousand = tup.0; // example of tuple indexing
    
    // lists
    // stored on stack
    // fixed size
    let a = [1,2,3,4,5]; // without type declation
    let b: [i32; 5] = [1,2,3,4,5]; // with type declaration
    
    let same_value = [3; 5]; // results in [3,3,3,3,3]
    
    let val = same_value[1]; // example of list indexing
    
    // includes memory safe indexing
    
}
*/

/*
Functions

fn main(){
    println!("Hello world");
    
    another_function(5);
    
    // expression sample
    let y = {
        let x = 3;
        x + 1 // no semicolon or it'll return a compile error 
    };
    
    println!("y: {y}");
    
    let val = five();
    
    println!("Five: {val}");
}

// function parameters
fn another_function(x:i32){
    println!("The value pass is {x}");
}

// function with return values
fn five()->i32{
    5 // no return keyword
    
    // or
    // return 5;
}
*/


/*
 Control flow
 fn main(){
     let number = 3;

     if number < 5{
         println!("Condition was true");
        } else if number > 7{
            println!("number was greater than seven");
        } else{
            println!("Condition was false");
        }
        
        // something like - if number{} will throw an error because conditional must be a boolean
        
        // using if in a declartion
        let condition = true;
        let val = if condition {5} else {6};
        // let val = if condition {5} else {"six"}; will throw an error 

        println!("Val: {val}");
        
        // 3 different types of loops
        // loop{} will repeat forever until explicitly stopped
        // can be used to return values
        
        let mut counter = 0;
        let result = loop{
            counter += 1;
            
            if counter == 10{
                break counter * 2;
            }
        };
        
        println!("Counter: {result}");
        
        // loop labels
        let mut count = 0;
        
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        
        loop{
            println!("Remaining = {remaining}");
            
            if remaining == 9{
                break;
            }
            
            if count == 2{
                break 'counting_up; // uses label to break outer loop
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    println!("End count = {count}");
    
    
    // while loops
    let mut number = 3;
    
    while number != 0{
        println!("{number}");

        number -= 1;
    }
    
    println!("LIFTOFF!!");
    
    // supports for loops directly
    let a = [10,20,30,40,50];
    
    for element in a{
        println!("the value is: {element}");
    }

    // or
    
    // ranges can be made by start..end (doesn't include end) or start..=end (does include end)
    for sam in (1..4).rev(){
        println!("Range: {sam}");
    }
    
}
*/

/*
Understanding Ownership
    uses ownership to guarantee memory safety without needing a garbage collector
        - set of rules that govern how rust manages memory
        - managed through a system of ownership with a set of rules that the compiler checks
        - if rules are violated, the program won't compile
    
    rules:
        - each value in Rust has an owner
        - there can obly be one owner at a time
        - when the owner goes out of scope, the value will be dropped
    
        
        fn main(){
            // let _a = "hello"; // stored on stack, immutable
            // let mut s = String::from("hello"); // stored on heap, can be mutable
            
            // s.push_str(" world");
            
            // println!("{s}");
    // since stored in the heap, memory is requested at runtime and size is unknown
    
    // let s1 = String::from("sample");
    // let s2 = s1; compiler will not like this
    // instead
    // let s2 = s1.clone();
    
    let s = String::from("hello");

    takes_ownership(s); // s's value moves into the function... and is no longer valid here
    
    let x = 5;
    
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's ok to still use after
    
    let s1 = String::from("hello");
    let s2 = takes_and_gives(s1);
    
    println!("{s2}");
    
    let v = calculate_len(&s2);
    
    println!("The size of s2: {v}");
    
    // let v = calculate_len(&s1);
}
// s is out of scope here and and rust calls drop on the memory used for storing s

fn takes_ownership(some_string:String){
    println!("{some_string}");
}  // memory is dropped after this brace

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}

fn takes_and_gives(some_string:String)->String{
    some_string
}

// this function takes a reference with &String passed instead of String
fn calculate_len(s: &String)->usize{
    s.len()
}
*/

/*
References and Borrowing
 */
fn main(){

}