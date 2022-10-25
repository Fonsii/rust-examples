fn main(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Albert"), 10);
    scores.insert(String::from("Bob"), 5);
    scores.insert(String::from("Alexa"), 7);
    scores.insert(String::from("Maria"), 9);

    println!("{:?}", scores);
    println!("{:?}", scores.get("Albert"));

    scores.insert(String::from("Bob"), 6);


    if !scores.contains_key("Ana") {
        println!("Ana is not in the scores");
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}