use uuid::Uuid;
use std::io::Write;
use std::str;
use std::u8;
use std::io::{Cursor, Error as IoError};
use models;
use chrono::NaiveDateTime;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub enum KeyComponent {
	Uuid(Uuid),
	UnsizedString(String),
	ShortSizedString(String),
	NaiveDateTime(NaiveDateTime)
}

impl KeyComponent {
	fn len(&self) -> usize {
		match *self {
			KeyComponent::Uuid(_) => 16,
			KeyComponent::UnsizedString(ref s) | KeyComponent::ShortSizedString(ref s) => s.len(),
			KeyComponent::NaiveDateTime(_) => 8
		}
	}

	fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
		match *self {
			KeyComponent::Uuid(ref uuid) => {
				try!(cursor.write(uuid.as_bytes()));
			},
			KeyComponent::UnsizedString(ref s) => {
				try!(cursor.write(s.as_bytes()));
			},
			KeyComponent::ShortSizedString(ref s) => {
				assert!(s.len() <= u8::MAX as usize);
				try!(cursor.write(&[s.len() as u8]));
				try!(cursor.write(s.as_bytes()));
			},
			KeyComponent::NaiveDateTime(ref datetime) => {
				let timestamp = datetime.timestamp();
				assert!(timestamp >= 0);
				try!(cursor.write_i64::<BigEndian>(timestamp));
			}
		};

		Ok(())
	}
}

pub fn build_key(components: Vec<KeyComponent>) -> Box<[u8]> {
	let len = components.iter().fold(0, |len, ref component| len + component.len());
	let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

	for component in components.iter() {
		if let Err(err) = component.write(&mut cursor) {
			panic!("Could not build key: {}", err);
		}
	}

	cursor.into_inner().into_boxed_slice()
}

pub fn parse_uuid_key(key: &[u8]) -> Uuid {
	assert!(key.len() == 16);
	Uuid::from_bytes(key).unwrap()
}

pub fn parse_edge_range_key(key: &[u8]) -> (Uuid, models::Type, NaiveDateTime) {
	assert!(key.len() >= 33);
	let first_id = Uuid::from_bytes(&key[0..16]).unwrap();
	let t_len = key[16] as usize;
	let t = str::from_utf8(&key[17..t_len+17]).unwrap();
	let mut reader = Cursor::new(&key[t_len+17..]);
	let timestamp = reader.read_i64::<BigEndian>().unwrap();
	let datetime = NaiveDateTime::from_timestamp(timestamp, 0);
	(first_id, models::Type::new(t.to_string()).unwrap(), datetime)
}
