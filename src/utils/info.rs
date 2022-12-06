use egui::{DroppedFile, HoveredFile};

/// Info
pub(crate) trait Info {
    fn info(&self) -> String;
}

impl Info for DroppedFile {
    fn info(&self) -> String {
        let mut buffer = String::new();
        if let Some(path) = &self.path {
            buffer = path.display().to_string();
        } else if !self.name.is_empty() {
            buffer += &self.name;
        } else {
            buffer += "<?>";
        };
        if let Some(bytes) = &self.bytes {
            if !buffer.is_empty() {
                buffer.push(' ');
            }
            buffer += &format!("({} bytes)", bytes.len());
        }
        buffer
    }
}

impl Info for HoveredFile {
    fn info(&self) -> String {
        let mut buffer = String::new();
        if let Some(path) = &self.path {
            buffer = path.display().to_string();
        } else {
            buffer += "<?>";
        }
        if !self.mime.is_empty() {
            if !buffer.is_empty() {
                buffer.push(' ');
            }
            buffer += &format!("[{}]", self.mime)
        }
        buffer
    }
}
