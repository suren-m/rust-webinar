fn main() {
    let mut num: i32 = 10;
    let num2: i32 = num;

    println!("Num is {} and address is {:p}", num, &num);
    println!("Num2 is {} and its address is {:p}", num2, &num2);

    let num_ref = &mut num; //mutable ref pointer
    *num_ref = 100;

    println!("After change: Num is {} and its address is {:p}", num, &num);
}
