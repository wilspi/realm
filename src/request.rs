use http;
use hyper;

pub struct Request {
    // headers: http::header::HeaderMap,
    // cookies:
    // path
    // query: Query(HashMap<String, Vec<String>>), Query.get, get_list
    // client_ip
    // body: []u8

    // response headers, cookies etc are to be put inside response object, like django
}

impl Request {
    pub fn from(head: http::request::Parts, body: Vec<u8>) -> Self {
        unimplemented!()
    }
}
