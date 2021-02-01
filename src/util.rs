#[macro_export]
macro_rules! box_str {
    ($str:expr) => {
        $str
            .to_owned()
            .into_boxed_str()
    }
}

pub mod regexes {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static!{
        pub static ref CUSTOM_EMOJI: Regex = Regex::new(r"<a?:\w+:(\d{16,20})>").unwrap();
        pub static ref USER_MENTION: Regex = Regex::new(r"<!?@(\d{16,20})>").unwrap();
    }
}

mod file_signatures {
    pub const GIF: [u8; 3] = [71, 73, 70];
    pub const JPEG: [u8; 3] = [255, 216, 255];
    pub const PNG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
}

pub fn get_buffer_filetype(buffer: &Vec<u8>) -> Option<&'static str> {
    let first_3_bytes = buffer.iter().take(3);
    if first_3_bytes.clone().eq(&file_signatures::GIF) {
        Some("gif")
    } else if first_3_bytes.eq(&file_signatures::JPEG) {
        Some("jpeg")
    } else {
        let first_8_bytes = buffer.iter().take(8);
        if first_8_bytes.eq(&file_signatures::PNG) {
            Some("png")
        } else {
            None
        }
    }
}