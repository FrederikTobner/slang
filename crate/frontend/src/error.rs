#[derive(Debug)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl CompilerError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "Error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

pub struct ErrorCollector {
    errors: Vec<CompilerError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn report_errors(&self) {
        for error in self.errors.iter() {
            eprintln!("{}", error.format());
        }
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }
}
