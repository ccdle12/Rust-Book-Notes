fn main() {
    // BASIC VARIABLES:
    // mutation
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // shadowing, we can keep reassigning the value 
    // to y (this is called shadowing).
    let y = 12;
    let y = y + 1;
    let y = y + 1;

    println!("Value of y: {}", y);
    
    // shadowing a different type
    let spaces = "      ";
    let spaces = spaces.len();

    // spaces is now a number
    println!("Value of spaces: {}", spaces);

    // mut is not able to reassign different types
    // let mut spaces = "  ";
    // spaces = spaces.len();

    // DATA TYPES: 
    // SCALAR TYPES: 
    // 4 types - integers, floating-points, bools and characters

    // INTEGERS
    // Length	Signed	Unsigned
    //  8-bit	  i8	    u8
    // 16-bit	  i16	   u16
    // 32-bit	  i32	   u32
    // 64-bit	  i64	   u64
    // 128-bit	  i128	  u128
    // arch	      isize	 usize

    // INTEGER OVERFLOW - when calling build release, i8 will wrap around and not panic
    // in dev this will panic
    // let z: i8 = 125;
    // let z = z + 100;
    // println!("z overflow: {}", z);

    // Will overflow
    // let mut a: i8 = 125;
    // a = 225;
    // println!("a overflow: {}", a);

    // FLOATING POINTS
    let b = 6.0; // f64
    let c: f32 = 3.0; //f32

    println!("b f64 floating point: {}", b); 
    println!("c f32 floating point: {}", c);

    // MATH OPERATION
    let d = 43 % 5;
    println!("d modulo: {}", d);

    // BOOL
    let e = true;
    let f: bool = false;

    println!("e: {} and f: {}", e, f);

    // CHARACTER - uses single quotes
    let character = '§';
    println!("character: {}", character);

    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("Tuple destructuring y: {}", y);

    // Accessing through Index
    println!("Tuple 0 by index: {}", tup.0);

    // ARRAYS - Fixed size
    // declaration of array containing i32 and length 5
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Index 3 of a: {}", a[3]);
}
