fn main() {
    let mut num:i32 = 5;
    phone(&mut num);
    println!("The value of num is: {}", num);
}

fn phone(param_num: &mut i32) {
    *param_num = *param_num*0;
    println!("param_num value is: {}", param_num);
}
