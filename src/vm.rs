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

    // Add methods to access and manipulate the VM's state
    #[allow(dead_code)]
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    pub fn get_variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }
    
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.variables.clear();
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), String> {
        self.ip = 0;
        while self.ip < chunk.code.len() {
            self.execute_instruction(chunk)?;
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
                    (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a + b)),
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
                    (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a + b)),
                    (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a + b)),
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
                    (Value::String(a), Value::String(b)) => {
                        Ok(Value::String(format!("{}{}", a, b)))
                    }
                    _ => Err("Cannot add these types".to_string()),
                })?;
            }
            OpCode::Subtract => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a - b)),
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a - b)),
                    (Value::U32(a), Value::U32(b)) => {
                        if a >= b {
                            Ok(Value::U32(a - b))
                        } else {
                            Err("Unsigned underflow".to_string())
                        }
                    }
                    (Value::U64(a), Value::U64(b)) => {
                        if a >= b {
                            Ok(Value::U64(a - b))
                        } else {
                            Err("Unsigned underflow".to_string())
                        }
                    }
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a - b)),
                    _ => Err("Cannot subtract these types".to_string()),
                })?;
            }
            OpCode::Multiply => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => match a.checked_mul(b) {
                        Some(result) => Ok(Value::I32(result)),
                        None => Err("Integer overflow in I32 multiplication".to_string()),
                    },
                    (Value::I64(a), Value::I64(b)) => match a.checked_mul(b) {
                        Some(result) => Ok(Value::I64(result)),
                        None => Err("Integer overflow in I64 multiplication".to_string()),
                    },
                    (Value::U32(a), Value::U32(b)) => match a.checked_mul(b) {
                        Some(result) => Ok(Value::U32(result)),
                        None => Err("Integer overflow in U32 multiplication".to_string()),
                    },
                    (Value::U64(a), Value::U64(b)) => match a.checked_mul(b) {
                        Some(result) => Ok(Value::U64(result)),
                        None => Err("Integer overflow in U64 multiplication".to_string()),
                    },
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a * b)),
                    _ => Err("Cannot multiply these types".to_string()),
                })?;
            }
            OpCode::Divide => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => {
                        if b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::I32(a / b))
                    }
                    (Value::I64(a), Value::I64(b)) => {
                        if b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::I64(a / b))
                    }
                    (Value::U32(a), Value::U32(b)) => {
                        if b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::U32(a / b))
                    }
                    (Value::U64(a), Value::U64(b)) => {
                        if b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::U64(a / b))
                    }
                    (Value::F64(a), Value::F64(b)) => {
                        if b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::F64(a / b))
                    }
                    _ => Err("Cannot divide these types".to_string()),
                })?;
            }
            OpCode::Negate => {
                let value = self.pop()?;
                match value {
                    Value::I32(i) => self.stack.push(Value::I32(-i)),
                    Value::I64(i) => self.stack.push(Value::I64(-i)),
                    Value::U32(_) => return Err("Cannot negate unsigned integer U32".to_string()),
                    Value::U64(_) => return Err("Cannot negate unsigned integer U64".to_string()),
                    Value::F64(f) => self.stack.push(Value::F64(-f)),
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
