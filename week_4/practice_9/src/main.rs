fn main() {
    let mut count = 0;

    for num in 1..12 {
        if num > 10 {
            println!("{:?}", num);
            continue;
        }
        count += 1;
    }

    println!("The count of values greater than 10 (between 1 and 20) is: {}", count);
}
