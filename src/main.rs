use std::collections::HashMap;

fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for value in list {
        sum += value;
    }
    sum as f64 / (list.len() as f64)
}

fn median(list: &Vec<i32>) -> f64 {
    let len = list.len();
    if len % 2 == 0 {
        // len is even
        return (list[(len / 2)] + list[(len / 2) - 1]) as f64 / 2.0;
    } else {
        // len is odd
        return list[(len / 2)] as f64;
    }
}

fn main() {
    // Creating a new Hash Map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only inserting a Value if the key has no value
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Mean
    let integers = vec![2, 5, 10, 34, 43, 87];
    println!("Given the sorted Vec: {:?}", integers);
    println!("Mean: {}", mean(&integers));
    println!("Median: {}", median(&integers));
}
