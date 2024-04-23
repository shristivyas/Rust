fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase(); 
    let reversed = s.chars().rev().collect::<String>(); 
    s == reversed 
}

fn main() {
    let s1 = "amama";
    let s2 = "cute";
    let s3 = "nine";
    let s4 = "naman";
    
    println!("Is '{}' a palindrome? {}", s1, is_palindrome(s1));
    println!("Is '{}' a palindrome? {}", s2, is_palindrome(s2));
    println!("Is '{}' a palindrome? {}", s3, is_palindrome(s3));
    println!("Is '{}' a palindrome? {}", s4, is_palindrome(s4));
}
