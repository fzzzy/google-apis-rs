Updates a revision with patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.appdata*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive3 --scope <scope> revisions update ...`
# Required Scalar Arguments
* **&lt;file-id&gt;** *(string)*
    - The ID of the file.
* **&lt;revision-id&gt;** *(string)*
    - The ID of the revision.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Revision:
  id: string
  keep-forever: boolean
  kind: string
  last-modifying-user:
    display-name: string
    email-address: string
    kind: string
    me: boolean
    permission-id: string
    photo-link: string
  md5-checksum: string
  mime-type: string
  modified-time: string
  original-filename: string
  publish-auto: boolean
  published: boolean
  published-outside-domain: boolean
  size: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    id=dolore`
    - The ID of the revision.
* `keep-forever=false`
    - Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file.
        This field is only applicable to files with binary content in Drive.
* `kind=ipsum`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#revision&#34;.
* `last-modifying-user    display-name=diam`
    - A plain text displayable name for this user.
* `email-address=sed`
    - The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
* `kind=et`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#user&#34;.
* `me=true`
    - Whether this user is the requesting user.
* `permission-id=justo`
    - The user&#39;s ID as visible in Permission resources.
* `photo-link=takimata`
    - A link to the user&#39;s profile photo, if available.

* `..    md5-checksum=invidunt`
    - The MD5 checksum of the revision&#39;s content. This is only applicable to files with binary content in Drive.
* `mime-type=amet`
    - The MIME type of the revision.
* `modified-time=dolor`
    - The last time the revision was modified (RFC 3339 date-time).
* `original-filename=voluptua.`
    - The original filename used to create this revision. This is only applicable to files with binary content in Drive.
* `publish-auto=false`
    - Whether subsequent revisions will be automatically republished. This is only applicable to Google Docs.
* `published=true`
    - Whether this revision is published. This is only applicable to Google Docs.
* `published-outside-domain=true`
    - Whether this revision is published outside the domain. This is only applicable to Google Docs.
* `size=sit`
    - The size of the revision&#39;s content in bytes. This is only applicable to files with binary content in Drive.


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