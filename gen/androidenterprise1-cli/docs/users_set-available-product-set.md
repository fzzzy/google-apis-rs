Modifies the set of products that a user is entitled to access (referred to as whitelisted products). Only products that are approved or products that were previously approved (products with revoked approval) can be whitelisted.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidenterprise* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidenterprise*.
You can set the scope for this method like this: `androidenterprise1 --scope <scope> users set-available-product-set ...`
# Required Scalar Arguments
* **&lt;enterprise-id&gt;** *(string)*
    - The ID of the enterprise.
* **&lt;user-id&gt;** *(string)*
    - The ID of the user.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ProductSet:
  kind: string
  product-id: [string]
  product-set-behavior: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    kind=ea`
    - Identifies what kind of resource this is. Value: the fixed string &#34;androidenterprise#productSet&#34;.
* `product-id=gubergren`
    - The list of product IDs making up the set of products.
    - Each invocation of this argument appends the given value to the array.
* `product-set-behavior=aliquyam`
    - The interpretation of this product set. &#34;unknown&#34; should never be sent and is ignored if received. &#34;whitelist&#34; means that the user is entitled to access the product set. &#34;includeAll&#34; means that all products are accessible, including products that are approved, products with revoked approval, and products that have never been approved. &#34;allApproved&#34; means that the user is entitled to access all products that are approved for the enterprise. If the value is &#34;allApproved&#34; or &#34;includeAll&#34;, the productId field is ignored. If no value is provided, it is interpreted as &#34;whitelist&#34; for backwards compatibility. Further &#34;allApproved&#34; or &#34;includeAll&#34; does not enable automatic visibility of &#34;alpha&#34; or &#34;beta&#34; tracks for Android app. Use ProductVisibility to enable &#34;alpha&#34; or &#34;beta&#34; tracks per user.


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
