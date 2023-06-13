pub struct ShellCore {
    pub pre_status: i32,
}

impl ShellCore {
    pub(crate) fn set_status(&mut self, status: i32) {
        self.pre_status = status;
    }
}

impl ShellCore {
    pub(crate) fn new() -> ShellCore {
        ShellCore { pre_status: 0 }
    }
}
