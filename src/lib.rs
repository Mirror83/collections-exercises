use std::collections::HashMap;

pub fn find_mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in v {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (num, count) in map {
        if count > max_count {
            mode = *num;
            max_count = count;
        }
    }

    return mode;
}
