
mod job_listener;
use job_listener::JobListener;
fn main(){
	let test = JobListener::new("0.0.0.0:8999".to_string());
	test.start("0.0.0.0:8999");
}
