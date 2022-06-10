// DATA TYPES

/* SCALAR TYPES
   A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 
   You may recognize these from other programming languages. Let’s jump into how they work in Rust.  */
   pub fn scalartypes(){
    println!("          
                      INTEGER TYPES
               LENGTH	     SIGNED	  UNSIGNED
                8-bit        i8              u8
                16-bit	     i16	     u16
                32-bit	     i32	     u32
                64-bit	     i64	     u64
                128-bit	     i128	     u128
                arch	     isize	     usize       ");            
    println!("           
                      INTEGER LITERALS
               NUMBER_LITERALS	   EXAMPLE
                 Decimal	    98_22
                 Hex	            0xff
                 Octal	            0o77
                 Binary	            0b1111_0000
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
    println!("CHARACTER_TYPES ARE: {} & {} & {}",c,z,heart_eyed_cat);
}

/* NUMERIC OPERATIONS */
pub fn numeric_operations(){
        let x: i32 = 5;
        let y: i32 = 9;
        let sum = x + y;             // addition 
        let difference = x - y;      // subtraction
        let product = x * y;         // multiplication
        let quotient = x / y;        // division
        let floored = x / y;         // Results in 0
        let remainder = x % y;       // remainder
        println!(" Numeric Operation for x = 5 & y = 9 is {}, {}, {}, {}, {}, {}",sum,difference,product,quotient,floored,remainder);
}