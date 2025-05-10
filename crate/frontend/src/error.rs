#[derive(Debug, Clone)]
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

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

pub type CompileResult<T> = Result<T, Vec<CompilerError>>;

/// Reports a list of compiler errors to stderr
pub fn report_errors(errors: &[CompilerError]) {
    for error in errors.iter() {
        eprintln!("{}", error);
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

    pub fn take_errors(&mut self) -> Vec<CompilerError> {
        std::mem::take(&mut self.errors)
    }
}
