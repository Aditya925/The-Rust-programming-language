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