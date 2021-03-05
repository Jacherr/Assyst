use std::cmp::min;
use std::ops::Range;

pub enum Type {
    GIF,
    JPEG,
    MP4,
    PNG
}
impl Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Type::GIF => "gif",
            Type::JPEG => "jpeg",
            Type::MP4 => "mp4",
            Type::PNG => "png"
        }
    }
}

const GIF: [u8; 3] = [71, 73, 70];
const JPEG: [u8; 3] = [255, 216, 255];
const MP4: [u8; 4] = [0x66, 0x74, 0x79, 0x70];
const PNG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

fn bounded_range(start: usize, end: usize, len: usize) -> Range<usize> {
    min(len, start)..min(len, end)
}

fn sig(that: &[u8], eq: &[u8]) -> bool {
    that[0..std::cmp::min(eq.len(), that.len())].eq(eq)
}

fn check_mp4(that: &[u8]) -> bool {
    let bytes_offset_removed = &that[bounded_range(4, 8, that.len())];
    sig(bytes_offset_removed, &MP4)
}

pub fn get_sig(buf: &[u8]) -> Option<Type> {
    if buf.len() < 8 { return None };
    if sig(buf, &GIF) { Some(Type::GIF) }
    else if sig(buf, &JPEG) { Some(Type::JPEG) }
    else if check_mp4(buf) { Some(Type::MP4) }
    else if sig(buf, &PNG) { Some(Type::PNG) }
    else { None }
}