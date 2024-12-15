mod item;
mod warehouse;

use chrono::{TimeZone, Utc};
use item::{Quality, SomeItem};
use warehouse::{PlacementStrategy, Warehouse};

fn main() {
    let mut warehouse: Warehouse<SomeItem> = Warehouse::new(2, 2, 2, 4);

    let normal_item = SomeItem {
        id: 125,
        name: "NormalItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };

    let oversized_item = SomeItem {
        id: 126,
        name: "OversizedItem".to_string(),
        quality: Quality::Oversized { size: 2 },
        quantity: 50,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };

    let time = Utc.ymd(2024, 12, 13).and_hms_opt(22, 10, 10);
    let fragile_item = SomeItem {
        id: 127,
        name: "FragileItem".to_string(),
        quality: Quality::Fragile {
            expiration_date: match time {
                Some(time) => time,
                None => Utc::now(),
            },
            storage_maxlevel: 1,
        },
        quantity: 30,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };

    println!("Testing FirstAvailable placement strategy:");
    if let Err(e) = warehouse.item_placement(PlacementStrategy::FirstAvailable, normal_item) {
        println!("Error: {}", e);
    }
    if let Err(e) = warehouse.item_placement(PlacementStrategy::FirstAvailable, oversized_item) {
        println!("Error: {}", e);
    }
    if let Err(e) = warehouse.item_placement(PlacementStrategy::FirstAvailable, fragile_item) {
        println!("Error: {}", e);
    }
    println!("Warehouse after FirstAvailable placement: {:#?}", warehouse);

    // TEST: Test
    // warehouse.get_expire_items(Utc::now());
    // let location = warehouse.get_item_location("NormalItem".to_string());
    // let sort_result = warehouse.sort_items();
    // let item_quantity = warehouse.get_item_quantity(warehouse::SearchType::ById { id: (125) });
    // println!("NormalItem location: {:?}", location);
    // println!("Sorted items: {:?}", sort_result);
    // println!("Item quantity: {:?}", item_quantity);

    let _ = warehouse.remove_item(&125);

    let another_normal_item = SomeItem {
        id: 128,
        name: "NormalItem".to_string(),
        quality: Quality::Normal,
        quantity: 100,
        timestamp: Utc::now().timestamp(),
        occupied_position: None,
    };

    println!("Testing RoundRobin placement strategy:");
    if let Err(e) = warehouse.item_placement(PlacementStrategy::RoundRobin, another_normal_item) {
        println!("Error: {}", e);
    }
    println!("Warehouse after RoundRobin placement: {:#?}", warehouse);
}
