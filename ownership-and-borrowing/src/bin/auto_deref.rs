fn main() {
    let mut num: i32 = 10;
    let num2: i32 = num;
    println!("Num is {} and address is {:p}", num, &num);
    println!("Num2 is {} and its address is {:p}", num2, &num2);

    change_num(&mut num);
    println!("After change: Num is {} and its address is {:p}", num, &num);
}

fn change_num(num_ref: &mut i32) {
    // let num_ref = &mut num; //mutable pointer
    *num_ref = 100;
}
