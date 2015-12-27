#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;

pub struct Config{
		serverName: String,
		serverAddress: String,
		maxConcurrentConnections: u8,
		maxConcurrentJobsRunning: u8
}

//A Simple Implementation To Get The Config Object
 impl ConfigManager{
	pub fn load(config_to_load: String) -> ConfigManager{
		//FileLoad
		let mut file = match File::open(&config_to_load){
			Err(err_desc) => panic!("Failed To Load BuckyDaemon Main Config (Reason: {})",Error::description(&err_desc)),
			Ok(file) => file,
		};
		
		Config {file_name: config_to_load}
	}
}