use chrono::{DateTime, Duration, Utc};

use crate::item::{OccupiedPosition, Quality, WarehouseItem};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Warehouse<T: WarehouseItem> {
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

#[derive(Debug)]
enum ZoneType {
    Empty,
    NormalItem { id: u64 },
    OversizedItem { id: u64 },
}

pub enum SearchType {
    ById { id: u64 },
    ByName(String),
}

impl<T: WarehouseItem> Warehouse<T> {
    pub fn new(rows: usize, shelves: usize, levels: usize, zones: usize) -> Self {
        let rows: Vec<Row> = (0..rows).map(|_| Row::new(shelves, levels, zones)).collect();
        let items: HashMap<u64, T> = HashMap::new();
        Self { rows, items }
    }

    pub fn add_item(&mut self, item: T) -> Result<(), &'static str> {
        if self.items.contains_key(&item.id()) {
            return Err("Item with specified id already exists.");
        }
        self.items.insert(item.id(), item);
        Ok(())
    }

    pub fn place_item(
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
            .ok_or("Invalid row.")?
            .shelves
            .get_mut(shelf)
            .ok_or("Invalid shelf.")?
            .levels
            .get_mut(level)
            .ok_or("Invalid level.")?;

        let zones_indexes = match item.quality() {
            Quality::Normal | Quality::Fragile { .. } => {
                if let Quality::Fragile { storage_maxlevel, .. } = item.quality() {
                    if level >= *storage_maxlevel {
                        return Err("Item cannot be stored above the maximum level.");
                    }
                }
                let placement_zone = item_level.zones.get_mut(start_zone).ok_or("Invalid zone.")?;
                *placement_zone = Zone::normal_item(item.id());
                vec![start_zone]
            }
            Quality::Oversized { size } => {
                let mut zones = vec![];
                for z_index in start_zone..start_zone + size {
                    let placement_zone = item_level.zones.get_mut(z_index).ok_or("Invalid zone.")?;
                    *placement_zone = Zone::oversized_item(item.id());
                    zones.push(z_index);
                }
                zones
            }
        };
        item.set_occupied_position(Some(OccupiedPosition {
            row,
            shelf,
            level,
            start_zone,
            zones_indexes,
        }));
        Ok(())
    }

    pub fn remove_item(&mut self, id: &u64) -> Result<(), &'static str> {
        if let Some(item) = self.items.get_mut(id) {
            item.set_occupied_position(None);
        }
        Ok(())
    }

    /// Since the exercise does not specify, search by name returns the first item found.
    pub fn get_item_quantity(&self, search_type: SearchType) -> (bool, Option<u32>) {
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

    pub fn get_item_location(&self, name: String) -> Vec<&OccupiedPosition> {
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

    pub fn sort_items(&self) -> Vec<&T> {
        let mut items: Vec<&T> = self.items.values().collect();
        items.sort_by(|a, b| a.name().cmp(b.name()));
        items
    }

    /// Get items that expired or are within 3 days of expiration
    /// # Arguments
    /// * `date` - DateTime to check expiration date
    pub fn get_expire_items(&self, date: DateTime<Utc>) -> Vec<&T> {
        let threshold = date + Duration::days(3);

        self.items
            .values()
            .filter(|item| match item.quality() {
                Quality::Fragile { expiration_date, .. } => {
                    println!("Expiration date: {:#?}", expiration_date);
                    *expiration_date <= threshold
                }
                _ => false,
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
