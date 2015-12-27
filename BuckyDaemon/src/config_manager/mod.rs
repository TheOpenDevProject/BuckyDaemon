extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;
pub struct ConfigManager{
	file_name : String
}

 impl ConfigManager{
	pub fn new(config_to_load: String) -> ConfigManager{
		ConfigManager {file_name: config_to_load}
	}

	pub fn load(&self){
		//Get CWD
		let p = env::current_dir().unwrap();
		println!("{:?}",p.display());
		//Open The Config File
		let mut file = match File::open(&self.file_name){
			Err(err_desc) => panic!("Failed To Load BuckyDaemon Main Config (Reason: {})",Error::description(&err_desc)),
			Ok(file) => file,
		};

		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();
		let json = Json::from_str(&data).unwrap();
		println!("{}",json.find_path(&["serverConfig"]).unwrap());
	}
}