Adds or updates an entitlement to an app for a user. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidenterprise* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidenterprise*.
You can set the scope for this method like this: `androidenterprise1 --scope <scope> entitlements patch ...`
# Required Scalar Arguments
* **&lt;enterprise-id&gt;** *(string)*
    - The ID of the enterprise.
* **&lt;user-id&gt;** *(string)*
    - The ID of the user.
* **&lt;entitlement-id&gt;** *(string)*
    - The ID of the entitlement (a product ID), e.g. &#34;app:com.google.android.gm&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Entitlement:
  kind: string
  product-id: string
  reason: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    kind=sadipscing`
    - Identifies what kind of resource this is. Value: the fixed string &#34;androidenterprise#entitlement&#34;.
* `product-id=dolor`
    - The ID of the product that the entitlement is for. For example, &#34;app:com.google.android.gm&#34;.
* `reason=eirmod`
    - The reason for the entitlement. For example, &#34;free&#34; for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses.


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

* **-p install=boolean**
    - Set to true to also install the product on all the user&#39;s devices where possible. Failure to install on one or more devices will not prevent this operation from returning successfully, as long as the entitlement was successfully assigned to the user.

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