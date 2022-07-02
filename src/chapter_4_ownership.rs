// OWNERSHIP

pub fn ownership(){
    /*               ------  Rules of ownership  ------
        1. Each value in Rust language has a variable that's called its owner.
        2. There can only be one owner at a time
        3. When the owner gets out of scope the value gets dropped.
    */

    { // s is not valid here, it's not declared yet.
        let s = String::from("Hello");  // s is valid from this point forward.
        // Do stuff with s
    } // This scope is now over, and s is no longer valid.
}

pub fn run(){
    let x = 5;
    let y = x; // Value of x is copied in y
    println!("x is: {} & y is: {}",x,y);

    let s1 = String::from("hello");
    let s2 = s1;
 // println!("s1 is: {}",s1);     -----> This statement produces error value of s1 is already moved to s2
    println!("s2 is: {}",s2);
}

pub fn example(){
    let s = String::from("Aditya");
    takes_ownership(s);
 // println!("s is :{}",s);       ------> This statement produces error as s is already moved to takes_ownership() function
}
fn takes_ownership(some_string: String){
    println!("{}",some_string);
}
