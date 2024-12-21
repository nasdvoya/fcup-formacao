pub fn exercise_store() {
    let mut ashop = Shop::new();
    println!("Shop: {:#?}", ashop);
    let mut soda_product = Product::new(String::from("ID234"), String::from("Pepsi"), 33, 2.45, 200);
    soda_product.change_price(2.55);
    ashop.add_product(soda_product, 2, "Soda");
    println!("Add product: {:#?}", ashop);
    ashop.restock(String::from("Pepsi"), 100);
    println!("Restock: {:#?}", ashop);
    ashop.move_product(1, "Sweets", String::from("ID234"));
    println!("Move: {:#?}", ashop);

    if let Some((row, zone)) = ashop.locate_product("ID234") {
        println!("Product ID234 is located in row {} and zone {}", row, zone);
    } else {
        println!("Product ID234 was not found.");
    }
}

#[derive(Debug)]
struct Shop {
    rows: Vec<Row>,
}

#[derive(Debug)]
struct Row {
    number: u8,
    zones: Vec<Zone>,
}

#[derive(Debug)]
struct Zone {
    name: &'static str,
    products: Vec<Product>,
}

#[derive(Debug)]
struct Product {
    id: String,
    name: String,
    exp_date: u16,
    price: f64,
    stock: u16,
}

impl Shop {
    fn new() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::new(
            1,
            vec![Zone::new("Salty"), Zone::new("Sweets"), Zone::new("Drinks")],
        ));
        rows.push(Row::new(2, vec![Zone::new("Soda"), Zone::new("Water"), Zone::new("Alcohol")]));
        rows.push(Row::new(
            3,
            vec![Zone::new("Yogurt"), Zone::new("Butter"), Zone::new("Salad")],
        ));

        Self { rows }
    }

    fn locate_product(&self, product_id: &str) -> Option<(u8, &str)> {
        for row in &self.rows {
            for zone in &row.zones {
                if zone.products.iter().any(|p| p.id == product_id) {
                    return Some((row.number, zone.name));
                }
            }
        }
        None
    }

    fn restock(&mut self, product: String, amount: u16) {
        for row in &mut self.rows {
            for zone in &mut row.zones {
                if let Some(product) = zone.products.iter_mut().find(|p| p.name == product) {
                    product.stock += amount;
                    return;
                }
            }
        }
    }

    fn add_product(&mut self, new_product: Product, add_row: u8, add_zone: &str) {
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

    fn move_product(&mut self, to_row: u8, to_zone: &str, id_to_move: String) {
        let mut product_to_move: Option<Product> = None;
        for row in &mut self.rows {
            for zone in &mut row.zones {
                println!("a222");
                println!("{}a", id_to_move);
                if let Some(pos) = zone.products.iter().position(|p| p.id == id_to_move) {
                    println!("222");
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
                        println!("Product moved.");
                        return;
                    } else {
                        println!("Product not moved.")
                    }
                }
            }
        } else {
            println!("Product with ID {} was not found", id_to_move);
        }
    }
}

impl Row {
    fn new(number: u8, zones: Vec<Zone>) -> Self {
        Row { number, zones }
    }
}

impl Zone {
    // TODO: Review life cycle
    fn new(name: &'static str) -> Self {
        Zone {
            name,
            products: Vec::new(),
        }
    }
}

impl Product {
    fn new(id: String, name: String, exp_date: u16, price: f64, stock: u16) -> Self {
        Self {
            name,
            id,
            exp_date,
            price,
            stock,
        }
    }

    fn change_price(&mut self, new_price: f64) {
        self.price = new_price;
    }
}
