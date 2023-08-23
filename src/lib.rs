wit_bindgen::generate!({
    world: "kvcounter",
    exports: {
        "wasi:http/incoming-handler": KvCounter
    }
});

use wasi::{
    http::http_types::{
        finish_outgoing_stream, incoming_request_method, new_fields, new_outgoing_response,
        outgoing_response_write, set_response_outparam, Method, ResponseOutparam, incoming_request_path_with_query,
    },
    io::streams::write,
    keyvalue::{
        readwrite::{get, set},
        types::{
            incoming_value_consume_sync, new_outgoing_value, open_bucket, outgoing_value_write_body,
        },
    },
};

use crate::exports::wasi::http::incoming_handler::{IncomingHandler, IncomingRequest};

const BUCKET: &str = "default";

struct KvCounter;

impl KvCounter {
    /// Increment (possibly negatively) the counter for a given key
    fn increment_counter(bucket: u32, key: &String, amount: i32) {
        let incoming_value = get(bucket, key).expect("failed to get resource");
        // TODO: set the value to 1 if key is missing

        let bytes =
            incoming_value_consume_sync(incoming_value).expect("failed to parse incoming bytes");
        let value: i32 = String::from_utf8(bytes)
            .expect("failed to parse string from returned bytes")
            .trim()
            .parse()
            .expect("failed to parse numeric value from bucket");

        let outgoing_value = new_outgoing_value();
        let stream =
            outgoing_value_write_body(outgoing_value).expect("failed to write outgoing value");
        write(stream, (value + amount).to_string().as_bytes())
            .expect("failed to write to outgoing value stream");
        set(bucket, key, outgoing_value).expect("failed to set value");
    }

    /// Retrieve static asset from disk
    fn get_asset(_path: impl AsRef<str>) -> Vec<u8> {
        todo!()
    }
}

/// Write a WASI HTTP response out
fn write_wasi_http_response(body: impl AsRef<[u8]>, response_outparam: ResponseOutparam) {
    let headers = new_fields(&Vec::new());
    let outgoing_response = new_outgoing_response(200, headers).expect("failed to create response");
    let outgoing_stream =
        outgoing_response_write(outgoing_response).expect("failed to write outgoing response");
    write(outgoing_stream, body.as_ref()).expect("failed to write output to stream");
    finish_outgoing_stream(outgoing_stream);
    set_response_outparam(response_outparam, Ok(outgoing_response))
        .expect("failed to set response");
}

impl IncomingHandler for KvCounter {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // Decipher method
        let method = incoming_request_method(request);

        // Decipher path
        let path_with_query = incoming_request_path_with_query(request).expect("invalid path");
        let path = match path_with_query.split_once("?") {
            Some((v, _)) => v,
            _ => "default",
        };
        let trimmed_path: Vec<&str> = path.trim_matches('/').split('/').collect();

        // Generate an outgoing request
        match (method, trimmed_path.as_slice()) {
            (Method::Get, ["api", "counter"]) => {
                let bucket = open_bucket(BUCKET).expect("failed to open bucket");
                KvCounter::increment_counter(bucket, &String::from("default"), 1);

                // TODO: actually perform counter update

                // Build & write the response the response
                write_wasi_http_response("0", response);
            }
            (Method::Get, ["api", "counter", counter]) => {
                let bucket = open_bucket(BUCKET).expect("failed to open bucket");
                KvCounter::increment_counter(bucket, &counter.to_string(), 1);

                // TODO: actually perform counter update

                write_wasi_http_response("0", response);
            }
            // Any other GET request is interpreted as a static asset request for the UI
            (Method::Get, asset_path) => {
                KvCounter::get_asset(asset_path.join("/"));

                // TODO: actually retrieve asset

                // TODO: write the body of the asset to disk

                write_wasi_http_response(
                    r#"
<!DOCTYPE html>
<html>
  <head><title>Example Page</title></head>
  <body>
  <h1>Not Implemented</h1>
  </body>
</html>
"#,
                    response,
                );
            }
            (_, _) => {
                // TODO: return not found
                write_wasi_http_response(
                    r#"
<!DOCTYPE html>
<html>
  <head><title>Not Found</title></head>
  <body>
  <h1>404 - Page not found</h1>
  </body>
</html>
"#,
                    response,
                );
            }
        };
    }
}
