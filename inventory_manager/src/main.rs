use std::{collections::HashMap, ops::Index, time::SystemTime, u8};

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
        occupied_position: None,
    };

    let oversized_item = SomeItem {
        id: 123,
        name: "CoolItem".to_string(),
        quality: Quality::Oversized { size: (2) },
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };
    println!("Before item: {:#?}", _bar);
    _bar.place_normal_item(&normal_item.id(), 0, 0, 0, 0);
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

    fn change_item_position(&mut self, id: &u64, row: usize, shelf: usize, level: usize, starting_zone: usize) -> Result<(), &'static str> {
        let item = self.items.get_mut(id).ok_or("Item not found")?;

        let zones_indexes = match item.quality() {
            Quality::Fragile {
                expiration_date,
                storage_maxlevel,
            } => vec![starting_zone],
            Quality::Oversized { size } => (starting_zone..starting_zone + *size).collect::<Vec<usize>>(),
            Quality::Normal => todo!(),
        };

        item.set_occupied_position(OccupiedPosition {
            row: row,
            shelf,
            level,
            starting_zone,
            zones_indexes,
        });

        Ok(())
    }

    fn place_normal_item(&mut self, id: &u64, row: usize, shelf: usize, level: usize, zone: usize) -> Result<(), &'static str> {
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

        match self.items.get_mut(id) {
            Some(item) => {
                // item.change_position(row, shelf, level, zone);
                *placement_zone = Zone::normal_item(item.id());
            }
            None => {
                println!("Item not found")
            }
        }

        Ok(())
    }

    // fn place_oversized_item(&mut self, id: &u64, row: usize, shelf: usize, level: usize, zone: usize) -> Result<(), &'static str> {
    //     let placement_zone = self
    //         .rows
    //         .get_mut(row)
    //         .ok_or("Invalid row.")?
    //         .shelves
    //         .get_mut(shelf)
    //         .ok_or("Invalid shelf.")?
    //         .levels
    //         .get_mut(level)
    //         .ok_or("Invalid level.")?;
    //
    //     match self.items.get_mut(id) {
    //         Some(item) => {
    //             item.change_position(row, shelf, level, zone);
    //         }
    //         None => {
    //             println!("Item not found")
    //         }
    //     }
    //     Ok(())
    // }
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

    fn normal_item(id: u64) -> Self {
        Self {
            zone_type: ZoneType::NormalItem { id },
        }
    }

    fn oversized_item(id: u64) -> Self {
        Self {
            zone_type: ZoneType::OversizedItem { id },
        }
    }
}

#[derive(Debug)]
enum ZoneType {
    Empty,
    NormalItem { id: u64 },
    OversizedItem { id: u64 },
}

#[derive(Debug)]
enum Quality {
    Fragile { expiration_date: String, storage_maxlevel: String },
    Oversized { size: usize },
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OccupiedPosition {
    row: usize,
    shelf: usize,
    level: usize,
    starting_zone: usize,
    zones_indexes: Vec<usize>,
}

trait WarehouseItem {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn quality(&self) -> &Quality;
    fn quantity(&self) -> u32;
    fn timestamp(&self) -> SystemTime;
    fn occupied_position(&self) -> Option<&OccupiedPosition>;
    fn set_occupied_position(&mut self, position: OccupiedPosition);
    fn change_position(&mut self, row: usize, shelf: usize, level: usize, starting_zone: usize);
}

#[derive(Debug)]
struct SomeItem {
    id: u64,
    name: String,
    quality: Quality,
    quantity: u32,
    timestamp: SystemTime,
    occupied_position: Option<OccupiedPosition>,
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

    fn set_occupied_position(&mut self, position: OccupiedPosition) {
        self.occupied_position = Some(position);
    }

    fn occupied_position(&self) -> Option<&OccupiedPosition> {
        self.occupied_position.as_ref()
    }

    /// TODO: Before using this method, check level (for fragile) and size (for oversized)
    fn change_position(&mut self, row: usize, shelf: usize, level: usize, starting_zone: usize) {
        match self.quality() {
            Quality::Fragile {
                expiration_date,
                storage_maxlevel,
            } => {
                self.occupied_position = Some(OccupiedPosition {
                    row,
                    shelf,
                    level,
                    starting_zone,
                    zones_indexes: vec![starting_zone],
                });
            }
            Quality::Oversized { size } => {
                let zones_indexes: Vec<usize> = (starting_zone..starting_zone + *size).collect::<Vec<_>>();
                self.occupied_position = Some(OccupiedPosition {
                    row,
                    shelf,
                    level,
                    starting_zone,
                    zones_indexes,
                });
            }
            Quality::Normal => {
                self.occupied_position = Some(OccupiedPosition {
                    row,
                    shelf,
                    level,
                    starting_zone,
                    zones_indexes: vec![starting_zone],
                });
            }
        }
    }
}
