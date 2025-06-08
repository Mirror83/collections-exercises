use std::collections::HashMap;

pub mod company;

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

pub fn find_median(v: &Vec<i32>) -> f32 {
    let mut v_copy = v.clone();
    v_copy.sort();

    let mut median: f32 = 0.0;
    let v_length = v.len();
    if v_length == 0 {
        median = median;
    } else if v_length % 2 != 0 {
        median = v[v_length / 2] as f32;
    } else {
        let median_index = v_length / 2;
        median = (v[median_index] + v[median_index - 1]) as f32 / 2.0;
    }

    return median;
}

pub fn to_pig_latin(s1: &str) -> String {
    assert!(s1.is_ascii());

    for letter in s1.chars() {
        if !letter.is_alphabetic() {
            continue;
        }
        match letter.to_lowercase().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => return format!("{s1}-hay"),
            _ => {
                let rest = &s1[1..s1.len()];
                return format!("{}-{letter}ay", rest);
            }
        }
    }

    return String::from(s1);
}
