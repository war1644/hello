#[derive(Debug, Clone)]
pub struct Goods {
    pub name: String,
    pub price: u32,
    standard_price: u32,
    pub describe: String,
    pub quantity: u32,
}

impl Goods {
    pub fn new(name: &str, price: u32) -> Goods {
        Goods {
            name: name.to_string(),
            price,
            standard_price: price,
            describe: String::from(""),
            quantity: 0,
        }
    }

    pub fn change_price(&mut self, price: u32) {
        self.price = price;
    }

    pub fn show(&self) {
        println!("goods name {}, price {}", self.name, self.price);
    }
}
