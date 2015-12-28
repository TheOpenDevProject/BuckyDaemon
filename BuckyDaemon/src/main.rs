
mod job_listener;
mod config_manager;
use config_manager::ConfigManager;
use job_listener::JobListener;
fn main(){
	let configTest = ConfigManager::load("config/main_config.json".to_string());
	let test = JobListener::new("0.0.0.0:8999".to_string());
	test.start("0.0.0.0:8999");
}
