#[macro_export]
macro_rules! box_str {
    ($str:expr) => {
        $str
            .to_owned()
            .into_boxed_str()
    }
}