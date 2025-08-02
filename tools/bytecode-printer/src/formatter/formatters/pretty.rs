use slang_backend::bytecode::{Chunk, OpCode};
use colored::Colorize;
use std::collections::{HashMap, HashSet};

/// Pretty formatter with colors and readable output
pub struct PrettyFormatter;

impl super::super::BytecodeFormatter for PrettyFormatter {
    fn format(&self, chunk: &Chunk, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        
        result.push_str(&format!("╭─ {} ─╮\n", name.cyan().bold()));
        
        // First pass: collect jump targets and scope information
        let jump_targets = self.collect_jump_targets(chunk);
        let scope_depth = self.calculate_scope_depth(chunk);
        
        let mut offset = 0;
        while offset < chunk.code.len() {
            let instruction_byte = chunk.code[offset];
            let line = if offset < chunk.lines.len() { chunk.lines[offset] } else { 0 };
            
            // Add jump target markers
            if jump_targets.contains(&offset) {
                let depth = scope_depth.get(&offset).unwrap_or(&0);
                let indent = "│ ".repeat(*depth + 1).dimmed();
                result.push_str(&format!("{}┌─ {}\n", indent, "Jump Target".bright_yellow()));
            }
            
            match OpCode::from_int(instruction_byte) {
                Some(op) => {
                    let depth = scope_depth.get(&offset).unwrap_or(&0);
                    let (formatted, new_offset) = self.format_instruction(chunk, offset, &op, line, *depth, &jump_targets);
                    result.push_str(&formatted);
                    offset = new_offset;
                }
                None => {
                    let depth = scope_depth.get(&offset).unwrap_or(&0);
                    let indent = "│ ".repeat(*depth + 1).dimmed();
                    result.push_str(&format!("{}{:04} {:4} {} ({})\n", 
                        indent,
                        offset.to_string().yellow(), 
                        line,
                        "UNKNOWN".red().bold(),
                        instruction_byte.to_string().red()));
                    offset += 1;
                }
            }
        }

        // Add constants table
        if !chunk.constants.is_empty() {
            result.push('\n');
            result.push_str(&"╭─ Constants ─╮\n".green().bold().to_string());
            for (i, constant) in chunk.constants.iter().enumerate() {
                result.push_str(&format!("│ [{}] {}\n", 
                    i.to_string().yellow(), 
                    format!("{}", constant).bright_white()));
            }
            result.push_str(&"╰─────────────╯\n".green().dimmed().to_string());
        }

        // Add identifiers table
        if !chunk.identifiers.is_empty() {
            result.push('\n');
            result.push_str(&"╭─ Identifiers ─╮\n".green().bold().to_string());
            for (i, identifier) in chunk.identifiers.iter().enumerate() {
                result.push_str(&format!("│ [{}] {}\n", 
                    i.to_string().yellow(), 
                    identifier.bright_white()));
            }
            result.push_str(&"╰───────────────╯\n".green().dimmed().to_string());
        }

        Ok(result)
    }
}

impl PrettyFormatter {
    fn collect_jump_targets(&self, chunk: &Chunk) -> HashSet<usize> {
        let mut targets = HashSet::new();
        let mut offset = 0;
        
        while offset < chunk.code.len() {
            let instruction_byte = chunk.code[offset];
            match OpCode::from_int(instruction_byte) {
                Some(OpCode::Jump) | Some(OpCode::JumpIfFalse) => {
                    if offset + 2 < chunk.code.len() {
                        let jump_offset = ((chunk.code[offset + 1] as usize) << 8) | (chunk.code[offset + 2] as usize);
                        let target = offset + 3 + jump_offset;
                        if target < chunk.code.len() {
                            targets.insert(target);
                        }
                    }
                    offset += 3;
                }
                Some(OpCode::Constant) | Some(OpCode::GetVariable) | Some(OpCode::SetVariable) | 
                Some(OpCode::Call) => {
                    offset += 2;
                }
                Some(OpCode::DefineFunction) => {
                    offset += 3;
                }
                _ => {
                    offset += 1;
                }
            }
        }
        
        targets
    }
    
    fn calculate_scope_depth(&self, chunk: &Chunk) -> HashMap<usize, usize> {
        let mut depth_map = HashMap::new();
        let mut current_depth = 0;
        let mut offset = 0;
        
        while offset < chunk.code.len() {
            let instruction_byte = chunk.code[offset];
            
            match OpCode::from_int(instruction_byte) {
                Some(OpCode::BeginScope) => {
                    depth_map.insert(offset, current_depth);
                    current_depth += 1;
                    offset += 1;
                }
                Some(OpCode::EndScope) => {
                    if current_depth > 0 {
                        current_depth -= 1;
                    }
                    depth_map.insert(offset, current_depth);
                    offset += 1;
                }
                Some(OpCode::Jump) | Some(OpCode::JumpIfFalse) => {
                    depth_map.insert(offset, current_depth);
                    offset += 3;
                }
                Some(OpCode::Constant) | Some(OpCode::GetVariable) | Some(OpCode::SetVariable) | 
                Some(OpCode::Call) => {
                    depth_map.insert(offset, current_depth);
                    offset += 2;
                }
                Some(OpCode::DefineFunction) => {
                    depth_map.insert(offset, current_depth);
                    offset += 3;
                }
                _ => {
                    depth_map.insert(offset, current_depth);
                    offset += 1;
                }
            }
        }
        
        depth_map
    }

    fn format_instruction(&self, chunk: &Chunk, offset: usize, op: &OpCode, line: usize, depth: usize, _jump_targets: &HashSet<usize>) -> (String, usize) {
        let indent = match op {
            OpCode::BeginScope => "├─ ".repeat(depth).dimmed().to_string() + &"╭─ ".bright_green(),
            OpCode::EndScope => "├─ ".repeat(depth).dimmed().to_string() + &"╰─ ".bright_green(),
            _ => "│ ".repeat(depth + 1).dimmed().to_string(),
        };
        
        let offset_str = format!("{:04}", offset).yellow();
        let line_str = if line > 0 { 
            format!("{:4}", line).dimmed().to_string() 
        } else { 
            "    ".dimmed().to_string() 
        };

        match op {
            OpCode::Constant => {
                let constant_index = chunk.code[offset + 1] as usize;
                let value = if constant_index < chunk.constants.len() {
                    format!("{}", chunk.constants[constant_index])
                } else {
                    "<??>".to_string()
                };
                (format!("{}{} {} {} [{}] '{}'\n", 
                    indent,
                    offset_str, 
                    line_str,
                    "CONST".green().bold(),
                    constant_index.to_string().cyan(),
                    value.bright_white()), offset + 2)
            }
            OpCode::GetVariable | OpCode::SetVariable => {
                let var_index = chunk.code[offset + 1] as usize;
                let name = if var_index < chunk.identifiers.len() {
                    &chunk.identifiers[var_index]
                } else {
                    "<?>"
                };
                let action = match op {
                    OpCode::GetVariable => "GET",
                    OpCode::SetVariable => "SET",
                    _ => unreachable!()
                };
                (format!("{}{} {} {} [{}] '{}'\n", 
                    indent,
                    offset_str, 
                    line_str,
                    action.green().bold(),
                    var_index.to_string().cyan(),
                    name.bright_white()), offset + 2)
            }
            OpCode::DefineFunction => {
                let var_index = chunk.code[offset + 1] as usize;
                let fn_constant_index = chunk.code[offset + 2] as usize;
                let name = if var_index < chunk.identifiers.len() {
                    &chunk.identifiers[var_index]
                } else {
                    "<?>"
                };
                let fn_info = if fn_constant_index < chunk.constants.len() {
                    format!("'{}'", chunk.constants[fn_constant_index])
                } else {
                    "<??>".to_string()
                };
                (format!("{}{} {} {} [{}->{}] {} \n", 
                    indent,
                    offset_str, 
                    line_str,
                    "DEF_FN".magenta().bold(),
                    var_index.to_string().cyan(),
                    fn_constant_index.to_string().cyan(),
                    format!("{} = {}", name, fn_info).bright_white()), offset + 3)
            }
            OpCode::Call => {
                let arg_count = chunk.code[offset + 1];
                (format!("{}{} {} {} ({} args)\n", 
                    indent,
                    offset_str, 
                    line_str,
                    "CALL".blue().bold(),
                    arg_count.to_string().cyan()), offset + 2)
            }
            OpCode::JumpIfFalse | OpCode::Jump => {
                let jump_offset = ((chunk.code[offset + 1] as usize) << 8) | (chunk.code[offset + 2] as usize);
                let target = offset + 3 + jump_offset;
                let jump_type = match op {
                    OpCode::Jump => "JUMP",
                    OpCode::JumpIfFalse => "JUMP_IF_FALSE",
                    _ => unreachable!()
                };
                (format!("{}{} {} {} {} → {:04}\n", 
                    indent,
                    offset_str, 
                    line_str,
                    jump_type.red().bold(),
                    format!("+{}", jump_offset).yellow(),
                    target.to_string().bright_cyan()), offset + 3)
            }
            OpCode::BeginScope => {
                (format!("{}{} {} {} {}\n", 
                    indent,
                    offset_str, 
                    line_str,
                    "BEGIN_SCOPE".bright_green().bold(),
                    "┐".bright_green()), offset + 1)
            }
            OpCode::EndScope => {
                (format!("{}{} {} {} {}\n", 
                    indent,
                    offset_str, 
                    line_str,
                    "END_SCOPE".bright_green().bold(),
                    "┘".bright_green()), offset + 1)
            }
            _ => {
                let op_name = format!("{:?}", op).to_uppercase();
                (format!("{}{} {} {}\n", 
                    indent,
                    offset_str, 
                    line_str,
                    op_name.green().bold()), offset + 1)
            }
        }
    }
}
