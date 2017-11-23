use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use serde_json::Value as JsonValue;
use braid::Datastore;
use super::run;
use serde_json;
use std::path::Path;
use uuid::Uuid;
use common::datastore;

lazy_static! {
    static ref OK_EXPECTED_PATTERN: Regex = Regex::new(r"-- ok: (.+)$").unwrap();
}

macro_rules! test_script {
    ($name:ident) => (
        #[test]
		fn $name() {
            let file_path_str = format!("test_scripts/{}.lua", stringify!($name));
            let file_path = Path::new(&file_path_str[..]);
            let mut file = File::open(file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // NOTE: we construct a new datastore for each test, and tests are
            // run in parallel by default, but not all datastores support
            // multiple concurrent instances. This should use the in-memory
            // datastore by default which works fine. If you swap that out for
            // another datastore (i.e. by changing the `DATASTORE_URL` env
            // var), then you may need to disable parallel execution of tests.
            let datastore = datastore();
            let trans = datastore.transaction(Uuid::default()).unwrap();
            let result = run(&trans, Uuid::default(), &contents[..], file_path, JsonValue::Null);

            match result {
                Ok(actual_result) => {
                    if let Some(cap) = OK_EXPECTED_PATTERN.captures(&contents[..]) {
                        let s = cap.get(1).unwrap().as_str();
                        let expected_result: JsonValue = serde_json::from_str(s).unwrap();
                        assert_eq!(expected_result, actual_result);
                    }
                },
                Err(err) => {
                    panic!(err);
                }
            }
		}
    )
}

test_script!(get_vertices);
test_script!(account_metadata);
test_script!(create_vertex_bad_type);
test_script!(create_vertex);
test_script!(delete_edges);
test_script!(delete_vertices);
test_script!(edge_metadata);
test_script!(get_edge_count);
test_script!(get_edges_bad_high);
test_script!(get_edges_bad_limit);
test_script!(get_edges_bad_low);
test_script!(get_edges);
test_script!(global_metadata);
test_script!(regression_float_serialization);
test_script!(return_array);
test_script!(return_boolean);
test_script!(return_int);
test_script!(return_nil);
test_script!(return_number);
test_script!(return_obj);
test_script!(return_string);
test_script!(set_and_get_edge);
test_script!(vertex_metadata);
