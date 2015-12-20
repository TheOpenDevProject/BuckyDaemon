pub struct JobListener{
	pub server_name: String,
	pub server_port: i32
}

impl JobListener{
	pub fn new(server: String,port: i32) -> JobListener{
		JobListener{server_name: server,server_port:port}
	}

	pub fn output(&self){
		println!("Test");
	}
}