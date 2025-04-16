use crate::bytecode::{Chunk, OpCode, Value};
use std::collections::HashMap;

pub struct VM {
    ip: usize, // Instruction pointer
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            ip: 0,
            stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), String> {
        self.ip = 0;
        while self.ip < chunk.code.len() {
            if let Err(e) = self.execute_instruction(&chunk) {
                return Err(e);
            }
        }

        // Print all values left on the stack when the program ends for debugging
        if !self.stack.is_empty() {
            println!("\n=== Values on stack at end of execution ===");
            for value in &self.stack {
                println!("{}", value);
            }
        }

        Ok(())
    }

    fn execute_instruction(&mut self, chunk: &Chunk) -> Result<(), String> {
        let instruction = self.read_byte(chunk);
        let op = OpCode::from_u8(instruction)
            .ok_or_else(|| format!("Unknown opcode: {}", instruction))?;

        match op {
            OpCode::Constant => {
                let constant_idx = self.read_byte(chunk) as usize;
                if constant_idx >= chunk.constants.len() {
                    return Err("Invalid constant index".to_string());
                }
                let constant = chunk.constants[constant_idx].clone();
                self.stack.push(constant);
            }
            OpCode::Add => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                    (Value::String(a), Value::String(b)) => {
                        Ok(Value::String(format!("{}{}", a, b)))
                    }
                    _ => Err("Cannot add these types".to_string()),
                })?;
            }
            OpCode::Subtract => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                    _ => Err("Cannot subtract these types".to_string()),
                })?;
            }
            OpCode::Multiply => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                    _ => Err("Cannot multiply these types".to_string()),
                })?;
            }
            OpCode::Divide => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::Integer(a / b))
                    }
                    _ => Err("Cannot divide these types".to_string()),
                })?;
            }
            OpCode::Negate => {
                let value = self.pop()?;
                match value {
                    Value::Integer(i) => self.stack.push(Value::Integer(-i)),
                    _ => return Err("Can only negate numbers".to_string()),
                }
            }
            OpCode::Return => {
                self.ip = chunk.code.len();
            }
            OpCode::Print => {
                let value = self.pop()?;
                println!("{}", value);
            }
            OpCode::GetVariable => {
                let var_index = self.read_byte(chunk) as usize;
                if var_index >= chunk.identifiers.len() {
                    return Err("Invalid variable index".to_string());
                }
                let var_name = &chunk.identifiers[var_index];
                if let Some(value) = self.variables.get(var_name) {
                    self.stack.push(value.clone());
                } else {
                    return Err(format!("Undefined variable '{}'", var_name));
                }
            }
            OpCode::SetVariable => {
                if self.stack.is_empty() {
                    return Err("Stack underflow".to_string());
                }
                let var_index = self.read_byte(chunk) as usize;
                if var_index >= chunk.identifiers.len() {
                    return Err("Invalid variable index".to_string());
                }
                let var_name = chunk.identifiers[var_index].clone();
                let value = self.stack.last().unwrap().clone();
                self.variables.insert(var_name, value);
            }
            OpCode::Pop => {
                self.pop()?;
            }
        }

        Ok(())
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn pop(&mut self) -> Result<Value, String> {
        self.stack
            .pop()
            .ok_or_else(|| "Stack underflow".to_string())
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(Value, Value) -> Result<Value, String>,
    {
        if self.stack.len() < 2 {
            return Err("Stack underflow".to_string());
        }
        let b = self.pop()?;
        let a = self.pop()?;
        let result = op(a, b)?;
        self.stack.push(result);
        Ok(())
    }
}
