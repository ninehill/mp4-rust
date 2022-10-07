use crate::*;

#[derive(Debug)]
pub enum MediaChunkBox {
    Emsg (EmsgBox),
    Moof (MoofBox),
    Mdat (MdatBox)
}

#[derive(Debug)]
pub struct MediaChunk {
    pub children: Vec<MediaChunkBox>
}

impl MediaChunk {
    pub fn new() -> Self {
        MediaChunk { children: Vec::<MediaChunkBox>::new() }
    }
}