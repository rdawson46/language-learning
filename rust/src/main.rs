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
    
    let v = calculate_len(&s2); // reference to the s2
    
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
// fn change(some_string: &String){ // throws an error
    fn change(some_string: &mut String){
        some_string.push_str(", world");
        println!("{some_string}");
    }
    fn main(){
        // let s = String::from("Hello");
        
        // change(&s); // throws an error because the function is borrowing s without permission to edit
        let mut s2 = String::from("hello");
        
        change(&mut s2);
    println!("{s2}"); // prints the same value from the change function because it mutates the original

    // RULE a value cannot be borrowed as mutable more than once at a time
    // prevents data race
}
*/

/*
The Slice Type
- lets you refer to a contiguous sequence of elements in a collection rather than the whole collection
fn first_word(s: &String)->&str{
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    
    &s[..]
}

fn first_word_literl(s: &str)->&str{
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    
    &s[..]
}
fn main(){
    // string slices
    let s = String::from("hello, world");
    
    // let hello = &s[0..5]; // same as &s[..5]; 
    // let world = &s[6..11]; // same as &s[6..]; can drop both values for &s[..];
    
    let word = first_word(&s);
    
    println!("{word}");
    
    // string literal as slices
}
*/

/*
========== USING STRUCTS TO STRUCTURE RELATED DATA ==========
// define a struct
struct User{
    active:bool,
    username:String,
    email: String,
    sign_in_count: u64,
}  

fn build_user(email:String, username:String)->User{
    // User { active: true, username: username, email: email, sign_in_count: 1 }
    // shorthand notation
    User { active: true, username, email, sign_in_count: 1 }
}

// using tuple structs without named fields to create different types
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// unit structs
struct AlwaysEqual;

fn main(){
    
    // create an instance
    let mut user1 = User{active:true, username: String::from("some_user"), email: String::from("some@email.com"), sign_in_count: 1}; // when mutable, all properties are mutable
    
    user1.email = String::from("another@email.com");
    
    // create another instance from other instances
    let user2 = User{
        email:String::from("another@email.com"),
        ..user1
    };
    
    // user1 can no longer be used here because the username String was moved to user2
    // if user2 had a new String for username and email then user1 would still be usable
    
    // tuples structs
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // unit struct
    let subject = AlwaysEqual;
}
*/

/*
Example Program using structs
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}

fn area(rectangle:&Rectangle)->u32{
    rectangle.width * rectangle.height
}

fn main(){
    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectanlge is {} square pixels",
        area(&rect)
    );
    
    // to print a struct replace {} with {:?} or {:#?}
    // and add #[derive(Debug)] to rectangle
    
    println!("rect is {:?}", rect);
}
*/

/*
Method Syntax

// define struct
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// call this with methods inside to associate with the type
// types can have multiple impl blocks
impl Rectangle{
    
    // all functions in impl are called associated functions
    fn area(&self) -> u32{
        self.width * self.height
    }
    
    // more params
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
    
    // none method associated function
    fn square(size: u32)->Self{
        Self { width: size, height: size }
    }
}

fn main(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle{
        width:20,
        height: 40,
    };

    // calling an associated method
    let sq = Rectangle::square(3);
    
    println!(
        "The area of rect1 is {}",
        rect1.area()
    );
    
    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );
}
*/


/*
========== ENUMS AND PATTERN MATCHING ==========

Defining an Enum

able to create enum methods with impl

enum IpAddrKind{
    V4,
    V6,
}

// function using enum type
// fn route(ip_kind: IpAddrKind){}

// struct IpAddr{
    //     kind: IpAddrKind,
    //     address: String,
    // }
    
    // or
    
    enum IpAddr{
        // defining associated value
        V4(String),
        V6(String),
    }
    
    fn main(){
        // user enums with
        // let four = IpAddrKind::V4;
        // let six = IpAddrKind::V6;
        
        // let home = IpAddr{
            //     kind: IpAddrKind::V4,
            //     address: String::from("127.0.0.1"),
            // };
            
            // let loopback = IpAddr{
                //     kind: IpAddrKind::V6,
                //     address: String::from("::1"),
    // };
    
    // or
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    // OPTION Enum
    // replacement of null
    
    /*
    Defined as 
    enum Option<T>{
        None,
        Some(T),
    }
    
    T is an generic type of parameter
     */

    let some_number = Some(5); // need to convert Option<T> to T before doing T operations
    let some_char = Some('c');
    
    let absent_number: Option<i32> = None; // need to declare type for None
}

// MATCH CONTROL FLOW

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // so on
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cnets(coin: Coin)-> u8{
    // match can return functions
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=> {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main(){
    value_in_cnets(Coin::Quarter(UsState::Alabama));
}


// MATCHING WITH OPTION<T>
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None=> None,  // must include None match or else run create a compile error
        Some(i)=> Some(i+1),
    }
}

// CONCISE CONTROL FLOW WITH IF LET
fn main(){
    let config_max = Some(3u8);
    
    match config_max{
        Some(max)=> println!("The maximum is configured to be {}", max),
        _ => (), // way of returning ignoring None value
    }
    
    // or
    
    if let Some(max) = config_max{
        println!("The maxiumu is configured to be {}", max);
    } else{
        // else is usable if patterns don't match
    }
}
*/

/*
CHAPTER 7
MANAGING GROWING PROJECTS WITH PACKAGES, CRATES, AND MODULES
*/