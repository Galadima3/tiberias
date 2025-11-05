use std::collections::HashMap;

// Function to Calculate Median
pub fn calculate_median(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let mut sorted = numbers.to_vec();
    sorted.sort();

    let len = sorted.len();
    let median = if len % 2 == 0 {
        let mid1 = sorted[len / 2 - 1];
        let mid2 = sorted[len / 2];
        (mid1 + mid2) as f64 / 2.0
    } else {
        sorted[len / 2] as f64
    };
    Some(median)
}

pub fn calculate_mode(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for &num in numbers {
        *frequency.entry(num).or_insert(0) += 1;
    }
    frequency
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}