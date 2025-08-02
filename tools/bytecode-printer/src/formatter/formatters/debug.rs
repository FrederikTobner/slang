use slang_backend::bytecode::{Chunk, OpCode};
use colored::Colorize;

/// Debug formatter showing raw bytes and detailed information
pub struct DebugFormatter;

impl super::super::BytecodeFormatter for DebugFormatter {
    fn format(&self, chunk: &Chunk, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        output.push_str(&format!("╭─ BYTECODE CHUNK: {} ─╮\n", name.cyan().bold()));
        
        // Chunk statistics
        output.push_str(&format!("│ Code size: {} bytes\n", chunk.code.len().to_string().yellow()));
        output.push_str(&format!("│ Constants: {}\n", chunk.constants.len().to_string().green()));
        output.push_str(&format!("│ Identifiers: {}\n", chunk.identifiers.len().to_string().blue()));
        output.push_str(&format!("│ Lines: {}\n", chunk.lines.len().to_string().purple()));
        output.push_str("╰──────────────────────╯\n\n");

        // Raw bytecode hex dump with improved formatting
        output.push_str(&"╭─ RAW BYTECODE (HEX) ─╮\n".bright_green().bold().to_string());
        for (i, byte) in chunk.code.iter().enumerate() {
            if i % 16 == 0 {
                output.push_str(&format!("│ {:04x}: ", i).dimmed().to_string());
            }
            output.push_str(&format!("{:02x} ", byte).yellow().to_string());
            if i % 16 == 15 || i == chunk.code.len() - 1 {
                // Pad the line if it's not complete
                if i == chunk.code.len() - 1 && i % 16 != 15 {
                    let remaining = 15 - (i % 16);
                    output.push_str(&"   ".repeat(remaining));
                }
                
                // Add ASCII representation
                output.push_str(&" │ ".dimmed().to_string());
                let start = (i / 16) * 16;
                let end = std::cmp::min(start + 16, chunk.code.len());
                for j in start..end {
                    let byte = chunk.code[j];
                    if byte >= 32 && byte <= 126 {
                        output.push_str(&format!("{}", byte as char).bright_white().to_string());
                    } else {
                        output.push_str(&".".dimmed().to_string());
                    }
                }
                output.push('\n');
            }
        }
        output.push_str(&"╰──────────────────────╯\n\n".bright_green().dimmed().to_string());

        // Disassembled instructions with better formatting
        output.push_str(&"╭─ DISASSEMBLED INSTRUCTIONS ─╮\n".bright_blue().bold().to_string());
        let mut offset = 0;
        while offset < chunk.code.len() {
            let instruction_byte = chunk.code[offset];
            let line = if offset < chunk.lines.len() { chunk.lines[offset] } else { 0 };
            
            output.push_str(&format!("│ {:04x}: {:02x} ", 
                offset, 
                instruction_byte).yellow().to_string());
            
            match OpCode::from_int(instruction_byte) {
                Some(op) => {
                    let op_name = format!("{:?}", op).green().bold();
                    output.push_str(&op_name.to_string());
                    offset = self.add_operands_debug(chunk, offset, &mut output);
                }
                None => {
                    output.push_str(&format!("UNKNOWN({})", instruction_byte).red().bold().to_string());
                    offset += 1;
                }
            }
            
            output.push_str(&format!(" {}\n", format!("(line {})", line).dimmed()));
        }
        output.push_str(&"╰─────────────────────────────╯\n".bright_blue().dimmed().to_string());

        // Constants table with improved formatting
        if !chunk.constants.is_empty() {
            output.push('\n');
            output.push_str(&"╭─ CONSTANTS TABLE ─╮\n".green().bold().to_string());
            for (i, constant) in chunk.constants.iter().enumerate() {
                output.push_str(&format!("│ {:3}: {}\n", 
                    i.to_string().yellow(), 
                    format!("{:?}", constant).bright_white()));
            }
            output.push_str(&"╰───────────────────╯\n".green().dimmed().to_string());
        }

        // Identifiers table with improved formatting
        if !chunk.identifiers.is_empty() {
            output.push('\n');
            output.push_str(&"╭─ IDENTIFIERS TABLE ─╮\n".blue().bold().to_string());
            for (i, identifier) in chunk.identifiers.iter().enumerate() {
                output.push_str(&format!("│ {:3}: \"{}\"\n", 
                    i.to_string().yellow(), 
                    identifier.bright_white()));
            }
            output.push_str(&"╰─────────────────────╯\n".blue().dimmed().to_string());
        }

        Ok(output)
    }
}

impl DebugFormatter {
    fn add_operands_debug(&self, chunk: &Chunk, offset: usize, output: &mut String) -> usize {
        let instruction_byte = chunk.code[offset];
        match OpCode::from_int(instruction_byte) {
            Some(OpCode::Constant) => {
                let constant_index = chunk.code[offset + 1];
                output.push_str(&format!(" {}", constant_index));
                offset + 2
            }
            Some(OpCode::GetVariable) | Some(OpCode::SetVariable) => {
                let var_index = chunk.code[offset + 1];
                output.push_str(&format!(" {}", var_index));
                offset + 2
            }
            Some(OpCode::DefineFunction) => {
                let var_index = chunk.code[offset + 1];
                let fn_constant_index = chunk.code[offset + 2];
                output.push_str(&format!(" {} {}", var_index, fn_constant_index));
                offset + 3
            }
            Some(OpCode::Call) => {
                let arg_count = chunk.code[offset + 1];
                output.push_str(&format!(" {}", arg_count));
                offset + 2
            }
            Some(OpCode::JumpIfFalse) | Some(OpCode::Jump) => {
                let jump_offset = ((chunk.code[offset + 1] as u16) << 8) | (chunk.code[offset + 2] as u16);
                output.push_str(&format!(" {}", jump_offset));
                offset + 3
            }
            _ => offset + 1,
        }
    }
}
