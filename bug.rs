fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);
    let first = vec.get(0).unwrap();
    let second = vec.get(1).unwrap();
    println!("First: {}, Second: {}", first, second);
    // Trying to access an element outside the vector
    // This will cause a panic at runtime!
    let third = vec.get(2).unwrap();
    println!("Third: {}", third);
}