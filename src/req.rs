use std::collections::HashMap;
use std::path::PathBuf;
use color_eyre::Report;
use thiserror::Error;
use crate::method::Method;

#[derive(Debug, Default)]
pub struct Request {
    pub method: Method,
    pub path: PathBuf,
    /// (Major, Minor)
    pub version: (usize, usize),
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Error)]
pub enum RequestParserError {
    #[error("HTTP first line is completely missing")]
    MissingFirstLine,

    #[error("HTTP first line is incomplete ({missing_part} is missing)")]
    MissingFirstLinePart { missing_part: String },

    #[error("The request's version formatting doesn't match the specification's standard. Expected HTTP/MAJOR.MINOR, got {version}")]
    InvalidVersionFormat { version: String },

    #[error("One of the headers doesn't separate the key from the value within the standard. Expected KEY: VALUE, got {header}")]
    InvalidHeaderSeparator { header: String },
}

impl TryFrom<String> for Request {
    type Error = Report;

    fn try_from(value: String) -> color_eyre::Result<Self, Self::Error> {
        use RequestParserError::*;

        let mut req = Request::default();
        let mut lines = value.lines();
        let mut headers = HashMap::new();

        let mut first_line = lines.next().ok_or(MissingFirstLine)?.split_whitespace();

        req.method = first_line
            .next()
            .ok_or(MissingFirstLinePart {
                missing_part: "Method".to_string(),
            })?
            .parse()?;

        req.path = first_line
            .next()
            .ok_or(MissingFirstLinePart {
                missing_part: "Path".to_string(),
            })?
            .parse()?;

        let version = first_line.next().ok_or(MissingFirstLinePart {
            missing_part: "Version".to_string(),
        })?;

        req.version = match version[5..].split_once('.') {
            Some((major, minor)) => {
                let major = major.parse().map_err(|_| InvalidVersionFormat {
                    version: version.to_string(),
                })?;
                let minor = minor.parse().map_err(|_| InvalidVersionFormat {
                    version: version.to_string(),
                })?;

                Ok((major, minor))
            }
            None => Err(InvalidVersionFormat {
                version: version.to_string(),
            }),
        }?;

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let (key, value) = line.split_once(':').ok_or(InvalidHeaderSeparator {
                header: line.to_string(),
            })?;

            headers.insert(key.to_string(), value.trim_start().to_string());
        }

        req.headers = headers;

        let body: String = lines.collect();
        req.body = match body.is_empty() {
            true => None,
            false => Some(body),
        };

        Ok(req)
    }
}