fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const PI: f64 = 3.14;
    println!("The value of PI is: {PI}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
}
