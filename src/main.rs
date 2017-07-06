extern crate regex;

use std::io;
use std::io::prelude::*;

use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let start_fn = Regex::new(r"^[A-Fa-f0-9]{16} <([^>]*)>:").unwrap();
    let insn = Regex::new(r"^  [A-Fa-f0-9]{6}: *(.*)\t(.*)").unwrap();
    let sym = Regex::new(r"(.*) [A-Fa-f0-9]{6} <([^>]*)>(.*)").unwrap();
    let rip = Regex::new(r"(?P<b>.*)0x[A-Fa-f0-9]{6}\(%rip\)(?P<e>.*)#\s+(?P<n>[^>]*).*").unwrap();
    'next_line: for line in stdin.lock().lines() {
        let line = line.unwrap();
        for cap in start_fn.captures_iter(&line) {
            println!("################ {} ################", &cap[1]);
            continue 'next_line;
        }
        for cap in insn.captures_iter(&line) {
            let line = &cap[2];
            let mut toprint = String::new();
            toprint.push_str(line);
            if sym.is_match(line) {
                for cap in sym.captures_iter(line) {
                    toprint.clear();
                    toprint.push_str(&format!("{} {}{}", &cap[1], &cap[2], &cap[3]));
                }
            }
            if rip.is_match(&toprint) {
                let new_str = String::from(rip.replace_all(&toprint, "$b$n$e"));
                toprint.clear();
                toprint.push_str(&new_str);
            }
            println!("\t{}", toprint);
            continue 'next_line;
        }
    }
}
