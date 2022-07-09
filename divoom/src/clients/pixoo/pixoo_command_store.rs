#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) enum PixooCommandStoreMode {
    Single,
    Batched,
}

pub(crate) trait PixooCommandStore {
    fn mode(&self) -> PixooCommandStoreMode;
    fn append(&mut self, command: String);
    fn to_payload(self: Box<Self>) -> (usize, String);
}

/// Command store for storing a single command.
pub(crate) struct PixooSingleCommandStore {
    command: String,
}

impl PixooSingleCommandStore {
    pub fn new() -> PixooSingleCommandStore {
        PixooSingleCommandStore {
            command: "".to_string(),
        }
    }
}

impl PixooCommandStore for PixooSingleCommandStore {
    fn mode(&self) -> PixooCommandStoreMode {
        PixooCommandStoreMode::Single
    }

    fn append(&mut self, command: String) {
        self.command = command;
    }

    fn to_payload(self: Box<Self>) -> (usize, String) {
        let command_count = if self.command.is_empty() { 0 } else { 1 };
        (command_count, self.command)
    }
}

/// Command store for storing multiple commands and batch execution
pub(crate) struct PixooBatchedCommandStore {
    commands: String,
    command_count: usize,
}

impl PixooBatchedCommandStore {
    pub fn new() -> PixooBatchedCommandStore {
        PixooBatchedCommandStore {
            commands: "{\"Command\":\"Draw/CommandList\",\"CommandList\": [".to_string(),
            command_count: 0,
        }
    }
}

impl PixooCommandStore for PixooBatchedCommandStore {
    fn mode(&self) -> PixooCommandStoreMode {
        PixooCommandStoreMode::Batched
    }

    fn append(&mut self, command: String) {
        if self.command_count > 0 {
            self.commands.push(',');
        }

        self.commands.push_str(&command);
        self.command_count += 1;
    }

    fn to_payload(self: Box<Self>) -> (usize, String) {
        let mut payload = self.commands;
        payload.push_str("]}");

        (self.command_count, payload)
    }
}
