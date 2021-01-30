use std::{fmt, error};

#[derive(Debug,PartialEq,Eq)]
pub enum ReadError {
    InvalidState(String),
    InvalidFormat(String),
    OverMaxBitLength,
    Undefined
}
impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::InvalidState(ref s) => write!(f, "Invalid State. ({})", s),
            ReadError::InvalidFormat(ref s) => write!(f, "Invalid format. ({})", s),
            ReadError::Undefined => write!(f, "{}", "There is no definition of a piece that matches the haffman code."),
            ReadError::OverMaxBitLength => {
                write!(f, "{}", "Searched for the longest defined code, but couldn't find the corresponding definition.")
            },
        }
    }
}
impl error::Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::InvalidState(_) => "Invalid State.",
            ReadError::InvalidFormat(_) => "Invalid format.",
            ReadError::Undefined => "There is no definition of a piece that matches the haffman code.",
            ReadError::OverMaxBitLength => "Searched for the longest defined code, but couldn't find the corresponding definition."
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ReadError::InvalidState(_) => None,
            ReadError::InvalidFormat(_) => None,
            ReadError::Undefined => None,
            ReadError::OverMaxBitLength => None,
        }
    }
}
/*
impl<'a> From<ReadError> for USIAgentRunningError<'a,SystemEventKind, ReadError>
    where SystemEventKind: fmt::Debug {
    fn from(err: ReadError) -> USIAgentRunningError<'a,SystemEventKind, ReadError> {
        USIAgentRunningError::from(USIAgentStartupError::PlayerError(err))
    }
}
*/