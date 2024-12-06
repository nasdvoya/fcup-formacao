use std::{collections::HashMap, ops::Index, time::SystemTime};

fn main() {
    test_function()
}

fn test_function() {
    let mut _bar: Warehouse<SomeItem> = Warehouse::new(1, 2, 2, 2);
    let normal_item = SomeItem {
        id: 123,
        name: "CoolItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: SystemTime::now(),
        allocated_position: None,
    };

    let oversized_item = SomeItem {
        id: 123,
        name: "CoolItem".to_string(),
        quality: Quality::Oversized { size: (2) },
        quantity: 100,
        timestamp: SystemTime::now(),
        allocated_position: None,
    };
    println!("Before item: {:#?}", _bar);
    _bar.place_normal_item(normal_item, 0, 0, 0, 0);
    // _bar.place_item(oversized_item, 0, 0, 0, 1);
    println!("After item: {:#?}", _bar)
}

#[derive(Debug)]
struct Warehouse<T: WarehouseItem> {
    rows: Vec<Row>,
    items: HashMap<u64, T>,
}

#[derive(Debug)]
struct Row {
    shelves: Vec<Shelf>,
}

#[derive(Debug)]
struct Shelf {
    levels: Vec<Level>,
}

#[derive(Debug)]
struct Level {
    zones: Vec<Zone>,
}

#[derive(Debug)]
struct Zone {
    zone_type: ZoneType,
}

impl<T: WarehouseItem> Warehouse<T> {
    fn new(rows: usize, shelves: usize, levels: usize, zones: usize) -> Self {
        let rows: Vec<Row> = (0..rows).map(|_| Row::new(shelves, levels, zones)).collect();
        let items: HashMap<u64, T> = HashMap::new();

        Self { rows, items }
    }

    // WARNING: Chnage item to id, name, etc. Read notes.
    fn place_normal_item(&mut self, mut item: T, row: usize, shelf: usize, level: usize, zone: usize) -> Result<(), &'static str> {
        let placement_zone = self
            .rows
            .get_mut(row)
            .ok_or("Invalid row.")?
            .shelves
            .get_mut(shelf)
            .ok_or("Invalid shelf.")?
            .levels
            .get_mut(level)
            .ok_or("Invalid level.")?
            .zones
            .get_mut(zone)
            .ok_or("Invalid zone.")?;

        match self.items.get_mut(&item.id()) {
            Some(item) => {
                item.change_position(row, shelf, level, zone);
                *placement_zone = Zone::normal_item();
            }
            None => {
                println!("Item not found")
            }
        }

        Ok(())
    }

    fn place_oversized_item(&mut self, item: T, row: usize, shelf: usize, level: usize, zone: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

impl Row {
    fn new(shelves: usize, levels: usize, zones: usize) -> Self {
        let shelves: Vec<Shelf> = (0..shelves).map(|_| Shelf::new(levels, zones)).collect();
        Self { shelves }
    }
}

impl Shelf {
    fn new(levels: usize, zones: usize) -> Self {
        let levels = (0..levels).map(|_| Level::new(zones)).collect();
        Self { levels }
    }
}

impl Level {
    fn new(zones: usize) -> Self {
        let zones = (0..zones).map(|_| Zone::new()).collect();
        Self { zones }
    }
}

impl Zone {
    fn new() -> Self {
        Self {
            zone_type: ZoneType::Empty,
        }
    }

    fn normal_item() -> Self {
        Self {
            zone_type: ZoneType::NormalItem,
        }
    }

    fn oversized_item() -> Self {
        Self {
            zone_type: ZoneType::OversizedItem,
        }
    }
}

#[derive(Debug)]
enum ZoneType {
    Empty,
    NormalItem,
    OversizedItem,
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
    fn occupied_position(&self) -> Option<(usize, usize, usize, usize)>;
    fn change_position(&mut self, row: usize, shelf: usize, level: usize, zone: usize);
    // fn occupied_position(&self) -> Option<(usize, usize, usize)>;
}

#[derive(Debug)]
struct SomeItem {
    id: u64,
    name: String,
    quality: Quality,
    quantity: u32,
    timestamp: SystemTime,
    allocated_position: Option<(usize, usize, usize, usize)>,
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
    fn occupied_position(&self) -> Option<(usize, usize, usize, usize)> {
        self.allocated_position
    }

    fn change_position(&mut self, row: usize, shelf: usize, level: usize, zone: usize) {
        self.allocated_position = Some((row, shelf, level, zone));
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
