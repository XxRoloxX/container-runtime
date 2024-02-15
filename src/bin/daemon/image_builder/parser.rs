use std::str::SplitWhitespace;

pub enum DockerfileInstruction {
    FROM(String),
    RUN(String),
    COPY(String, String),
    ENTRYPOINT(String),
}

pub fn parse_dockerfile(dockerfile_path: &str) -> Result<Vec<DockerfileInstruction>, String> {
    let dockerfile = std::fs::read_to_string(dockerfile_path)
        .map_err(|e| format!("Couldn't read Dockerfile {}: {}", dockerfile_path, e))?;

    let mut instructions = Vec::new();
    for line in dockerfile.lines().filter(|l| !is_line_comment(l)) {
        let mut parts = line.split_whitespace();
        instructions.push(map_dockerfile_instruction(parts.next().unwrap(), parts)?);
    }
    Ok(instructions)
}

pub fn is_line_comment(line: &str) -> bool {
    line.starts_with("#")
}

pub fn map_dockerfile_instruction(
    instruction: &str,
    mut values: SplitWhitespace<'_>,
) -> Result<DockerfileInstruction, String> {
    match instruction {
        "FROM" => Ok(DockerfileInstruction::FROM(values.collect())),
        "RUN" => Ok(DockerfileInstruction::RUN(
            values.collect::<Vec<&str>>().join(" "),
        )),
        "ENTRYPOINT" => Ok(DockerfileInstruction::ENTRYPOINT(
            values.collect::<Vec<&str>>().join(" "),
        )),
        "COPY" => {
            let source = values.next().map_or("", |s| s);
            let destination = values.next().map_or("", |s| s);
            Ok(DockerfileInstruction::COPY(
                source.to_string(),
                destination.to_string(),
            ))
        }
        _ => Err(format!("Instruction {} is invalid", instruction)),
    }
}
