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

pub enum Component<'a> {
    SizedId(&'a models::Id),
    UnsizedId(&'a models::Id),
    UnsizedString(&'a str),
    Type(&'a models::Type),
    DateTime(DateTime<Utc>),
}

impl<'a> Component<'a> {
    fn len(&self) -> usize {
        match *self {
            Component::SizedId(i) => {
                if i.0.len() >= 0xFF {
                    i.0.len() + 3
                } else {
                    i.0.len() + 2
                }
            },
            Component::UnsizedId(i) => i.0.len(),
            Component::UnsizedString(s) => s.len(),
            Component::Type(t) => t.0.len() + 1,
            Component::DateTime(_) => 8,
        }
    }

    fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
        match *self {
            Component::SizedId(id) if id.0.len() >= 0xFF => {
                cursor.write_u8(0xFFu8)?;
                cursor.write_u16::<BigEndian>(id.0.len() as u16)?;
                cursor.write_all(&id.0)?;
            }
            Component::SizedId(id) => {
                cursor.write_u8(id.0.len() as u8)?;
                cursor.write_all(&id.0)?;
            }
            Component::UnsizedId(id) => {
                cursor.write_all(&id.0)?;
            }
            Component::UnsizedString(s) => {
                cursor.write_all(s.as_bytes())?;
            }
            Component::Type(t) => {
                cursor.write_all(&[t.0.len() as u8])?;
                cursor.write_all(t.0.as_bytes())?;
            }
            Component::DateTime(datetime) => {
                let time_to_end = nanos_since_epoch(&MAX_DATETIME) - nanos_since_epoch(&datetime);
                cursor.write_u64::<BigEndian>(time_to_end)?;
            }
        };

        Ok(())
    }
}

pub fn build(components: &[Component]) -> Vec<u8> {
    let len = components.iter().fold(0, |len, component| len + component.len());
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

    for component in components {
        if let Err(err) = component.write(&mut cursor) {
            panic!("Could not write bytes: {}", err);
        }
    }

    cursor.into_inner()
}

pub fn read_sized_id<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> models::Id {
    let short_id_len = cursor.read_u8().unwrap();

    let id_len = if short_id_len == 0xFFu8 {
        cursor.read_u16::<BigEndian>().unwrap() as usize
    } else {
        short_id_len as usize
    };

    let mut buf = vec![0u8; id_len];
    cursor.read_exact(&mut buf).unwrap();
    models::Id::new(buf).unwrap()
}

pub fn read_unsized_id<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> models::Id {
    let mut buf = Vec::new();
    cursor.read_to_end(&mut buf).unwrap();
    models::Id::new(buf).unwrap()
}

pub fn read_type<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> models::Type {
    let t_len = {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };

    let mut buf = vec![0u8; t_len];
    cursor.read_exact(&mut buf).unwrap();

    unsafe {
        let s = str::from_utf8_unchecked(&buf).to_string();
        models::Type::new_unchecked(s)
    }
}

pub fn read_unsized_string<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> String {
    let mut buf = String::new();
    cursor.read_to_string(&mut buf).unwrap();
    buf
}

pub fn read_datetime<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> DateTime<Utc> {
    let time_to_end = cursor.read_u64::<BigEndian>().unwrap();
    assert!(time_to_end <= i64::MAX as u64);
    *MAX_DATETIME - Duration::nanoseconds(time_to_end as i64)
}
