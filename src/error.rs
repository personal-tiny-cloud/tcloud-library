use std::io;
use thiserror::Error;

/// Plugin error type, will be handled when receiving requests from clients
#[derive(Error, Debug)]
pub enum PluginError {
    /// To be used only during plugin's initialization
    #[error("Plugin's initialization failed: `{0}`")]
    InitFailed(String),
    /// Used if an internal IO error happens
    #[error("IO Error occurred")]
    IOError(#[from] io::Error),
    /// Used if the request isn't valid, the `String` will be sent back as a body
    #[error("The request is invalid: `{0}`")]
    InvalidRequest(String),
    /// Used if the request couldn't be completed, the `String` will be sent back as response body
    /// and `u16` as the HTTP status code (panics if it's an invalid code)
    #[error("An error occurred. Error Code: `{0}`, Body: `{1}`")]
    RequestFailed(u16, String),
}
