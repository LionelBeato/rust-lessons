fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // all floats are f64 by default
    // b and the 3.0 literal need not be casted
    let average = (a as f64 + b + c as f64) / 3.0; 
    assert_eq!(average, 45.1); 
    println!("Test passed!");
}