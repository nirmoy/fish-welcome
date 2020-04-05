extern crate termion;

use termion::{color, style};
use std::process::{Command};

fn main() {
    let os = Command::new("uname")
            .args(&["-s", "-n", "-v"])
            .output()
            .expect("Error");
    let uptime = Command::new("uptime")
            .output()
            .expect("Error");
    let disk = Command::new("sh")
            .arg("-c")
            .arg("df -h|grep -E 'dev/(xvda|sd|mapper|nvme)'|grep -v efi")
            .output()
            .expect("Error");
    print!("{}{}OS:{}", color::Fg(color::Green), style::Bold, style::Reset);
    print!("{} {}", color::Fg(color::Red), String::from_utf8_lossy(&os.stdout));
    print!("{}{}Uptime:{}", color::Fg(color::Green), style::Bold, style::Reset);
    print!("{}{}", color::Fg(color::Red), String::from_utf8_lossy(&uptime.stdout));
    println!("{}{}Disk Usage:{}", color::Fg(color::Green), style::Bold, style::Reset);
    print!("{}{}", color::Fg(color::Red), String::from_utf8_lossy(&disk.stdout));
}
