#[macro_use]
extern crate rocket;

// use dashmap::DashMap;
use rocket::serde::{Serialize, json::Json};
use rocket_okapi::{
    openapi, openapi_get_routes,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};
use schemars::JsonSchema;
use serde::Deserialize;

//use crate::modules::etf::Etf;

// import ./src/modules/etf.rs module
mod etf;

use etf::Etf;
use etf::read_etfs_from_csv;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Person {
    id: u64,
    name: String,
    age: u8,
    city: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct Message {
    text: String,
}

#[openapi]
#[get("/")]
fn index() -> Json<Vec<Message>> {
    // define 2 messages
    let messages = vec![
        Message {
            text: "Hello, World!".to_string(),
        },
        Message {
            text: "This is a Rocket API!".to_string(),
        },
    ];

    Json(messages)
}

#[openapi]
#[get("/etfs")]
fn get_etfs() -> Json<Vec<Etf>> {
    let csv_file_path = "src/etfs.csv"; // Adjust the path as needed
    let etfs = read_etfs_from_csv(csv_file_path).unwrap_or_else(|err| {
        eprintln!("Error reading ETFs from CSV: {}", err);
        Vec::new()
    });

    Json(etfs)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", openapi_get_routes![index, get_etfs])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

/*
fn main() {
    // let file_path = "src/etfs.json"; // Adjust the path as needed
    // let etfs = read_json_file(file_path).unwrap_or_default();
    // println!("ETFs: {:#?}", etfs.len());

    let csv_file_path = "src/etfs.csv"; // Adjust the path as needed
    let etfs = modules::etf::read_etfs_from_csv(csv_file_path).unwrap_or_else(|err| {
        eprintln!("Error reading ETFs from CSV: {}", err);
        Vec::new()
    });
    println!("ETFs: {:#?}", etfs.len());

    // print first 100 ETFs
    for etf in etfs.iter().take(10) {
        // print out etf minified
        println!("{:?}", etf);

        // print out etf in pretty format
        //println!("{:#?}", etf);
    }

    let db: DashMap<u64, Person> = DashMap::new();

    // 1️⃣ CREATE
    let alice = Person {
        id: 1,
        name: "Alice".into(),
        age: 30,
        city: "Cape Town".into(),
    };

    let bob = Person {
        id: 2,
        name: "Bob".into(),
        age: 25,
        city: "Johannesburg".into(),
    };

    let lyall = Person {
        id: 3,
        name: "Lyall".into(),
        age: 25,
        city: "Johannesburg".into(),
    };

    db.insert(alice.id, alice.clone());
    db.insert(bob.id, bob.clone());
    db.insert(lyall.id, lyall.clone());

    // 2️⃣ READ
    if let Some(p) = db.get(&1) {
        println!("Read: {:?}", p);
    }

    // 3️⃣ UPDATE
    if let Some(mut p) = db.get_mut(&2) {
        p.age = 26;
        println!("Updated: {:?}", *p);
    }

    // 4️⃣ DELETE
    db.remove(&1);
    println!(
        "Remaining IDs: {:?}",
        db.iter().map(|r| *r.key()).collect::<Vec<_>>()
    );

    // 5️⃣ QUERY (manual "WHERE age > 25")
    let results: Vec<Person> = db
        .iter()
        .filter_map(|entry| {
            let person = entry.value();
            if person.name.eq("Lyall") {
                Some(person.clone())
            } else {
                None
            }
        })
        .collect();

    println!("Query name = Alice: {:#?}", results);
}
*/
