fn main() {
    let mut txt = String::from("Hello, World!");
    println!("{}", txt);
    txt.clear();
    println!("{}", txt);
    if txt == String::from("") {
        println!("True");
    }
}
