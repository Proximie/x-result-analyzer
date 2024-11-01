# XCResult Analyzer

A tool to analyze xcresult file after running tests on xcode.

## Usage

It takes path to `.xcresult`, a handlebar template, and output path.

```shell
xcresult_analyzer -p ./output-report.xcresult -t ./html.hbs -o index.html
```

## Write a template

The expected body that will be produced by the tool is an array under the name `test_results`, each element has the following structure:

```
name: String
failure_count: i64
failure_reasons: String
error_locations: String
average_duration: f64
```

### Example template

```hbs
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Failed Tests Report</title>
    <style>
      body { font-family: Arial, sans-serif; } table { border-collapse:
      collapse; width: 100%; } th, td { border: 1px solid #ddd; padding: 8px; }
      th { background-color: #f2f2f2; text-align: left; } tr:nth-child(even) {
      background-color: #f9f9f9; } tr:hover { background-color: #ddd; }
    </style>
  </head>
  <body>
    <h1>Failed Tests Report</h1>
    <table>
      <thead>
        <tr>
          <th>#</th>
          <th>Test Name</th>
          <th>Failure Count</th>
          <th>Failure Reasons</th>
          <th>Error Locations</th>
          <th>Average Duration (in sec)</th>
        </tr>
      </thead>
      <tbody>
        {{#each test_results}}
          <tr>
            <td>{{@index}}</td>
            <td>{{name}}</td>
            <td>{{failure_count}}</td>
            <td>{{#if
                failure_reasons
              }}{{failure_reasons}}{{else}}N/A{{/if}}</td>
            <td>{{#if
                error_locations
              }}{{error_locations}}{{else}}N/A{{/if}}</td>
            <td>{{#if
                average_duration
              }}{{average_duration}}{{else}}N/A{{/if}}</td>
          </tr>
        {{/each}}
      </tbody>
    </table>
  </body>
</html>
```
