fn main() {

    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_heights);

    mountain_heights.2 = "Lhoste";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}", mountain_heights);
}
