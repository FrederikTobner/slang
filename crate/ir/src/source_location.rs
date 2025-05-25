/// Represents a location in the source code (position, line, column)
#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    /// The position in the source code (byte offset)
    pub position: usize,
    /// The line number (1-based)
    pub line: usize,
    /// The column number (1-based)
    pub column: usize,
}

impl SourceLocation {
    /// Creates a new SourceLocation
    ///
    /// ### Arguments
    /// * `position` - Position in the source (byte offset)
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// ### Returns
    /// A new SourceLocation
    pub fn new(position: usize, line: usize, column: usize) -> Self {
        Self {
            position,
            line,
            column,
        }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            position: 0,
            line: 1,
            column: 1,
        }
    }
}
