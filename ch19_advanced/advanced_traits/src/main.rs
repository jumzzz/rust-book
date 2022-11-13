use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(f64);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 as u32 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, other: Millimeters) -> Meters {
        Meters(self.0 + (other.0 as f64 ) / 1000_f64)
    }
}

fn operator_overloading() {
    
    let m0 = Millimeters(1000_u32);
    let m1 = Meters (1_f64);

    let r0 = m0 + m1;

    let m2 = Meters(2_f64);
    let m3 = Millimeters(2000_u32);

    let r1 = m2 + m3;


    println!("r0 = {:?}", r0);
    println!("r1 = {:?}", r1);
}


// Calling methods with same name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Expecto Patronium! (Ooops wrong anime)");
    }
}

impl Human {
    fn fly(&self) {
        println!("*Adds rocket booster*");
    }
}

fn methods_same_name() {
    let person = Human;

    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

}

// Newtype Pattern
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn fmt_vector() {
    let w = Wrapper(vec![
        String::from("new"), 
        String::from("type"),
        String::from("wrapper"),
    ]
    );

    println!("w = {}", w);
}


fn main() {
    // methods_same_name();
    fmt_vector();
}
