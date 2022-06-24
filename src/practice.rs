pub fn run() {
    println!("Aditya");
    let mut a: [f64; 10] = [1.0, 12.35, 23.0, 34.0, 45.9, 89.0, 45.8, 78.56, 9.0, 10.0];
    println!("The etire array is: {:?}",a);
    println!("Fist and Lat elements are: {} & {}",a[0],a[9]);
    a[0] = 5.0;
    println!("New array: {:?}",a);
    for i in a{
        println!("{}",i);
    }
}