use uuid::Uuid;
use std::io::Write;
use std::i32;
use std::str;
use std::u8;
use std::io::Read;
use std::io::{Cursor, Error as IoError};
use models;
use chrono::NaiveDateTime;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub enum KeyComponent<'a> {
    Uuid(Uuid),
    UnsizedString(&'a str),
    Type(&'a models::Type),
    NaiveDateTime(NaiveDateTime),
}

impl<'a> KeyComponent<'a> {
    fn len(&self) -> usize {
        match *self {
            KeyComponent::Uuid(_) => 16,
            KeyComponent::UnsizedString(ref s) => s.len(),
            KeyComponent::Type(ref t) => t.0.len() + 1, 
            KeyComponent::NaiveDateTime(_) => 8,
        }
    }

    fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
        match *self {
            KeyComponent::Uuid(ref uuid) => {
                try!(cursor.write(uuid.as_bytes()));
            }
            KeyComponent::UnsizedString(ref s) => {
                try!(cursor.write(s.as_bytes()));
            }
            KeyComponent::Type(ref t) => {
                try!(cursor.write(&[t.0.len() as u8]));
                try!(cursor.write(t.0.as_bytes()));
            }
            KeyComponent::NaiveDateTime(ref datetime) => {
                let time_to_end = max_datetime().timestamp() - datetime.timestamp();
                debug_assert!(time_to_end >= 0);
                try!(cursor.write_i64::<BigEndian>(time_to_end));
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

pub fn parse_uuid_key(key: Box<[u8]>) -> Uuid {
    debug_assert_eq!(key.len(), 16);
    let mut cursor = Cursor::new(key);
    read_uuid(&mut cursor)
}

pub fn read_uuid(cursor: &mut Cursor<Box<[u8]>>) -> Uuid {
    let mut buf: [u8; 16] = [0; 16];
    cursor.read_exact(&mut buf).unwrap();
    Uuid::from_bytes(&buf).unwrap()
}

pub fn read_short_sized_string(cursor: &mut Cursor<Box<[u8]>>) -> String {
    let t_len = {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };

    let mut buf = vec![0u8; t_len];
    cursor.read_exact(&mut buf).unwrap();
    str::from_utf8(&buf).unwrap().to_string()
}

pub fn read_type(mut cursor: &mut Cursor<Box<[u8]>>) -> models::Type {
    models::Type::new(read_short_sized_string(&mut cursor)).unwrap()
}

pub fn read_unsized_string(cursor: &mut Cursor<Box<[u8]>>) -> String {
    let mut buf = String::new();
    cursor.read_to_string(&mut buf).unwrap();
    buf

}

pub fn read_datetime(cursor: &mut Cursor<Box<[u8]>>) -> NaiveDateTime {
    let time_to_end = cursor.read_i64::<BigEndian>().unwrap();
    let timestamp = max_datetime().timestamp() - time_to_end;
    NaiveDateTime::from_timestamp(timestamp, 0)
}

pub fn max_datetime() -> NaiveDateTime {
    // NOTE: this suffers from the year 2038 problem, but we can't use
    // i64::MAX because chrono sees it as an invalid time
    NaiveDateTime::from_timestamp(i32::MAX as i64, 0)
}
