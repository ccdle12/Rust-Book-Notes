/// Using generics in structs.
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    // mixup takes another Point parameter, extracts the y value from other and x from the origin
    // point and returns a new point object combining both.
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y
        }
    }

}

/// We can have multiple implementations of Point, this one is using explicit declaration of f32,
/// this means the distance_from_origin function will ONLY be available to f32 instances of Point.
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2).sqrt())
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}i32", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    /// Using generic structs.
    let _integer = Point{x: 5, y: 10};
    let float = Point{x: 0.5, y: 1.2};

    // Must be the same type.
    // let wont_work = Point{x: 0.5, y: 10};

    // After adding two different generics, T, U - we can now use different types in the struct.
    let float_and_integer = Point{x: 0.5, y: 10};

    let x = float_and_integer.x();
    let y = float_and_integer.y();
    println!("x is: {}", x);
    println!("y is: {}", y);

    let point_f32 = Point{x: 5.32, y: 10.4};
    let distance = point_f32.distance_from_origin();
    println!("distance is: {}", distance);

    // We will call mixup, combining x from p1 and y from p2.
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

/// This fails for now because, we have not implemented the std::cmp::PartialOrd trait.
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
