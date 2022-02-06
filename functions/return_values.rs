fn main() {
    let result = square(13);
    println!("result is {}", result);
    let result_tuple = square_tup(13);
    // debug printout :? 
    println!("result_tuple is {:?}", result_tuple);
}

fn square(x: i32) -> i32 {
    println!("squaring {}", x);  
    // implicit return without semicolon or return keyword
    x * x 
    // will throw error! 
    // println!("End of function!"); 
    // you can return early while skipping unreachable code
}

fn square_tup(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x); 
}

// functions will implicitly return unit data type
// represented as -> ()