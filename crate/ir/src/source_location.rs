/// Represents a location in the source code (position, line, column, length)
#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    /// The position in the source code (byte offset)
    pub position: usize,
    /// The line number (1-based)
    pub line: usize,
    /// The column number (1-based)
    pub column: usize,
    /// The length of the token/node in characters
    pub length: usize,
}

impl SourceLocation {
    /// Creates a new SourceLocation
    ///
    /// ### Arguments
    /// * `position` - Position in the source (byte offset)
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    /// * `length` - Length of the token/node in characters
    ///
    /// ### Returns
    /// A new SourceLocation
    pub fn new(position: usize, line: usize, column: usize, length: usize) -> Self {
        Self {
            position,
            line,
            column,
            length,
        }
    }

    /// Creates a new SourceLocation with the old 3-parameter signature for backward compatibility
    ///
    /// ### Arguments
    /// * `position` - Position in the source (byte offset)
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// ### Returns
    /// A new SourceLocation with length = 1 (default for single character tokens)
    pub fn new_simple(position: usize, line: usize, column: usize) -> Self {
        Self {
            position,
            line,
            column,
            length: 1,
        }
    }

    /// Get the end position of this location
    /// 
    /// ### Returns
    /// The byte offset position at the end of this location
    pub fn end_position(&self) -> usize {
        self.position + self.length
    }

    /// Get the end column of this location (assuming single line)
    /// 
    /// ### Returns
    /// The column number at the end of this location
    pub fn end_column(&self) -> usize {
        self.column + self.length
    }

    /// Create a span from this location to another location
    /// 
    /// ### Arguments
    /// 
    /// * `other` - The other SourceLocation to create a span to
    /// 
    /// ### Returns
    /// A new SourceLocation representing the span from this location to the other
    pub fn span_to(&self, other: &SourceLocation) -> SourceLocation {
        let start_pos = self.position.min(other.position);
        let end_pos = self.end_position().max(other.end_position());

        SourceLocation {
            position: start_pos,
            line: self.line.min(other.line),
            column: if self.line == other.line {
                self.column.min(other.column)
            } else {
                self.column
            },
            length: end_pos - start_pos,
        }
    }
}

impl Default for SourceLocation {
    /// Creates a default SourceLocation at the start of the source code
    fn default() -> Self {
        Self {
            position: 0,
            line: 1,
            column: 1,
            length: 1,
        }
    }
}
