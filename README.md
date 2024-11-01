# XCResult Analyzer

A tool designed to process and analyze .xcresult bundles generated from Xcode test runs. It generates sqlite3 database and outputs a detailed report summarizing test results such as test summary and failure details.

## Example Usage

```shell
xcra generate report --path ./output-report.xcresult --template markdown -o result.md
```

```shell
xcra generate failure-report --path ./output-report.xcresult --template slack -o result.json
```

```shell
xcra generate database --path ./output-report.xcresult
```

```shell
xcra get summary --path ./output-report.xcresult
```
