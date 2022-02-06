fn main() {

    // let stuff = (10, 3.14, 'x'); 
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x'); 
    let first_item = stuff.0; // how we access items from tuples

    println!("first_item is {}", first_item); 

    // we can also destructure tuples; 

    let (a, b, c) = stuff;
    println!(b); 

}