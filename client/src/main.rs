mod errors;
use std::error::Error as StdError;

use clap::{App, AppSettings, Arg, SubCommand};
use indradb::{
    EdgeKey, EdgePropertyQuery, EdgeQuery, SpecificEdgeQuery, SpecificVertexQuery, VertexPropertyQuery, VertexQuery,
};
use indradb_proto as proto;
use std::convert::TryInto;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn StdError>> {
    let vertex_id_arg = Arg::with_name("uuid")
        .help("the UUID of the target vertex")
        .required(true);
    let outbound_id_arg = Arg::with_name("outbound_id")
        .help("the outbound vertex ID")
        .required(true);
    let edge_type_arg = Arg::with_name("type").help("the edge type").required(true);
    let inbound_id_arg = Arg::with_name("inbound_id")
        .help("the inbound vertex ID")
        .required(true);

    let edge_query_arg = [outbound_id_arg, edge_type_arg, inbound_id_arg];

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
                        .args(&edge_query_arg),
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
                        .args(&edge_query_arg)
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
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("vertex")
                        .about("gets vertices by query")
                        .arg(&vertex_id_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge")
                        .about("gets edges by query")
                        .args(&edge_query_arg),
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
                        .args(&edge_query_arg)
                        .arg(&optional_property_name_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("vertex")
                        .about("deletes vertices by query")
                        .arg(&vertex_id_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge")
                        .about("deletes edges by query")
                        .args(&edge_query_arg),
                )
                .subcommand(
                    SubCommand::with_name("vertex-property")
                        .about("deletes vertex properties")
                        .arg(&vertex_id_arg)
                        .arg(&required_property_name_arg),
                )
                .subcommand(
                    SubCommand::with_name("edge-property")
                        .about("deletes edge properties")
                        .args(&edge_query_arg)
                        .arg(&required_property_name_arg),
                ),
        )
        .get_matches();

    run(matches).await
}

async fn run(matches: clap::ArgMatches<'_>) -> Result<(), Box<dyn StdError>> {
    let address = matches.value_of("address").unwrap();
    let mut client = proto::Client::new(String::from(address).try_into().unwrap()).await?;

    if matches.subcommand_matches("ping").is_some() {
        client.ping().await?;
        println!("ok");
        return Ok(());
    }

    let mut trans = client.transaction().await?;

    if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            let vertex_type = indradb::Identifier::new(matches.value_of("type").unwrap())?;
            let uuid = match matches.value_of("id") {
                Some(id) => uuid::Uuid::parse_str(id)?,
                None => indradb::util::generate_uuid_v1(),
            };
            let vertex = indradb::Vertex::with_id(uuid, vertex_type);
            let res = trans.create_vertex(&vertex).await?;
            if !res {
                return Err(Box::new(indradb::Error::UuidTaken));
            }

            println!("{:?}", vertex);
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let edge_key = build_edge_key(matches)?;
            let res = trans.create_edge(&edge_key).await?;
            if !res {
                return Err(Box::new(errors::VertexInvalidError));
            }

            println!("{:?}", edge_key);
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let vertex_query = build_vertex_query(matches)?;
            let property_name = matches.value_of("name").unwrap();
            let property_value = serde_json::from_str(matches.value_of("value").unwrap())?;
            trans
                .set_vertex_properties(VertexPropertyQuery::new(vertex_query, property_name), &property_value)
                .await?;
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let property_name = matches.value_of("name").unwrap();
            let property_value = serde_json::from_str(matches.value_of("value").unwrap())?;
            let edge_query = build_edge_query(build_edge_key(matches)?);
            trans
                .set_edge_properties(EdgePropertyQuery::new(edge_query, property_name), &property_value)
                .await?;
        }
    } else if let Some(matches) = matches.subcommand_matches("count") {
        if matches.subcommand_matches("vertex").is_some() {
            let vertex_count = trans.get_vertex_count().await?;
            println!("{}", vertex_count);
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let vertex_id = uuid::Uuid::parse_str(matches.value_of("id").unwrap())?;
            let edge_direction = match matches.value_of("inbound") {
                Some(_) => indradb::EdgeDirection::Inbound,
                None => indradb::EdgeDirection::Outbound,
            };
            let edge_type = match matches.value_of("type") {
                Some(edge_type) => Some(indradb::Identifier::new(edge_type)?),
                None => None,
            };
            let res = trans
                .get_edge_count(vertex_id, edge_type.as_ref(), edge_direction)
                .await?;

            println!("{}", res);
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            let vertex_query = build_vertex_query(matches)?;
            let vertices = trans.get_vertices(vertex_query).await?;

            println!("{:?}", vertices);
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let edge_query = build_edge_query(build_edge_key(matches)?);
            let edges = trans.get_edges(edge_query).await?;

            println!("{:?}", edges);
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let property_name = matches.value_of("name");
            match property_name {
                Some(property_name) => {
                    let vertex_property = trans
                        .get_vertex_properties(VertexPropertyQuery::new(build_vertex_query(matches)?, property_name))
                        .await?;

                    println!("{:?}", vertex_property);
                }
                None => {
                    let vertex_properties = trans.get_all_vertex_properties(build_vertex_query(matches)?).await?;

                    println!("{:?}", vertex_properties);
                }
            }
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let property_name = matches.value_of("name");
            let edge_query = build_edge_query(build_edge_key(matches)?);
            match property_name {
                Some(property_name) => {
                    let edge_property = trans
                        .get_edge_properties(EdgePropertyQuery::new(edge_query, property_name))
                        .await?;

                    println!("{:?}", edge_property);
                }
                None => {
                    let edge_property = trans.get_all_edge_properties(edge_query).await?;

                    println!("{:?}", edge_property);
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            trans.delete_vertices(build_vertex_query(matches)?).await?;
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            trans.delete_edges(build_edge_query(build_edge_key(matches)?)).await?;
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let property_name = matches.value_of("name").unwrap();

            trans
                .delete_vertex_properties(VertexPropertyQuery::new(build_vertex_query(matches)?, property_name))
                .await?;
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let property_name = matches.value_of("name").unwrap();

            trans
                .delete_edge_properties(EdgePropertyQuery::new(
                    build_edge_query(build_edge_key(matches)?),
                    property_name,
                ))
                .await?;
        }
    }

    Ok(())
}

fn build_vertex_query(matches: &clap::ArgMatches) -> Result<VertexQuery, Box<dyn StdError>> {
    let vertex_id = uuid::Uuid::parse_str(matches.value_of("uuid").unwrap())?;

    Ok(VertexQuery::Specific(SpecificVertexQuery::single(vertex_id)))
}

fn build_edge_key(matches: &clap::ArgMatches) -> Result<EdgeKey, Box<dyn StdError>> {
    let edge_type = indradb::Identifier::new(matches.value_of("type").unwrap())?;
    let outbound_id = uuid::Uuid::parse_str(matches.value_of("outbound_id").unwrap())?;
    let inbound_id = uuid::Uuid::parse_str(matches.value_of("inbound_id").unwrap())?;
    let edge_key = indradb::EdgeKey::new(outbound_id, edge_type, inbound_id);

    Ok(edge_key)
}

fn build_edge_query(edge_key: EdgeKey) -> EdgeQuery {
    EdgeQuery::Specific(SpecificEdgeQuery::single(edge_key))
}
