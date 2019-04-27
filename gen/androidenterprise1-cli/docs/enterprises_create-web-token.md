Returns a unique token to access an embeddable UI. To generate a web UI, pass the generated token into the managed Google Play javascript API. Each token may only be used to start one UI session. See the javascript API documentation for further information.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidenterprise* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidenterprise*.
You can set the scope for this method like this: `androidenterprise1 --scope <scope> enterprises create-web-token ...`
# Required Scalar Argument
* **&lt;enterprise-id&gt;** *(string)*
    - The ID of the enterprise.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AdministratorWebTokenSpec:
  kind: string
  parent: string
  permission: [string]
  play-search:
    approve-apps: boolean
    enabled: boolean
  private-apps:
    enabled: boolean
  store-builder:
    enabled: boolean
  web-apps:
    enabled: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    kind=gubergren`
    - Identifies what kind of resource this is. Value: the fixed string &#34;androidenterprise#administratorWebTokenSpec&#34;.
* `parent=sadipscing`
    - The URI of the parent frame hosting the iframe. To prevent XSS, the iframe may not be hosted at other URIs. This URI must be https.
* `permission=aliquyam`
    - Deprecated. Use PlaySearch.approveApps.
    - Each invocation of this argument appends the given value to the array.
* `play-search    approve-apps=false`
    - Allow access to the iframe in approve mode. Default is false.
* `enabled=false`
    - Whether the Play Search page is displayed. Default is true.

* `..private-apps    enabled=true`
    - Whether the Private Apps page is displayed. Default is true.

* `..store-builder    enabled=true`
    - Whether the Store Builder page is displayed. Default is true.

* `..web-apps    enabled=true`
    - Whether the Web Apps page is displayed. Default is true.



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
