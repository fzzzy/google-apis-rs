Inserts a new floodlight activity group.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting2d8 --scope <scope> floodlight-activity-groups insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
FloodlightActivityGroup:
  account-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  floodlight-configuration-id: string
  floodlight-configuration-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  kind: string
  name: string
  subaccount-id: string
  tag-string: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=no`
    - Account ID of this floodlight activity group. This is a read-only field that can be left blank.
* `advertiser-id=kasd`
    - Advertiser ID of this floodlight activity group. If this field is left blank, the value will be copied over either from the floodlight configuration&#39;s advertiser or from the existing activity group&#39;s advertiser.
* `advertiser-id-dimension-value    dimension-name=diam`
    - The name of the dimension.
* `etag=eos`
    - The eTag of this response for caching purposes.
* `id=eos`
    - The ID associated with the value if available.
* `kind=kasd`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sadipscing`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=sit`
    - The value of the dimension.

* `..    floodlight-configuration-id=et`
    - Floodlight configuration ID of this floodlight activity group. This is a required field.
* `floodlight-configuration-id-dimension-value    dimension-name=rebum.`
    - The name of the dimension.
* `etag=rebum.`
    - The eTag of this response for caching purposes.
* `id=sit`
    - The ID associated with the value if available.
* `kind=labore`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=et`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=amet`
    - The value of the dimension.

* `..    id=stet`
    - ID of this floodlight activity group. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=at`
    - The name of the dimension.
* `etag=sea`
    - The eTag of this response for caching purposes.
* `id=duo`
    - The ID associated with the value if available.
* `kind=diam`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sanctus`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=diam`
    - The value of the dimension.

* `..    kind=ut`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#floodlightActivityGroup&#34;.
* `name=elitr`
    - Name of this floodlight activity group. This is a required field. Must be less than 65 characters long and cannot contain quotes.
* `subaccount-id=eirmod`
    - Subaccount ID of this floodlight activity group. This is a read-only field that can be left blank.
* `tag-string=clita`
    - Value of the type= parameter in the floodlight tag, which the ad servers use to identify the activity group that the activity belongs to. This is optional: if empty, a new tag string will be generated for you. This string must be 1 to 8 characters long, with valid characters being [a-z][A-Z][0-9][-][ _ ]. This tag string must also be unique among activity groups of the same floodlight configuration. This field is read-only after insertion.
* `type=sanctus`
    - Type of the floodlight activity group. This is a required field that is read-only after insertion.


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
