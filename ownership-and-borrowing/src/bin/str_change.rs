// fn main() {
//     let s = String::from("Bob");
//     change_str(s);
//     println!("String after func call: {}", s);

//     fn change_str(s: String) {
//         s = "Alice".to_string();
//         println!("String inside func: {}", s);
//     }
// }

fn main() {
    let mut s = String::from("Bob");
    // let s2 = &s;
    change_str(&mut s);
    println!("String after func call: {}", s);

    fn change_str(s: &mut String) {
        *s = "Alice".to_string();
        println!("String inside func: {}", s);
    }
}
