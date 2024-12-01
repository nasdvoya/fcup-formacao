use std::time::SystemTime;

fn main() {}

struct Warehouse<T: WarehouseItem> {
    Rows: Vec<Row<T>>,
}

impl<T: WarehouseItem> Warehouse<T> {
    fn new() -> Self {
        todo!()
    }
}

struct Row<T: WarehouseItem> {
    Shelfs: Vec<Shelf<T>>,
}

struct Shelf<T: WarehouseItem> {
    Zones: Vec<Zone<T>>,
}

struct Zone<T: WarehouseItem> {
    item: Option<T>,
}

trait WarehouseItem {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn quality(&self) -> &Quality;
    fn quantity(&self) -> u32;
    fn timestamp(&self) -> SystemTime;
}

struct SomeTime {
    id: u64,
    name: String,
    quality: Quality,
    quantity: u32,
    timestamp: SystemTime,
}

impl WarehouseItem for SomeTime {
    fn id(&self) -> u64 {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn quality(&self) -> &Quality {
        &self.quality
    }
    fn quantity(&self) -> u32 {
        self.quantity
    }
    fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

enum Quality {
    Fragile { expiration_date: String, storage_maxlevel: String },
    Oversized { size: u16 },
    Normal,
}
