use std::fmt;

#[derive(Debug)]
pub struct SysLogData {
    log_lines: Vec<SysLogLine>,
}

impl SysLogData {
    pub fn new<'b>() -> Self {
        SysLogData {
            log_lines: Vec::new(),
        }
    }

    pub fn lines(&self) -> &Vec<SysLogLine> {
        &self.log_lines
    }

    pub fn add_line<'b>(&'b mut self, line: SysLogLine) {
        self.log_lines.push(line);
    }

    pub fn print_lines(&self) {
        for line in self.log_lines.iter() {
            println!("{:?}", line);
        }
    }

    pub fn get_disconnect_count(&self) -> i32 {
        let mut count = 0;
        for line in self.log_lines.iter() {
            count += 1;
        }
        count
    }
}

#[derive(Debug)]
pub struct SysLogLine {
    pub ip_address: String,
    pub time: String,
    pub hostname: String,
    pub facility: String,
    pub priority: String,
    pub tag: String,
    pub message: String,
}


impl fmt::Display for SysLogLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {} ",
              &self.ip_address, self.time, self.hostname, self.facility, self.priority, self.tag, self.message)
    }

}