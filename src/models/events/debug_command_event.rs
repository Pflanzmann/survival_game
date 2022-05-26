pub struct DebugCommandEvent {
    pub debug_command: String,

    pub key: String,
    pub values: Vec<String>,
    pub arguments: Vec<String>,
}

impl DebugCommandEvent {
    pub fn send_command(command: String) -> Self {
        let debug_command = command.trim().to_string();
        let mut cmd_split = debug_command.split_whitespace();

        let key = match cmd_split.next() {
            Some(value) => value.to_lowercase().trim().to_string(),
            None => String::new(),
        };

        let mut arguments: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        for input_string in cmd_split {
            if input_string.contains('-') {
                arguments.push(input_string.to_lowercase().trim().to_string())
            } else {
                values.push(input_string.to_lowercase().trim().to_string())
            }
        }

        DebugCommandEvent {
            debug_command,

            key,
            values,
            arguments,
        }
    }
}