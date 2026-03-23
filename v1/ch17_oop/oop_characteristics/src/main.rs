use std::process;

pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {

    pub fn build() -> Result<AverageCollection, &'static str> {
        Ok(AverageCollection {
            list: vec![],
            average: 0.0,
        })
    }


    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {

    let mut avg_collection = AverageCollection::build().unwrap_or_else(|err| {
            eprintln!("Problem Initializing Build: {}", err);
            process::exit(1);
        }
    );
    avg_collection.add(2);
    avg_collection.add(4);
    avg_collection.add(8);
    avg_collection.add(16);
    avg_collection.add(32);

    println!("Average Value = {}", avg_collection.average());
}
