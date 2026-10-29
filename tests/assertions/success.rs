use predicates::prelude::*;
use tempfile::TempDir;

/// Assertions available after successful execution
pub struct SuccessAssertion {
    pub(super) assert_result: assert_cmd::assert::Assert,
    pub(super) _temp_dir: TempDir,
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
}
