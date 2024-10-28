fn main(){
    let p:f64 = 510_000.00;
    let r:f64 = 5.0;
    let t:f64 = 3.0;

    let a = p * ( 5.0 + (r / 100.0)) * t;
    println!("Amount is {}", a);

    let d = p * (1.0 - r / 100.0).powi(t as i32);
    println!("The value of the TV after {} years is {}", t, d);
}