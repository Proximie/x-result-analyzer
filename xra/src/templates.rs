#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ResultTemplate {
    Markdown,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum FailureResultTemplate {
    Markdown,
    Slack,
}

impl ResultTemplate {
    pub fn template(&self) -> String {
        r#"## Test Results
| Test Name | Result |
|-----------|--------|
{{#each test_results}}
| {{name}} | {{toEmoji result}} |
{{/each}}
            "#
        .to_owned()
    }
}

impl FailureResultTemplate {
    pub fn template(&self) -> String {
        match self {
            Self::Markdown => r#"## Summary
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
            .to_owned(),
            Self::Slack => r#"
{
	"blocks": [
		{
			"type": "header",
			"text": {
				"type": "plain_text",
				"text": "Failed Tests",
				"emoji": true
			}
		},
        {{#each test_results}}
		{
			"type": "section",
			"text": {
				"type": "plain_text",
				"text": "❌ {{name}}",
				"emoji": true
			}
		},
		{
			"type": "context",
			"elements": [
				{
					"type": "plain_text",
					"text": "Failure count: {{failure_count}}",
					"emoji": true
				}
			]
		},
		{
			"type": "divider"
		}{{#unless @last}},{{/unless}}
        {{/each}}
	]
}
                "#
            .to_owned(),
        }
    }
}
