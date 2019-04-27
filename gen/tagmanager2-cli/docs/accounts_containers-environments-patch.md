Updates a GTM Environment. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager2 --scope <scope> accounts containers-environments-patch ...`
# Required Scalar Argument
* **&lt;path&gt;** *(string)*
    - GTM Environment&#39;s API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Environment:
  account-id: string
  authorization-code: string
  authorization-timestamp:
    nanos: integer
    seconds: string
  container-id: string
  container-version-id: string
  description: string
  enable-debug: boolean
  environment-id: string
  fingerprint: string
  name: string
  path: string
  tag-manager-url: string
  type: string
  url: string
  workspace-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=ipsum`
    - GTM Account ID.
* `authorization-code=lorem`
    - The environment authorization code.
* `authorization-timestamp    nanos=80`
    - Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive.
* `seconds=duo`
    - Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive.

* `..    container-id=aliquyam`
    - GTM Container ID.
* `container-version-id=sea`
    - Represents a link to a container version.
* `description=lorem`
    - The environment description. Can be set or changed only on USER type environments.
* `enable-debug=false`
    - Whether or not to enable debug by default for the environment.
* `environment-id=erat`
    - GTM Environment ID uniquely identifies the GTM Environment.
* `fingerprint=sadipscing`
    - The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified.
* `name=dolor`
    - The environment display name. Can be set or changed only on USER type environments.
* `path=eirmod`
    - GTM Environment&#39;s API relative path.
* `tag-manager-url=elitr`
    - Auto generated link to the tag manager UI
* `type=amet`
    - The type of this environment.
* `url=no`
    - Default preview page url for the environment.
* `workspace-id=labore`
    - Represents a link to a quick preview of a workspace.


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

* **-p fingerprint=string**
    - When provided, this fingerprint must match the fingerprint of the environment in storage.

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
