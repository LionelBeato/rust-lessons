mod generic_struct; 
mod traits_lesson;

use traits_lesson::Description; 
// structs are like templates/contracts for tuples
// struct data are stored on the stack
// contents of strings live on the heap
// structs are the closest thing to a class
#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

// tuple struct, if a tuple and a struct had a baby
struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

// cannot use a Color for this function! 
fn get_y(p: Point) -> u8 {
    p.1
}
 
// the behaviors are removed from attributes
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    // you can tell Rust to use the rest of the fields
    // from another Struct instance
    let vehicle2 = Shuttle {
        name : String::from("Discovery"),
        ..vehicle
    };

    let vehicle_name = vehicle.get_name();
    // you can make a constructor lol 
    let vehicle3 = Shuttle::new("Topanga"); 

    let coord = Point(4,5,6); 
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0); 

    println!("vehicle_name is {}", vehicle_name);

    println!("name is {:?}", vehicle);

    // fields of the same generic
    // should always end up as the same type!
    let x = generic_struct::Rectangle {
        width: 1u8,
        height: 1u8
    }; 

    let my_cow = traits_lesson::Cow {
        size: String::from("pretty big"),
        legs: 4,
        cry: String::from("moooo")
    }; 

    println!("{}", my_cow.describe());
    
    let dasds = 1;

    


}
