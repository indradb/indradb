use std::error::Error;

use clap::{App, Arg, SubCommand};
use indradb_proto as proto;
use std::convert::TryInto;
use failure::Fail;

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

    let required_property_name_arg = Arg::with_name("name").help("the property name").required(true).index(2);

    let property_value_arg = Arg::with_name("value")
        .help("the property value as JSON")
        .required(true)
        .index(3);

    let matches = App::new("indradb-client")
        .arg(
            Arg::with_name("address")
                .help("address to the IndraDB server")
                .required(true)
                .index(1),
        )
        .subcommand(SubCommand::with_name("ping").about("pings the server"))
        .subcommand(
            SubCommand::with_name("set")
                .subcommand(
                    SubCommand::with_name("vertex")
                        .about("creates a vertex")
                        .arg(Arg::with_name("type").help("the vertex type").required(true).index(1))
                        .arg(
                            Arg::with_name("id")
                                .help("the optional vertex ID, as a UUID string; if not set, an ID will be generated")
                                .long("id")
                                .value_name("uuid")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("edge")
                        .about("creates an edge")
                        .arg(
                            Arg::with_name("outbound_id")
                                .help("the outbound vertex ID")
                                .required(true)
                                .index(1),
                        )
                        .arg(Arg::with_name("type").help("the edge type").required(true).index(2))
                        .arg(
                            Arg::with_name("inbound_id")
                                .help("the inbound vertex ID")
                                .required(true)
                                .index(3),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("vertex-property")
                        .about("sets vertex properties")
                        .arg(&vertex_query_arg)
                        .arg(&required_property_name_arg)
                        .arg(&property_value_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge-property")
                        .about("sets edge properties")
                        .arg(&edge_query_arg)
                        .arg(&required_property_name_arg)
                        .arg(&property_value_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name("count")
                .subcommand(SubCommand::with_name("vertex").about("counts the number of vertices"))
                .subcommand(
                    SubCommand::with_name("edge")
                        .about("counts the number of edges")
                        .arg(
                            Arg::with_name("id")
                                .help("the vertex ID, as a UUID string")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("inbound")
                                .help("get inbound edges; if not set, outbound edges will be fetched instead")
                                .long("inbound"),
                        )
                        .arg(
                            Arg::with_name("type")
                                .help("the type of edges to count; if not set, all edge types will be counted")
                                .long("type")
                                .value_name("type")
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .subcommand(
                    SubCommand::with_name("vertex")
                        .about("gets vertices by query")
                        .arg(&vertex_query_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge")
                        .about("gets edges by query")
                        .arg(&edge_query_arg),
                )
                .subcommand(
                    SubCommand::with_name("vertex-property")
                        .about("gets vertex properties")
                        .arg(&vertex_query_arg)
                        .arg(&optional_property_name_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge-property")
                        .about("gets edge properties")
                        .arg(&edge_query_arg)
                        .arg(&optional_property_name_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .subcommand(
                    SubCommand::with_name("vertex")
                        .about("deletes vertices by query")
                        .arg(&vertex_query_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge")
                        .about("deletes edges by query")
                        .arg(&edge_query_arg)
                        .subcommand(
                            SubCommand::with_name("vertex-property")
                                .about("deletes vertex properties")
                                .arg(&vertex_query_arg)
                                .arg(&required_property_name_arg),
                        )
                        .subcommand(
                            SubCommand::with_name("edge-property")
                                .about("deletes edge properties")
                                .arg(&edge_query_arg)
                                .arg(&required_property_name_arg),
                        ),
                ),
        )
        .get_matches();

    let _address = matches.value_of("address").unwrap();
    let mut client = proto::Client::new(String::from(_address).try_into().unwrap()).await.map_err(|err| err.compat())?;

    if let Some(_matches) = matches.subcommand_matches("ping") {
        client.ping().await.map_err(|err| err.compat())?;
        println!("PING OK");
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(_matches) = matches.subcommand_matches("vertex") {
            let link_type = indradb::Type::new(_matches.value_of("type").unwrap()).map_err(|err| err.compat())?;
            let uuid = match _matches.value_of("id") {
                Some(id) => uuid::Uuid::parse_str(id)?,
                None => indradb::util::generate_uuid_v1(),
            };
            let vertex = indradb::Vertex::with_id(uuid, link_type);
            let res = client.transaction()
                .await.map_err(|err| err.compat())?
                .create_vertex(&vertex)
                .await.map_err(|err| err.compat())?;
            if res == true {
                println!("Created vertex with id: {} and type: {}", vertex.id, vertex.t.0);
            } else {
                panic!("Failed to create vertex with id: {} and type: {}", vertex.id, vertex.t.0);
            }
        } else if let Some(_matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("vertex-property") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge-property") {
            unimplemented!();
        }
    } else if let Some(matches) = matches.subcommand_matches("count") {
        if let Some(_matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(_matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("vertex-property") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge-property") {
            unimplemented!();
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(_matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("vertex-property") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge-property") {
            unimplemented!();
        }
    } else {
        panic!("unknown command");
    }
    Ok(())
}
