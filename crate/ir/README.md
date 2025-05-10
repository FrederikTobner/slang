# Slang IR

Intermediate Representation (IR) of the Slang programming language.

## Purpose & Scope

The IR crate defines the intermediate representation that serves as the bridge between the frontend compiler and the backend virtual machine. It provides:

- A platform-independent bytecode format
- Runtime value representation
- Serialization/deserialization for compiled code

## Structure

The IR consists of three main components:

- **Bytecode (`bytecode.rs`)**: Defines the structure of compiled Slang bytecode, including opcodes, constants, and program instructions
- **Value (`value.rs`)**: Implements the runtime value system, supporting various primitive types (integers, floats, booleans, strings) and structured data
- **Core Library (`lib.rs`)**: Exports the public API and connects the components

## Usage

The IR is used:
- By the frontend to emit compiled bytecode
- By the backend to load and execute Slang programs
- For serializing programs to the `.sip` bytecode format
- For deserializing bytecode files back into executable form
