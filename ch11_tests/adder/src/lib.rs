pub mod sample_struct {
    
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 0 {
                panic!("Value must be 0 <= value <= 100");

            }
            else if value > 100 {
                panic!("Value must be 0 <= value <= 100");
            }
            Guess {value}
        }

    }

}

pub mod util_funcs {
    
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    pub fn greeting(name: &str) -> String {
        format!("Hello {}", name)
    }
}


#[cfg(test)]
mod tests {

    use crate::sample_struct::*;
    use crate::util_funcs::*;


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(3 + 3, 6);
    }


    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        println!("larger (Rectangle) = {:?}", larger);
        println!("smaller (Rectangle) = {:?}", smaller);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        println!("larger (Rectangle) = {:?}", larger);
        println!("smaller (Rectangle) = {:?}", smaller);

        assert!(!smaller.can_hold(&larger));

    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Gatching");
        println!("result = {}", result);

        assert!(result.contains("Gatching"));
    }

    #[test]
    fn greeting_does_not_contain_name() {
        let result = greeting("Machi");
        
        let other = "Kenchan";
        let encoded = format!("Hello {}", other);

        println!("result = {}", result);
        println!("other = {}", other);
        println!("encoded = {}", encoded);

        assert_ne!(result, encoded);

    }

    // This is expected to panic
    #[test]
    #[should_panic(expected = "Value must be 0 <= value <= 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("two plus two does not equal four."))
        }
    }

}
