use std::collections::HashMap;

#[derive(Debug)]
pub struct NumberCollection {
    pub values: Vec<i32>,
    pub mean: i32,
    pub median: i32,
    pub mode: Option<i32>,
}

impl NumberCollection {
    pub fn new(values: Vec<i32>) -> Self {
        let (mean, median, mode) = mean_median_mode(&values);

        Self {
            values,
            mean,
            median,
            mode,
        }
    }
}

fn mean_median_mode(nums: &Vec<i32>) -> (i32, i32, Option<i32>) {
    let nums = nums.clone();

    let sum: i32 = nums.iter().sum();

    let mean = sum / i32::try_from(nums.len()).unwrap();
    let median = nums[(nums.len() / 2)];

    let mut nums_hash: HashMap<i32, i32> = HashMap::new();

    for num in nums {
        let count = nums_hash.entry(num).or_insert(0);

        *count += 1;
    }

    let mut most_recurring_number = (0, 1);

    for (key, value) in nums_hash {
        if value > most_recurring_number.1 {
            most_recurring_number = (key, value);
        }
    }

    let mode: Option<i32>;

    if most_recurring_number.1 > 1 {
        mode = Some(most_recurring_number.0);
    } else {
        mode = None
    }

    (mean, median, mode)
}
