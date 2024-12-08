mod item;
mod warehouse;

use item::{Quality, SomeItem};
use std::time::SystemTime;
use warehouse::{SearchType, Warehouse};

fn main() {
    let mut _warehouse: Warehouse<SomeItem> = Warehouse::new(1, 2, 2, 8);
    let normal_item = SomeItem {
        id: 125,
        name: "BCoolItem".to_string(),
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
        name: "ACoolOversized".to_string(),
        quality: Quality::Oversized { size: 2 },
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };
    let expired_item = SomeItem {
        id: 123,
        name: "ExpiredItem".to_string(),
        quality: Quality::Fragile {
            expiration_date: "2022-01-01".to_string(),
            storage_maxlevel: 100,
        },
        quantity: 100,
        timestamp: SystemTime::now(),
        occupied_position: None,
    };

    // Add items and update positions
    println!("Before item: {:#?}", _warehouse);
    _warehouse.add_item(normal_item).unwrap();
    _warehouse.add_item(normal_item_again).unwrap();
    _warehouse.add_item(oversized_item).unwrap();
    _warehouse.add_item(expired_item).unwrap();

    println!("After item: {:#?}", _warehouse);
    _warehouse.update_item_position(&125, 0, 0, 0, 0).unwrap();
    _warehouse.update_item_position(&127, 0, 1, 1, 0).unwrap();
    _warehouse.update_item_position(&121, 0, 1, 0, 1).unwrap();
    _warehouse.update_item_position(&123, 0, 0, 1, 0).unwrap();

    // println!("After update: {:#?}", _warehouse);
    //
    // // Get item quantity example
    // let item_quantity = _warehouse.get_item_quantity(SearchType::ByName("CoolItem".to_string()));
    // println!("Item quantity: {:#?}", item_quantity);
    //
    // // Get item location example
    // let locations = _warehouse.get_item_location("CoolItem".to_string());
    // println!("Locations: {:#?}", locations);
    //
    // // Sort example
    // let items = _warehouse.sort_items();
    // println!("Items: {:#?}", items);
    //
    // // Get expired items example
    let expired_items = _warehouse.get_expire_items();
    println!("Expired items: {:#?}", expired_items);
}
