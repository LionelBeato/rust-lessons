fn main() {
    let mut x = 10; 
    // {} is a place holder in our string
    println!("X is {}", x);
    /*
    you cannot assign x again!
    all variables are immutable by default
    use the keyword mut to declare a variable as mutable
    */
    x = 20;
    println!("X is {}", x);

    let y: u8 = 255;
    // this will cause a panic!
    // y = y + 1; 
    println!("y is {}", y); 

    let a = 10;
    let b = 3.0;
    let c = a as f64 / b;  // example of casting; be wary of casting between major types
    // formating for printing! 
    // we can pad with spaced or zeroes
    println!("c is {:.3}", c);
    println!("c is {:08.3}\na is {}", c, a); // zeroes
}
