pub struct Mercearia {
    rows: Vec<Row>,
}
impl Mercearia {
    pub fn new() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::new(1, vec![Zones::new("Salgados"), Zones::new("Doces"), Zones::new("Bebidas")]));
        rows.push(Row::new(2, vec![Zones::new("Sumos"), Zones::new("Aguas"), Zones::new("Alcol")]));
        rows.push(Row::new(3, vec![Zones::new("Iogurtes"), Zones::new("Manteigas"), Zones::new("Saladas")]));

        Self { rows }
    }
    pub fn move_product(row: u8, zone: &str) {
        todo!()
    }
}

pub struct Row {
    number: u8,
    zones: Vec<Zones>,
}

impl Row {
    fn new(number: u8, zones: Vec<Zones>) -> Self {
        Row { number, zones }
    }
}

pub struct Zones {
    name: &'static str,
    products: Vec<Product>,
}

impl Zones {
    // TODO: Review life cycle
    fn new(name: &'static str) -> Self {
        Zones { name, products: Vec::new() }
    }
}

#[derive(Debug)]
pub struct Product {
    id: String,
    name: String,
    exp_date: u16,
    price: f64,
    stock: u16,
}

impl Product {
    pub fn new(id: String, name: String, exp_date: u16, price: f64, stock: u16) -> Self {
        Self {
            name,
            id,
            exp_date,
            price,
            stock,
        }
    }

    pub fn change_price(&mut self, new_price: f64) {
        self.price = new_price;
    }
    pub fn update_stock(&mut self, new_stock: u16) {
        self.stock = new_stock;
    }
}
