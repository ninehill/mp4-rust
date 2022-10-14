use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct TfdtBox {
    pub version: u8,
    pub flags: u32,
    pub base_media_decode_time: u64,
}

impl TfdtBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::TfdtBox
    }

    pub fn get_size(&self) -> u64 {
        // Version 2 allows 64-bit decode media size
        match self.version {
            0 => HEADER_SIZE + HEADER_EXT_SIZE + 4,
            0x1 => HEADER_SIZE + HEADER_EXT_SIZE + 8,
            _ => panic!("unsupported tfdt header size ({})", self.version)
        }
    }
}

impl Mp4Box for TfdtBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("base_media_decode_time={}", self.base_media_decode_time);
        Ok(s)
    }
}

impl <R: Read + Seek> ReadBox<&mut R> for TfdtBox {
    fn read_box(reader: &mut R, _: u64) -> Result<Self> {
        let (version, flags) = read_box_header_ext(reader)?;
        let base_media_decode_time = match version {
            0 => reader.read_u32::<BigEndian>()? as u64,
            0x1 => reader.read_u64::<BigEndian>()?,
            _ => panic!("unsupported tfdt header size ({})", version)
        };

        Ok(TfdtBox {
            version,
            flags,
            base_media_decode_time,
        })

    }
}

impl<W: Write> WriteBox<&mut W> for TfdtBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        write_box_header_ext(writer, self.version, self.flags)?;
        match self.version {
            0 => writer.write_u32::<BigEndian>(self.base_media_decode_time as u32)?,
            _ => writer.write_u64::<BigEndian>(self.base_media_decode_time)?
        }

        Ok(size)
    }
}