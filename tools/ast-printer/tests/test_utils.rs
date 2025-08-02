use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

/// Create a temporary Slang source file for testing
pub fn create_temp_slang_file(content: &str) -> Result<(TempDir, PathBuf), std::io::Error> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.sl");
    fs::write(&file_path, content)?;
    Ok((temp_dir, file_path))
}

/// Sample Slang code for testing
pub const SAMPLE_SLANG_CODE: &str = r#"
fn main() -> i32 {
    let x: i32 = 42;
    let y: i32 = x + 10;
    return y;
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
"#;

pub const SIMPLE_EXPRESSION: &str = r#"
fn simple() -> i32 {
    return 1 + 2 * 3;
}
"#;

pub const COMPLEX_CODE: &str = r#"
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() -> i32 {
    let x: i32 = 5;
    let y: i32 = factorial(x);
    let z: i32 = fibonacci(x);
    
    if y + z > 100 {
        return 1;
    } else {
        return 0;
    }
}
"#;
