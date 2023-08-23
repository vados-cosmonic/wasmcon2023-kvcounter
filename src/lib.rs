wit_bindgen::generate!({
    world: "kvcounter",
    exports: {
        "wasi:http/incoming-handler": KvCounter
    }
});

use anyhow::{anyhow, Context};

use wasi::{
    http::http_types::{
        finish_outgoing_stream, incoming_request_method, incoming_request_path_with_query,
        new_fields, new_outgoing_response, outgoing_response_write, set_response_outparam, Method,
        ResponseOutparam,
    },
    io::streams::write,
    keyvalue::{
        readwrite::{get, set},
        types::{
            incoming_value_consume_sync, new_outgoing_value, open_bucket, outgoing_value_write_body,
        },
    },
};

mod ui;
use ui::get_static_asset;

use crate::exports::wasi::http::incoming_handler::{IncomingHandler, IncomingRequest};

const BUCKET: &str = "default";

/// Implementation struct for the 'kvcounter' world (see: wit/kvcounter.wit)
struct KvCounter;

impl KvCounter {
    /// Increment (possibly negatively) the counter for a given key
    fn increment_counter(bucket: u32, key: &String, amount: i32) -> anyhow::Result<()> {
        let current_value: i32 = match get(bucket, key) {
            // If the value exists, parse it into an i32
            Ok(incoming_value) => {
                // Read bytes from incoming value
                let bytes = incoming_value_consume_sync(incoming_value)
                    .map_err(|count| anyhow!("failed to parse incoming bytes, read [{count}]"))?;
                // Convert the bytes to a i32
                String::from_utf8(bytes)
                    .context("failed to parse string from returned bytes")?
                    .trim()
                    .parse()
                    .context("failed to parse i32 from bytes")?
            }
            // If the value is missing or we fail to get it, assume it is zero
            Err(_) => {
                eprintln!("encountered missing key [{key}], defaulting to 0");
                0
            }
        };

        // Calculate the new value
        let new_value: i32 = current_value + amount;

        // Build outgoing value to use
        let outgoing_value = new_outgoing_value();
        let stream =
            outgoing_value_write_body(outgoing_value).expect("failed to write outgoing value");

        // Write out the new value
        write(stream, (new_value).to_string().as_bytes())
            .expect("failed to write to outgoing value stream");

        // Set the key to the updated value
        set(bucket, key, outgoing_value).expect("failed to set value");

        Ok(())
    }
}

/// Write a W HTTP response out, using WIT-driven (WASI) interfaces
fn write_http_response(
    response_outparam: ResponseOutparam,
    status_code: u16,
    headers: &Vec<(String, Vec<u8>)>,
    body: impl AsRef<[u8]>,
) {
    // Add headers
    let headers = new_fields(headers);

    // Create new outgoing response and related stream
    let outgoing_response =
        new_outgoing_response(status_code, headers).expect("failed to create response");
    let outgoing_stream =
        outgoing_response_write(outgoing_response).expect("failed to write outgoing response");

    // Write out repsonse body to outgoing straem
    write(outgoing_stream, body.as_ref()).expect("failed to write output to stream");
    finish_outgoing_stream(outgoing_stream);

    // Set the response on the param
    set_response_outparam(response_outparam, Ok(outgoing_response))
        .expect("failed to set response");
}

/// Implementation of the WIT-driven incoming-handler interface for our implementation struct
impl IncomingHandler for KvCounter {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // Decipher method
        let method = incoming_request_method(request);

        // Decipher path
        let path_with_query = incoming_request_path_with_query(request).expect("invalid path");
        let path = match path_with_query.split_once('?') {
            Some((v, _)) => v,
            None => path_with_query.as_ref(),
        };
        let trimmed_path: Vec<&str> = path.trim_matches('/').split('/').collect();

        // Generate an outgoing request
        match (method, trimmed_path.as_slice()) {
            // Retrieve value of the counter
            (Method::Get, ["api", "counter"]) => {
                // Retrieve the bucket
                // Retrieve bucket or return early with error
                let bucket = if let Ok(v) = open_bucket(BUCKET) {
                    v
                } else {
                    write_http_response(
                        response,
                        500,
                        &Vec::new(),
                        r#"{"status": "error", "error": "unexpected server error: failed to retreive error"}"#,
                    );
                    return;
                };

                // Increment the counter
                if let Err(_) = KvCounter::increment_counter(bucket, &String::from("default"), 1) {
                    write_http_response(
                        response,
                        500,
                        &Vec::new(),
                        r#"{"status": "error", "error": "unexpected server error: failed to increment default counter"}"#,
                    );
                    return;
                }

                // Build & write the response the response
                write_http_response(response, 200, &Vec::new(), "0");
            }

            // Update a counter
            (Method::Get, ["api", "counter", counter]) => {
                // Retrieve bucket or return early with error
                let bucket = if let Ok(v) = open_bucket(BUCKET) {
                    v
                } else {
                    write_http_response(
                        response,
                        500,
                        &Vec::new(),
                        r#"{"status": "error", "error": "unexpected server error: failed to retreive error"}"#,
                    );
                    return;
                };

                // Increment the counter
                if let Err(_) = KvCounter::increment_counter(bucket, &counter.to_string(), 1) {
                    write_http_response(
                        response,
                        500,
                        &Vec::new(),
                        r#"{"status": "error", "error": "unexpected server error: failed to increment counter"}"#,
                    );
                    return;
                }

                // Write out HTTP response
                write_http_response(response, 200, &Vec::new(), "0");
            }

            // Any other GET request is interpreted as a static asset request for the UI
            (Method::Get, asset_path) => {
                let path = asset_path.join("/");
                match get_static_asset(&path) {
                    Ok(bytes) => write_http_response(response, 200, &Vec::new(), bytes),
                    Err(err) => {
                        eprintln!("failed to retreive static asset @ [{path}]: {err:?}");
                        write_http_response(response, 404, &Vec::new(), "not found");
                    }
                };
            }

            // All other method + path combinations are unrecognized operations
            _ => write_http_response(
                response,
                400,
                &Vec::new(),
                r#"{"status":"error,"error": "unrecognized operation"}"#,
            ),
        };
    }
}
