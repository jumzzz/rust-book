
pub trait Draw {
    fn draw(&self);
}

pub struct DrawDog {
    pub breed: String,
}

impl Draw for DrawDog {
    fn draw(&self) {
        println!("Drawing Dog Breed = {}", self.breed);
    }
}

pub struct DrawCat {
    pub chads: String, 
}

impl Draw for DrawCat {
    fn draw(&self) {
        println!("We are Chads...");
        println!("Drawing Cat breed = {}", self.chads);
    }
}

// This works: Static Dispatch
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw, 
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

}

// This also works: Dynamic Dispatch
// BUT you need to declare each vector with Box::new
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// fn box_new_ver() {

//     println!("Printing Cats Implementation...");
//     let mut dog_screen = Screen {
//         components: vec![
//             Box::new(DrawDog { breed: String::from("corgi")}),
//             Box::new(DrawDog { breed: String::from("pitbull")}),
//             Box::new(DrawDog { breed: String::from("chihuahua")}),
//             Box::new(DrawDog { breed: String::from("shitzu")}),
//             Box::new(DrawDog { breed: String::from("shiba")}),
//         ],
//     };
//     dog_screen.run();

//     println!("Printing Cats Implementation...");

//     let mut cat_screen = Screen {
//         components: vec![
//             Box::new(DrawCat { chads: String::from("meow")}),
//             Box::new(DrawCat { chads: String::from("roar")}),
//             Box::new(DrawCat { chads: String::from("tiger")}),
//             Box::new(DrawCat { chads: String::from("lion")}),
//             Box::new(DrawCat { chads: String::from("mewtwo")}),
//         ],
//     };
//     cat_screen.run();
// }

fn generic_version() {
    println!("Printing Cats Implementation...");
    let mut dog_screen = Screen {
        components: vec![
            DrawDog { breed: String::from("corgi")},
            DrawDog { breed: String::from("pitbull")},
            DrawDog { breed: String::from("chihuahua")},
            DrawDog { breed: String::from("shitzu")},
            DrawDog { breed: String::from("shiba")},
        ],
    };
    dog_screen.run();

    println!("Printing Cats Implementation...");

    let mut cat_screen = Screen {
        components: vec![
            DrawCat { chads: String::from("meow")},
            DrawCat { chads: String::from("roar")},
            DrawCat { chads: String::from("tiger")},
            DrawCat { chads: String::from("lion")},
            DrawCat { chads: String::from("mewtwo")},
        ],
    };
    cat_screen.run();
}


fn main() {
    generic_version();
    // box_new_ver();
    
}
