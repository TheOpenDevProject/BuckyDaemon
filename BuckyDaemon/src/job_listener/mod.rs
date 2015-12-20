extern crate tiny_http;
use std::env;
use self::tiny_http::{Server, Response};

pub struct JobListener{
	alertAddress:String
}

impl JobListener{
	pub fn new(alertAddress_param:String) -> JobListener{
		JobListener{alertAddress: alertAddress_param}
	}

	pub fn start(&self,server: &str ){
	let server = Server::http(server).unwrap();

	for request in server.incoming_requests() {
    println!("received request! method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    let response = Response::from_string("hello world");
    request.respond(response);
	}
}
}