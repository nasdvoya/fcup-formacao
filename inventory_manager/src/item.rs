use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccupiedPosition {
    pub row: usize,
    pub shelf: usize,
    pub level: usize,
    pub start_zone: usize,
    pub zones_indexes: Vec<usize>,
}

pub trait WarehouseItem {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn quality(&self) -> &Quality;
    fn quantity(&self) -> u32;
    fn timestamp(&self) -> SystemTime;
    fn occupied_position(&self) -> Option<&OccupiedPosition>;
    fn set_occupied_position(&mut self, position: OccupiedPosition);
}

#[derive(Debug)]
pub struct SomeItem {
    pub id: u64,
    pub name: String,
    pub quality: Quality,
    pub quantity: u32,
    pub timestamp: SystemTime,
    pub occupied_position: Option<OccupiedPosition>,
}

#[derive(Debug)]
pub enum Quality {
    Fragile {
        expiration_date: String,
        storage_maxlevel: usize,
    },
    Oversized {
        size: usize,
    },
    Normal,
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
    fn occupied_position(&self) -> Option<&OccupiedPosition> {
        self.occupied_position.as_ref()
    }
    fn set_occupied_position(&mut self, position: OccupiedPosition) {
        self.occupied_position = Some(position);
    }
}
