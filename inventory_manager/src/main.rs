use std::{collections::HashMap, time::SystemTime};

fn main() {
    let mut _warehouse: Warehouse<SomeItem> = Warehouse::new(1, 2, 2, 8);
    let normal_item = SomeItem {
        id: 125,
        name: "CoolItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };

    let normal_item_again = SomeItem {
        id: 127,
        name: "CoolItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };
    let oversized_item = SomeItem {
        id: 121,
        name: "CoolOversized".to_string(),
        quality: Quality::Oversized { size: (2) },
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };
    println!("Before item: {:#?}", _warehouse);
    let item_ref: &u64 = &normal_item.id();
    let item_ref_again: &u64 = &normal_item_again.id();
    let oversized_item_ref: &u64 = &oversized_item.id();

    match _warehouse.add_item(normal_item) {
        Ok(_) => {
            println!("Success, item added");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    match _warehouse.add_item(normal_item_again) {
        Ok(_) => {
            println!("Success, item added again");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    match _warehouse.add_item(oversized_item) {
        Ok(_) => {
            println!("Success, item added oversized");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    println!("After item: {:#?}", _warehouse);
    match _warehouse.update_item_position(item_ref, 0, 0, 0, 0) {
        Ok(_) => {
            println!("Success, item position updated");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    match _warehouse.update_item_position(item_ref_again, 0, 1, 1, 0) {
        Ok(_) => {
            println!("Success, item position updated again");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    match _warehouse.update_item_position(oversized_item_ref, 0, 1, 0, 1) {
        Ok(_) => {
            println!("Success, item position updated oversized");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    println!("After udate: {:#?}", _warehouse);
    let (found, quantity) = _warehouse.get_item_quantity(SearchType::ById { id: 125 });
    println!("Found: {:?}, quantity: {:?}", found, quantity);
    let (found, quantity) = _warehouse.get_item_quantity(SearchType::ByName("CoolItem".to_string()));
    println!("Found: {:?}, quantity: {:?}", found, quantity);
    let location = _warehouse.get_item_location("CoolItem".to_string());
    println!("Location: {:?}", location);
}

#[derive(Debug)]
enum ZoneType {
    Empty,
    NormalItem { id: u64 },
    OversizedItem { id: u64 },
}

#[derive(Debug)]
enum Quality {
    Fragile {
        expiration_date: String,
        storage_maxlevel: String,
    },
    Oversized {
        size: usize,
    },
    Normal,
}

enum SearchType {
    ById { id: u64 },
    ByName(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OccupiedPosition {
    row: usize,
    shelf: usize,
    level: usize,
    start_zone: usize,
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

    fn add_item(&mut self, item: T) -> Result<(), &'static str> {
        if self.items.contains_key(&item.id()) {
            return Err("Item with specified id already exists.");
        }
        self.items.insert(item.id(), item);
        Ok(())
    }

    fn update_item_position(
        &mut self,
        id: &u64,
        row: usize,
        shelf: usize,
        level: usize,
        start_zone: usize,
    ) -> Result<(), &'static str> {
        let item = self.items.get_mut(id).ok_or("Item not found")?;

        let item_level = self
            .rows
            .get_mut(row)
            .ok_or("Item location: Invalid row.")?
            .shelves
            .get_mut(shelf)
            .ok_or("Item location: Invalid shelf.")?
            .levels
            .get_mut(level)
            .ok_or("Item location: Invalid level.")?;

        let zones_indexes = match item.quality() {
            Quality::Fragile { .. } | Quality::Normal => {
                let placement_zone = item_level
                    .zones
                    .get_mut(start_zone)
                    .ok_or("Item location: Invalid zone.")?;

                *placement_zone = Zone::normal_item(item.id());
                vec![start_zone]
            }
            Quality::Oversized { size } => {
                for z_index in start_zone..start_zone + size {
                    let placement_zone = item_level
                        .zones
                        .get_mut(z_index)
                        .ok_or("Item location: Invalid zone.")?;

                    *placement_zone = Zone::oversized_item(item.id());
                }
                (start_zone..start_zone + size).collect::<Vec<usize>>()
            }
        };

        item.set_occupied_position(OccupiedPosition {
            row,
            shelf,
            level,
            start_zone,
            zones_indexes,
        });
        Ok(())
    }

    /// Since the exercise does not specify, search by name returns the first item found.
    fn get_item_quantity(&self, search_type: SearchType) -> (bool, Option<u32>) {
        match search_type {
            SearchType::ById { id } => match self.items.get(&id) {
                Some(item) => (true, Some(item.quantity())),
                None => (false, None),
            },
            SearchType::ByName(name) => match self.items.values().find(|item| item.name() == name.as_str()) {
                Some(item) => (true, Some(item.quantity())),
                None => (false, None),
            },
        }
    }

    fn get_item_location(&self, name: String) -> Vec<&OccupiedPosition> {
        self.items
            .values()
            .filter_map(|item| {
                if item.name() == name.as_str() {
                    item.occupied_position()
                } else {
                    None
                }
            })
            .collect()
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
}
