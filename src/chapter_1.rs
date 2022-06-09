/* COMMON PROGRAMMING CONCEPTS */

// Printing

pub fn print(){
    let x: i32 = 50;
    println!("  Well, hello there
                My, it's been a long, long time
                How am I doin'?
                Oh, well, I guess I'm doin' fine
                It's been so long now and it seems that
                It was only yesterday
                Mmm, ain't it funny how time slips away? ");
    println!("The value of x is: {}", x);
}

// Variables and Mutability

/*  A variable is a value that can change, depending on conditions or on information passed to the program. 
    In rust,variables once declared are immutable by default. */

pub fn run(){
    let mut x = 5;                    // Using mut keyword allows to change that variables's value
    println!("The value of x is: {}", x);
    x = 6;                                 // We can assign new values to the variable
    println!("The value of x is: {}", x);
}

// Constants

// Like immutable variables, constants are values that are bound to a name and are not allowed to change 

pub fn constant(){
    const pi: f64 = 3.14;
    println!("Value of pi is {}", pi);
}