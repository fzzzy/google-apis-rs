Starts a new asynchronous job. Requires the Can View project role.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/bigquery*
* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/devstorage.full_control*
* *https://www.googleapis.com/auth/devstorage.read_only*
* *https://www.googleapis.com/auth/devstorage.read_write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/bigquery*.
You can set the scope for this method like this: `bigquery2 --scope <scope> jobs insert ...`
# Required Scalar Argument
* **&lt;project-id&gt;** *(string)*
    - Project ID of the project that will be billed for the job
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Job:
  configuration:
    copy:
      create-disposition: string
      destination-encryption-configuration:
        kms-key-name: string
      destination-table:
        dataset-id: string
        project-id: string
        table-id: string
      source-table:
        dataset-id: string
        project-id: string
        table-id: string
      write-disposition: string
    dry-run: boolean
    extract:
      compression: string
      destination-format: string
      destination-uri: string
      destination-uris: [string]
      field-delimiter: string
      print-header: boolean
      source-table:
        dataset-id: string
        project-id: string
        table-id: string
    job-timeout-ms: string
    job-type: string
    labels: { string: string }
    load:
      allow-jagged-rows: boolean
      allow-quoted-newlines: boolean
      autodetect: boolean
      clustering:
        fields: [string]
      create-disposition: string
      destination-encryption-configuration:
        kms-key-name: string
      destination-table:
        dataset-id: string
        project-id: string
        table-id: string
      destination-table-properties:
        description: string
        friendly-name: string
      encoding: string
      field-delimiter: string
      ignore-unknown-values: boolean
      max-bad-records: integer
      null-marker: string
      projection-fields: [string]
      quote: string
      schema-inline: string
      schema-inline-format: string
      schema-update-options: [string]
      skip-leading-rows: integer
      source-format: string
      source-uris: [string]
      time-partitioning:
        expiration-ms: string
        field: string
        require-partition-filter: boolean
        type: string
      use-avro-logical-types: boolean
      write-disposition: string
    query:
      allow-large-results: boolean
      clustering:
        fields: [string]
      create-disposition: string
      default-dataset:
        dataset-id: string
        project-id: string
      destination-encryption-configuration:
        kms-key-name: string
      destination-table:
        dataset-id: string
        project-id: string
        table-id: string
      flatten-results: boolean
      maximum-billing-tier: integer
      maximum-bytes-billed: string
      parameter-mode: string
      preserve-nulls: boolean
      priority: string
      query: string
      schema-update-options: [string]
      time-partitioning:
        expiration-ms: string
        field: string
        require-partition-filter: boolean
        type: string
      use-legacy-sql: boolean
      use-query-cache: boolean
      write-disposition: string
  etag: string
  id: string
  job-reference:
    job-id: string
    location: string
    project-id: string
  kind: string
  self-link: string
  statistics:
    completion-ratio: number
    creation-time: string
    end-time: string
    extract:
      destination-uri-file-counts: [string]
    load:
      bad-records: string
      input-file-bytes: string
      input-files: string
      output-bytes: string
      output-rows: string
    query:
      billing-tier: integer
      cache-hit: boolean
      ddl-operation-performed: string
      ddl-target-table:
        dataset-id: string
        project-id: string
        table-id: string
      estimated-bytes-processed: string
      model-training:
        current-iteration: integer
        expected-total-iterations: string
      model-training-current-iteration: integer
      model-training-expected-total-iteration: string
      num-dml-affected-rows: string
      statement-type: string
      total-bytes-billed: string
      total-bytes-processed: string
      total-partitions-processed: string
      total-slot-ms: string
    quota-deferments: [string]
    start-time: string
    total-bytes-processed: string
  status:
    error-result:
      debug-info: string
      location: string
      message: string
      reason: string
    state: string
  user-email: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .configuration.copy    create-disposition=eirmod`
    - [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a &#39;notFound&#39; error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
* `destination-encryption-configuration    kms-key-name=dolore`
    - [Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key.

* `..destination-table    dataset-id=invidunt`
    - [Required] The ID of the dataset containing this table.
* `project-id=aliquyam`
    - [Required] The ID of the project containing this table.
* `table-id=accusam`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.

* `..source-table    dataset-id=lorem`
    - [Required] The ID of the dataset containing this table.
* `project-id=sea`
    - [Required] The ID of the project containing this table.
* `table-id=et`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.

* `..    write-disposition=duo`
    - [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a &#39;duplicate&#39; error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.

* `..    dry-run=true`
    - [Optional] If set, don&#39;t actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn&#39;t a dry run. Behavior of non-query jobs is undefined.
* `extract    compression=eirmod`
    - [Optional] The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE. The default value is NONE. DEFLATE and SNAPPY are only supported for Avro.
* `destination-format=sanctus`
    - [Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO. The default value is CSV. Tables with nested or repeated fields cannot be exported as CSV.
* `destination-uri=et`
    - [Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written.
* `destination-uris=amet`
    - [Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    - Each invocation of this argument appends the given value to the array.
* `field-delimiter=et`
    - [Optional] Delimiter to use between fields in the exported data. Default is &#39;,&#39;
* `print-header=true`
    - [Optional] Whether to print out a header row in the results. Default is true.
* `source-table    dataset-id=ut`
    - [Required] The ID of the dataset containing this table.
* `project-id=ea`
    - [Required] The ID of the project containing this table.
* `table-id=sed`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.


* `...    job-timeout-ms=dolor`
    - [Optional] Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job.
* `job-type=dolor`
    - [Output-only] The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN.
* `labels=key=dolor`
    - The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    - the value will be associated with the given `key`
* `load    allow-jagged-rows=true`
    - [Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
* `allow-quoted-newlines=false`
    - Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
* `autodetect=false`
    - [Optional] Indicates if we should automatically infer the options and schema for CSV and JSON sources.
* `clustering    fields=voluptua.`
    - [Repeated] One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. When you cluster a table using multiple columns, the order of columns you specify is important. The order of the specified columns determines the sort order of the data.
    - Each invocation of this argument appends the given value to the array.

* `..    create-disposition=lorem`
    - [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a &#39;notFound&#39; error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
* `destination-encryption-configuration    kms-key-name=gubergren`
    - [Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key.

* `..destination-table    dataset-id=justo`
    - [Required] The ID of the dataset containing this table.
* `project-id=sit`
    - [Required] The ID of the project containing this table.
* `table-id=vero`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.

* `..destination-table-properties    description=diam`
    - [Optional] The description for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current description is provided, the job will fail.
* `friendly-name=rebum.`
    - [Optional] The friendly name for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current friendly name is provided, the job will fail.

* `..    encoding=consetetur`
    - [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
* `field-delimiter=sadipscing`
    - [Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence &#34;\t&#34; to specify a tab separator. The default value is a comma (&#39;,&#39;).
* `ignore-unknown-values=false`
    - [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don&#39;t match any column names
* `max-bad-records=6`
    - [Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV and JSON. The default value is 0, which requires that all records are valid.
* `null-marker=invidunt`
    - [Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify &#34;\N&#34;, BigQuery interprets &#34;\N&#34; as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value.
* `projection-fields=consetetur`
    - If sourceFormat is set to &#34;DATASTORE_BACKUP&#34;, indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn&#39;t found in the Cloud Datastore backup, an invalid error is returned in the job result.
    - Each invocation of this argument appends the given value to the array.
* `quote=dolore`
    - [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote (&#39;&#34;&#39;). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
* `schema-inline=duo`
    - [Deprecated] The inline schema. For CSV schemas, specify as &#34;Field1:Type1[,Field2:Type2]*&#34;. For example, &#34;foo:STRING, bar:INTEGER, baz:FLOAT&#34;.
* `schema-inline-format=aliquyam`
    - [Deprecated] The format of the schemaInline property.
* `schema-update-options=lorem`
    - Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    - Each invocation of this argument appends the given value to the array.
* `skip-leading-rows=84`
    - [Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped.
* `source-format=clita`
    - [Optional] The format of the data files. For CSV files, specify &#34;CSV&#34;. For datastore backups, specify &#34;DATASTORE_BACKUP&#34;. For newline-delimited JSON, specify &#34;NEWLINE_DELIMITED_JSON&#34;. For Avro, specify &#34;AVRO&#34;. For parquet, specify &#34;PARQUET&#34;. For orc, specify &#34;ORC&#34;. The default value is CSV.
* `source-uris=consetetur`
    - [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one &#39;*&#39; wildcard character and it must come after the &#39;bucket&#39; name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the &#39;*&#39; wildcard character is not allowed.
    - Each invocation of this argument appends the given value to the array.
* `time-partitioning    expiration-ms=takimata`
    - [Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value.
* `field=nonumy`
    - [Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either &#39;_PARTITIONTIME&#39; as TIMESTAMP type, or &#39;_PARTITIONDATE&#39; as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.
* `require-partition-filter=true`
    - [Beta] [Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified.
* `type=sanctus`
    - [Required] The only type supported is DAY, which will generate one partition per day.

* `..    use-avro-logical-types=false`
    - If sourceFormat is set to &#34;AVRO&#34;, indicates whether to enable interpreting logical types into their corresponding types (ie. TIMESTAMP), instead of only using their raw types (ie. INTEGER). The default value will be true once this feature launches, but can be set now in preparation.
* `write-disposition=at`
    - [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a &#39;duplicate&#39; error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.

* `..query    allow-large-results=false`
    - [Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size.
* `clustering    fields=invidunt`
    - [Repeated] One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. When you cluster a table using multiple columns, the order of columns you specify is important. The order of the specified columns determines the sort order of the data.
    - Each invocation of this argument appends the given value to the array.

* `..    create-disposition=ea`
    - [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a &#39;notFound&#39; error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
* `default-dataset    dataset-id=sadipscing`
    - [Required] A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
* `project-id=rebum.`
    - [Optional] The ID of the project containing this dataset.

* `..destination-encryption-configuration    kms-key-name=dolore`
    - [Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key.

* `..destination-table    dataset-id=nonumy`
    - [Required] The ID of the dataset containing this table.
* `project-id=sed`
    - [Required] The ID of the project containing this table.
* `table-id=aliquyam`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.

* `..    flatten-results=false`
    - [Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened.
* `maximum-billing-tier=61`
    - [Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default.
* `maximum-bytes-billed=consetetur`
    - [Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default.
* `parameter-mode=labore`
    - Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
* `preserve-nulls=true`
    - [Deprecated] This property is deprecated.
* `priority=ea`
    - [Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE.
* `query=gubergren`
    - [Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.
* `schema-update-options=aliquyam`
    - Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    - Each invocation of this argument appends the given value to the array.
* `time-partitioning    expiration-ms=eos`
    - [Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value.
* `field=tempor`
    - [Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either &#39;_PARTITIONTIME&#39; as TIMESTAMP type, or &#39;_PARTITIONDATE&#39; as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.
* `require-partition-filter=false`
    - [Beta] [Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified.
* `type=labore`
    - [Required] The only type supported is DAY, which will generate one partition per day.

* `..    use-legacy-sql=false`
    - Specifies whether to use BigQuery&#39;s legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery&#39;s standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
* `use-query-cache=true`
    - [Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true.
* `write-disposition=dolores`
    - [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a &#39;duplicate&#39; error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.


* `...    etag=sit`
    - [Output-only] A hash of this resource.
* `id=diam`
    - [Output-only] Opaque ID field of the job
* `job-reference    job-id=ut`
    - [Required] The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters.
* `location=justo`
    - The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/dataset-locations#specifying_your_location.
* `project-id=est`
    - [Required] The ID of the project containing this job.

* `..    kind=amet`
    - [Output-only] The type of the resource.
* `self-link=accusam`
    - [Output-only] A URL that can be used to access this resource again.
* `statistics    completion-ratio=0.87399646162`
    - [TrustedTester] [Output-only] Job progress (0.0 -&gt; 1.0) for LOAD and EXTRACT jobs.
* `creation-time=diam`
    - [Output-only] Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs.
* `end-time=justo`
    - [Output-only] End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state.
* `extract    destination-uri-file-counts=est`
    - [Output-only] Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the &#39;destinationUris&#39; field.
    - Each invocation of this argument appends the given value to the array.

* `..load    bad-records=clita`
    - [Output-only] The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data.
* `input-file-bytes=invidunt`
    - [Output-only] Number of bytes of source data in a load job.
* `input-files=ut`
    - [Output-only] Number of source files in a load job.
* `output-bytes=dolores`
    - [Output-only] Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change.
* `output-rows=eos`
    - [Output-only] Number of rows imported in a load job. Note that while an import job is in the running state, this value may change.

* `..query    billing-tier=23`
    - [Output-only] Billing tier for the job.
* `cache-hit=true`
    - [Output-only] Whether the query result was fetched from the query cache.
* `ddl-operation-performed=sed`
    - The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): &#34;CREATE&#34;: The query created the DDL target. &#34;SKIP&#34;: No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. &#34;REPLACE&#34;: The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. &#34;DROP&#34;: The query deleted the DDL target.
* `ddl-target-table    dataset-id=aliquyam`
    - [Required] The ID of the dataset containing this table.
* `project-id=ea`
    - [Required] The ID of the project containing this table.
* `table-id=ea`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.

* `..    estimated-bytes-processed=et`
    - [Output-only] The original estimate of bytes processed for the job.
* `model-training    current-iteration=53`
    - [Output-only, Beta] Index of current ML training iteration. Updated during create model query job to show job progress.
* `expected-total-iterations=diam`
    - [Output-only, Beta] Expected number of iterations for the create model query job specified as num_iterations in the input query. The actual total number of iterations may be less than this number due to early stop.

* `..    model-training-current-iteration=39`
    - [Output-only, Beta] Deprecated; do not use.
* `model-training-expected-total-iteration=invidunt`
    - [Output-only, Beta] Deprecated; do not use.
* `num-dml-affected-rows=rebum.`
    - [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
* `statement-type=lorem`
    - The type of query statement, if valid. Possible values (new values might be added in the future): &#34;SELECT&#34;: SELECT query. &#34;INSERT&#34;: INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language &#34;UPDATE&#34;: UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language &#34;DELETE&#34;: DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language &#34;MERGE&#34;: MERGE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language &#34;CREATE_TABLE&#34;: CREATE [OR REPLACE] TABLE without AS SELECT. &#34;CREATE_TABLE_AS_SELECT&#34;: CREATE [OR REPLACE] TABLE ... AS SELECT ... &#34;DROP_TABLE&#34;: DROP TABLE query. &#34;CREATE_VIEW&#34;: CREATE [OR REPLACE] VIEW ... AS SELECT ... &#34;DROP_VIEW&#34;: DROP VIEW query.
* `total-bytes-billed=clita`
    - [Output-only] Total bytes billed for the job.
* `total-bytes-processed=invidunt`
    - [Output-only] Total bytes processed for the job.
* `total-partitions-processed=eirmod`
    - [Output-only] Total number of partitions processed from all partitioned tables referenced in the job.
* `total-slot-ms=at`
    - [Output-only] Slot-milliseconds for the job.

* `..    quota-deferments=consetetur`
    - [Output-only] Quotas which delayed this job&#39;s start time.
    - Each invocation of this argument appends the given value to the array.
* `start-time=et`
    - [Output-only] Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE.
* `total-bytes-processed=sed`
    - [Output-only] [Deprecated] Use the bytes processed in the query statistics instead.

* `..status.error-result    debug-info=sit`
    - Debugging information. This property is internal to Google and should not be used.
* `location=takimata`
    - Specifies where the error occurred, if present.
* `message=elitr`
    - A human-readable description of the error.
* `reason=nonumy`
    - A short error code that summarizes the error.

* `..    state=rebum.`
    - [Output-only] Running state of the job.

* `..    user-email=lorem`
    - [Output-only] Email address of the user who ran the job.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

# Required Upload Flags

This method supports the upload of data, which *requires* all of the following flags to be set:

* **-u (simple|resumable)**
    - **simple** - Upload media all at once.
    - **resumable** - Upload media in a resumable fashion.
* **-f file**
    - Path to file to upload. It must be seekable.

The following flag *may* be set: 

* **-m mime**
    - the mime type, like 'application/octet-stream', which is the default


# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
