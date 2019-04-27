Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL dump or CSV file.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `sqladmin1-beta4 --scope <scope> instances export ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID of the project that contains the instance to be exported.
* **&lt;instance&gt;** *(string)*
    - Cloud SQL instance ID. This does not include the project ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
InstancesExportRequest:
  export-context:
    csv-export-options:
      select-query: string
    databases: [string]
    file-type: string
    kind: string
    sql-export-options:
      schema-only: boolean
      tables: [string]
    uri: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .export-context.csv-export-options    select-query=et`
    - The select query used to extract the data.

* `..    databases=amet`
    - Databases to be exported.
        MySQL instances: If fileType is SQL and no database is specified, all databases are exported, except for the mysql system database. If fileType is CSV, you can specify one database, either by using this property or by using the csvExportOptions.selectQuery property, which takes precedence over this property.
        PostgreSQL instances: If fileType is SQL, you must specify one database to be exported. A fileType of CSV is not supported for PostgreSQL instances.
    - Each invocation of this argument appends the given value to the array.
* `file-type=et`
    - The file type for the specified uri.
        SQL: The file contains SQL statements.
        CSV: The file contains CSV data.
        CSV is not supported for PostgreSQL instances.
* `kind=consetetur`
    - This is always sql#exportContext.
* `sql-export-options    schema-only=true`
    - Export only schemas.
* `tables=ea`
    - Tables to export, or that were exported, from the specified database. If you specify tables, specify one and only one database. For PostgreSQL instances, you can specify only one table.
    - Each invocation of this argument appends the given value to the array.

* `..    uri=sed`
    - The path to the file in Google Cloud Storage where the export will be stored. The URI is in the form gs://bucketName/fileName. If the file already exists, the requests succeeds, but the operation fails. If fileType is SQL and the filename ends with .gz, the contents are compressed.



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
