#[macro_use]
extern crate clap;

use std::error::Error;
use std::net::ToSocketAddrs;

use indradb_proto as proto;
use tokio::net::TcpListener;
use clap::{Arg, App, SubCommand};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let vertex_query_arg = Arg::with_name("query")
        .help("the JSON vertex query")
        .required(true)
        .index(1);

    let edge_query_arg = Arg::with_name("query")
        .help("the JSON edge query")
        .required(true)
        .index(1);

    let optional_property_name_arg = Arg::with_name("name")
        .help("the property name; if not set, all properties will be fetched")
        .long("name")
        .value_name("name")
        .takes_value(true);

    let required_property_name_arg = Arg::with_name("name")
        .help("the property name")
        .required(true)
        .index(2);

    let property_value_arg = Arg::with_name("value")
        .help("the property value as JSON")
        .required(true)
        .index(3);

    let matches = App::new("indradb-client")
        .arg(Arg::with_name("address")
            .help("address to the IndraDB server")
            .required(true)
            .index(1))
        .subcommand(SubCommand::with_name("ping")
            .about("pings the server"))
        .subcommand(SubCommand::with_name("set")
            .subcommand(SubCommand::with_name("vertex")
                .about("creates a vertex")
                .arg(Arg::with_name("type")
                    .help("the vertex type")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("id")
                    .help("the optional vertex ID, as a UUID string; if not set, an ID will be generated")
                    .long("id")
                    .value_name("uuid")
                    .takes_value(true)))
            .subcommand(SubCommand::with_name("edge")
                .about("creates an edge")
                .arg(Arg::with_name("outbound_id")
                    .help("the outbound vertex ID")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("type")
                    .help("the edge type")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("inbound_id")
                    .help("the inbound vertex ID")
                    .required(true)
                    .index(3)))
            .subcommand(SubCommand::with_name("vertex-property")
                .about("sets vertex properties")
                .arg(&vertex_query_arg)
                .arg(&required_property_name_arg)
                .arg(&property_value_arg))
            .subcommand(SubCommand::with_name("edge-property")
                .about("sets edge properties")
                .arg(&edge_query_arg)
                .arg(&required_property_name_arg)
                .arg(&property_value_arg)))
        .subcommand(SubCommand::with_name("count")
            .subcommand(SubCommand::with_name("vertex")
                .about("counts the number of vertices"))
            .subcommand(SubCommand::with_name("edge")
                .about("counts the number of edges")
                .arg(Arg::with_name("id")
                    .help("the vertex ID, as a UUID string")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("inbound")
                    .help("get inbound edges; if not set, outbound edges will be fetched instead")
                    .long("inbound"))
                .arg(Arg::with_name("type")
                    .help("the type of edges to count; if not set, all edge types will be counted")
                    .long("type")
                    .value_name("type")
                    .takes_value(true))))
        .subcommand(SubCommand::with_name("get")
            .subcommand(SubCommand::with_name("vertex")
                .about("gets vertices by query")
                .arg(&vertex_query_arg))
            .subcommand(SubCommand::with_name("edge")
                .about("gets edges by query")
                .arg(&edge_query_arg))
            .subcommand(SubCommand::with_name("vertex-property")
                .about("gets vertex properties")
                .arg(&vertex_query_arg)
                .arg(&optional_property_name_arg))
            .subcommand(SubCommand::with_name("edge-property")
                .about("gets edge properties")
                .arg(&edge_query_arg)
                .arg(&optional_property_name_arg)))
        .subcommand(SubCommand::with_name("delete")
            .subcommand(SubCommand::with_name("vertex")
                .about("deletes vertices by query")
                .arg(&vertex_query_arg))
            .subcommand(SubCommand::with_name("edge")
                .about("deletes edges by query")
                .arg(&edge_query_arg)
            .subcommand(SubCommand::with_name("vertex-property")
                .about("deletes vertex properties")
                .arg(&vertex_query_arg)
                .arg(&required_property_name_arg))
            .subcommand(SubCommand::with_name("edge-property")
                .about("deletes edge properties")
                .arg(&edge_query_arg)
                .arg(&required_property_name_arg))))
        .get_matches();

    let address = matches.value_of("address").unwrap();

    if let Some(matches) = matches.subcommand_matches("ping") {
        unimplemented!();
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            unimplemented!();
        } else {
            panic!("unknown command");
        }
    } else if let Some(matches) = matches.subcommand_matches("count") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else {
            panic!("unknown command");
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            unimplemented!();
        } else {
            panic!("unknown command");
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            unimplemented!();
        } else {
            panic!("unknown command");
        }
    }

    Ok(())
}
