Creates an Export.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/ediscovery* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/ediscovery*.
You can set the scope for this method like this: `vault1 --scope <scope> matters exports-create ...`
# Required Scalar Argument
* **&lt;matter-id&gt;** *(string)*
    - The matter ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Export:
  create-time: string
  export-options:
    drive-options:
      include-access-info: boolean
    groups-options:
      export-format: string
    hangouts-chat-options:
      export-format: string
    mail-options:
      export-format: string
  id: string
  matter-id: string
  name: string
  query:
    account-info:
      emails: [string]
    corpus: string
    data-scope: string
    drive-options:
      include-team-drives: boolean
      version-date: string
    end-time: string
    hangouts-chat-info:
      room-id: [string]
    hangouts-chat-options:
      include-rooms: boolean
    mail-options:
      exclude-drafts: boolean
    org-unit-info:
      org-unit-id: string
    search-method: string
    start-time: string
    team-drive-info:
      team-drive-ids: [string]
    terms: string
    time-zone: string
  requester:
    display-name: string
    email: string
  stats:
    exported-artifact-count: int64
    size-in-bytes: string
    total-artifact-count: int64
  status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=takimata`
    - Output only. The time when the export was created.
* `export-options.drive-options    include-access-info=false`
    - Set to true to include access level information for users
        with &lt;a href=&#34;https://support.google.com/vault/answer/6099459#metadata&#34;&gt;indirect access&lt;/a&gt;
        to files.

* `..groups-options    export-format=amet.`
    - The export format for groups export.

* `..hangouts-chat-options    export-format=erat`
    - The export format for hangouts chat export.

* `..mail-options    export-format=labore`
    - The export file format.


* `...    id=sea`
    - Output only. The generated export ID.
* `matter-id=nonumy`
    - Output only. The matter ID.
* `name=dolores`
    - The export name.
* `query.account-info    emails=gubergren`
    - A set of accounts to search.
    - Each invocation of this argument appends the given value to the array.

* `..    corpus=sadipscing`
    - The corpus to search.
* `data-scope=aliquyam`
    - The data source to search from.
* `drive-options    include-team-drives=false`
    - Set to true to include Team Drive.
* `version-date=no`
    - Search the versions of the Drive file
        as of the reference date. These timestamps are in GMT and
        rounded down to the given date.

* `..    end-time=justo`
    - The end time range for the search query. These timestamps are in GMT and
        rounded down to the start of the given date.
* `hangouts-chat-info    room-id=justo`
    - A set of rooms to search.
    - Each invocation of this argument appends the given value to the array.

* `..hangouts-chat-options    include-rooms=true`
    - Set to true to include rooms.

* `..mail-options    exclude-drafts=true`
    - Set to true to exclude drafts.

* `..org-unit-info    org-unit-id=diam`
    - Org unit to search, as provided by the
        &lt;a href=&#34;https://developers.google.com/admin-sdk/directory/&#34;&gt;Admin SDK Directory API&lt;/a&gt;.

* `..    search-method=ipsum`
    - The search method to use.
* `start-time=lorem`
    - The start time range for the search query. These timestamps are in GMT and
        rounded down to the start of the given date.
* `team-drive-info    team-drive-ids=et`
    - List of Team Drive ids, as provided by &lt;a
        href=&#34;https://developers.google.com/drive&#34;&gt;Drive API&lt;/a&gt;.
    - Each invocation of this argument appends the given value to the array.

* `..    terms=duo`
    - The corpus-specific
        &lt;a href=&#34;https://support.google.com/vault/answer/2474474&#34;&gt;search operators&lt;/a&gt;
        used to generate search results.
* `time-zone=aliquyam`
    - The time zone name.
        It should be an IANA TZ name, such as &#34;America/Los_Angeles&#34;.
        For more information, see
        &lt;a href=&#34;https://en.wikipedia.org/wiki/List_of_tz_database_time_zones&#34;&gt;Time
        Zone&lt;/a&gt;.

* `..requester    display-name=sea`
    - The displayed name of the user.
* `email=lorem`
    - The email address of the user.

* `..stats    exported-artifact-count=-75`
    - The number of documents already processed by the export.
* `size-in-bytes=erat`
    - The size of export in bytes.
* `total-artifact-count=-95`
    - The number of documents to be exported.

* `..    status=dolor`
    - Output only. The export status.


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
