use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;


static FILE_LOCATION: &'static str = "C:/Users/Cameron/AppData/Local/visualsyslog/syslog1";

#[derive(Debug)]
struct SysLogData {
    log_lines: Vec<SysLogLine>,
}

impl SysLogData {
    fn lines(&self) -> &Vec<SysLogLine> {
        &self.log_lines
    }
}

#[derive(Debug)]
struct SysLogLine {
    //time
    ip_address: String,
    hostname: String,
    facility: String,
    priority: String,
    tag: String,
    message: String,
}


//Take a text file
//Read each line and convert into a line struct
//Place all line structs into a general data struct.

fn main() {
    let f = File::open(FILE_LOCATION).expect("Unable to open file");

    let sys_log_data = retrieve_sys_log_data(BufReader::new(&f));

    for line in sys_log_data.log_lines {
        println!("{:?}", line);
    }
}

fn retrieve_sys_log_data<R: BufRead>(reader : R) -> SysLogData {
    let mut log_lines : Vec<SysLogLine> = Vec::new();

    for line in reader.lines() {
        let line = convert_line(line.unwrap());

        log_lines.push(line);
    }


    SysLogData {
        log_lines: log_lines,
    }
}

fn convert_line(line_string: String) -> SysLogLine {
    let mut ip_address = String::new();
    let mut hostname = String::new();
    let mut facility = String::new();
    let mut priority = String::new();
    let mut tag_and_msg = String::new();

    for (i, token) in line_string.split("\t").enumerate() {
        match i {
            0 => ip_address += token,
            1 => (),
            2 => hostname += token,
            3 => facility += token,
            4 => priority += token,
            _ => tag_and_msg = tag_and_msg + token,
        }
    }

    let mut tag_and_msg = tag_and_msg.split("]");
    let tag = String::new() + tag_and_msg.next().unwrap().trim();
    let mut msg = String::new();
    while let Some(string) = tag_and_msg.next() {
        msg.push_str(string.trim());
    }

    SysLogLine {
        ip_address: ip_address,
        hostname: hostname,
        facility: facility,
        priority: priority,
        tag: tag,
        message: msg,
    }
}



//fn read_lines<R: BufRead>(reader: &mut R) {
//    let mut count = 0;
//
//
//        let line_string = line.unwrap();

//            println!("{:?} ", log_line);
//
//        }
//        println!("");
//
//    }
//
//
//    println!("There are {} lines.", count);
//}



