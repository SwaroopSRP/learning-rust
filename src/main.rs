fn main() {
    let fet = 15;
    let net = if fet == 15 {true} else {false};
    println!("{}", net);
    let net = false; // Changed value while being same dtype. (Shadowing)
    println!("{}", net);
    let net = 20;
    println!("{}", net) // Shadowing to diff dtype value
    // Have a good reason to shadow as it breaks code quality. Under control whern using loops.
}