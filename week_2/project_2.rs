fn main(){
    let sales_records = [
        ("Toshiba", 2, 450_000.0),
        ("Mac", 1, 1_500_000.0),
        ("HP", 3, 750_000.0),
        ("Dell", 3, 2_850_000.0),
        ("Acer", 1, 250_000.0),
    ];
    
    let total_amount:f64 = sales_records.sum();
    
    let average_amount:f64 = total_amount / sales_records.len();

    println!("Total Amount is {}", total_amount);

    println!("Average Amount is {}", average_amount);
}