fn main() {
    let height = 190;
    let result = if height >= 190 {
        "Tall" // No semi-colon since it is a value being assigned to result via condition
    }
    else if height > 170 {
        "Average"
    }
    else {
        "Short"
    }; // The semi-colon that comes after assigning
    println!("{}", result);
}
