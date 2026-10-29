use predicates::prelude::*;
use slang_error::ErrorCode;
use tempfile::TempDir;

/// Assertions available after failed execution
pub struct FailureAssertion {
    pub(super) assert_result: assert_cmd::assert::Assert,
    pub(super) _temp_dir: TempDir,
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

    /// Assert diagnostic location and a readable two-line snippet in one call.
    ///
    /// `snippet` should contain exactly two lines:
    /// 1. The source line as shown in the diagnostic
    /// 2. The caret/underline line aligned under it
    ///
    /// The snippet can be written as an indented multiline raw string for readability.
    ///
    /// Use `|` as a left margin marker. Everything before `|` is stripped, while
    /// everything after it is preserved exactly. This keeps the asserted alignment
    /// visually readable in the test source while still matching the exact spaces
    /// that appear in the diagnostic output.
    ///
    /// Example:
    /// ```text
    /// r#"
    /// |        get_identity()(1, 2);
    /// |        ^^^^^^^^^^^^^^^^^^^^
    /// "#
    /// ```
    pub fn diagnostic_snippet(self, line: usize, snippet: &str) -> Self {
        let expected = normalize_multiline_snippet(snippet);
        let mut expected_lines = expected.lines();
        let expected_source = expected_lines
            .next()
            .expect("diagnostic_snippet requires a source line");
        let expected_caret = expected_lines
            .next()
            .expect("diagnostic_snippet requires a caret line");
        assert!(
            expected_lines.next().is_none(),
            "diagnostic_snippet expects exactly two lines"
        );

        let stderr = String::from_utf8_lossy(self.assert_result.get_output().stderr.as_slice());
        let location_marker = format!("--> main:{line}:");
        let lines: Vec<&str> = stderr.lines().collect();

        let marker_index = lines
            .iter()
            .position(|current| current.contains(&location_marker))
            .unwrap_or_else(|| {
                panic!("Diagnostic marker not found: {location_marker}\n\nStderr:\n{stderr}")
            });

        let source_rendered = lines
            .get(marker_index + 2)
            .and_then(|line| line.split_once('|').map(|(_, right)| right))
            .map(|right| right.strip_prefix(' ').unwrap_or(right))
            .unwrap_or_else(|| {
                panic!(
                    "Diagnostic source line missing after {location_marker}\n\nStderr:\n{stderr}"
                )
            });

        let caret_rendered = lines
            .get(marker_index + 3)
            .and_then(|line| line.split_once('|').map(|(_, right)| right))
            .map(|right| right.strip_prefix(' ').unwrap_or(right))
            .unwrap_or_else(|| {
                panic!("Diagnostic caret line missing after {location_marker}\n\nStderr:\n{stderr}")
            });

        let actual = format!("{source_rendered}\n{caret_rendered}");
        let expected = format!("{expected_source}\n{expected_caret}");

        assert!(
            actual == expected,
            "Diagnostic snippet mismatch at line {line}\n\nExpected:\n{expected}\n\nActual:\n{actual}\n\nFull stderr:\n{stderr}"
        );

        self
    }
}

fn normalize_multiline_snippet(snippet: &str) -> String {
    snippet
        .trim_matches('\n')
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_once('|')
                .map(|(_, right)| right.to_string())
                .unwrap_or_else(|| line.to_string())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
