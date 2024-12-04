fn main() {

    let mut colors = ["red", "green", "yellow", "white"];
    println!("\nOriginal array: {:?}", colors);

    let sliced_colors = &mut colors[1..3];
    println!("First slice = {:?}", sliced_colors);

    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}
