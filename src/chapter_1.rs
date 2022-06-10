/* COMMON PROGRAMMING CONCEPTS */

/* PRINTING */
pub fn print(){
    println!("  
                Extreme ways are back again
                Extreme places I didn't know
                I broke everything new again
                Everything that I'd owned
                I threw it out the windows, came along
                Extreme ways I know move apart The colors of my sea
                Perfect color me"  );
    let x: i32 = 50;
    println!("The value of x is: {}", x);
}

/* VARIABLES
   A variable is a value that can change, depending on conditions or on information passed to the program. 
   In rust,variables once declared are immutable by default. */
pub fn run(){
    let mut x = 5;                    // Using mut keyword allows to change that variables's value
    println!("The value of x is: {}", x);
    x = 6;                                 // We can assign new values to the variable
    println!("The value of x is: {}", x);
}

/* CONSTANT
   Like immutable variables, constants are values that are bound to a name and are not allowed to change. */
pub fn constant(){
    const PI: f64 = 3.14;
    println!("Value of pi is {}", PI);
}

/* SHADOWING
   We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows. */
pub fn shadowing(){
    let x: i32 = 5;
    let x: i32 = x +1 ;
    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}