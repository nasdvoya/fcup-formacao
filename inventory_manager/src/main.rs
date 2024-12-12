mod item;
mod warehouse;

use chrono::{TimeZone, Utc};
use item::{Quality, SomeItem};
use warehouse::{SearchType, Warehouse};

fn main() {
    let mut _warehouse: Warehouse<SomeItem> = Warehouse::new(4, 2, 2, 8);
    let normal_item = SomeItem {
        id: 125,
        name: "BCoolItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };

    let normal_item_again = SomeItem {
        id: 127,
        name: "CoolItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };
    let oversized_item = SomeItem {
        id: 121,
        name: "ACoolOversized".to_string(),
        quality: Quality::Oversized { size: 2 },
        quantity: 100,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };
    let fragile_item = SomeItem {
        id: 1,
        name: "FragileItem".to_string(),
        quality: Quality::Fragile {
            expiration_date: Utc::now().timestamp() + (2 * 24 * 60 * 60), // Simular 2 dias
            storage_maxlevel: 1,
        },
        quantity: 50,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };

    // Add items to "pile"
    println!("Before item: {:#?}", _warehouse);
    if let Err(e) = _warehouse.add_item(normal_item) {
        println!("Error: {}", e);
    }
    if let Err(e) = _warehouse.add_item(normal_item_again) {
        println!("Error: {}", e);
    }
    if let Err(e) = _warehouse.add_item(oversized_item) {
        println!("Error: {}", e);
    }
    if let Err(e) = _warehouse.add_item(fragile_item) {
        println!("Error: {}", e);
    }

    // Update item position
    println!("After item: {:#?}", _warehouse);
    if let Err(e) = _warehouse.place_item(&125, 0, 0, 0, 0) {
        println!("Error: {}", e);
    }
    if let Err(e) = _warehouse.place_item(&125, 1, 0, 0, 0) {
        println!("Error: {}", e);
    }
    if let Err(e) = _warehouse.place_item(&1, 0, 0, 0, 0) {
        println!("Error: {}", e);
    }
    // if let Err(e) = _warehouse.place_item(&2, 0, 0, 0, 0) {
    //     panic!("Error: {}", e);
    // }

    println!("After update: {:#?}", _warehouse);

    // Get item quantity example
    let item_quantity = _warehouse.get_item_quantity(SearchType::ByName("CoolItem".to_string()));
    println!("Item quantity: {:#?}", item_quantity);

    // Get item location example
    let locations = _warehouse.get_item_location("CoolItem".to_string());
    println!("Locations: {:#?}", locations);

    // Sort example
    let items = _warehouse.sort_items();
    println!("Items: {:#?}", items);

    // Get expired items example
    let some_date = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
    let expired_items = _warehouse.get_expire_items(some_date);
    println!("Expired items: {:#?}", expired_items);
}
