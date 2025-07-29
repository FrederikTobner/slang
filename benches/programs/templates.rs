// Placeholder program builder - simplified for current benchmarks

use std::fmt;

#[derive(Debug, Clone)]
pub struct GeneratedProgram {
    pub source: String,
}

impl fmt::Display for GeneratedProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

pub struct ProgramTemplates;

impl ProgramTemplates {
    pub fn function_heavy(function_count: usize) -> GeneratedProgram {
        let mut source = String::new();

        // Add some basic functions
        source.push_str("fn add(a: i32, b: i32) -> i32 { return a + b; }\n");
        source.push_str("fn multiply(a: i32, b: i32) -> i32 { return a * b; }\n");

        // Add additional simple functions
        for i in 2..function_count {
            source.push_str(&format!(
                "fn func_{i}(x: i32) -> i32 {{ return x + {i}; }}\n"
            ));
        }

        // Add function calls
        source.push_str("let x: i32 = 5;\n");
        source.push_str("let y: i32 = 10;\n");
        source.push_str("add(x, y);\n");
        source.push_str("multiply(x, y);\n");
        source.push_str("print_value(x);\n");

        GeneratedProgram { source }
    }

    pub fn variable_heavy(var_count: usize) -> GeneratedProgram {
        let mut source = String::new();

        for i in 0..var_count {
            source.push_str(&format!("let var_{i}: i32 = {i};\n"));
        }

        // Add a simple computation
        if var_count >= 2 {
            source.push_str("let sum = var_0 + var_1;\n");
            source.push_str("print_value(sum);\n");
        } else if var_count >= 1 {
            source.push_str("print_value(var_0);\n");
        }

        GeneratedProgram { source }
    }

    pub fn deeply_nested(depth: usize) -> GeneratedProgram {
        let mut source = String::new();

        // Add nested blocks
        for i in 0..depth {
            source.push_str("{\n");
            source.push_str(&format!("    let x_{}: i32 = {};\n", i, i * 10));
        }

        // Add a computation using the nested variables
        source.push_str("    let result = x_0");
        for i in 1..depth {
            source.push_str(&format!(" + x_{i}"));
        }
        source.push_str(";\n");
        source.push_str("    print_value(result);\n");

        // Close all blocks
        for _ in 0..depth {
            source.push_str("}\n");
        }

        GeneratedProgram { source }
    }
}
