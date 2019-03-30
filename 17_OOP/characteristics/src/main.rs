fn main() {}

// pub indicates this object is accessible.
pub struct AverageCollection {
    // The fields list, average are not marked as public meaning they are
    // private.
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    // Public, borrowing mutable self to update the private fields.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // Public getting a privtate field.
    pub fn average(&self) -> f64 {
        self.average
    }

    // Private function.
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    //NOTE: NO INHERITANCE IN RUST, rust favours the use of trait objects instead.
}
