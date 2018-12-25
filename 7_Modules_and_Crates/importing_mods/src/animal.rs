pub mod animal {
    pub struct Dog {
        pub name: String,
    }

    impl Dog {
        pub fn new(name: &str) -> Dog {
            Dog { name }
        }
    }
}
