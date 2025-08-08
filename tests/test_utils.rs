use assert_cmd::prelude::*;
use predicates::prelude::*;
use slang_error::ErrorCode;
use std::fs;
use std::process::Command;
use tempfile::{TempDir, tempdir};

/// Execution modes for program testing
#[derive(Clone, Copy)]
pub enum ExecutionMode {
    Execute,       // Direct execution: slang execute file.sl
    CompileAndRun, // Compile then run: slang compile + slang run
    CompileOnly,   // Only compile: slang compile
}

/// Main assertion builder that executes immediately upon creation
pub struct ProgramAssertion {
    temp_dir: TempDir,
    source_path: std::path::PathBuf,
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

    /// Create with compile-and-run mode
    pub fn compile_and_run(program: &str) -> Self {
        Self::with_mode(program, ExecutionMode::CompileAndRun)
    }

    /// Create with compile-only mode
    pub fn compile_only(program: &str) -> Self {
        Self::with_mode(program, ExecutionMode::CompileOnly)
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

/// Assertions available after successful execution
pub struct SuccessAssertion {
    assert_result: assert_cmd::assert::Assert,
    _temp_dir: TempDir, // Keep temp dir alive
}

impl SuccessAssertion {
    /// Assert stdout contains the expected text
    pub fn stdout(self, expected: &str) -> Self {
        Self {
            assert_result: self
                .assert_result
                .stdout(predicate::str::contains(expected)),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert stdout exactly matches the expected text
    pub fn stdout_eq(self, expected: &str) -> Self {
        Self {
            assert_result: self.assert_result.stdout(expected.to_string()),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert stdout is empty
    pub fn no_stdout(self) -> Self {
        Self {
            assert_result: self.assert_result.stdout(""),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert stderr contains the expected text
    pub fn stderr(self, expected: &str) -> Self {
        Self {
            assert_result: self
                .assert_result
                .stderr(predicate::str::contains(expected)),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert stderr is empty
    pub fn no_stderr(self) -> Self {
        Self {
            assert_result: self.assert_result.stderr(""),
            _temp_dir: self._temp_dir,
        }
    }
}

/// Assertions available after failed execution
pub struct FailureAssertion {
    assert_result: assert_cmd::assert::Assert,
    _temp_dir: TempDir,
}

impl FailureAssertion {
    /// Assert that stderr contains the error code
    pub fn error_code(self, code: ErrorCode) -> Self {
        Self {
            assert_result: self
                .assert_result
                .stderr(predicate::str::contains(code.to_string())),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert that stderr contains the expected message
    pub fn stderr(self, expected: &str) -> Self {
        Self {
            assert_result: self
                .assert_result
                .stderr(predicate::str::contains(expected)),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert that stderr exactly matches the expected text
    pub fn stderr_eq(self, expected: &str) -> Self {
        Self {
            assert_result: self.assert_result.stderr(expected.to_string()),
            _temp_dir: self._temp_dir,
        }
    }

    /// Assert stdout contains the expected text (for errors that still produce output)
    pub fn stdout(self, expected: &str) -> Self {
        Self {
            assert_result: self
                .assert_result
                .stdout(predicate::str::contains(expected)),
            _temp_dir: self._temp_dir,
        }
    }
}

/// Convenience macro for simple success assertions
#[macro_export]
macro_rules! assert_output {
    ($program:expr, $output:expr) => {
        $crate::test_utils::ProgramAssertion::new($program)
            .succeeds()
            .stdout($output)
    };
}

/// Convenience macro for simple error assertions
#[macro_export]
macro_rules! assert_error {
    ($program:expr, $code:expr, $message:expr) => {
        $crate::test_utils::ProgramAssertion::new($program)
            .fails()
            .error_code($code)
            .stderr($message)
    };
}

/// Convenience macro for type error assertions
#[macro_export]
macro_rules! assert_type_error {
    ($program:expr, $expected_type:expr, $actual_type:expr) => {
        $crate::test_utils::ProgramAssertion::new($program)
            .fails()
            .error_code(slang_error::ErrorCode::TypeMismatch)
            .stderr(&format!(
                "Type mismatch: variable x is {} but expression is {}",
                $expected_type, $actual_type
            ))
    };
}
