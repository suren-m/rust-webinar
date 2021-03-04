use scientific_lib::factorial;
fn main() {
    println!("\n..Advanced Calculator...\n");
    let res = factorial::fact_recur(5);
    println!("Factorial of 5 is {}", res);     
}
