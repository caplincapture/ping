use std::fmt;

pub enum Error {
    Rawsock(rawsock::Error),
    IO(std::io::Error),
    WMI(wmi::utils::WMIError),
}

impl From<rawsock::Error> for Error {
    fn from(e: rawsock::Error) -> Self {
        Self::Rawsock(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

// (cut: Rawsock, IO impls)

impl From<wmi::utils::WMIError> for Error {
    fn from(e: wmi::utils::WMIError) -> Self {
        Self::WMI(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Rawsock(e) => write!(f, "{}", e),
            Self::IO(e) => write!(f, "{}", e),
            Self::WMI(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}
