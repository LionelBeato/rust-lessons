use std::env;
// std::env does not come with prelude
// so we will need to import it! 

fn main() {
   for el in env::args() {
       println!("argument {}" , el);
   } 
}
