use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use nix::unistd::Pid;
use regex::Regex;

pub fn run_strace(pid: Pid) {
    let pid_stringified = format!("{}", pid);
    let p_stdout = format!("/proc/{}/fd/1", pid);
    let args = vec![
        "-p",
        pid_stringified.as_str(),
        "-e",
        "write",
        "-P",
        p_stdout.as_str(),
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
    let re = Regex::new(r#"write\(\d+, "(?<stdout>.*?)", \d+\)"#).unwrap();
    if let Some(matches) = re.captures(output) {
        let stdout = matches["stdout"].to_string();
        let sanitized_stdout = stdout.replace("\\n", "\n");
        print!("{}", sanitized_stdout);
    }
}
