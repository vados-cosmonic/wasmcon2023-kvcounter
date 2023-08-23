wit_bindgen::generate!({
    world: "kvcounter",
    exports: {
        "wasi:http/incoming-handler": KvCounter
    }
});

use wasi::{
    http::http_types::{
        finish_outgoing_stream, incoming_request_method, new_fields, new_outgoing_response,
        outgoing_response_write, set_response_outparam, Method, ResponseOutparam, incoming_request_path,
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

// ///////////
// // Types //
// ///////////

// struct KvCounterTypes;

// impl types for KvCounterType {
//     async fn drop_fields(&mut self, fields: types::Fields) -> anyhow::Result<()> {
//         todo!()
//     }
//     async fn new_fields(
//         &mut self,
//         entries: Vec<(String, Vec<u8>)>,
//     ) -> anyhow::Result<types::Fields> {
//         todo!()
//     }
//     async fn fields_get(
//         &mut self,
//         fields: types::Fields,
//         name: String,
//     ) -> anyhow::Result<Vec<Vec<u8>>> {
//         let fields = self
//             .table
//             .get_fields(fields)
//             .context("failed to get fields")?;
//         let fields = fields.get(&name).context("key does not exist")?;
//         Ok(fields.clone())
//     }
//     async fn fields_set(
//         &mut self,
//         fields: types::Fields,
//         name: String,
//         value: Vec<Vec<u8>>,
//     ) -> anyhow::Result<()> {
//         let fields = self
//             .table
//             .get_fields_mut(fields)
//             .context("failed to get fields")?;
//         fields.insert(name, value);
//         Ok(())
//     }
//     async fn fields_delete(&mut self, fields: types::Fields, name: String) -> anyhow::Result<()> {
//         let fields = self
//             .table
//             .get_fields_mut(fields)
//             .context("failed to get fields")?;
//         fields.remove(&name).context("key not found")?;
//         Ok(())
//     }
//     async fn fields_append(
//         &mut self,
//         fields: types::Fields,
//         name: String,
//         value: Vec<u8>,
//     ) -> anyhow::Result<()> {
//         let fields = self
//             .table
//             .get_fields_mut(fields)
//             .context("failed to get fields")?;
//         fields.entry(name).or_default().push(value);
//         Ok(())
//     }
//     async fn fields_entries(
//         &mut self,
//         fields: types::Fields,
//     ) -> anyhow::Result<Vec<(String, Vec<u8>)>> {
//         let fields = self
//             .table
//             .get_fields(fields)
//             .context("failed to get fields")?;
//         Ok(fields
//             .iter()
//             .flat_map(|(k, v)| v.iter().map(|v| (k.clone(), v.clone())).collect::<Vec<_>>())
//             .collect())
//     }
//     async fn fields_clone(&mut self, fields: types::Fields) -> anyhow::Result<types::Fields> {
//         let fields = self
//             .table
//             .get_fields(fields)
//             .context("failed to get fields")?;
//         let fields = fields.clone();
//         self.table
//             .push_fields(fields)
//             .context("failed to push fields")
//     }
//     async fn finish_incoming_stream(
//         &mut self,
//         s: types::IncomingStream,
//     ) -> anyhow::Result<Option<types::Trailers>> {
//         self.table
//             .get_input_stream_mut(s)
//             .context("failed to get output stream")?;
//         // TODO: Read to end and get trailers
//         Ok(None)
//     }
//     async fn finish_outgoing_stream(&mut self, s: types::OutgoingStream) -> anyhow::Result<()> {
//         self.table
//             .get_output_stream_mut(s)
//             .context("failed to get output stream")?;
//         // TODO: Close
//         Ok(())
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn finish_outgoing_stream_with_trailers(
//         &mut self,
//         s: types::OutgoingStream,
//         trailers: types::Trailers,
//     ) -> anyhow::Result<types::FutureWriteTrailersResult> {
//         self.table
//             .get_output_stream_mut(s)
//             .context("failed to get output stream")?;
//         // TODO: Close
//         bail!("trailers not supported yet")
//     }

//     #[allow(unused)] // TODO: Remove
//     async fn drop_future_trailers(&mut self, f: types::FutureTrailers) -> anyhow::Result<()> {
//         bail!("trailers not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn future_trailers_get(
//         &mut self,
//         f: types::FutureTrailers,
//     ) -> anyhow::Result<Option<Result<types::Trailers, types::Error>>> {
//         bail!("trailers not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn listen_to_future_trailers(
//         &mut self,
//         f: types::FutureTrailers,
//     ) -> anyhow::Result<types::Pollable> {
//         bail!("trailers not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn drop_future_write_trailers_result(
//         &mut self,
//         f: types::FutureWriteTrailersResult,
//     ) -> anyhow::Result<()> {
//         bail!("trailers not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn future_write_trailers_result_get(
//         &mut self,
//         f: types::FutureWriteTrailersResult,
//     ) -> anyhow::Result<Option<Result<(), types::Error>>> {
//         bail!("trailers not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn listen_to_future_write_trailers_result(
//         &mut self,
//         f: types::FutureWriteTrailersResult,
//     ) -> anyhow::Result<types::Pollable> {
//         bail!("trailers not supported yet")
//     }

//     async fn drop_incoming_request(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<()> {
//         self.table
//             .delete_incoming_request(request)
//             .context("failed to delete incoming request")?;
//         Ok(())
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn drop_outgoing_request(
//         &mut self,
//         request: types::OutgoingRequest,
//     ) -> anyhow::Result<()> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     async fn incoming_request_method(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<types::Method> {
//         let IncomingRequest { method, .. } = self
//             .table
//             .get_incoming_request(request)
//             .context("failed to get incoming request")?;
//         Ok(method.clone())
//     }
//     async fn incoming_request_path_with_query(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<Option<String>> {
//         let IncomingRequest {
//             path_with_query, ..
//         } = self
//             .table
//             .get_incoming_request(request)
//             .context("failed to get incoming request")?;
//         Ok(path_with_query.clone())
//     }
//     async fn incoming_request_scheme(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<Option<types::Scheme>> {
//         let IncomingRequest { scheme, .. } = self
//             .table
//             .get_incoming_request(request)
//             .context("failed to get incoming request")?;
//         Ok(scheme.clone())
//     }
//     async fn incoming_request_authority(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<Option<String>> {
//         let IncomingRequest { authority, .. } = self
//             .table
//             .get_incoming_request(request)
//             .context("failed to get incoming request")?;
//         Ok(authority.clone())
//     }
//     async fn incoming_request_headers(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<types::Headers> {
//         let IncomingRequest { headers, .. } = self
//             .table
//             .get_incoming_request(request)
//             .context("failed to get incoming request")?;
//         Ok(*headers)
//     }
//     async fn incoming_request_consume(
//         &mut self,
//         request: types::IncomingRequest,
//     ) -> anyhow::Result<Result<types::IncomingStream, ()>> {
//         let IncomingRequest { body, .. } = self
//             .table
//             .delete_incoming_request(request)
//             .context("failed to delete incoming request")?;
//         let stream = self
//             .table
//             .push_input_stream(Box::new(AsyncStream(body)))
//             .context("failed to push input stream")?;
//         Ok(Ok(stream))
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn new_outgoing_request(
//         &mut self,
//         method: types::Method,
//         path_with_query: Option<String>,
//         scheme: Option<types::Scheme>,
//         authority: Option<String>,
//         headers: types::Headers,
//     ) -> anyhow::Result<Result<types::OutgoingRequest, types::Error>> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn outgoing_request_write(
//         &mut self,
//         request: types::OutgoingRequest,
//     ) -> anyhow::Result<Result<types::OutgoingStream, ()>> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     async fn drop_response_outparam(
//         &mut self,
//         response: types::ResponseOutparam,
//     ) -> anyhow::Result<()> {
//         self.table
//             .delete_response_outparam(response)
//             .context("failed to delete outgoing response parameter")?;
//         Ok(())
//     }
//     async fn set_response_outparam(
//         &mut self,
//         param: types::ResponseOutparam,
//         response: Result<types::OutgoingResponse, types::Error>,
//     ) -> anyhow::Result<Result<(), ()>> {
//         let param = self
//             .table
//             .get_response_outparam_mut(param)
//             .context("failed to get outgoing response parameter")?;
//         let _ = param.insert(response);
//         Ok(Ok(()))
//     }

//     #[allow(unused)] // TODO: Remove
//     async fn drop_incoming_response(
//         &mut self,
//         response: types::IncomingResponse,
//     ) -> anyhow::Result<()> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     async fn drop_outgoing_response(
//         &mut self,
//         response: types::OutgoingResponse,
//     ) -> anyhow::Result<()> {
//         self.table
//             .delete_outgoing_response(response)
//             .context("failed to delete outgoing response")?;
//         Ok(())
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn incoming_response_status(
//         &mut self,
//         response: types::IncomingResponse,
//     ) -> anyhow::Result<types::StatusCode> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn incoming_response_headers(
//         &mut self,
//         response: types::IncomingResponse,
//     ) -> anyhow::Result<types::Headers> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn incoming_response_consume(
//         &mut self,
//         response: types::IncomingResponse,
//     ) -> anyhow::Result<Result<types::IncomingStream, ()>> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     async fn new_outgoing_response(
//         &mut self,
//         status_code: types::StatusCode,
//         headers: types::Headers,
//     ) -> anyhow::Result<Result<types::OutgoingResponse, types::Error>> {
//         let response = self
//             .table
//             .push_outgoing_response(OutgoingResponse {
//                 status_code,
//                 headers,
//                 body: AsyncVec::default(),
//             })
//             .context("failed to push fields")?;
//         Ok(Ok(response))
//     }
//     async fn outgoing_response_write(
//         &mut self,
//         response: types::OutgoingResponse,
//     ) -> anyhow::Result<Result<types::OutgoingStream, ()>> {
//         let OutgoingResponse { body, .. } = self
//             .table
//             .get_outgoing_response(response)
//             .context("failed to delete outgoing response")?;
//         let stream = self
//             .table
//             .push_output_stream(Box::new(AsyncStream(body.clone())))
//             .context("failed to push output stream")?;
//         Ok(Ok(stream))
//     }

//     #[allow(unused)] // TODO: Remove
//     async fn drop_future_incoming_response(
//         &mut self,
//         f: types::FutureIncomingResponse,
//     ) -> anyhow::Result<()> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn future_incoming_response_get(
//         &mut self,
//         f: types::FutureIncomingResponse,
//     ) -> anyhow::Result<Option<Result<types::IncomingResponse, types::Error>>> {
//         bail!("outgoing HTTP not supported yet")
//     }
//     #[allow(unused)] // TODO: Remove
//     async fn listen_to_future_incoming_response(
//         &mut self,
//         f: types::FutureIncomingResponse,
//     ) -> anyhow::Result<types::Pollable> {
//         bail!("outgoing HTTP not supported yet")
//     }
// }


/////////////
// Handler //
/////////////

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
fn write_wasi_http_response(body: impl AsRef<[u8]>, _response_outparam: ResponseOutparam) {
    let headers = new_fields(&Vec::new());
    let outgoing_response = new_outgoing_response(200, headers);
    let outgoing_stream =
        outgoing_response_write(outgoing_response).expect("failed to write outgoing response");
    write(outgoing_stream, body.as_ref()).expect("failed to write output to stream");
    finish_outgoing_stream(outgoing_stream, None);
    // ??? This no longer takes the response outparam... Are we limited to one request at a time?
    set_response_outparam(Ok(outgoing_response)).expect("failed to set response");
}

impl IncomingHandler for KvCounter {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // Decipher method
        let method = incoming_request_method(request);

        // Decipher path
        let path = incoming_request_path(request);
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
