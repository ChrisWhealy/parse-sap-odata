use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<std::io::Error> for ParseError {
    fn from(io_err: std::io::Error) -> ParseError {
        ParseError {
            msg: io_err.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(utf8_err: std::string::FromUtf8Error) -> ParseError {
        ParseError {
            msg: utf8_err.to_string(),
        }
    }
}

impl From<std::io::ErrorKind> for ParseError {
    fn from(io_err: std::io::ErrorKind) -> ParseError {
        ParseError {
            msg: io_err.to_string(),
        }
    }
}

impl From<quick_xml::DeError> for ParseError {
    fn from(xml_err: quick_xml::DeError) -> ParseError {
        ParseError {
            msg: xml_err.to_string(),
        }
    }
}
