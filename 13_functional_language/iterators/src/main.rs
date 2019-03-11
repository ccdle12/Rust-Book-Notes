fn main() {
    // iterators allow easy looping/iteration over a vector.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// Iterator is a trait in the standard library.
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        // Sum takes ownershop of ther iterator.
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn produce_other_iterators() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}
