use std::env;
use std::process;
use fastq_cmd::Config;

fn main() {
    let args:Vec<String>=env::args().collect();

    let config=Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Time set:{}",config.time_hr);
    println!("File name:{}",config.file_name);
    if let Err(e)=fastq_cmd::run(config){
        println!("Application error:{e}");
        process::exit(1);
    }

}

