mod handlers;
mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebServerConfig<'a> {
    pub port: &'a str,
}
