use std::error::Error;

use clap::{App, Arg, SubCommand, AppSettings};
use indradb_proto as proto;
use std::convert::TryInto;
use failure::Fail; 
use indradb::{VertexQuery, SpecificVertexQuery, VertexPropertyQuery, EdgeQuery, SpecificEdgeQuery, EdgePropertyQuery};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let vertex_id_arg = Arg::with_name("uuid")
        .help("the UUID of the target vertex")
        .required(true);
    let vertex_query_arg = Arg::with_name("query")
        .help("the JSON vertex query")
        .required(true)
        .index(1);

    let edge_query_arg = Arg::with_name("query")
        .help("the JSON edge query")
        .required(true)
        .index(1);
    let outbound_id_arg = Arg::with_name("outbound_id")
        .help("the outbound vertex ID")
        .required(true);
    let edge_type_arg = Arg::with_name("type")
        .help("the edge type")
        .required(true);

    let inbound_id_arg = Arg::with_name("inbound_id")
        .help("the inbound vertex ID")
        .required(true);

    let _edge_query_arg = [outbound_id_arg, edge_type_arg, inbound_id_arg];

    let optional_property_name_arg = Arg::with_name("name")
        .help("the property name; if not set, all properties will be fetched")
        .long("name")
        .value_name("name")
        .takes_value(true);

    let required_property_name_arg = Arg::with_name("name").help("the property name").required(true);

    let property_value_arg = Arg::with_name("value")
        .help("the property value as JSON")
        .required(true);

    let matches = App::new("indradb-client")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("address")
                .help("address to the IndraDB server")
                .required(true)
                .index(1),
        )
        .subcommand(SubCommand::with_name("ping").about("pings the server"))
        .subcommand(
            SubCommand::with_name("set")
                .setting(AppSettings::SubcommandRequiredElseHelp)
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
                        .args(&_edge_query_arg),
                )
                .subcommand(
                    SubCommand::with_name("vertex-property")
                        .about("sets vertex properties")
                        .arg(&vertex_id_arg)
                        .arg(&required_property_name_arg)
                        .arg(&property_value_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge-property")
                        .about("sets edge properties")
                        .args(&_edge_query_arg)
                        .arg(&required_property_name_arg)
                        .arg(&property_value_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name("count")
                .setting(AppSettings::SubcommandRequiredElseHelp)
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
                        )
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .setting(AppSettings::SubcommandRequiredElseHelp)
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
                        .arg(&vertex_id_arg)
                        .arg(&optional_property_name_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge-property")
                        .about("gets edge properties")
                        .args(&_edge_query_arg)
                        .arg(&optional_property_name_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .setting(AppSettings::SubcommandRequiredElseHelp)
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

    let address = matches.value_of("address").unwrap();
    let mut client = proto::Client::new(String::from(address).try_into().unwrap()).await.map_err(|err| err.compat())?;

    if let Some(_) = matches.subcommand_matches("ping") {
        client.ping().await.map_err(|err| err.compat())?;
        println!("ok");
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            let vertex_type = indradb::Type::new(matches.value_of("type").unwrap()).map_err(|err| err.compat())?;
            let uuid = match matches.value_of("id") {
                Some(id) => uuid::Uuid::parse_str(id)?,
                None => indradb::util::generate_uuid_v1(),
            };
            let vertex = indradb::Vertex::with_id(uuid, vertex_type);
            let res = client.transaction()
                .await.map_err(|err| err.compat())?
                .create_vertex(&vertex)
                .await.map_err(|err| err.compat())?;
            if !res {
                return Err(indradb::Error::UuidTaken.compat())?;
            }
            println!("{:?}", vertex);

        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let edge_type = indradb::Type::new(matches.value_of("type").unwrap()).map_err(|err| err.compat())?;
            let outbound_id = uuid::Uuid::parse_str(matches.value_of("outbound_id").unwrap())?;
            let inbound_id = uuid::Uuid::parse_str(matches.value_of("inbound_id").unwrap())?;
            let edge_key = indradb::EdgeKey::new(outbound_id, edge_type, inbound_id);
            let res = client.transaction()
                .await.map_err(|err| err.compat())?
                .create_edge(&edge_key)
                .await.map_err(|err| err.compat())?;
            if !res {
                return Err(indradb::Error::VertexInvalid.compat())?;
            }
            println!("{:?}", edge_key);

        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let vertex_id = uuid::Uuid::parse_str(matches.value_of("uuid").unwrap()).map_err(|err| err.compat())?;
            let property_name = matches.value_of("name").unwrap();
            let property_value = serde_json::from_str(matches.value_of("value").unwrap())?;
            let vertex_query = VertexQuery::Specific(SpecificVertexQuery::single(vertex_id));
            client.transaction()
                .await.map_err(|err| err.compat())?
                .set_vertex_properties(VertexPropertyQuery::new(vertex_query, property_name), &property_value)
                .await.map_err(|err| err.compat())?;
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let edge_type = indradb::Type::new(matches.value_of("type").unwrap()).map_err(|err| err.compat())?;
            let outbound_id = uuid::Uuid::parse_str(matches.value_of("outbound_id").unwrap())?;
            let inbound_id = uuid::Uuid::parse_str(matches.value_of("inbound_id").unwrap())?;
            let edge_key = indradb::EdgeKey::new(outbound_id, edge_type, inbound_id);
            let property_name = matches.value_of("name").unwrap();
            let property_value = serde_json::from_str(matches.value_of("value").unwrap())?;
            let edge_query = EdgeQuery::Specific(SpecificEdgeQuery::single(edge_key));
            client.transaction()
                .await.map_err(|err| err.compat())?
                .set_edge_properties(EdgePropertyQuery::new(edge_query, property_name), &property_value)
                .await.map_err(|err| err.compat())?;
        }
    } else if let Some(matches) = matches.subcommand_matches("count") {
        if let Some(_) = matches.subcommand_matches("vertex") {
            let vertex_count = client.transaction()
            .await.map_err(|err| err.compat())?
            .get_vertex_count()
            .await.map_err(|err| err.compat())?;
            println!("{}", vertex_count);
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let vertex_id = uuid::Uuid::parse_str(matches.value_of("id").unwrap()).map_err(|err| err.compat())?;
            let edge_direction = match matches.value_of("inbound") {
                Some(_) =>  indradb::EdgeDirection::Inbound,
                None => indradb::EdgeDirection::Outbound,
            };
            let edge_type = match matches.value_of("type") {
                Some(edge_type) =>  Some(indradb::Type::new(edge_type).map_err(|err| err.compat())?),
                None => None,
            };
            let res = client.transaction()
                .await.map_err(|err| err.compat())?
                .get_edge_count(vertex_id, edge_type.as_ref(), edge_direction)
                .await.map_err(|err| err.compat())?;
            println!("{}", res);
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(_matches) = matches.subcommand_matches("vertex") {
            unimplemented!();
        } else if let Some(_matches) = matches.subcommand_matches("edge") {
            unimplemented!();
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let vertex_id = uuid::Uuid::parse_str(matches.value_of("uuid").unwrap()).map_err(|err| err.compat())?;
            let property_name = matches.value_of("name");
            match property_name {
                Some(property_name) =>  {
                    let vertex_property = client.transaction()
                        .await.map_err(|err| err.compat())?
                        .get_vertex_properties(VertexPropertyQuery::new(VertexQuery::Specific(SpecificVertexQuery::single(vertex_id)), property_name))
                        .await.map_err(|err| err.compat())?;

                    println!("{:?}", vertex_property);
                },
                None => {
                    let vertex_properties = client.transaction()
                        .await.map_err(|err| err.compat())?
                        .get_all_vertex_properties(VertexQuery::Specific(SpecificVertexQuery::single(vertex_id)))
                        .await.map_err(|err| err.compat())?;
                    
                    println!("{:?}", vertex_properties);
                }
            }
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let edge_type = indradb::Type::new(matches.value_of("type").unwrap()).map_err(|err| err.compat())?;
            let outbound_id = uuid::Uuid::parse_str(matches.value_of("outbound_id").unwrap())?;
            let inbound_id = uuid::Uuid::parse_str(matches.value_of("inbound_id").unwrap())?;
            let edge_key = indradb::EdgeKey::new(outbound_id, edge_type, inbound_id);
            let property_name = matches.value_of("name");
            let edge_query = EdgeQuery::Specific(SpecificEdgeQuery::single(edge_key));
            match property_name {
                Some(property_name) =>  {
                    let edge_property = client.transaction()
                        .await.map_err(|err| err.compat())?
                        .get_edge_properties(EdgePropertyQuery::new(edge_query, property_name))
                        .await.map_err(|err| err.compat())?;

                    println!("{:?}", edge_property);
                },
                None => {
                    let edge_property = client.transaction()
                        .await.map_err(|err| err.compat())?
                        .get_all_edge_properties(edge_query)
                        .await.map_err(|err| err.compat())?;

                    println!("{:?}", edge_property);
                }
            }
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
    }
    
    Ok(())
}
