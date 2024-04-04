use std::str;

pub struct DiffPrinter {
    buf: String,
    changes: bool,
    was_real_print: bool,
}

impl DiffPrinter {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            changes: false,
            was_real_print: false,
        }
    }

    pub fn println(&mut self, diff: &str) {
        if is_new_file(diff) {
            self.flush()
        }
        if has_changes(diff) {
            self.changes = true
        }
        self.buf += diff;
        self.buf += "\n"
    }

    pub fn flush(&mut self) {
        if self.changes {
            if self.was_real_print {
                println!();
            }
            println!("{}", self.buf.trim_end());
            self.was_real_print = true
        }
        self.changes = false;
        self.buf.clear()
    }
}

fn has_changes(diff: &str) -> bool {
    if is_new_file(diff) {
        // мы не можем знать об изменениях в api по заголовку файла
        return false;
    }
    diff.starts_with("+") || diff.starts_with("-")
}

fn is_new_file(diff: &str) -> bool {
    diff.starts_with("---") || diff.starts_with("+++")
}
