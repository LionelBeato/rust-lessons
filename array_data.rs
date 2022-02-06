fn main() {
    // let letters = ['a', 'b', 'c']; 
    let mut letters = ['a', 'b', 'c']; 
    letters[0] = 'x'; 
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter); 

    let numbers: [i32; 5]; // define empty array 
    // numbers = [0, 0, 0, 0, 0];
    numbers = [0; 5]; // initialize values; note the use of shorthand
    let index: usize = numbers.len(); 
    println!("last number is {}", numbers[4]);

}