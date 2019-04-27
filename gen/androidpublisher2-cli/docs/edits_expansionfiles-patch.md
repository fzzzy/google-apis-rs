Updates the APK&#39;s Expansion File configuration to reference another APK&#39;s Expansion Files. To add a new Expansion File use the Upload method. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `androidpublisher2 --scope <scope> edits expansionfiles-patch ...`
# Required Scalar Arguments
* **&lt;package-name&gt;** *(string)*
    - Unique identifier for the Android app that is being updated; for example, &#34;com.spiffygame&#34;.
* **&lt;edit-id&gt;** *(string)*
    - Unique identifier for this edit.
* **&lt;apk-version-code&gt;** *(integer)*
    - The version code of the APK whose Expansion File configuration is being read or modified.
* **&lt;expansion-file-type&gt;** *(string)*
    - No description provided.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ExpansionFile:
  file-size: string
  references-version: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    file-size=ipsum`
    - If set this field indicates that this APK has an Expansion File uploaded to it: this APK does not reference another APK&#39;s Expansion File. The field&#39;s value is the size of the uploaded Expansion File in bytes.
* `references-version=96`
    - If set this APK&#39;s Expansion File references another APK&#39;s Expansion File. The file_size field will not be set.


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
