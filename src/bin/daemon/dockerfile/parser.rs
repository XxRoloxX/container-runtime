pub enum DockerfileInstruction {
    // FROM(String),
    RUN(String),
    COPY(String, String),
    // CMD(String),
}

pub fn parse_dockerfile(dockerfile_path: &str) -> Result<Vec<DockerfileInstruction>, String> {
    let dockerfile = std::fs::read_to_string(dockerfile_path)
        .map_err(|e| format!("Couldn't read Dockerfile {}: {}", dockerfile_path, e))?;

    let mut instructions = Vec::new();
    for line in dockerfile.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            // Some("FROM") => {
            //     instructions.push(DockerfileInstruction::FROM(parts.collect()));
            // }
            Some("RUN") => {
                instructions.push(DockerfileInstruction::RUN(
                    parts.collect::<Vec<&str>>().join(" "),
                ));
            }
            Some("COPY") => {
                let source = parts.next().unwrap();
                let destination = parts.next().unwrap();
                instructions.push(DockerfileInstruction::COPY(
                    source.to_string(),
                    destination.to_string(),
                ));
            }
            // Some("CMD") => {
            //     instructions.push(DockerfileInstruction::CMD(parts.collect()));
            // }
            Some(any) => return Err(format!("Instruction {} is invalid", any)),
            None => {}
        }
    }
    Ok(instructions)
}
