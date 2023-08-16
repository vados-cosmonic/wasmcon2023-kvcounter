wit_bindgen::generate!("kvcounter");

use wasi::{
    http::types::{
        finish_outgoing_stream, incoming_request_method, incoming_request_path_with_query,
        new_fields, new_outgoing_response, outgoing_response_write, set_response_outparam, Method,
        ResponseOutparam,
    },
    io::streams::write,
};

use crate::exports::wasi::http::incoming_handler::{IncomingHandler, IncomingRequest};

struct KvCounter;

impl KvCounter {
    /// Increment (possibly negatively) the counter for a given key
    fn increment_counter(_key: impl AsRef<str>, _amount: i32) {
        todo!()
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
    let outgoing_stream = outgoing_response_write(outgoing_response).expect("failed to write outgoing response");
    write(outgoing_stream, body.as_ref()).expect("failed to write output to stream");
    finish_outgoing_stream(outgoing_stream);
    set_response_outparam(response_outparam, Ok(outgoing_response)).expect("failed to set response");
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
                KvCounter::increment_counter("default", 1);

                // TODO: actually perform counter update

                // Build & write the response the response
                write_wasi_http_response("0", response);
            }
            (Method::Get, ["api", "counter", counter]) => {
                KvCounter::increment_counter(counter, 1);

                // TODO: actually perform counter update

                write_wasi_http_response("0", response);
            }
            // Any other GET request is interpreted as a static asset request for the UI
            (Method::Get, asset_path) => {
                KvCounter::get_asset(asset_path.join("/"));

                // TODO: actually retrieve asset

                // TODO: write the body of the asset to disk

                write_wasi_http_response(r#"
<!DOCTYPE html>
<html>
  <head><title>Example Page</title></head>
  <body>
  <h1>Not Implemented</h1>
  </body>
</html>
"#, response);
            }
            (_, _) => {
                // TODO: return not found
                write_wasi_http_response(r#"
<!DOCTYPE html>
<html>
  <head><title>Not Found</title></head>
  <body>
  <h1>404 - Page not found</h1>
  </body>
</html>
"#, response);
            }
        };

        // Build response
    }
}

export_kvcounter!(KvCounter);
