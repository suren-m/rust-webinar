// toggle inlay hints when needed
fn main() {
    let msg = "hello"; // this is a string literal

    let s = String::from("Bob");
    get_len(s);
    println!("String usage after func call: {}", s);

    fn get_len(s: String) -> usize {
        s.len()
    }
}

// fn main() {
//     let s = String::from("Bob");
//     get_len(&s);
//     println!("String usage after func call: {}", s);

//     fn get_len(s: &Str) -> usize {
//         s.len()
//     }
// }
