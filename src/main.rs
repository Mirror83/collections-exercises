use collections_exercises::find_mode;

fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5, 6, 7, 7, 9, 7];
    let mode = find_mode(&v);
    println!("The mode of {v:?} is {mode}");
}
