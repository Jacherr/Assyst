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