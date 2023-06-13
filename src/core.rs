pub struct ShellCore {
    pub pre_status: i32,
}

impl ShellCore {
    pub(crate) fn new() -> ShellCore {
        ShellCore { pre_status: 0 }
    }
}
