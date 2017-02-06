use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;



static FILE_LOCATION: &'static str = "C:/Users/Cameron/AppData/Local/visualsyslog/syslog1";

#[derive(Debug)]
struct SysLogData<'a> {
    log_lines: Vec<SysLogLine<'a>>,
}

impl<'a> SysLogData<'a> {
    fn lines(&self) -> &Vec<SysLogLine> {
        &self.log_lines
    }
}



#[derive(Debug)]
struct SysLogLine<'a> {
    //time
    ip_address: &'a str,
    hostname: &'a str,
    facility: &'a str,
    priority: &'a str,
    tag: &'a str,
    message: &'a str,
}



fn main() {
    let f = File::open(FILE_LOCATION).expect("Unable to open file");
    let mut reader = BufReader::new(&f);

    read_lines(&mut reader);
}



fn read_lines<R: BufRead>(reader: &mut R) {
    let mut count = 0;

    for line in reader.lines() {
        let line_string = line.unwrap();
        count += 1;

        //convert a line into a SysLogLine struct
        let mut ip_address: &str;
        let mut hostname: &str;
        let mut facility: &str;
        let mut priority: &str;
        let mut tag: &str;
        let mut message: &str;

        for (i, token) in line_string.split_whitespace().enumerate() {
            match i {
                1 => ip_address = token,
                2 => hostname = token,
                3 => facility = token,
                4 => priority = token,
                _ => (),
            }
            print!("{} ", token);
        }
        println!("");

    }


    println!("There are {} lines.", count);
}



