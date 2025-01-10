fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);
    if let Some(first) = vec.get(0) {
        if let Some(second) = vec.get(1) {
            println!("First: {}, Second: {}", first, second);
        }
    }
    // Safe way to access elements, handling potential out-of-bounds access
    if vec.len() > 2 {
        if let Some(third) = vec.get(2) {
            println!("Third: {}", third);
        }
    }
} 