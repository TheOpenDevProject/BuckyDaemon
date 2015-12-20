struct JobListener{
	pub server_name: String,
	pub server_port: i32
}

impl JobListener{
	fn new(server: String,port: i32) -> JobListener{
		JobListener{server_name: server,server_port:port}
	}

	fn output(){
		println!("Test");
	}
}