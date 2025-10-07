use std::fs::File;
use std::io::Write;
use chrono::{Utc, SecondsFormat};
use super::MMU;

impl MMU {
    pub fn dump_ram(&self) {
        let now = Utc::now();
        let ts = now.to_rfc3339_opts(SecondsFormat::Millis, true).replace(":", "").replace(".", "-");
        let mut file = File::create(format!("dev/dumps/ram_{}.bin", ts)).unwrap();
        file.write_all(&self.mem).unwrap();
    }
}