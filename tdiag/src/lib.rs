//! Tools to inspect a timely-dataflow computation.

#![deny(missing_docs)]

pub mod commands;

/// An error generated by one of the commands of this tool.
pub struct DiagError(pub String);

impl From<std::io::Error> for DiagError {
    fn from(error: std::io::Error) -> Self {
        DiagError(format!("io error: {}", error.to_string()))
    }
}

impl From<tdiag_connect::ConnectError> for DiagError {
    fn from(error: tdiag_connect::ConnectError) -> Self {
        match error {
            tdiag_connect::ConnectError::IoError(e) => DiagError(format!("io error: {}", e)),
            tdiag_connect::ConnectError::Other(e) => DiagError(e),
        }
    }
}

type LoggingTuple = (std::time::Duration, timely::logging::WorkerIdentifier, timely::logging::TimelyEvent);
