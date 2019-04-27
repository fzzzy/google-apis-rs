Search for folders that match specific filter criteria.
Search provides an eventually consistent view of the folders a user has
access to which meet the specified filter criteria.

This will only return folders on which the caller has the
permission `resourcemanager.folders.get`.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud-platform.read-only*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager2 --scope <scope> folders search ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SearchFoldersRequest:
  page-size: integer
  page-token: string
  query: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    page-size=20`
    - The maximum number of folders to return in the response.
        This field is optional.
* `page-token=labore`
    - A pagination token returned from a previous call to `SearchFolders`
        that indicates from where search should continue.
        This field is optional.
* `query=sea`
    - Search criteria used to select the Folders to return.
        If no search criteria is specified then all accessible folders will be
        returned.
        
        Query expressions can be used to restrict results based upon displayName,
        lifecycleState and parent, where the operators `=`, `NOT`, `AND` and `OR`
        can be used along with the suffix wildcard symbol `*`.
        
        The displayName field in a query expression should use escaped quotes
        for values that include whitespace to prevent unexpected behavior.
        
        Some example queries are:
        
        |Query | Description|
        |----- | -----------|
        |displayName=Test* | Folders whose display name starts with &#34;Test&#34;.|
        |lifecycleState=ACTIVE | Folders whose lifecycleState is ACTIVE.|
        |parent=folders/123 | Folders whose parent is &#34;folders/123&#34;.|
        |parent=folders/123 AND lifecycleState=ACTIVE | Active folders whose parent is &#34;folders/123&#34;.|
        |displayName=\\&#34;Test String\\&#34;|Folders whose display name includes both &#34;Test&#34; and &#34;String&#34;.|


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
