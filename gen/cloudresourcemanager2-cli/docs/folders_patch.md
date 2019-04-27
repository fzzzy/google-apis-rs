Updates a Folder, changing its display_name.
Changes to the folder display_name will be rejected if they violate either
the display_name formatting rules or naming constraints described in
the CreateFolder documentation.

The Folder&#39;s display name must start and end with a letter or digit,
may contain letters, digits, spaces, hyphens and underscores and can be
no longer than 30 characters. This is captured by the regular expression:
[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?.
The caller must have `resourcemanager.folders.update` permission on the
identified folder.

If the update fails due to the unique name constraint then a
PreconditionFailure explaining this violation will be returned
in the Status.details field.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager2 --scope <scope> folders patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Output only. The resource name of the Folder.
        Its format is `folders/{folder_id}`, for example: &#34;folders/1234&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Folder:
  create-time: string
  display-name: string
  lifecycle-state: string
  name: string
  parent: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=kasd`
    - Output only. Timestamp when the Folder was created. Assigned by the server.
* `display-name=accusam`
    - The folder’s display name.
        A folder’s display name must be unique amongst its siblings, e.g.
        no two folders with the same parent can share the same display name.
        The display name must start and end with a letter or digit, may contain
        letters, digits, spaces, hyphens and underscores and can be no longer
        than 30 characters. This is captured by the regular expression:
        [\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?.
* `lifecycle-state=takimata`
    - Output only.  The lifecycle state of the folder.
        Updates to the lifecycle_state must be performed via
        DeleteFolder and
        UndeleteFolder.
* `name=justo`
    - Output only. The resource name of the Folder.
        Its format is `folders/{folder_id}`, for example: &#34;folders/1234&#34;.
* `parent=amet.`
    - The Folder’s parent&#39;s resource name.
        Updates to the folder&#39;s parent must be performed via
        MoveFolder.


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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p update-mask=string**
    - Fields to be updated.
        Only the `display_name` can be updated.

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
