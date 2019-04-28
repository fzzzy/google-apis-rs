Updates a revision.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.appdata*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive2 --scope <scope> revisions update ...`
# Required Scalar Arguments
* **&lt;file-id&gt;** *(string)*
    - The ID for the file.
* **&lt;revision-id&gt;** *(string)*
    - The ID for the revision.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Revision:
  download-url: string
  etag: string
  export-links: { string: string }
  file-size: string
  id: string
  kind: string
  last-modifying-user:
    display-name: string
    email-address: string
    is-authenticated-user: boolean
    kind: string
    permission-id: string
    picture:
      url: string
  last-modifying-user-name: string
  md5-checksum: string
  mime-type: string
  modified-date: string
  original-filename: string
  pinned: boolean
  publish-auto: boolean
  published: boolean
  published-link: string
  published-outside-domain: boolean
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    download-url=tempor`
    - Short term download URL for the file. This will only be populated on files with content stored in Drive.
* `etag=sea`
    - The ETag of the revision.
* `export-links=key=vero`
    - Links for exporting Google Docs to specific formats.
    - the value will be associated with the given `key`
* `file-size=vero`
    - The size of the revision in bytes. This will only be populated on files with content stored in Drive.
* `id=dolore`
    - The ID of the revision.
* `kind=aliquyam`
    - This is always drive#revision.
* `last-modifying-user    display-name=sed`
    - A plain text displayable name for this user.
* `email-address=et`
    - The email address of the user.
* `is-authenticated-user=false`
    - Whether this user is the same as the authenticated user for whom the request was made.
* `kind=consetetur`
    - This is always drive#user.
* `permission-id=amet.`
    - The user&#39;s ID as visible in the permissions collection.
* `picture    url=clita`
    - A URL that points to a profile picture of this user.


* `...    last-modifying-user-name=sed`
    - Name of the last user to modify this revision.
* `md5-checksum=nonumy`
    - An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive.
* `mime-type=sanctus`
    - The MIME type of the revision.
* `modified-date=consetetur`
    - Last time this revision was modified (formatted RFC 3339 timestamp).
* `original-filename=sit`
    - The original filename when this revision was created. This will only be populated on files with content stored in Drive.
* `pinned=false`
    - Whether this revision is pinned to prevent automatic purging. This will only be populated and can only be modified on files with content stored in Drive which are not Google Docs. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter.
* `publish-auto=false`
    - Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Google Docs.
* `published=false`
    - Whether this revision is published. This is only populated and can only be modified for Google Docs.
* `published-link=gubergren`
    - A link to the published revision.
* `published-outside-domain=false`
    - Whether this revision is published outside the domain. This is only populated and can only be modified for Google Docs.
* `self-link=sed`
    - A link back to this revision.


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