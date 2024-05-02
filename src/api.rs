use crate::Coverage;
use reqwest::{blocking::{Client, multipart::{Form, Part}}, StatusCode};
use simple_error::SimpleError;
use std::io::{Result, Error, ErrorKind, Read};

macro_rules! http_try {
    ($e:expr) => {match $e {
        Ok(v) => v,
        Err(err) => { return Err(Error::new(ErrorKind::Other, err)); }
    }};
}

pub(super) fn send_to_api(coverage: &Coverage) -> Result<()> {
    println!("Sending coverage to coveralls.io");

    let file = {
        let mut buf = String::new();
        let mut reader = coverage.new_reader()?;

        reader.read_to_string(&mut buf)?;
        buf
    };

    let part = http_try! { Part::text(file).file_name("json_file").mime_str("application/json") };
    let form = Form::new().part("json_file", part);

    let client = Client::new();

    let req = client.post("https://coveralls.io/api/v1/jobs").multipart(form);
    let resp = http_try! { req.send() };
    let status = resp.status();

    if status == StatusCode::OK {
        println!("Coverage sent successfully");
        Ok(())
    } else {
        let text = resp.text().unwrap_or_else(|_| status.to_string());
        let msg = format!("API status {}: {}", status, text);

        Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
    }
}
