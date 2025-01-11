fn main() {
    let v = vec!{101,250,330,400};
    // vector v owns the objects in heap
    let v2 = v.clone();
    // two variables cant point to the same content of heap value
    println!("{:?}",v.clone());
}
