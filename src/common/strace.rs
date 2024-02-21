use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use log::info;
use nix::unistd::Pid;
use regex::Regex;

pub fn run_strace(pid: Pid) {
    let pid_stringified = format!("{}", pid);
    let args = vec![
        "-ff", // follow forks
        "-p",
        pid_stringified.as_str(),
        "-e",
        "write",
        "-s",
        "1000",
    ];

    let mut thread = Command::new("strace")
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = thread.stderr.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.unwrap();
        parse_strace_output(&line);
    }
}

pub fn parse_strace_output(output: &str) {
    let re = Regex::new(r#"write\(\d+, "(?<stdout>.*)".*, \d+\)"#).unwrap();
    if let Some(matches) = re.captures(output) {
        let stdout = matches["stdout"].to_string();
        let sanitized_stdout = stdout.replace("\\n", "\n");
        info!("{}", sanitized_stdout);
    }
}
