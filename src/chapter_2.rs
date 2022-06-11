// DATA TYPES

/* SCALAR TYPES
   A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 
   You may recognize these from other programming languages. Let’s jump into how they work in Rust.  */
pub fn scalartypes(){
        println!("          
                               INTEGER TYPES
                        LENGTH	     SIGNED	  UNSIGNED
                         8-bit        i8             u8
                         16-bit	      i16	     u16
                         32-bit	      i32	     u32
                         64-bit	      i64	     u64
                         128-bit      i128	     u128
                         arch	      isize	     usize       ");            
        println!("           
                               INTEGER LITERALS
                        NUMBER_LITERALS	   EXAMPLE
                          Decimal	    98_22
                          Hex	            0xff
                          Octal	            0o77
                          Binary	    0b1111_0000
                          Byte(u8 only)	    b'A'                 ");

        let x: f64 = 3.2;
        let y: f32 = 2.2;
        println!(" FLOATING_TYPE POINTS ARE: {} & {}",x,y);

        let t = true;
        let f: bool = false;
        println!(" BOOLEAN_TYPES ARE: {} & {}",t,f);

        let c: char = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat: char = '😻';
        println!(" CHARACTER_TYPES ARE: {} & {} & {}",c,z,heart_eyed_cat);
}


// NUMERIC OPERATIONS
pub fn numeric_operations(){
        let x: i32 = 5;
        let y: i32 = 9;
        let sum = x + y;             // Addition 
        let difference = x - y;      // Subtraction
        let product = x * y;         // Multiplication
        let quotient = x / y;        // Division
        let floored = x / y;         // Floor division
        let remainder = x % y;       // Remainder
        println!(" Numeric Operation for x = 5 & y = 9 is Sum:{}, Difference:{}, Product:{}, Quotient:{}, 
                   Floored:{}, Remainder:{}",sum,difference,product,quotient,floored,remainder);
}


/* COMPOUND TYPES
   Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. */
// TUPLE
pub fn tuple(){
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let tup1 = (100, 200, 3969);
        let (x, y, z) = tup1;
        println!("tup:{:?}, tup1:{:?}, Value of x in tup1:{}, Value of y in tup1:{}, Value of z in tup1:{}",tup,tup1,x,y,z);

        // {:?}  <-- This symbol is used to print entire list, tuple or vectors.

        let k: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = k.0;
        let six_point_four = k.1;
        let one = k.2;
        println!("{}, {}, {}",five_hundred,six_point_four,one);
}

// ARRAY
pub fn array(){
        let a = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June", "July",
                                  "August", "September", "October", "November", "December"];
        
        // Accessing elements
        let first = a[0];
        let second = a[1];

        // Another way to declare list
        let b: [i32; 10] = [10,20,30,40,50,60,70,80,90,100];              
        let days: [&str; 7] = ["Sunday","Monday","Tuesday","Wednesday",
                               "Thursday","Friday","Saturday"];

        println!("Arrays are {:?} {:?} {:?} {:?}",a,months,b,days);
        println!("Access elements first {} & second {}",first,second);
        
        /* You can also initialize an array to contain the same value for each element by specifying the initial value, 
           followed by a semicolon, and then the length of the array in square brackets, as shown here: */
        let k = [3; 5];
        println!(" Initialised array {:?}",k);
}

// FUNCTIONS
pub fn function(){
        println!("Hello World");
        another_function();
}
fn another_function(){
        println!("Hii from Another function");
}

// PARAMETERS
pub fn parameters(){
        cube_of_number(10);
}
fn cube_of_number(x :i32){
        println!("Cube of number entered is: {}",x*x*x);
}
// Another example
pub fn display() {
        print_labeled_measurement(5, 'h');
}
fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {}{}", value, unit_label);
}

// RETURN VALUES
pub fn no_parameter() {
        let x = five();
        println!("The value of x is: {}", x);
}
fn five() -> i32 {
        5
}
// Example with parameters
pub fn with_parameter() {
        let x = plus_one(5);
        println!("The value of x is: {}", x);
}
fn plus_one(x: i32) -> i32 {
        x + 1
}
/* Running this code will print The value of x is: 6. But if we place a semicolon at the end of the line containing x + 1, 
   changing it from an expression to a statement, we’ll get an error. */