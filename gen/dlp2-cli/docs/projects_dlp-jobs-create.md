Creates a new job to inspect storage or calculate risk metrics.
See https://cloud.google.com/dlp/docs/inspecting-storage and
https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.

When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the
system will automatically choose what detectors to run. By default this may
be all types, but may change over time as detectors are updated.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dlp2 --scope <scope> projects dlp-jobs-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent resource name, for example projects/my-project-id.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GooglePrivacyDlpV2CreateDlpJobRequest:
  inspect-job:
    inspect-config:
      content-options: [string]
      exclude-info-types: boolean
      include-quote: boolean
      limits:
        max-findings-per-item: integer
        max-findings-per-request: integer
      min-likelihood: string
    inspect-template-name: string
    storage-config:
      big-query-options:
        rows-limit: string
        rows-limit-percent: integer
        sample-method: string
        table-reference:
          dataset-id: string
          project-id: string
          table-id: string
      cloud-storage-options:
        bytes-limit-per-file: string
        bytes-limit-per-file-percent: integer
        file-set:
          regex-file-set:
            bucket-name: string
            exclude-regex: [string]
            include-regex: [string]
          url: string
        file-types: [string]
        files-limit-percent: integer
        sample-method: string
      datastore-options:
        kind:
          name: string
        partition-id:
          namespace-id: string
          project-id: string
      timespan-config:
        enable-auto-population-of-timespan-config: boolean
        end-time: string
        start-time: string
        timestamp-field:
          name: string
  job-id: string
  risk-job:
    privacy-metric:
      categorical-stats-config:
        field:
          name: string
      delta-presence-estimation-config:
        region-code: string
      k-anonymity-config:
        entity-id:
          field:
            name: string
      k-map-estimation-config:
        region-code: string
      l-diversity-config:
        sensitive-attribute:
          name: string
      numerical-stats-config:
        field:
          name: string
    source-table:
      dataset-id: string
      project-id: string
      table-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .inspect-job.inspect-config    content-options=rebum.`
    - List of options defining data content to scan.
        If empty, text, images, and other content will be included.
    - Each invocation of this argument appends the given value to the array.
* `exclude-info-types=true`
    - When true, excludes type information of the findings.
* `include-quote=true`
    - When true, a contextual quote from the data that triggered a finding is
        included in the response; see Finding.quote.
* `limits    max-findings-per-item=72`
    - Max number of findings that will be returned for each item scanned.
        When set within `InspectDataSourceRequest`,
        the maximum returned is 1000 regardless if this is set higher.
        When set within `InspectContentRequest`, this field is ignored.
* `max-findings-per-request=19`
    - Max number of findings that will be returned per request/job.
        When set within `InspectContentRequest`, the maximum returned is 1000
        regardless if this is set higher.

* `..    min-likelihood=sit`
    - Only returns findings equal or above this threshold. The default is
        POSSIBLE.
        See https://cloud.google.com/dlp/docs/likelihood to learn more.

* `..    inspect-template-name=eirmod`
    - If provided, will be used as the default for all values in InspectConfig.
        `inspect_config` will be merged into the values persisted as part of the
        template.
* `storage-config.big-query-options    rows-limit=consetetur`
    - Max number of rows to scan. If the table has more rows than this value, the
        rest of the rows are omitted. If not set, or if set to 0, all rows will be
        scanned. Only one of rows_limit and rows_limit_percent can be specified.
        Cannot be used in conjunction with TimespanConfig.
* `rows-limit-percent=16`
    - Max percentage of rows to scan. The rest are omitted. The number of rows
        scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and
        100 means no limit. Defaults to 0. Only one of rows_limit and
        rows_limit_percent can be specified. Cannot be used in conjunction with
        TimespanConfig.
* `sample-method=sed`
    - No description provided.
* `table-reference    dataset-id=ea`
    - Dataset ID of the table.
* `project-id=gubergren`
    - The Google Cloud Platform project ID of the project containing the table.
        If omitted, project ID is inferred from the API call.
* `table-id=aliquyam`
    - Name of the table.


* `...cloud-storage-options    bytes-limit-per-file=eos`
    - Max number of bytes to scan from a file. If a scanned file&#39;s size is bigger
        than this value then the rest of the bytes are omitted. Only one
        of bytes_limit_per_file and bytes_limit_per_file_percent can be specified.
* `bytes-limit-per-file-percent=63`
    - Max percentage of bytes to scan from a file. The rest are omitted. The
        number of bytes scanned is rounded down. Must be between 0 and 100,
        inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one
        of bytes_limit_per_file and bytes_limit_per_file_percent can be specified.
* `file-set.regex-file-set    bucket-name=sea`
    - The name of a Cloud Storage bucket. Required.
* `exclude-regex=labore`
    - A list of regular expressions matching file paths to exclude. All files in
        the bucket that match at least one of these regular expressions will be
        excluded from the scan.
        
        Regular expressions use RE2
        [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found
        under the google/re2 repository on GitHub.
    - Each invocation of this argument appends the given value to the array.
* `include-regex=ipsum`
    - A list of regular expressions matching file paths to include. All files in
        the bucket that match at least one of these regular expressions will be
        included in the set of files, except for those that also match an item in
        `exclude_regex`. Leaving this field empty will match all files by default
        (this is equivalent to including `.*` in the list).
        
        Regular expressions use RE2
        [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found
        under the google/re2 repository on GitHub.
    - Each invocation of this argument appends the given value to the array.

* `..    url=aliquyam`
    - The Cloud Storage url of the file(s) to scan, in the format
        `gs://&lt;bucket&gt;/&lt;path&gt;`. Trailing wildcard in the path is allowed. Exactly
        one of `url` or `regex_file_set` must be set.

* `..    file-types=dolores`
    - List of file type groups to include in the scan.
        If empty, all files are scanned and available data format processors
        are applied.
    - Each invocation of this argument appends the given value to the array.
* `files-limit-percent=3`
    - Limits the number of files to scan to this percentage of the input FileSet.
        Number of files scanned is rounded down. Must be between 0 and 100,
        inclusively. Both 0 and 100 means no limit. Defaults to 0.
* `sample-method=diam`
    - No description provided.

* `..datastore-options.kind    name=ut`
    - The name of the kind.

* `..partition-id    namespace-id=justo`
    - If not empty, the ID of the namespace to which the entities belong.
* `project-id=est`
    - The ID of the project to which the entities belong.


* `...timespan-config    enable-auto-population-of-timespan-config=true`
    - When the job is started by a JobTrigger we will automatically figure out
        a valid start_time to avoid scanning files that have not been modified
        since the last time the JobTrigger executed. This will be based on the
        time of the execution of the last run of the JobTrigger.
* `end-time=accusam`
    - Exclude files or rows newer than this value.
        If set to zero, no upper time limit is applied.
* `start-time=clita`
    - Exclude files or rows older than this value.
* `timestamp-field    name=diam`
    - Name describing the field.




* `.....    job-id=justo`
    - The job id can contain uppercase and lowercase letters,
        numbers, and hyphens; that is, it must match the regular
        expression: `[a-zA-Z\\d-]+`. The maximum length is 100
        characters. Can be empty to allow the system to generate one.
* `risk-job.privacy-metric.categorical-stats-config.field    name=est`
    - Name describing the field.


* `...delta-presence-estimation-config    region-code=clita`
    - ISO 3166-1 alpha-2 region code to use in the statistical modeling.
        Required if no column is tagged with a region-specific InfoType (like
        US_ZIP_5) or a region code.

* `..k-anonymity-config.entity-id.field    name=invidunt`
    - Name describing the field.



* `....k-map-estimation-config    region-code=ut`
    - ISO 3166-1 alpha-2 region code to use in the statistical modeling.
        Required if no column is tagged with a region-specific InfoType (like
        US_ZIP_5) or a region code.

* `..l-diversity-config.sensitive-attribute    name=dolores`
    - Name describing the field.


* `...numerical-stats-config.field    name=eos`
    - Name describing the field.



* `....source-table    dataset-id=voluptua.`
    - Dataset ID of the table.
* `project-id=duo`
    - The Google Cloud Platform project ID of the project containing the table.
        If omitted, project ID is inferred from the API call.
* `table-id=sed`
    - Name of the table.




### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).