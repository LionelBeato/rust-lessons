// traits are directly similar to 
// interfaces with a some differences

// trait to print out struct
pub trait Description {
    fn describe(&self) -> String; 
}

pub struct Cow {
    pub size: String,
    pub legs: u8,
    pub cry: String
}

impl Description for Cow {
    fn describe(&self) -> String {
        format!("this cow is {}, has {} legs, and goes {}", 
        self.size, self.legs, self.cry)
    }
}