mod errors;

use std::convert::TryInto;
use std::error::Error as StdError;

use clap::{value_t, App, AppSettings, Arg, SubCommand};
use indradb::util::{
    extract_count, extract_edge_properties, extract_edges, extract_vertex_properties, extract_vertices,
};
use indradb::{
    AllEdgeQuery, AllVertexQuery, CountQueryExt, Edge, Error, Identifier, QueryExt, SpecificEdgeQuery,
    SpecificVertexQuery, Vertex,
};
use indradb_proto as proto;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn StdError>> {
    let vertex_id_arg = Arg::with_name("id").help("the ID of the target vertex").required(true);
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
                                .help("the optional vertex ID; if not set, an ID will be generated")
                                .long("id")
                                .value_name("id"),
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
                .subcommand(SubCommand::with_name("edge").about("counts the number of edges")),
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
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            let vertex_type = Identifier::new(matches.value_of("type").unwrap())?;
            let vertex = if let Some(id) = matches.value_of("id") {
                let vertex = Vertex::new(id.parse::<u64>()?, vertex_type);
                if !client.create_vertex(&vertex).await? {
                    return Err(Box::new(Error::IdTaken));
                }
                vertex
            } else {
                let id = client.create_vertex_from_type(vertex_type).await?;
                Vertex::new(id, vertex_type)
            };
            println!("{:?}", vertex);
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let edge = build_edge(matches)?;
            let res = client.create_edge(&edge).await?;
            if !res {
                return Err(Box::new(errors::VertexInvalidError));
            }
            println!("{:?}", edge);
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let vertex_query = build_vertex_query(matches)?;
            let property_name = Identifier::new(matches.value_of("name").unwrap())?;
            let property_value = serde_json::from_str(matches.value_of("value").unwrap())?;
            client
                .set_properties(vertex_query, property_name, property_value)
                .await?;
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let property_name = Identifier::new(matches.value_of("name").unwrap())?;
            let property_value = serde_json::from_str(matches.value_of("value").unwrap())?;
            let edge_query = SpecificEdgeQuery::single(build_edge(matches)?);
            client.set_properties(edge_query, property_name, property_value).await?;
        }
    } else if let Some(matches) = matches.subcommand_matches("count") {
        if matches.subcommand_matches("vertex").is_some() {
            let output = client.get(AllVertexQuery.count()?).await?;
            println!("{}", extract_count(output).unwrap());
        } else if matches.subcommand_matches("edge").is_some() {
            let output = client.get(AllEdgeQuery.count()?).await?;
            println!("{}", extract_count(output).unwrap());
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            let vertex_query = build_vertex_query(matches)?;
            let output = client.get(vertex_query).await?;
            println!("{:?}", extract_vertices(output));
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let edge_query = SpecificEdgeQuery::single(build_edge(matches)?);
            let output = client.get(edge_query).await?;
            println!("{:?}", extract_edges(output));
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let property_name = matches.value_of("name");
            let q = match property_name {
                Some(property_name) => {
                    let property_name = Identifier::new(property_name)?;
                    build_vertex_query(matches)?.properties()?.name(property_name)
                }
                None => build_vertex_query(matches)?.properties()?,
            };
            let output = client.get(q).await?;
            println!("{:?}", extract_vertex_properties(output));
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let property_name = matches.value_of("name");
            let edge_query = SpecificEdgeQuery::single(build_edge(matches)?);
            let q = match property_name {
                Some(property_name) => {
                    let property_name = Identifier::new(property_name)?;
                    edge_query.properties()?.name(property_name)
                }
                None => edge_query.properties()?,
            };
            let output = client.get(q).await?;
            println!("{:?}", extract_edge_properties(output));
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(matches) = matches.subcommand_matches("vertex") {
            let q = build_vertex_query(matches)?;
            client.delete(q).await?;
        } else if let Some(matches) = matches.subcommand_matches("edge") {
            let q = SpecificEdgeQuery::single(build_edge(matches)?);
            client.delete(q).await?;
        } else if let Some(matches) = matches.subcommand_matches("vertex-property") {
            let property_name = Identifier::new(matches.value_of("name").unwrap())?;
            let q = build_vertex_query(matches)?.properties()?.name(property_name);
            client.delete(q).await?;
        } else if let Some(matches) = matches.subcommand_matches("edge-property") {
            let property_name = Identifier::new(matches.value_of("name").unwrap())?;
            let q = SpecificEdgeQuery::single(build_edge(matches)?)
                .properties()?
                .name(property_name);
            client.delete(q).await?;
        }
    }

    Ok(())
}

fn build_vertex_query(matches: &clap::ArgMatches) -> Result<SpecificVertexQuery, Box<dyn StdError>> {
    let vertex_id = value_t!(matches, "id", u64)?;
    Ok(SpecificVertexQuery::single(vertex_id))
}

fn build_edge(matches: &clap::ArgMatches) -> Result<Edge, Box<dyn StdError>> {
    let edge_type = Identifier::new(matches.value_of("type").unwrap())?;
    let outbound_id = value_t!(matches, "outbound_id", u64)?;
    let inbound_id = value_t!(matches, "inbound_id", u64)?;
    Ok(Edge::new(outbound_id, edge_type, inbound_id))
}
