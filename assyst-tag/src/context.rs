use anyhow::anyhow;

pub struct NopContext;

fn not_implemented<T>() -> anyhow::Result<T> {
    Err(anyhow!("Not implemented"))
}

pub trait Context {
    /// Executes provided JavaScript code and returns the result as a string
    fn execute_javascript(&self, code: &str) -> anyhow::Result<String>;
    /// Returns the URL of the last attachment
    fn get_last_attachment(&self) -> anyhow::Result<String>;
    /// Returns the avatar URL of the provided user, or the message author
    fn get_avatar(&self, user_id: Option<u64>) -> anyhow::Result<String>;
    /// Downloads the URL and returns the contents as a string
    fn download(&self, url: &str) -> anyhow::Result<String>;
}

impl Context for NopContext {
    fn execute_javascript(&self, _code: &str) -> anyhow::Result<String> {
        not_implemented()
    }

    fn get_last_attachment(&self) -> anyhow::Result<String> {
        not_implemented()
    }

    fn get_avatar(&self, _user_id: Option<u64>) -> anyhow::Result<String> {
        not_implemented()
    }

    fn download(&self, _url: &str) -> anyhow::Result<String> {
        not_implemented()
    }
}

impl Context for &dyn Context {
    fn execute_javascript(&self, code: &str) -> anyhow::Result<String> {
        (**self).execute_javascript(code)
    }

    fn get_last_attachment(&self) -> anyhow::Result<String> {
        (**self).get_last_attachment()
    }

    fn get_avatar(&self, user_id: Option<u64>) -> anyhow::Result<String> {
        (**self).get_avatar(user_id)
    }

    fn download(&self, url: &str) -> anyhow::Result<String> {
        (**self).download(url)
    }
}
