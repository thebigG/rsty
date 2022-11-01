///Very simple example of encapsulation in Rust
///
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) {
        let result = match self.list.pop() {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        };
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }
}
