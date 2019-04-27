Creates a new Cloud Spanner database and starts to prepare it for serving.
The returned long-running operation will
have a name of the format `&lt;database_name&gt;/operations/&lt;operation_id&gt;` and
can be used to track preparation of the database. The
metadata field type is
CreateDatabaseMetadata. The
response field type is
Database, if successful.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/spanner.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `spanner1 --scope <scope> projects instances-databases-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The name of the instance that will serve the new database.
        Values are of the form `projects/&lt;project&gt;/instances/&lt;instance&gt;`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateDatabaseRequest:
  create-statement: string
  extra-statements: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-statement=accusam`
    - Required. A `CREATE DATABASE` statement, which specifies the ID of the
        new database.  The database ID must conform to the regular expression
        `a-z*[a-z0-9]` and be between 2 and 30 characters in length.
        If the database ID is a reserved word or if it contains a hyphen, the
        database ID must be enclosed in backticks (`` ` ``).
* `extra-statements=takimata`
    - An optional list of DDL statements to run inside the newly created
        database. Statements can create tables, indexes, etc. These
        statements execute atomically with the creation of the database:
        if there is an error in any statement, the database is not created.
    - Each invocation of this argument appends the given value to the array.


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
