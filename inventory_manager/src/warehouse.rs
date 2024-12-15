use chrono::{DateTime, Duration, Utc};

use crate::item::{OccupiedPosition, Quality, WarehouseItem};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Warehouse<T: WarehouseItem> {
    rows: Vec<Row>,
    items: HashMap<u64, T>,
    robin_tracker: (usize, usize, usize, usize),
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

pub enum PlacementStrategy {
    FirstAvailable,
    RoundRobin,
}

impl<T: WarehouseItem> Warehouse<T> {
    pub fn new(rows: usize, shelves: usize, levels: usize, zones: usize) -> Self {
        let rows: Vec<Row> = (0..rows).map(|_| Row::new(shelves, levels, zones)).collect();
        let items: HashMap<u64, T> = HashMap::new();
        Self {
            rows,
            items,
            robin_tracker: (0, 0, 0, 0),
        }
    }

    pub fn item_placement(&mut self, strategy: PlacementStrategy, item: T) -> Result<(), &'static str> {
        if self.items.contains_key(&item.id()) {
            return Err("Item with specified ID already exists.");
        }

        match strategy {
            PlacementStrategy::FirstAvailable => self.first_available_placement(item),
            PlacementStrategy::RoundRobin => self.round_robin_placement(item),
        }
    }

    fn first_available_placement(&mut self, item: T) -> Result<(), &'static str> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (shelf_idx, shelf) in row.shelves.iter().enumerate() {
                for (level_idx, level) in shelf.levels.iter().enumerate() {
                    if let Some(valid_zones) = self.find_valid_zones(&item, &level.zones, 0) {
                        return self.place_item(item, row_idx, shelf_idx, level_idx, valid_zones[0]);
                    }
                }
            }
        }
        Err("No suitable space available in the warehouse.")
    }

    fn round_robin_placement(&mut self, item: T) -> Result<(), &'static str> {
        let (mut row, mut shelf, mut level, mut zone) = self.robin_tracker;

        loop {
            let current_row = self.rows.get(row).ok_or("RoundRobin: Invalid row")?;
            let current_shelf = current_row.shelves.get(shelf).ok_or("RoundRobin: Invalid shelf")?;
            let current_level = current_shelf.levels.get(level).ok_or("RoundRobin: Invalid level")?;

            if let Some(valid_zones) = self.find_valid_zones(&item, &current_level.zones, zone) {
                // Move the item into place_item
                let result = self.place_item(item, row, shelf, level, valid_zones[0]);
                if result.is_ok() {
                    println!("Item placed successfully");
                    self.update_robin_tracker(row, shelf, level, valid_zones.last().unwrap() + 1);
                }
                return result;
            }

            // Increment through the zones in round-robin fashion
            zone += 1;
            if zone >= current_level.zones.len() {
                zone = 0;
                level += 1;

                if level >= current_shelf.levels.len() {
                    level = 0;
                    shelf += 1;

                    if shelf >= current_row.shelves.len() {
                        shelf = 0;
                        row += 1;

                        if row >= self.rows.len() {
                            return Err("No suitable space available in the warehouse.");
                        }
                    }
                }
            }
        }
    }

    fn find_valid_zones(&self, item: &T, zones: &[Zone], start_zone: usize) -> Option<Vec<usize>> {
        match item.quality() {
            Quality::Normal | Quality::Fragile { .. } => zones
                .iter()
                .enumerate()
                .skip(start_zone)
                .find(|(_, zone)| matches!(zone.zone_type, ZoneType::Empty))
                .map(|(index, _)| vec![index]),
            Quality::Oversized { size } => {
                for start in start_zone..=zones.len() - size {
                    let block = &zones[start..start + size];
                    if block.iter().all(|zone| matches!(zone.zone_type, ZoneType::Empty)) {
                        return Some((start..start + size).collect());
                    }
                }
                None
            }
        }
    }

    fn update_robin_tracker(&mut self, row: usize, shelf: usize, level: usize, zone: usize) {
        self.robin_tracker = (row, shelf, level, zone);
    }

    pub fn place_item(
        &mut self,
        item: T,
        row: usize,
        shelf: usize,
        level: usize,
        start_zone: usize,
    ) -> Result<(), &'static str> {
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

        // Update the item's occupied position.
        let occupied_position = OccupiedPosition {
            row,
            shelf,
            level,
            start_zone,
            zones_indexes,
        };

        // Move the item into the warehouse's item list.
        let id = item.id();
        self.items.insert(id, {
            let mut moved_item = item;
            moved_item.set_occupied_position(Some(occupied_position));
            moved_item
        });

        Ok(())
    }

    pub fn remove_item(&mut self, id: &u64) -> Result<(), &'static str> {
        if let Some(item) = self.items.remove(id) {
            if let Some(position) = item.occupied_position() {
                let row = self.rows.get_mut(position.row).ok_or("Invalid row")?;
                let shelf = row.shelves.get_mut(position.shelf).ok_or("Invalid shelf")?;
                let level = shelf.levels.get_mut(position.level).ok_or("Invalid level")?;

                for zone_index in &position.zones_indexes {
                    if let Some(zone) = level.zones.get_mut(*zone_index) {
                        *zone = Zone::new();
                    }
                }
            }
            Ok(())
        } else {
            Err("Item not found")
        }
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
