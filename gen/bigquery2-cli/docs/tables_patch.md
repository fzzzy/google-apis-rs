Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/bigquery*
* *https://www.googleapis.com/auth/cloud-platform*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/bigquery*.
You can set the scope for this method like this: `bigquery2 --scope <scope> tables patch ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - Project ID of the table to update
* **&lt;dataset-id&gt;** *(string)*
    - Dataset ID of the table to update
* **&lt;table-id&gt;** *(string)*
    - Table ID of the table to update
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Table:
  clustering:
    fields: [string]
  creation-time: string
  description: string
  encryption-configuration:
    kms-key-name: string
  etag: string
  expiration-time: string
  external-data-configuration:
    autodetect: boolean
    bigtable-options:
      ignore-unspecified-column-families: boolean
      read-rowkey-as-string: boolean
    compression: string
    csv-options:
      allow-jagged-rows: boolean
      allow-quoted-newlines: boolean
      encoding: string
      field-delimiter: string
      quote: string
      skip-leading-rows: string
    google-sheets-options:
      range: string
      skip-leading-rows: string
    ignore-unknown-values: boolean
    max-bad-records: integer
    source-format: string
    source-uris: [string]
  friendly-name: string
  id: string
  kind: string
  labels: { string: string }
  last-modified-time: string
  location: string
  model:
    model-options:
      labels: [string]
      loss-type: string
      model-type: string
  num-bytes: string
  num-long-term-bytes: string
  num-rows: string
  self-link: string
  streaming-buffer:
    estimated-bytes: string
    estimated-rows: string
    oldest-entry-time: string
  table-reference:
    dataset-id: string
    project-id: string
    table-id: string
  time-partitioning:
    expiration-ms: string
    field: string
    require-partition-filter: boolean
    type: string
  type: string
  view:
    query: string
    use-legacy-sql: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .clustering    fields=accusam`
    - [Repeated] One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. When you cluster a table using multiple columns, the order of columns you specify is important. The order of the specified columns determines the sort order of the data.
    - Each invocation of this argument appends the given value to the array.

* `..    creation-time=magna`
    - [Output-only] The time when this table was created, in milliseconds since the epoch.
* `description=lorem`
    - [Optional] A user-friendly description of this table.
* `encryption-configuration    kms-key-name=rebum.`
    - [Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key.

* `..    etag=et`
    - [Output-only] A hash of the table metadata. Used to ensure there were no concurrent modifications to the resource when attempting an update. Not guaranteed to change when the table contents or the fields numRows, numBytes, numLongTermBytes or lastModifiedTime change.
* `expiration-time=clita`
    - [Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables.
* `external-data-configuration    autodetect=true`
    - Try to detect schema and format options automatically. Any option specified explicitly will be honored.
* `bigtable-options    ignore-unspecified-column-families=false`
    - [Optional] If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false.
* `read-rowkey-as-string=true`
    - [Optional] If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false.

* `..    compression=consetetur`
    - [Optional] The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats.
* `csv-options    allow-jagged-rows=true`
    - [Optional] Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false.
* `allow-quoted-newlines=false`
    - [Optional] Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
* `encoding=eos`
    - [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
* `field-delimiter=justo`
    - [Optional] The separator for fields in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence &#34;\t&#34; to specify a tab separator. The default value is a comma (&#39;,&#39;).
* `quote=tempor`
    - [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote (&#39;&#34;&#39;). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
* `skip-leading-rows=gubergren`
    - [Optional] The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped.

* `..google-sheets-options    range=dolore`
    - [Beta] [Optional] Range of a sheet to query from. Only used when non-empty. Typical format: !:
* `skip-leading-rows=amet.`
    - [Optional] The number of rows at the top of a sheet that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows that should be skipped. When autodetect is on, behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N &gt; 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema.

* `..    ignore-unknown-values=true`
    - [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don&#39;t match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored.
* `max-bad-records=18`
    - [Optional] The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV, JSON, and Google Sheets. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats.
* `source-format=elitr`
    - [Required] The data format. For CSV files, specify &#34;CSV&#34;. For Google sheets, specify &#34;GOOGLE_SHEETS&#34;. For newline-delimited JSON, specify &#34;NEWLINE_DELIMITED_JSON&#34;. For Avro files, specify &#34;AVRO&#34;. For Google Cloud Datastore backups, specify &#34;DATASTORE_BACKUP&#34;. [Beta] For Google Cloud Bigtable, specify &#34;BIGTABLE&#34;.
* `source-uris=magna`
    - [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one &#39;*&#39; wildcard character and it must come after the &#39;bucket&#39; name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the &#39;*&#39; wildcard character is not allowed.
    - Each invocation of this argument appends the given value to the array.

* `..    friendly-name=ipsum`
    - [Optional] A descriptive name for this table.
* `id=invidunt`
    - [Output-only] An opaque ID uniquely identifying the table.
* `kind=accusam`
    - [Output-only] The type of the resource.
* `labels=key=labore`
    - The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    - the value will be associated with the given `key`
* `last-modified-time=diam`
    - [Output-only] The time when this table was last modified, in milliseconds since the epoch.
* `location=nonumy`
    - [Output-only] The geographic location where the table resides. This value is inherited from the dataset.
* `model.model-options    labels=sed`
    - No description provided.
    - Each invocation of this argument appends the given value to the array.
* `loss-type=diam`
    - No description provided.
* `model-type=magna`
    - No description provided.


* `...    num-bytes=dolor`
    - [Output-only] The size of this table in bytes, excluding any data in the streaming buffer.
* `num-long-term-bytes=lorem`
    - [Output-only] The number of bytes in the table that are considered &#34;long-term storage&#34;.
* `num-rows=dolor`
    - [Output-only] The number of rows of data in this table, excluding any data in the streaming buffer.
* `self-link=vero`
    - [Output-only] A URL that can be used to access this resource again.
* `streaming-buffer    estimated-bytes=nonumy`
    - [Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer.
* `estimated-rows=takimata`
    - [Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer.
* `oldest-entry-time=dolores`
    - [Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available.

* `..table-reference    dataset-id=consetetur`
    - [Required] The ID of the dataset containing this table.
* `project-id=erat`
    - [Required] The ID of the project containing this table.
* `table-id=amet.`
    - [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.

* `..time-partitioning    expiration-ms=dolores`
    - [Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value.
* `field=dolores`
    - [Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either &#39;_PARTITIONTIME&#39; as TIMESTAMP type, or &#39;_PARTITIONDATE&#39; as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.
* `require-partition-filter=true`
    - [Beta] [Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified.
* `type=sed`
    - [Required] The only type supported is DAY, which will generate one partition per day.

* `..    type=et`
    - [Output-only] Describes the table type. The following values are supported: TABLE: A normal BigQuery table. VIEW: A virtual table defined by a SQL query. EXTERNAL: A table that references data stored in an external storage system, such as Google Cloud Storage. The default value is TABLE.
* `view    query=aliquyam`
    - [Required] A query that BigQuery executes when the view is referenced.
* `use-legacy-sql=false`
    - Specifies whether to use BigQuery&#39;s legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery&#39;s standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value.



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
