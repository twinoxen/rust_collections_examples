mod numbers;
mod strings;
mod company;

use crate::numbers::NumberCollection;
use crate::strings::convert_to_pig_latin;
use crate::company::{company_interface, Company};

fn main() {
    // Collection of numbers
    let nums = NumberCollection::new(vec![1, 2, 3, 3, 3, 3, 5]);

    println!(
        "Mean: {}, Median: {}, Mode: {:?}",
        nums.mean, nums.median, nums.mode
    );

    println!("{:?}", NumberCollection::new(vec![1, 2, 3]));
    println!(
        "{:?}",
        NumberCollection::new(vec![8, 90, 85, 70, 28, 18, 93, 68, 99, 98, 89, 11])
    );
    println!(
        "{:?}",
        NumberCollection::new(vec![
            51, 46, 16, 1, 53, 66, 35, 79, 69, 72, 82, 86, 22, 20, 71, 96, 54, 67, 38, 32, 75, 81,
            76, 65, 3, 95, 49, 83, 19
        ])
    );

    // strings
    println!(
        "{}",
        convert_to_pig_latin(&"the brown fox jumped over the lazy dog".to_string())
    );

    // Terminal interface for CRU for a company and its employees
    company_interface(&mut Company::new("Funco"))
}
