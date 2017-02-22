use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//
extern crate syslog_reader;
////mod sys_log;
use syslog_reader::sys_log::sys_log_data;


static FILE_LOCATION: &'static str = "C:/Users/Cameron/AppData/Local/visualsyslog/syslog1";

fn main() {
    println!("Hello World");
    let f = File::open(FILE_LOCATION).expect("Unable to open file");

    let sys_log_data = retrieve_sys_log_data(BufReader::new(&f));

    sys_log_data.print_lines();
    println!("Disconnected {} times.", sys_log_data.get_disconnect_count());
}




fn retrieve_sys_log_data<'a, R: BufRead>(reader : R) -> sys_log_data::SysLogData {
    let mut data = sys_log_data::SysLogData::new();

    for line in reader.lines() {
        let log_line = convert_line(line.unwrap());

        data.add_line(log_line);
    }

    data
}

fn convert_line(line_string: String) -> sys_log_data::SysLogLine {
    let mut ip_address = String::new();
    let mut time = String::new();
    let mut hostname = String::new();
    let mut facility = String::new();
    let mut priority = String::new();
    let mut tag_and_msg = String::new();

    for (i, token) in line_string.split("\t").enumerate() {
        match i {
            0 => ip_address += token,
            1 => time += token,
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


    sys_log_data::SysLogLine {
        ip_address: ip_address,
        time: time,
        hostname: hostname,
        facility: facility,
        priority: priority,
        tag: tag,
        message: msg,
    }
}



