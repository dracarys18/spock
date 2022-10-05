use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpocError {
    #[error("Error while running command")]
    CommandError(#[from] std::io::Error),
    #[error("Utf8 to string error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Nvim Oxi error")]
    NvimOxiError(#[from] nvim_oxi::Error),
}
