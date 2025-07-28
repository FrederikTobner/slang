use super::BenchmarkProgram;

/// Scope-related programs
pub const NESTED_SCOPES: BenchmarkProgram = BenchmarkProgram::new(
    "nested_scopes",
    r#"
let global = 1;
{
    let local = 2;
    {
        let inner = 3;
        let result = global + local + inner;
    }
}
"#,
);

pub const SCOPE_RESOLUTION: BenchmarkProgram = BenchmarkProgram::new(
    "scope_resolution",
    r#"
let global_var = 1;
{
    let local_var = 2;
    {
        let inner_var = 3;
        let result = global_var + local_var + inner_var;
        {
            let deep_var = 4;
            let deep_result = global_var + local_var + inner_var + deep_var;
        }
    }
}
"#,
);

