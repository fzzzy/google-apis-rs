Creates a new comment on the given file.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive2 --scope <scope> comments insert ...`
# Required Scalar Argument
* **&lt;file-id&gt;** *(string)*
    - The ID of the file.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Comment:
  anchor: string
  author:
    display-name: string
    email-address: string
    is-authenticated-user: boolean
    kind: string
    permission-id: string
    picture:
      url: string
  comment-id: string
  content: string
  context:
    type: string
    value: string
  created-date: string
  deleted: boolean
  file-id: string
  file-title: string
  html-content: string
  kind: string
  modified-date: string
  self-link: string
  status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    anchor=et`
    - A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties.
* `author    display-name=diam`
    - A plain text displayable name for this user.
* `email-address=ipsum`
    - The email address of the user.
* `is-authenticated-user=true`
    - Whether this user is the same as the authenticated user for whom the request was made.
* `kind=et`
    - This is always drive#user.
* `permission-id=duo`
    - The user&#39;s ID as visible in the permissions collection.
* `picture    url=aliquyam`
    - A URL that points to a profile picture of this user.


* `...    comment-id=sea`
    - The ID of the comment.
* `content=lorem`
    - The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment&#39;s content.
* `context    type=eos`
    - The MIME type of the context snippet.
* `value=erat`
    - Data representation of the segment of the file being commented on. In the case of a text file for example, this would be the actual text that the comment is about.

* `..    created-date=sadipscing`
    - The date when this comment was first created.
* `deleted=true`
    - Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed.
* `file-id=eirmod`
    - The file which this comment is addressing.
* `file-title=elitr`
    - The title of the file which this comment is addressing.
* `html-content=amet`
    - HTML formatted content for this comment.
* `kind=no`
    - This is always drive#comment.
* `modified-date=labore`
    - The date when this comment or any of its replies were last modified.
* `self-link=eirmod`
    - A link back to this comment.
* `status=dolore`
    - The status of this comment. Status can be changed by posting a reply to a comment with the desired status.  
        - &#34;open&#34; - The comment is still open. 
        - &#34;resolved&#34; - The comment has been resolved by one of its replies.


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
