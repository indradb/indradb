use std::io::prelude::*;
use std::fs::File;
use serde_json::Value as JsonValue;
use super::super::MapReducer;
use std::path::Path;
use uuid::Uuid;
use indradb::{Vertex, Type};

fn run(insert_count: usize, expected_result: JsonValue) {
    let file_path_str = "test_scripts/mapreduce/count.lua";
    let file_path = Path::new(file_path_str);
    let mut file = File::open(file_path).expect("Could not open script file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not get script file contents");
    
    let engine = MapReducer::start(Uuid::default(), contents, file_path_str.to_string(), json!(2));
    
    for _ in 0..insert_count {
        engine.add_vertex(Vertex::new(Uuid::new_v4(), Type::new("foo".to_string()).unwrap()));
    }

    let count = engine.join().unwrap();
    assert_eq!(count, expected_result);
}

#[test]
fn should_handle_zero_items() {
    run(0, JsonValue::Null);
}

#[test]
fn should_handle_one_items() {
    run(1, json!(2.0));
}

#[test]
fn should_handle_many_even_items() {
    run(6, json!(12.0));
}

#[test]
fn should_handle_many_odd_items() {
    run(5, json!(10.0));
}
