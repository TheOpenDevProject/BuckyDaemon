mod job_listener;
use job_listener::JobListener;
fn main(){
	let test = JobListener::new("0.0.0.0".to_string(),9999);
	test.output();
}
