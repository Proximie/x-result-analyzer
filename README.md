#XCResult Analyzer

A tool designed to process and analyze
            .xcresult bundles generated from Xcode test runs
            .It generates sqlite3 database and outputs a detailed report
                summarizing test results such as test summary and failure
                    details
            .

        ##Example Usage

```shell xra generate report-- path./
        output -
    report.xcresult-- template markdown -
    o result.md
```

```shell xra generate failure - report-- path./ output -
    report.xcresult-- template slack -
    o result.json
```

```shell xra generate database-- path./ output -
    report.xcresult
```

```shell xra get summary-- path./ output - report.xcresult
```
