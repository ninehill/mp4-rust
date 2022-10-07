use std::io::{Seek, Read, Write};
use serde::Serialize;

use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MdatBox {
    pub data: Vec<u8>,
    pub start_offset: u64,
}

impl MdatBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::MdatBox
    }

    pub fn get_size(&self) -> u64 {
        HEADER_SIZE + self.data.len() as u64
    }
}

impl Default for MdatBox {
    fn default() -> Self {
        MdatBox {
            data: Vec::<u8>::new(),
            start_offset: 0,
        }
    }
}

impl Mp4Box for MdatBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    fn to_json(&self) -> crate::Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> crate::Result<String> {
        Ok(format!("start offset:={}, data len={}", self.start_offset, self.data.len()))
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for MdatBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start_pos = reader.stream_position()? - HEADER_SIZE;
        let body_len = size - HEADER_SIZE;
        let mut data = vec![0; body_len as usize];
        
        reader.read_exact(&mut data)?;
        
        Ok(MdatBox{
            data,
            start_offset: start_pos,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for MdatBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        writer.write(&self.data)?;
        Ok(size)
    }
}