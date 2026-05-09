use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::{TempDir, tempdir};

use super::{FailureAssertion, SuccessAssertion};

/// Execution modes for program testing
#[derive(Clone, Copy)]
pub enum ExecutionMode {
    Execute,       // Direct execution: slang execute file.sl
    CompileAndRun, // Compile then run: slang compile + slang run
    CompileOnly,   // Only compile: slang compile
}

/// Main assertion builder that executes immediately upon creation
pub struct ProgramAssertion {
    pub(super) temp_dir: TempDir,
    pub(super) source_path: std::path::PathBuf,
    mode: ExecutionMode,
}

impl ProgramAssertion {
    /// Create a new program assertion with default execution mode
    pub fn new(program: &str) -> Self {
        Self::with_mode(program, ExecutionMode::Execute)
    }

    /// Create with specific execution mode
    pub fn with_mode(program: &str, mode: ExecutionMode) -> Self {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let source_path = temp_dir.path().join("test_program.sl");
        fs::write(&source_path, program).expect("Failed to write source file");

        Self {
            temp_dir,
            source_path,
            mode,
        }
    }

    /// Assert that execution succeeds
    pub fn succeeds(self) -> SuccessAssertion {
        match self.mode {
            ExecutionMode::Execute => self.execute_and_succeed(),
            ExecutionMode::CompileAndRun => self.compile_run_and_succeed(),
            ExecutionMode::CompileOnly => self.compile_and_succeed(),
        }
    }

    /// Assert that execution fails
    pub fn fails(self) -> FailureAssertion {
        match self.mode {
            ExecutionMode::Execute => self.execute_and_fail(),
            ExecutionMode::CompileAndRun => self.compile_run_and_fail(),
            ExecutionMode::CompileOnly => self.compile_and_fail(),
        }
    }

    fn execute_and_succeed(self) -> SuccessAssertion {
        let mut cmd = Command::cargo_bin("slang").unwrap();
        let assert_result = cmd.arg("execute").arg(&self.source_path).assert().success();

        SuccessAssertion {
            assert_result,
            _temp_dir: self.temp_dir,
        }
    }

    fn execute_and_fail(self) -> FailureAssertion {
        let mut cmd = Command::cargo_bin("slang").unwrap();
        let assert_result = cmd.arg("execute").arg(&self.source_path).assert().failure();

        FailureAssertion {
            assert_result,
            _temp_dir: self.temp_dir,
        }
    }

    fn compile_run_and_succeed(self) -> SuccessAssertion {
        let bytecode_path = self.temp_dir.path().join("test_program.sip");

        // First compile
        let mut compile_cmd = Command::cargo_bin("slang").unwrap();
        compile_cmd
            .arg("compile")
            .arg(&self.source_path)
            .arg("-o")
            .arg(&bytecode_path)
            .assert()
            .success();

        // Then run
        let mut run_cmd = Command::cargo_bin("slang").unwrap();
        let assert_result = run_cmd.arg("run").arg(&bytecode_path).assert().success();

        SuccessAssertion {
            assert_result,
            _temp_dir: self.temp_dir,
        }
    }

    fn compile_run_and_fail(self) -> FailureAssertion {
        let bytecode_path = self.temp_dir.path().join("test_program.sip");

        // Try to compile first - if this fails, return the compile failure
        let compile_output = Command::cargo_bin("slang")
            .unwrap()
            .arg("compile")
            .arg(&self.source_path)
            .arg("-o")
            .arg(&bytecode_path)
            .output()
            .expect("Failed to execute compile command");

        if !compile_output.status.success() {
            // Compilation failed, create assertion from compile failure
            let mut cmd = Command::cargo_bin("slang").unwrap();
            let assert_result = cmd
                .arg("compile")
                .arg(&self.source_path)
                .arg("-o")
                .arg(&bytecode_path)
                .assert()
                .failure();

            return FailureAssertion {
                assert_result,
                _temp_dir: self.temp_dir,
            };
        }

        // Compilation succeeded, run and expect failure
        let mut run_cmd = Command::cargo_bin("slang").unwrap();
        let assert_result = run_cmd.arg("run").arg(&bytecode_path).assert().failure();

        FailureAssertion {
            assert_result,
            _temp_dir: self.temp_dir,
        }
    }

    fn compile_and_succeed(self) -> SuccessAssertion {
        let bytecode_path = self.temp_dir.path().join("test_program.sip");

        let mut cmd = Command::cargo_bin("slang").unwrap();
        let assert_result = cmd
            .arg("compile")
            .arg(&self.source_path)
            .arg("-o")
            .arg(&bytecode_path)
            .assert()
            .success();

        SuccessAssertion {
            assert_result,
            _temp_dir: self.temp_dir,
        }
    }

    fn compile_and_fail(self) -> FailureAssertion {
        let bytecode_path = self.temp_dir.path().join("test_program.sip");

        let mut cmd = Command::cargo_bin("slang").unwrap();
        let assert_result = cmd
            .arg("compile")
            .arg(&self.source_path)
            .arg("-o")
            .arg(&bytecode_path)
            .assert()
            .failure();

        FailureAssertion {
            assert_result,
            _temp_dir: self.temp_dir,
        }
    }
}
