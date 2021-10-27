use std::io::prelude::*;
use std::net::TcpListener;
use serde_json::Value;

fn get_response_header<'a>() -> &'a str {
    r#"
{
  "holeNumber": "hole12",
  "archiveFilename": "Archive_211001_140321",
  "archiveTickCount": "845"
}
"#
}

fn get_response_items<'a>() -> Vec<&'a str> {
    vec![r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 1,
   "events" : [
      "TEE_RECT"
   ],
   "label" : 0,
   "m_sphere.radius" : 2.43117189407349,        // radius of ball
   "pos" : [
      35.7651251552959,
      10.1044384407889,
      1.99999999999943
   ],
   "shot_count" : 0,
   "tick" : 104792,
   "time_sec" : 55675.611559,
   "vel" : [
      -1.43420169544014,
      -2.60537792789491,
      0
   ]
}
"#,r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 2,
   "events" : [
      "STOPPED"
   ],
   "label" : 0,
   "m_sphere.radius" : 1.81232150395711,
   "pos" : [
      35.4990601849867,
      10.256205154202,
      1.16103691994983
   ],
   "shot_count" : 0,
   "tick" : 104798,
   "time_sec" : 55676.211993,
   "vel" : [
      0.223588302871757,
      -0.378118526708058,
      0
   ]
}
"#,r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 3,
   "events" : [
      "MOVING"
   ],
   "label" : 0,
   "m_sphere.radius" : 1.75107336044312,
   "pos" : [
      441.823339168579,
      426.79626979401,
      -12
   ],
   "shot_count" : 1,
   "tick" : 105147,
   "time_sec" : 55711.134976,
   "vel" : [
      -7.20664948775637,
      -37.8342280718204,
      0
   ]
}
"#,r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 4,
   "events" : [
      "CUP"
   ],
   "label" : 0,
   "m_sphere.radius" : 1.74538373947144,
   "pos" : [
      439.592231994182,
      429.815724232712,
      -12
   ],
   "shot_count" : 1,
   "tick" : 105157,
   "time_sec" : 55712.134988,
   "vel" : [
      0.119569116498076,
      0.153994787065098,
      0
   ]
}
"#]
}

fn get_response_footer<'a>() -> &'a str {
    r#"
{
  "holeNumber": "hole12",
  "archiveFilename": "Archive_211001_140321",
  "Results": "DONE"
}
"#
}

fn get_error_response<'a>() -> &'a str {
    r#"
{
  "holeNumber": "hole12",
  "archiveFilename": "Archive_211001_140321",
  "Results": "ERROR",
  "Details": "This error occurred because ...."
}
"#
}

const REQUEST_PORT: u32 = 8080;
const RESPONSE_PORT: u32 = 8081;

fn get_url(port: u32) -> String {
    format!("localhost:{}", port)
}

/// parse_json_request: parse json request
/// return value is error state (true for error)
fn parse_json_request<'a>(r: &'a str) -> bool {
    let r_json = serde_json::from_str::<Value>(&r);
    match r_json {
        Ok(json) => {
            let pp =
                serde_json::to_string_pretty(&json).unwrap();
            println!("Received Request Json Parsed: {}", pp);
            false
        },
        Err(e) => {
            println!("Error: parsing json request: {}", e);
            true
        }
    }
}

fn handle_connections(request_listener: TcpListener, 
                      response_listener: TcpListener) {

    println!("accepting request connections on {}", REQUEST_PORT);
    for request_stream in request_listener.incoming() {
        match request_stream {
            Ok(mut request_stream) => {
                let mut buffer = [0; 1024];
                request_stream.read(&mut buffer).unwrap();

                let request = String::from_utf8_lossy(&buffer);
                let error = parse_json_request(&request);
                println!("received=[{}]", request);
                println!("error={}", error);
                println!("accepting response connections on {}", RESPONSE_PORT);

                let response_stream = response_listener.accept();
                match response_stream {
                    Ok((mut response_stream, _addr)) => {
                        if error {
                            let error_response = get_error_response();
                            response_stream.write(error_response.as_bytes()).unwrap();
                        } else {
                            let response_header = get_response_header();
                            println!("sending response_header=[{}]", response_header);
                            response_stream.write(response_header.as_bytes()).unwrap();
                            let response_items = get_response_items();
                            let response_footer = get_response_footer();
                            for response_item in response_items.iter() {
                                println!("sending response_item=[{}]", response_item);
                                response_stream.write(response_item.as_bytes()).unwrap();
                            }
                            println!("sending response_footer=[{}]", response_footer);
                            response_stream.write(response_footer.as_bytes()).unwrap();
                            response_stream.flush().unwrap();
                        }
                    }
                    Err(e) => {
                        println!("Error: response connection failure: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error: request connection failure: {:?}", e);
                break;
            }
        }
    }
}

fn main() {
    let request_listener = TcpListener::bind(get_url(REQUEST_PORT)).unwrap();
    let response_listener = TcpListener::bind(get_url(RESPONSE_PORT)).unwrap();

    handle_connections(request_listener, response_listener);
}
