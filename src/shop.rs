pub struct Shop {
    rows: Vec<Row>,
}
impl Shop {
    pub fn new() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::new(1, vec![Zones::new("Salgados"), Zones::new("Doces"), Zones::new("Bebidas")]));
        rows.push(Row::new(2, vec![Zones::new("Sumos"), Zones::new("Aguas"), Zones::new("Alcol")]));
        rows.push(Row::new(3, vec![Zones::new("Iogurtes"), Zones::new("Manteigas"), Zones::new("Saladas")]));

        Self { rows }
    }
    pub fn move_product(&mut self, to_row: u8, to_zone: &str, id_to_move: String) {
        let mut product_to_move: Option<Product> = None;
        for row in &mut self.rows {
            for zone in &mut row.zones {
                if let Some(pos) = zone.products.iter().position(|p| p.id == id_to_move) {
                    product_to_move = Some(zone.products.remove(pos));
                    break;
                }
            }
            if product_to_move.is_some() {
                break;
            }
        }
        if let Some(product) = product_to_move {
            for row in &mut self.rows {
                if row.number == to_row {
                    if let Some(zone) = row.zones.iter_mut().find(|z| z.name == to_zone) {
                        zone.products.push(product);
                        println!("Product moved");
                        return;
                    }
                }
            }
        } else {
            println!("Product with ID {} was not found", id_to_move);
        }
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
