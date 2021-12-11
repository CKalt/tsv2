use std::io::prelude::*;
use std::net::TcpListener;
use serde_json::Value;
use std::str;
use structopt::StructOpt;

const DEFAULT_REQUEST_PORT:  &str = "8080";
const DEFAULT_RESPONSE_PORT: &str = "8081";

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(default_value = DEFAULT_REQUEST_PORT, short = "i", long =           "input-port")]
    input_port: u32,
    #[structopt(default_value = DEFAULT_RESPONSE_PORT, short = "o", long =          "output-port")]
    output_port: u32,
    #[structopt(default_value = "localhost", short = "h", long = "host")]
    host: String,
}

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

fn get_url(host: &String, port: u32) -> String {
    format!("{}:{}", host, port)
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
            println!("{:?}", r.as_bytes());
            true
        }
    }
}

fn handle_connections(request_listener: TcpListener, 
                      response_listener: TcpListener,
                      opt: Opt) {

    println!("TP000: waiting for incoming connection on request_stream");
    for request_stream in request_listener.incoming() {
        match request_stream {
            Ok(mut request_stream) => {
                // read exactly 8 bytes for subsequent content length
                // and convert them from a hex number to a usize
                let mut len_buf: [u8; 8] = [0; 8];
                println!("TP001: calling req-strm read_exact to obtain <length-method> \n\
                          bug len={:?}", len_buf.len());
                request_stream.read_exact(&mut len_buf).unwrap();
                println!("TP001.1: returned <length message> got len={} \n\
                          bytes read_exact buf={:?}",
                    len_buf.len(), len_buf);

                let len_str = str::from_utf8(&len_buf).unwrap();
                let bytes_to_read: usize
                    = usize::from_str_radix(len_str, 16).unwrap();
                println!("TP001.2: converts to hex str={} or bytes_to_read={}",
                    len_str, bytes_to_read);

                // read exactly `bytes_to_read` len and error if not 
                // valid json
                let mut request_buf = vec![0u8; bytes_to_read];
                println!("TP002: calling req-strm read_exact on request port \n\
                          into buffer size={} ",
                        request_buf.len());
                request_stream.read_exact(&mut request_buf).unwrap();
                let request = str::from_utf8(&request_buf).unwrap();

                println!("TP002.1: {} bytes received as request = [{}]", bytes_to_read, request);
                let error = parse_json_request(&request);
                println!("{}", if error { "invalid JSON" } else { "valid JSON" });

                println!("accepting response connections on {}", opt.output_port);

                println!("TP003: accepting connection for response_stream");

                let response_stream = response_listener.accept();
                println!("TP003.1: accepted connection for response_stream");
                match response_stream {
                    Ok((mut response_stream, _addr)) => {
                        if error {
                            let error_response = get_error_response();
                            response_stream.write(error_response.as_bytes()).unwrap();
                        } else {
                            let response_header = get_response_header();

                            // first send len msg for response header.
                            let len_msg = format!("{:08x}", response_header.len());
                            println!("TP004: response_stream write len msg for\n\
                                      response header len={} msg={}",
                                response_header.len(),
                                len_msg);
                            let wresult = response_stream.write(len_msg.as_bytes());
                            if let Err(e) = wresult {
                                panic!("error writing len_msg: e={}", e);
                            }

                            // now send the header itself
                            println!("TP005: sending response_header=[{}]", response_header);
                            response_stream.write(response_header.as_bytes()).unwrap();
                            println!("TP005.1: send returned");
                            let response_items = get_response_items();
                            let response_footer = get_response_footer();

                            let mut tp_i = 6;

                            for response_item in response_items.iter() {
                                // first send len msg for response item.
                                let len_msg = format!("{:08x}", response_item.len());
                                println!("TP00{}: response_stream write len msg for\n\
                                          response header len={} msg={}",
                                    tp_i,
                                    response_header.len(),
                                    len_msg);
                                let wresult = response_stream.write(len_msg.as_bytes());
                                if let Err(e) = wresult {
                                    panic!("error writing len_msg: e={}", e);
                                }

                                tp_i += 1;

                                println!("TP00{}: sending response_item=[{}]", tp_i, response_item);
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
    let opt = Opt::from_args();
    println!("opt={:?}", opt);

    let request_listener =
        TcpListener::bind(get_url(&opt.host, opt.input_port)).unwrap();
    println!("accepting request connections on {}",
        get_url(&opt.host, opt.input_port));

    let response_listener =
        TcpListener::bind(get_url(&opt.host, opt.output_port)).unwrap();
    println!("accepting response connections on {}",
        get_url(&opt.host, opt.output_port));

    handle_connections(request_listener, response_listener, opt);
}
