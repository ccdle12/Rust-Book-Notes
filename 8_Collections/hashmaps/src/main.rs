use std::collections::HashMap;

fn main() {
    // Simple initialisation.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    // Initialising using existing vectors.
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 20];

    // iterative and zip the initial score to each iteration of the teams vector, also we use the
    // HashMap to infer the type, because .collect() can use many different types.
    let _scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Ownership.
    // For copy variables like i32, the values will be copied into the HashMap. For owned values
    // like Strings, ownership will change.
    let field_name = String::from("Favourite Colour");
    let field_value = String::from("Blue");

    let mut ownership_map = HashMap::new();
    ownership_map.insert(field_name, field_value);
    // field_name and field_value are now invlaid and cannot be used.
    // println!("{}", field_name);

    // Retrieving values from the hash map.
    // Returns Some() because get() returns an Option<&v>.
    let blue_score = scores.get("Blue");
    println!("{:?}", blue_score);

    // Iteration over a HashMap is possible.
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    // Overwriting a value for an existing key.
    // scores2.insert(String::from("Blue"), 5);
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores.get("Blue"));

    // Only insert a value if a key has no value.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Update a value based on an old value.
    // This updates the frequency of each word.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Finds the word, if it doesn't exist, set the value to 0.
        // or_insert returns a &mut ref to the value of the key. We can point to the &mut ref and
        // update its value.
        let count = map.entry(word).or_insert(0);

        // Point to count, increase it's value by 1.
        *count += 1;
    }

    println!("{:?}", map);
}
