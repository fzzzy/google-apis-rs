Moves entities to a GTM Folder.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager1 --scope <scope> accounts containers-move-folders-update ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - The GTM Account ID.
* **&lt;container-id&gt;** *(string)*
    - The GTM Container ID.
* **&lt;folder-id&gt;** *(string)*
    - The GTM Folder ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Folder:
  account-id: string
  container-id: string
  fingerprint: string
  folder-id: string
  name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=consetetur`
    - GTM Account ID.
* `container-id=ut`
    - GTM Container ID.
* `fingerprint=ea`
    - The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified.
* `folder-id=sed`
    - The Folder ID uniquely identifies the GTM Folder.
* `name=dolor`
    - Folder display name.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p trigger-id=string**
    - The triggers to be moved to the folder.

* **-p tag-id=string**
    - The tags to be moved to the folder.

* **-p variable-id=string**
    - The variables to be moved to the folder.

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
