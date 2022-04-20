pub struct ExecutionInformation {
    pub need_to_render: bool,
    pub new_pc: Option<usize>
}

impl Default for ExecutionInformation {
    fn default() -> Self {
        Self {
            need_to_render: false,
            new_pc: None
        }
    }
}