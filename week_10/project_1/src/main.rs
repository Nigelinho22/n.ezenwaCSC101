struct Laptop {
    brand: &'static str,
    price:u32,
    quantity:u32,
}

impl Laptop {
    fn total_cost(&self, quantity_to_buy:u32)-> u32 {
        self.price * quantity_to_buy
    }
}

fn main() {
    let hp = Laptop {
        brand: "HP",
        price: 650_000,
        quantity:10,
    };
    let ibm = Laptop {
        brand: "IBM",
        price: 755_000,
        quantity:6,
    };
    let toshiba = Laptop {
        brand: "Toshiba",
        price: 550_000,
        quantity:10,
    };
    let dell = Laptop {
        brand: "Dell",
        price: 850_000,
        quantity:4,
    };

    let quantity_to_buy = 3;

    let total_cost = hp.total_cost(quantity_to_buy) + ibm.total_cost(quantity_to_buy) + toshiba.total_cost(quantity_to_buy) + dell.total_cost(quantity_to_buy);

    println!("The total cost of buying 3 laptops from each brand is: {} Naira", total_cost);
}