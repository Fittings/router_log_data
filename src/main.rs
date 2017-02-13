use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;

static FILE_LOCATION: &'static str = "C:/Users/Cameron/AppData/Local/visualsyslog/syslog1";

fn main() {
    let f = File::open(FILE_LOCATION).expect("Unable to open file");

    let sys_log_data = retrieve_sys_log_data(BufReader::new(&f));

    sys_log_data.print_lines();
    println!("Disconnected {} times.", sys_log_data.get_disconnect_count());
}


#[derive(Debug)]
struct SysLogData {
    log_lines: Vec<SysLogLine>,
}

impl SysLogData {
    fn new<'b>() -> Self {
        SysLogData {
            log_lines: Vec::new(),
        }
    }

    fn lines(&self) -> &Vec<SysLogLine> {
        &self.log_lines
    }

    fn add_line<'b>(&'b mut self, line: SysLogLine) {
        self.log_lines.push(line);
    }

    fn print_lines(&self) {
        for line in self.log_lines.iter() {
            println!("{:?}", line);
        }
    }

    fn get_disconnect_count(&self) -> i32 {
        let mut count = 0;
        for line in self.log_lines.iter() {
            count += 1;
        }
        count
    }
}

#[derive(Debug)]
struct SysLogLine {
    ip_address: String,
    time: String,
    hostname: String,
    facility: String,
    priority: String,
    tag: String,
    message: String,
}

impl fmt::Display for SysLogLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {} ",
              &self.ip_address, self.time, self.hostname, self.facility, self.priority, self.tag, self.message)
    }

}

fn retrieve_sys_log_data<'a, R: BufRead>(reader : R) -> SysLogData {
    let mut data = SysLogData::new();

    for line in reader.lines() {
        let log_line = convert_line(line.unwrap());

        data.add_line(log_line);
    }

    data
}

fn convert_line(line_string: String) -> SysLogLine {
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

    SysLogLine {
        ip_address: ip_address,
        time: time,
        hostname: hostname,
        facility: facility,
        priority: priority,
        tag: tag,
        message: msg,
    }
}



