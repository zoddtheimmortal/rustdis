pub enum Command {
    Get,
    Set,
    Invalid,
}

impl Command {
    pub fn get_command(str: &String) -> Command {
        match str.as_bytes() {
            b"set" => return Command::Set,
            b"get" => return Command::Get,
            _ => return Command::Invalid,
        }
    }
}
