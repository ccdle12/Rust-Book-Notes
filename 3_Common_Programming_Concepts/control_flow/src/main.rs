fn main() {
    let num = 3;

    if num == 4 {
        println!("Num is 4");
    } else {
        println!("Num is not 4");
    }

    if num != 0 {
        println!("number was something other than zero")
    }

    // INVALID
    //    if num {
    //        println!("number was something other than zero")
    //    }

    //ELSE IF
    if num % 4 == 0 {
        println!("Mod 4");
    } else if num % 3 == 0 {
        println!("Mod 3");
    } else if num % 2 == 0 {
        println!("Mod 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // IF is an expression and can be assigned to a statement
    // Depending on the condition, assign the int to the statement
    let condition = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("number should be 5: {}", number);

    // This should fail. If and Else should be returning the same types,
    // if we are assigning to a statement.
    //    let condition = true;
    //    let number = if condition {
    //        5
    //    } else {
    //        "hello"
    //    };
    //    println!("number should be 5: {}", number);

    // LOOPS
    loop {
        println!("again!");
        break;
    }

    // RETURNING FROM LOOPS
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // WHILE LOOP, will break when number reaches 0;
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // LOOP THROUGH A COLLECTION (ARRAY OR TUPLE)
    let b = [10, 20, 30, 40 ,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", b[index]);
        
        index = index + 1;
    };

    // FOR LOOP 'in'
    for element in b.iter() {
        println!("the value is in iter loop: {}", element);
    };

    // Safe for loop with range 1..4
    // it DOES NOT include 4.
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    // Loop in range and reverse. 
    // DOES NOT include 4.
    for number in (1..4).rev() {
        println!("reversed {}!", number);
    }
}
