use std::{ops::Index, time::SystemTime};

fn main() {
    test_function()
}

fn test_function() {
    let mut _bar: Warehouse<SomeItem> = Warehouse::new(3, 3, 4, 4);
    let normal_item = SomeItem {
        id: 123,
        name: "CoolItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: SystemTime::now(),
    };

    let oversized_item = SomeItem {
        id: 123,
        name: "CoolItem".to_string(),
        quality: Quality::Oversized { size: (2) },
        quantity: 100,
        timestamp: SystemTime::now(),
    };
    // _bar.place_item(normal_item, 0, 0, 0, 0);
    // _bar.place_item(oversized_item, 0, 0, 0, 1);
    println!("This warehouse: {:#?}", _bar)
}

#[derive(Debug)]
struct Warehouse<T: WarehouseItem> {
    rows: Vec<Row<T>>,
}

#[derive(Debug)]
struct Row<T: WarehouseItem> {
    shelves: Vec<Shelf<T>>,
}

#[derive(Debug)]
struct Shelf<T: WarehouseItem> {
    levels: Vec<Level<T>>,
}

#[derive(Debug)]
struct Level<T: WarehouseItem> {
    zones: Vec<Zone<T>>,
}

#[derive(Debug)]
struct Zone<T: WarehouseItem> {
    zone_type: ZoneType<T>,
}

impl<T: WarehouseItem> Warehouse<T> {
    fn new(rows: usize, shelves: usize, levels: usize, zones: usize) -> Self {
        let rows: Vec<Row<T>> = (0..rows).map(|_| Row::new(shelves, levels, zones)).collect();
        Self { rows }
    }
}

impl<T: WarehouseItem> Row<T> {
    fn new(shelves: usize, levels: usize, zones: usize) -> Self {
        let shelves: Vec<Shelf<T>> = (0..shelves).map(|_| Shelf::new(levels, zones)).collect();
        Self { shelves }
    }
}

impl<T: WarehouseItem> Shelf<T> {
    fn new(levels: usize, zones: usize) -> Self {
        let levels = (0..levels).map(|_| Level::new(zones)).collect();
        Self { levels }
    }
}

impl<T: WarehouseItem> Level<T> {
    fn new(zones: usize) -> Self {
        let zones = (0..zones).map(|_| Zone::new_empty()).collect();
        Self { zones }
    }
}

impl<T: WarehouseItem> Zone<T> {
    fn new_empty() -> Self {
        Self {
            zone_type: ZoneType::Empty,
        }
    }
    fn new_normal(item: T) -> Self {
        Self {
            zone_type: ZoneType::NormalItem(item),
        }
    }
    fn new_oversized(item: T) -> Self {
        Self {
            zone_type: ZoneType::OversizedItem(item),
        }
    }
}

#[derive(Debug)]
enum ZoneType<T: WarehouseItem> {
    Empty,
    NormalItem(T),
    OversizedItem(T),
}

#[derive(Debug)]
enum Quality {
    Fragile { expiration_date: String, storage_maxlevel: String },
    Oversized { size: u32 },
    Normal,
}

trait WarehouseItem {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn quality(&self) -> &Quality;
    fn quantity(&self) -> u32;
    fn timestamp(&self) -> SystemTime;
    // fn occupied_position(&self) -> Option<(usize, usize, usize)>;
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

// impl<'a, T: WarehouseItem> Warehouse<T> {
//     fn new(rows: usize, shelves: usize, levels: usize, zones: usize) -> Self {
//         let rows: Vec<Row<T>> = (0..rows).map(|_| Row::new(shelves, levels, zones)).collect();
//         Self { rows }
//     }
//
//     fn place_normal_item(&mut self, item: T, row: usize, shelf: usize, level: usize, zone: usize) -> Result<(), &'static str> {
//         let placement_zone = self
//             .rows
//             .get_mut(row)
//             .ok_or("Invalid row.")?
//             .shelves
//             .get_mut(shelf)
//             .ok_or("Invalid shelf.")?
//             .levels
//             .get_mut(level)
//             .ok_or("Invalid level.")?
//             .zones
//             .get_mut(zone)
//             .ok_or("Invalid zone.")?;
//
//         *placement_zone = Zone::new_normal(item);
//
//         Ok(())
//     }
//
//     fn place_oversized_item(&mut self, item: T, row: usize, shelf: usize, level: usize, zone: usize) -> Result<(), &'static str> {
//         let size = match item.quality() {
//             Quality::Oversized { size } => size,
//             _ => return Err("Item is not oversized"),
//         };
//         let placement_zone = self
//             .rows
//             .get_mut(row)
//             .ok_or("Invalid row.")?
//             .shelves
//             .get_mut(shelf)
//             .ok_or("Invalid shelf.")?
//             .levels
//             .get_mut(level)
//             .ok_or("Invalid level.")?;
//
//         placement_zone.zones[zone] = Zone::new_oversized(item);
//
//         Ok(())
//     }
// }
