use crate::garden::vegetables;

pub mod garden;

fn main() {
    let plant0 = vegetables::Asparagus {};
    let plant1 = vegetables::Brocolli {};
    
    println!("I'm growing {:?}!", plant0);
    println!("I'm growing {:?}!", plant1);
}
