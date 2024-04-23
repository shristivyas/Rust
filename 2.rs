use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // continue searching to the left
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = [1, 2, 3, 3, 3, 4, 5, 6, 7, 8, 9];
    
    println!("Enter the number you want to search:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Please enter a number");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not found in the array", target),
    }
}
