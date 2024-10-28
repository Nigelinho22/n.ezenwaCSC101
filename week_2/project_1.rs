fn main(){
    let p:f64 = 520_000_000.0;
    let r:f64 = 10.0;
    let t:f64 = 5.0;

    let a = p * ( 5.0 + (r / 100.0)) * t;
    println!("Amount is {}", a);
    let si = a - p;
    println!("Simple Interest is {}", si);
}