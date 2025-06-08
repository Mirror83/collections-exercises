use collections_exercises::{find_median, find_mode, to_pig_latin};

fn main() {
    println!("Hello, world!");
    let v = vec![3, 1, 2];
    let mode = find_mode(&v);
    let median = find_median(&v);
    println!("The mode of {v:?} is {mode}, the median is {median}");

    let words = [
        "some,", "124", "eager", "'apple'", "one-shot", "portal", "first",
    ];

    for word in words {
        println!("{word} -> {}", to_pig_latin(word));
    }
}
