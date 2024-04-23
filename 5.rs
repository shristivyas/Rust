fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        arr[len / 2] as f64
    } else {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0
    }
}

fn main() {
    let arr_odd = [1, 2, 3, 4, 5];
    println!("Median of odd length array: {}", find_median(&arr_odd));

    let arr_even = [1, 2, 3, 4];
    println!("Median of even length array: {}", find_median(&arr_even));
}
