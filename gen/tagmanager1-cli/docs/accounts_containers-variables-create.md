Creates a GTM Variable.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager1 --scope <scope> accounts containers-variables-create ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - The GTM Account ID.
* **&lt;container-id&gt;** *(string)*
    - The GTM Container ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Variable:
  account-id: string
  container-id: string
  disabling-trigger-id: [string]
  enabling-trigger-id: [string]
  fingerprint: string
  name: string
  notes: string
  parent-folder-id: string
  schedule-end-ms: string
  schedule-start-ms: string
  type: string
  variable-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=amet.`
    - GTM Account ID.
* `container-id=dolore`
    - GTM Container ID.
* `disabling-trigger-id=magna`
    - For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set.
    - Each invocation of this argument appends the given value to the array.
* `enabling-trigger-id=elitr`
    - For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set.
    - Each invocation of this argument appends the given value to the array.
* `fingerprint=magna`
    - The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified.
* `name=ipsum`
    - Variable display name.
* `notes=invidunt`
    - User notes on how to apply this variable in the container.
* `parent-folder-id=accusam`
    - Parent folder id.
* `schedule-end-ms=labore`
    - The end timestamp in milliseconds to schedule a variable.
* `schedule-start-ms=diam`
    - The start timestamp in milliseconds to schedule a variable.
* `type=nonumy`
    - GTM Variable Type.
* `variable-id=sed`
    - The Variable ID uniquely identifies the GTM Variable.


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
