use uuid::Uuid;
use errors::Error;
use std::io::Write;
use std::str;
use std::u8;
use std::io::Cursor;

pub enum KeyComponent {
	Uuid(Uuid),
	String(String),
	Byte(u8)
}

pub fn build_key(components: Vec<KeyComponent>) -> Box<[u8]> {
	let mut len = 0;

	for component in components.iter() {
		len += match *component {
			KeyComponent::Uuid(_) => 16,
			KeyComponent::String(ref s) => s.len(),
			KeyComponent::Byte(_) => 1
		};
	}

	let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

	for component in components.iter() {
		let res = match *component {
			KeyComponent::Uuid(ref uuid) => cursor.write(uuid.as_bytes()),
			KeyComponent::String(ref s) => cursor.write(s.as_bytes()),
			KeyComponent::Byte(b) => cursor.write(&[b])
		};

		if let Err(err) = res {
			panic!("Could not build key: {}", err);
		}

	}

	cursor.into_inner().into_boxed_slice()
}

pub const ACCOUNT_PRELUDE: u8 = 0;
pub const VERTEX_PRELUDE: u8 = 1;
pub const EDGE_PRELUDE: u8 = 2;
pub const REVERSED_EDGE_PRELUDE: u8 = 3;
pub const GLOBAL_METADATA_PRELUDE: u8 = 10;
pub const ACCOUNT_METADATA_PRELUDE: u8 = 11;
pub const VERTEX_METADATA_PRELUDE: u8 = 12;
pub const EDGE_METADATA_PRELUDE: u8 = 13;

pub fn account_key(id: Uuid) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(ACCOUNT_PRELUDE),
		KeyComponent::Uuid(id)
	])
}

pub fn vertex_key(id: Uuid) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(VERTEX_PRELUDE),
		KeyComponent::Uuid(id)
	])
}

pub fn edge_key(outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<Box<[u8]>, Error> {
	if t.len() > u8::MAX as usize {
		return Err(Error::Unexpected("`type` is too long".to_string()));
	}

	Ok(build_key(vec![
		KeyComponent::Byte(EDGE_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t),
		KeyComponent::Uuid(inbound_id)
	]))
}

pub fn edge_without_inbound_id_key_pattern(outbound_id: Uuid, t: String) -> Result<Box<[u8]>, Error> {
	if t.len() > u8::MAX as usize {
		return Err(Error::Unexpected("`type` is too long".to_string()));
	}

	Ok(build_key(vec![
		KeyComponent::Byte(EDGE_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t)
	]))
}

pub fn reversed_edge_key(inbound_id: Uuid, t: String, outbound_id: Uuid) -> Result<Box<[u8]>, Error> {
	if t.len() > u8::MAX as usize {
		return Err(Error::Unexpected("`type` is too long".to_string()));
	}

	Ok(build_key(vec![
		KeyComponent::Byte(REVERSED_EDGE_PRELUDE),
		KeyComponent::Uuid(inbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t),
		KeyComponent::Uuid(outbound_id)
	]))
}

pub fn reversed_edge_without_outbound_id_key_pattern(inbound_id: Uuid, t: String) -> Result<Box<[u8]>, Error> {
	if t.len() > u8::MAX as usize {
		return Err(Error::Unexpected("`type` is too long".to_string()));
	}

	Ok(build_key(vec![
		KeyComponent::Byte(REVERSED_EDGE_PRELUDE),
		KeyComponent::Uuid(inbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t)
	]))
}

pub fn global_metadata_key(key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(GLOBAL_METADATA_PRELUDE),
		KeyComponent::String(key)
	])
}

pub fn account_metadata_key(id: Uuid, key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(ACCOUNT_METADATA_PRELUDE),
		KeyComponent::Uuid(id),
		KeyComponent::String(key)
	])
}

pub fn vertex_metadata_key(id: Uuid, key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(VERTEX_METADATA_PRELUDE),
		KeyComponent::Uuid(id),
		KeyComponent::String(key)
	])
}

pub fn edge_metadata_key(outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(EDGE_METADATA_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t),
		KeyComponent::Uuid(inbound_id),
		KeyComponent::String(key)
	])
}

fn handle_parse_edge_key(key: &[u8], expected_prelude: u8) -> (Uuid, String, Uuid) {
	if key.len() < 34 {
		panic!("Unexpected key length: {}", key.len());
	} else if key[0] != expected_prelude {
		panic!("Unexpected prelude: {:x}", key[0]);
	}

	let first_id = Uuid::from_bytes(&key[1..17]).unwrap();
	let t_len = key[17] as usize;
	let t = str::from_utf8(&key[18..t_len+18]).unwrap();
	let second_id = Uuid::from_bytes(&key[t_len+18..key.len()]).unwrap();
	(first_id, t.to_string(), second_id)
}

pub fn parse_edge_key(key: &[u8]) -> (Uuid, String, Uuid) {
	handle_parse_edge_key(key, EDGE_PRELUDE)
}

pub fn parse_reversed_edge_key(key: &[u8]) -> (Uuid, String, Uuid) {
	let (inbound_id, t, outbound_id) = handle_parse_edge_key(key, REVERSED_EDGE_PRELUDE);
	(outbound_id, t, inbound_id)
}

pub fn parse_vertex_key(key: &[u8]) -> Uuid {
	if key.len() != 17 {
		panic!("Unexpected key length: {}", key.len());
	} else if key[0] != VERTEX_PRELUDE {
		panic!("Unexpected prelude: {:x}", key[0]);
	}

	Uuid::from_bytes(&key[1..17]).unwrap()
}