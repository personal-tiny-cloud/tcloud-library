pub mod error;
use crate::error::PluginError;
pub use async_trait::async_trait;
use std::path::Path;

/// Plugin trait, every pCloud plugin must implement this trait.
/// It will be used during plugin's initialization and when processing requests.
#[async_trait]
pub trait Plugin {
    /// Initializes plugin.<p>
    /// The returned `String` is the name of the plugin. It is going to be used
    /// to access the plugin through http APIs (/pcloud/api/app/<plugin name>)
    fn new() -> Result<(String, Self), PluginError>
    where
        Self: Sized;
    /// Processes request.<p>
    /// Request must be parsed by the implementor. Returned `String`
    /// will be sent as is to the client.
    /// `path` is the user files directory.
    async fn process_api_request(&mut self, req: String, path: &Path) -> Result<String, PluginError>;
}

#[cfg(test)]
mod tests {}
