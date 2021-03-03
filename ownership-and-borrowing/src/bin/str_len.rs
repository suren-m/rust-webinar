fn main() {
    let s = String::from("Bob");
    get_len(s);
    println!("String usage after func call: {}", s);

    fn get_len(s: String) -> usize {
        s.len()
    }
}
