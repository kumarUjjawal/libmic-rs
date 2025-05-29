#[derive(Debug)]
pub enum MicError {
    DeviceNotFound,
    StreamBuildFailed,
    Io(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for MicError {
    fn from(e: std::io::Error) -> Self {
        MicError::Io(e)
    }
}
