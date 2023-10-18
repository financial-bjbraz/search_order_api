


extern crate mongodb;
use mongodb::{bson, doc};


pub fn connect_mongo(){
    let client = Client::connect("localhost", 32769)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("test").collection("movies");

    let doc = doc! {
        "title": "Jaws",
        "array": [ 1, 2, 3 ],
    };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");
}