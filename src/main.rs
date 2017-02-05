use std::fs::File;
use std::io::Read;

static FILE_LOCATION: &'static str = "C:/Users/Cameron/AppData/Local/visualsyslog/syslog";

fn main() {
    let mut data = String::new();
    let mut syslog_file = File::open(FILE_LOCATION).expect("Unable to open file");
    syslog_file.read_to_string(&mut data).expect("Unable to read to String");
}




