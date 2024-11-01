#[derive(Clone, clap::ValueEnum)]
pub enum BuiltInTemplate {
    Markdown,
}

impl BuiltInTemplate {
    pub fn template(&self) -> String {
        r#"## Summary
| Test Name | Failure Count |
|-----------|---------------|
{{#each test_results}}
| ❌ {{name}} | {{failure_count}} |
{{/each}}

## More Details

{{#each test_results}}
### {{name}}

#### Failed {{failure_count}} times

#### Reason

{{failure_reasons}}

#### Location in Code

{{error_locations}}

#### Average Duration

{{average_duration}} seconds

---

{{/each}}
        "#
        .to_owned()
    }
}
