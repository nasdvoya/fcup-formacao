use std::{ops::Index, time::SystemTime};

fn main() {
    test_function()
}

fn test_function() {
    let _foo: Shelf<SomeItem> = Shelf::new(10);
    println!("This test shelf {:#?}", _foo);
    let _bar: Warehouse<SomeItem> = Warehouse::new(2, 3, 1);
    println!("This test warehouse {:#?}", _bar);
}

#[derive(Debug)]
struct Warehouse<T: WarehouseItem> {
    rows: Vec<Row<T>>,
}

impl<T: WarehouseItem> Warehouse<T> {}

#[derive(Debug)]
struct Row<T: WarehouseItem> {
    shelves: Vec<Shelf<T>>,
}

impl<T: WarehouseItem> Row<T> {
    fn new(shelves: usize, zones: usize) -> Self {
        let shelves: Vec<Shelf<T>> = (0..shelves).map(|_| Shelf::new(zones)).collect();
        Self { shelves }
    }
}

#[derive(Debug)]
struct Shelf<'a, T: WarehouseItem> {
    levels: Vec<Level<'a, T>>,
}

impl<'a, T: WarehouseItem> Shelf<'a, T> {
    fn new(levels: usize, zones: usize) -> Self {
        let levels = (0..zones).map(|_| Level::new(zones)).collect();
        Self { levels }
    }
}

#[derive(Debug)]
struct Level<'a, T: WarehouseItem> {
    zones: Vec<Zone<'a, T>>,
}

impl<'a, T: WarehouseItem> Level<'a, T> {
    fn new(zones: usize) -> Self {
        let zones = (0..zones).map(|_| Zone::new_empty()).collect();
        Self { zones }
    }
}

#[derive(Debug)]
struct Zone<'a, T: WarehouseItem> {
    zone_type: ZoneType<'a, T>,
}

impl<'a, T: WarehouseItem> Zone<'a, T> {
    fn new_empty() -> Self {
        Self { zone_type: ZoneType::Empty }
    }
    fn new_normal(item: T) -> Self {
        Self {
            zone_type: ZoneType::NormalItem(item),
        }
    }
    fn new_oversized(item: &'a T) -> Self {
        Self {
            zone_type: ZoneType::OversizedItem(item),
        }
    }
}

#[derive(Debug)]
enum ZoneType<'a, T: WarehouseItem> {
    Empty,
    NormalItem(T),
    OversizedItem(&'a T),
}

#[derive(Debug)]
enum Quality {
    Fragile { expiration_date: String, storage_maxlevel: String },
    Oversized { size: u16 },
    Normal,
}

trait WarehouseItem {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn quality(&self) -> &Quality;
    fn quantity(&self) -> u32;
    fn timestamp(&self) -> SystemTime;
}

#[derive(Debug)]
struct SomeItem {
    id: u64,
    name: String,
    quality: Quality,
    quantity: u32,
    timestamp: SystemTime,
}

impl WarehouseItem for SomeItem {
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

// impl<T: WarehouseItem> Warehouse<T> {
//     fn new(rows: usize, shelves: usize, levels: usize, zones: usize) -> Self {
//         let rows: Vec<Row<T>> = (0..rows).map(|_| Row::new(shelves, zones)).collect();
//         Self { rows }
//     }
//
//     // methods
//     //
//     fn item_allocation(&mut self, item: T) -> (usize, usize, usize) {
//         match item.quality() {
//             Quality::Normal => {
//                 for (row_index, row) in self.rows.iter_mut().enumerate() {
//                     for (shelf_index, shelf) in row.shelves.iter_mut().enumerate() {
//                         for (zone_index, zone) in shelf.zones.iter_mut().enumerate() {
//                             if let None = &zone.item {
//                                 zone.item = Some(item);
//                                 return (row_index, shelf_index, zone_index);
//                             }
//                         }
//                     }
//                 }
//             }
//             Quality::Fragile {
//                 expiration_date,
//                 storage_maxlevel,
//             } => {}
//             Quality::Oversized { size } => {}
//         }
//         todo!()
//     }
//
//     fn item_placement(&mut self, item: T) {
//         match item.quality() {
//             Quality::Fragile {
//                 expiration_date,
//                 storage_maxlevel,
//             } => todo!(),
//             Quality::Oversized { size } => todo!(),
//             Quality::Normal => {
//                 for (row_index, row) in self.rows.iter_mut().enumerate() {
//                     for (shelf_index, shelf) in row.shelves.iter_mut().enumerate() {
//                         for (zone_index, zone) in shelf.zones.iter_mut().enumerate() {
//                             if let None = &zone.item {
//                                 zone.item = Some(item);
//                                 return;
//                             } else {
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
