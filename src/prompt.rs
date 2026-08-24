use crate::task::Task;

pub fn assemble(task: &Task, prompt: Option<&str>, stdin: Option<&str>) -> String {
    let mut parts = Vec::new();
    let instructions = task.def.instructions.trim();
    if !instructions.is_empty() {
        parts.push(instructions.to_string());
    }
    if let Some(p) = prompt.map(str::trim).filter(|s| !s.is_empty()) {
        parts.push(p.to_string());
    }
    if let Some(s) = stdin.map(str::trim_end).filter(|s| !s.is_empty()) {
        parts.push(s.to_string());
    }
    parts.join("\n\n")
}

pub fn repair_prompt(original: &str, previous: &str, errors: &str) -> String {
    format!(
        "{original}\n\nYour previous JSON failed schema validation:\n{errors}\n\nPrevious output:\n{previous}\n\nEmit corrected JSON only. No markdown, no commentary."
    )
}

pub fn json_instruction(schema: Option<&serde_json::Value>) -> String {
    match schema {
        Some(s) => format!(
            "Respond with a single JSON value that validates against this JSON Schema and nothing else.\n{}",
            serde_json::to_string_pretty(s).unwrap_or_else(|_| s.to_string())
        ),
        None => "Respond with a single JSON value and nothing else. No markdown fences.".into(),
    }
}
