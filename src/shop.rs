#[derive(Debug)]
pub struct Shop {
    rows: Vec<Row>,
}

pub fn StoreExample() -> () {
    let mut ashop = Shop::new();
    println!("This is a shop {:?}", ashop);
    let product = Product::new(String::from("ID234"), String::from("Pepsi"), 33, 2.45, 200);
    ashop.add_product(product, 2, "Soda");
}

impl Shop {
    pub fn new() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::new(1, vec![Zone::new("Salty"), Zone::new("Sweets"), Zone::new("Drinks")]));
        rows.push(Row::new(2, vec![Zone::new("Soda"), Zone::new("Water"), Zone::new("Alcohol")]));
        rows.push(Row::new(3, vec![Zone::new("Yogurt"), Zone::new("Butter"), Zone::new("Salad")]));

        Self { rows }
    }

    pub fn restock(&mut self, product: String, amount: u16) {
        for row in &mut self.rows {
            for zone in &mut row.zones {
                if let Some(product) = zone.products.iter_mut().find(|p| p.name == product) {
                    product.stock += amount;
                    return;
                }
            }
        }
    }

    pub fn add_product(&mut self, new_product: Product, add_row: u8, add_zone: &str) {
        if let Some(row) = self.rows.iter_mut().find(|r| r.number == add_row) {
            if let Some(zone) = row.zones.iter_mut().find(|z| z.name == add_zone) {
                zone.products.push(new_product);
            } else {
                println!("Adding product to store: The zone:{} was not found", add_zone);
            }
        } else {
            println!("Adding product to store: The row:{} was not found", add_row);
        }
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

#[derive(Debug)]
pub struct Row {
    number: u8,
    zones: Vec<Zone>,
}

impl Row {
    fn new(number: u8, zones: Vec<Zone>) -> Self {
        Row { number, zones }
    }
}

#[derive(Debug)]
pub struct Zone {
    name: &'static str,
    products: Vec<Product>,
}

impl Zone {
    // TODO: Review life cycle
    fn new(name: &'static str) -> Self {
        Zone { name, products: Vec::new() }
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
