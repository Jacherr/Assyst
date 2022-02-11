pub struct NopContext;

pub trait Context {
    /// Executes provided JavaScript code and returns the result as a string
    fn execute_javascript(&self, code: &str) -> Option<String>;
    /// Returns the URL of the last attachment
    fn get_last_attachment(&self) -> Option<String>;
    /// Returns the avatar URL of the provided user, or the message author
    fn get_avatar(&self, user_id: Option<u64>) -> Option<String>;
    /// Downloads the URL and returns the contents as a string
    fn download(&self, url: &str) -> Option<String>;
}

impl Context for NopContext {
    fn execute_javascript(&self, _code: &str) -> Option<String> {
        None
    }

    fn get_last_attachment(&self) -> Option<String> {
        None
    }

    fn get_avatar(&self, _user_id: Option<u64>) -> Option<String> {
        None
    }

    fn download(&self, _url: &str) -> Option<String> {
        None
    }
}

impl Context for &dyn Context {
    fn execute_javascript(&self, code: &str) -> Option<String> {
        (**self).execute_javascript(code)
    }

    fn get_last_attachment(&self) -> Option<String> {
        (**self).get_last_attachment()
    }

    fn get_avatar(&self, user_id: Option<u64>) -> Option<String> {
        (**self).get_avatar(user_id)
    }

    fn download(&self, url: &str) -> Option<String> {
        (**self).download(url)
    }
}
