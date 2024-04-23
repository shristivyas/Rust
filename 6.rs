fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    let mut chars_iterators: Vec<_> = strings.iter().map(|s| s.chars()).collect();
    
    'outer: loop {
        let mut current_char = None;
        
        for iter in &mut chars_iterators {
            match iter.next() {
                Some(c) => {
                    if let Some(first_char) = current_char {
                        if c != first_char {
                            break 'outer;
                        }
                    } else {
                        current_char = Some(c);
                    }
                }
                None => {
                    break 'outer;
                }
            }
        }
        
        if let Some(c) = current_char {
            prefix.push(c);
        } else {
            break;
        }
    }
    
    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
