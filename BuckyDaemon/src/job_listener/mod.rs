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

			match request.method(){
				&tiny_http::Method::Get => self.handle_post(request.url()),
				&tiny_http::Method::Post => self.handle_get(request.url()),
				_ => println!("Unknown Method::Cant_Resolve")
			}
		}
	}

	fn handle_post(&self, url: &str){
		println!("POST request made {:?}",url);
	}

	fn handle_get(&self,url: &str){
		println!("GET request made", );
	}
}