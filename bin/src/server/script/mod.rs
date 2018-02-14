mod api;
mod context;
mod converters;
mod errors;
mod mapreduce;

use serde_json::value::Value as JsonValue;

pub use self::mapreduce::{execute_mapreduce, Update, ResponseSender, ResponseReceiver, bounded};

/// Runs a script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn execute(
    contents: &str,
    path: &str,
    arg: JsonValue,
) -> Result<JsonValue, errors::ScriptError> {
    let l = context::create(arg)?;
    let value: converters::JsonValue = l.exec(contents, Some(path))?;
    Ok(value.0)
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use regex::Regex;
    use serde_json::Value as JsonValue;
    use super::execute;
    use serde_json;
    use std::path::Path;

    lazy_static! {
        static ref OK_EXPECTED_PATTERN: Regex = Regex::new(r"-- ok: ([^\n]+)").unwrap();
        static ref ERR_EXPECTED_PATTERN: Regex = Regex::new(r"-- err: ([^\n]+)").unwrap();
    }

    macro_rules! test_script {
        ($name:ident) => (
            #[test]
            fn $name() {
                let file_path_str = format!("test_scripts/execute/{}.lua", stringify!($name));
                let file_path = Path::new(&file_path_str);
                let mut file = File::open(file_path).expect("Could not open script file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Could not get script file contents");

                // NOTE: we construct a new datastore for each test, and tests are
                // run in parallel by default, but not all datastores support
                // multiple concurrent instances. This should use the in-memory
                // datastore by default which works fine. If you swap that out for
                // another datastore (i.e. by changing the `DATASTORE_URL` env
                // var), then you may need to disable parallel execution of tests.
                match execute(&contents, &file_path_str, JsonValue::Null) {
                    Ok(actual_result) => {
                        if let Some(cap) = OK_EXPECTED_PATTERN.captures(&contents) {
                            let s = cap.get(1).unwrap().as_str();
                            let expected_result: JsonValue = serde_json::from_str(s).expect("Could not parse expected JSON response");
                            assert_eq!(expected_result, actual_result);
                        }
                    },
                    Err(err) => {
                        if let Some(cap) = ERR_EXPECTED_PATTERN.captures(&contents) {
                            let s = cap.get(1).unwrap().as_str();
                            assert_eq!(format!("{:?}", err), s);
                        } else {
                            panic!(format!("Script failed to execute: {:?}", err));
                        }
                    }
                }
            }
        )
    }

    test_script!(get_vertices);
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
    test_script!(return_coroutine);
    test_script!(return_function);
    test_script!(return_int);
    test_script!(return_nil);
    test_script!(return_number);
    test_script!(return_obj);
    test_script!(return_string);
    test_script!(set_and_get_edge);
    test_script!(vertex_metadata);
}
