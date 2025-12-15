use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ProcessState {
    pub pid: u32,
    pub open_fds: HashMap<i32, String>, // Map fd to path
    pub memory_maps: Vec<(u64, u64, String)>, // Start, End, Permissions
}

impl ProcessState {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            open_fds: HashMap::new(),
            memory_maps: Vec::new(),
        }
    }

    pub fn track_open(&mut self, fd: i32, path: String) {
        self.open_fds.insert(fd, path);
    }

    pub fn track_close(&mut self, fd: i32) {
        self.open_fds.remove(&fd);
    }
}
