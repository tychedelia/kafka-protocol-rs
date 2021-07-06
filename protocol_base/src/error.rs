#[derive(Debug)]
pub enum ErrorKind {
    NotEnoughBytes
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::NotEnoughBytes => {
                write!(f, "Not enough bytes left to decode message!")
            }
        }
   }
}

impl std::error::Error for ErrorKind {
}
