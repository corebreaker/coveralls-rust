//! HTTP client for the Coveralls upload endpoint.
//!
//! This module performs the single network call of the crate: a multipart `POST` of the serialized
//! coverage report to the Coveralls `jobs` API.

use crate::coverage::Coverage;
use reqwest::{
    blocking::{
        Client,
        multipart::{Form, Part},
    },
    StatusCode,
};

use simple_error::SimpleError;
use log::{debug, error, info, warn};
use std::io::{Result, Error, ErrorKind, Read};

/// Convert a [`reqwest`] error into an [`std::io::Error`], returning early on failure.
macro_rules! http_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err));
            }
        }
    };
}

/// Upload a serialized coverage report to <https://coveralls.io/api/v1/jobs>.
///
/// The report is sent as the `json_file` part of a multipart form. Returns an error when the
/// request cannot be sent or when the API responds with anything other than `200 OK`.
pub(super) fn send_to_api(coverage: &Coverage) -> Result<()> {
    info!("Sending coverage to coveralls.io");

    let file = {
        let mut buf = String::new();
        let mut reader = coverage.new_reader()?;

        reader.read_to_string(&mut buf)?;
        buf
    };

    debug!("Coverage payload is {} bytes", file.len());

    let part = http_try! { Part::text(file).file_name("json_file").mime_str("application/json") };
    let form = Form::new().part("json_file", part);

    let client = Client::new();
    let url = "https://coveralls.io/api/v1/jobs";

    debug!("POSTing coverage to {url}");

    let req = client.post(url).multipart(form);
    let resp = http_try! { req.send() };
    let status = resp.status();

    if status == StatusCode::OK {
        info!("Coverage sent successfully");

        match resp.text() {
            Ok(text) => {
                debug!("Coveralls API response: {text}");
            }
            Err(err) => {
                warn!("Could not read the Coveralls API response body: {err}");
            }
        }

        Ok(())
    } else {
        let text = resp.text().unwrap_or_else(|err| {
            warn!("Could not read the Coveralls API response body: {err}");

            status.to_string()
        });

        error!("Coveralls API rejected the upload (status {status}): {text}");

        let msg = format!("API status {status}: {text}");

        Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
    }
}
