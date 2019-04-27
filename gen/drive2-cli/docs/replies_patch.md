Updates an existing reply. This method supports patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive2 --scope <scope> replies patch ...`
# Required Scalar Arguments
* **&lt;file-id&gt;** *(string)*
    - The ID of the file.
* **&lt;comment-id&gt;** *(string)*
    - The ID of the comment.
* **&lt;reply-id&gt;** *(string)*
    - The ID of the reply.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CommentReply:
  author:
    display-name: string
    email-address: string
    is-authenticated-user: boolean
    kind: string
    permission-id: string
    picture:
      url: string
  content: string
  created-date: string
  deleted: boolean
  html-content: string
  kind: string
  modified-date: string
  reply-id: string
  verb: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .author    display-name=sed`
    - A plain text displayable name for this user.
* `email-address=ut`
    - The email address of the user.
* `is-authenticated-user=true`
    - Whether this user is the same as the authenticated user for whom the request was made.
* `kind=stet`
    - This is always drive#user.
* `permission-id=sanctus`
    - The user&#39;s ID as visible in the permissions collection.
* `picture    url=no`
    - A URL that points to a profile picture of this user.


* `...    content=rebum.`
    - The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply&#39;s content. This field is required on inserts if no verb is specified (resolve/reopen).
* `created-date=gubergren`
    - The date when this reply was first created.
* `deleted=false`
    - Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed.
* `html-content=accusam`
    - HTML formatted content for this reply.
* `kind=gubergren`
    - This is always drive#commentReply.
* `modified-date=invidunt`
    - The date when this reply was last modified.
* `reply-id=diam`
    - The ID of the reply.
* `verb=at`
    - The action this reply performed to the parent comment. When creating a new reply this is the action to be perform to the parent comment. Possible values are:  
        - &#34;resolve&#34; - To resolve a comment. 
        - &#34;reopen&#34; - To reopen (un-resolve) a comment.


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
