use std::fmt::Debug;

pub fn exercise_store() {
    let mut shop: Shop<Product> = Shop::new();
    println!("Initial shop state: {:#?}", shop);

    let soda_product = match Product::new(
        String::from("ID234"),
        String::from("Pepsi"),
        33,
        2.45,
        200,
    ) {
        Ok(product) => product,
        Err(e) => {
            println!("Error creating product: {:?}", e);
            return;
        }
    };

    let mut soda_product = soda_product;
    if let Err(e) = soda_product.change_price(2.55) {
        println!("Error changing price: {:?}", e);
        return;
    }

    match shop.add_product(soda_product, 2, "Soda") {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {:?}", e),
    }
}
pub trait StoreItem: Debug {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn price(&self) -> f64;
    fn set_price(&mut self, new_price: f64) -> Result<(), StoreError>;
    fn stock(&self) -> u16;
    fn add_stock(&mut self, amount: u16);
}

#[derive(Debug)]
pub enum StoreError {
    RowNotFound(u8),
    ZoneNotFound(String),
    ProductNotFound(String),
    ProductAlreadyExists(String),
    InvalidPrice(f64),
    InvalidStock(u16),
    InvalidMove { id: String, row: u8, zone: String },
}

#[derive(Debug)]
struct Shop<T: StoreItem> {
    rows: Vec<Row<T>>,
}

#[derive(Debug)]
struct Row<T: StoreItem> {
    number: u8,
    zones: Vec<Zone<T>>,
}

#[derive(Debug)]
struct Zone<T: StoreItem> {
    name: &'static str,
    products: Vec<T>,
}

// Keep the original Product as one implementation of StoreItem
#[derive(Debug)]
struct Product {
    id: String,
    name: String,
    exp_date: u16,
    price: f64,
    stock: u16,
}

// Implement the StoreItem trait for Product
impl StoreItem for Product {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn set_price(&mut self, new_price: f64) -> Result<(), StoreError> {
        if new_price <= 0.0 {
            return Err(StoreError::InvalidPrice(new_price));
        }
        self.price = new_price;
        Ok(())
    }

    fn stock(&self) -> u16 {
        self.stock
    }

    fn add_stock(&mut self, amount: u16) {
        self.stock += amount;
    }
}

impl<T: StoreItem> Shop<T> {
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
                if zone.products.iter().any(|p| p.id() == product_id) {
                    return Some((row.number, zone.name));
                }
            }
        }
        None
    }

    fn restock(&mut self, product_name: &str, amount: u16) -> Result<(), StoreError> {
        if amount == 0 {
            return Err(StoreError::InvalidStock(amount));
        }

        for row in &mut self.rows {
            for zone in &mut row.zones {
                if let Some(product) = zone.products.iter_mut().find(|p| p.name() == product_name) {
                    product.add_stock(amount);
                    return Ok(());
                }
            }
        }
        Err(StoreError::ProductNotFound(product_name.to_string()))
    }

    fn add_product(&mut self, new_product: T, add_row: u8, add_zone: &str) -> Result<(), StoreError> {
        if self.find_product(new_product.name()).is_some() {
            return Err(StoreError::ProductAlreadyExists(new_product.name().to_string()));
        }

        let row = self
            .rows
            .iter_mut()
            .find(|r| r.number == add_row)
            .ok_or(StoreError::RowNotFound(add_row))?;

        let zone = row
            .zones
            .iter_mut()
            .find(|z| z.name == add_zone)
            .ok_or(StoreError::ZoneNotFound(add_zone.to_string()))?;

        zone.products.push(new_product);
        Ok(())
    }

    fn find_product(&self, product_name: &str) -> Option<&T> {
        for row in &self.rows {
            for zone in &row.zones {
                if let Some(product) = zone.products.iter().find(|p| p.name() == product_name) {
                    return Some(product);
                }
            }
        }
        None
    }

    fn move_product(&mut self, to_row: u8, to_zone: &str, id_to_move: &str) -> Result<(), StoreError> {
        let mut source_row = None;
        let mut source_zone = None;
        let mut product_idx = None;

        'outer: for (row_idx, row) in self.rows.iter().enumerate() {
            for (zone_idx, zone) in row.zones.iter().enumerate() {
                if let Some(idx) = zone.products.iter().position(|p| p.id() == id_to_move) {
                    source_row = Some(row_idx);
                    source_zone = Some(zone_idx);
                    product_idx = Some(idx);
                    break 'outer;
                }
            }
        }

        let (source_row, source_zone, product_idx) = match (source_row, source_zone, product_idx) {
            (Some(r), Some(z), Some(i)) => (r, z, i),
            _ => return Err(StoreError::ProductNotFound(id_to_move.to_string())),
        };

        let dest_row_idx = self
            .rows
            .iter()
            .position(|r| r.number == to_row)
            .ok_or(StoreError::RowNotFound(to_row))?;

        let dest_zone_idx = self.rows[dest_row_idx]
            .zones
            .iter()
            .position(|z| z.name == to_zone)
            .ok_or(StoreError::ZoneNotFound(to_zone.to_string()))?;

        let product = self.rows[source_row].zones[source_zone].products.remove(product_idx);
        self.rows[dest_row_idx].zones[dest_zone_idx].products.push(product);

        Ok(())
    }

    fn remove_product(&mut self, product_id: &str) -> Result<T, StoreError> {
        for row in &mut self.rows {
            for zone in &mut row.zones {
                if let Some(pos) = zone.products.iter().position(|p| p.id() == product_id) {
                    return Ok(zone.products.remove(pos));
                }
            }
        }
        Err(StoreError::ProductNotFound(product_id.to_string()))
    }
}

impl<T: StoreItem> Row<T> {
    fn new(number: u8, zones: Vec<Zone<T>>) -> Self {
        Row { number, zones }
    }
}

impl<T: StoreItem> Zone<T> {
    fn new(name: &'static str) -> Self {
        Zone {
            name,
            products: Vec::new(),
        }
    }
}

impl Product {
    fn new(id: String, name: String, exp_date: u16, price: f64, stock: u16) -> Result<Self, StoreError> {
        if price <= 0.0 {
            return Err(StoreError::InvalidPrice(price));
        }
        if stock == 0 {
            return Err(StoreError::InvalidStock(stock));
        }

        Ok(Self {
            name,
            id,
            exp_date,
            price,
            stock,
        })
    }

    fn change_price(&mut self, new_price: f64) -> Result<(), StoreError> {
        self.set_price(new_price)
    }
}


