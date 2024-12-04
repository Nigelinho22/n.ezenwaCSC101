fn main() {

    let name = vec!["mary", "sam", "sally","greg", "ade", "mark", "june", "ife"];

    let age = vec![16,17,19,22,20,21,18,23];

    print!("\nAge allocation");

    for i in 0..age.len() {
        print!("{} is {} years old\n", name[i], age[i]);
    }
}
