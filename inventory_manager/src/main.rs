mod item;
mod warehouse;

use item::{Quality, SomeItem};
use std::time::SystemTime;
use warehouse::Warehouse;

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
        quality: Quality::Oversized { size: 2 },
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };

    // Add items and update positions
    println!("Before item: {:#?}", _warehouse);
    _warehouse.add_item(normal_item).unwrap();
    _warehouse.add_item(normal_item_again).unwrap();
    _warehouse.add_item(oversized_item).unwrap();

    println!("After item: {:#?}", _warehouse);
    _warehouse.update_item_position(&125, 0, 0, 0, 0).unwrap();
    _warehouse.update_item_position(&127, 0, 1, 1, 0).unwrap();
    _warehouse.update_item_position(&121, 0, 1, 0, 1).unwrap();

    println!("After update: {:#?}", _warehouse);

    let locations = _warehouse.get_item_location("CoolItem".to_string());
    for location in locations {
        println!("Location: {:?}", location);
    }
}
