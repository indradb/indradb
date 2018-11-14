use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::offset::Utc;
use chrono::{DateTime, NaiveDateTime};
use chrono::{Duration, Timelike};
use models;
use std::i32;
use std::i64;
use std::io::Read;
use std::io::Write;
use std::io::{Cursor, Error as IoError};
use std::str;
use std::u8;
use util::nanos_since_epoch;

lazy_static! {
    pub static ref MAX_DATETIME: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(i64::from(i32::MAX), 0), Utc)
            .with_nanosecond(1_999_999_999u32)
            .unwrap();
}

pub enum KeyComponent<'a> {
    SizedId(&'a models::Id),
    UnsizedId(&'a models::Id),
    UnsizedString(&'a str),
    Type(&'a models::Type),
    DateTime(DateTime<Utc>),
}

impl<'a> KeyComponent<'a> {
    fn len(&self) -> usize {
        match *self {
            KeyComponent::SizedId(i) => i.0.len() + 2,
            KeyComponent::UnsizedId(i) => i.0.len(),
            KeyComponent::UnsizedString(s) => s.len(),
            KeyComponent::Type(t) => t.0.len() + 1,
            KeyComponent::DateTime(_) => 8,
        }
    }

    fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
        match *self {
            KeyComponent::SizedId(id) => {
                cursor.write_u16::<BigEndian>(id.0.len() as u16)?;
                cursor.write_all(&id.0)?;
            }
            KeyComponent::UnsizedId(id) => {
                cursor.write_all(&id.0)?;
            }
            KeyComponent::UnsizedString(s) => {
                cursor.write_all(s.as_bytes())?;
            }
            KeyComponent::Type(t) => {
                cursor.write_all(&[t.0.len() as u8])?;
                cursor.write_all(t.0.as_bytes())?;
            }
            KeyComponent::DateTime(datetime) => {
                let time_to_end = nanos_since_epoch(&MAX_DATETIME) - nanos_since_epoch(&datetime);
                cursor.write_u64::<BigEndian>(time_to_end)?;
            }
        };

        Ok(())
    }
}

pub fn build_key(components: &[KeyComponent]) -> Box<[u8]> {
    let len = components.iter().fold(0, |len, component| len + component.len());
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

    for component in components {
        if let Err(err) = component.write(&mut cursor) {
            panic!("Could not build key: {}", err);
        }
    }

    cursor.into_inner().into_boxed_slice()
}

pub fn read_sized_id(cursor: &mut Cursor<Box<[u8]>>) -> models::Id {
    let id_len = cursor.read_u16::<BigEndian>().unwrap() as usize;
    let mut buf = vec![0u8; id_len];
    cursor.read_exact(&mut buf).unwrap();
    models::Id::new(buf).unwrap()
}

pub fn read_unsized_id(cursor: &mut Cursor<Box<[u8]>>) -> models::Id {
    let mut buf = Vec::new();
    cursor.read_to_end(&mut buf).unwrap();
    models::Id::new(buf).unwrap()
}

pub fn read_type(cursor: &mut Cursor<Box<[u8]>>) -> models::Type {
    let t_len = {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };

    let mut buf = vec![0u8; t_len];
    cursor.read_exact(&mut buf).unwrap();
    let s = str::from_utf8(&buf).unwrap().to_string();
    models::Type::new(s).unwrap()
}

pub fn read_unsized_string(cursor: &mut Cursor<Box<[u8]>>) -> String {
    let mut buf = String::new();
    cursor.read_to_string(&mut buf).unwrap();
    buf
}

pub fn read_datetime(cursor: &mut Cursor<Box<[u8]>>) -> DateTime<Utc> {
    let time_to_end = cursor.read_u64::<BigEndian>().unwrap();
    assert!(time_to_end <= i64::MAX as u64);
    *MAX_DATETIME - Duration::nanoseconds(time_to_end as i64)
}
