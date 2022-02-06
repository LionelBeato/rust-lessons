fn main() {
    say_hello(); 
    say_hello();
    let x = 1;
    let y = 2; 
    // rust will never cast implicitly
    // compiler will use datatype specified for argument
    say_the_sum(x, y); 
}


fn say_hello() {
    println!("Hello!");
    say_a_number(13); 
}

fn say_a_number(number: i32) {
    println!("number is {}", number); 
}

fn say_the_sum(a:u8, b:u8) {
    let sum = a + b; 
    println!("sum is {}", sum);
}