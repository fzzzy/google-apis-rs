Updates an existing floodlight activity.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> floodlight-activities update ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
FloodlightActivity:
  account-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  cache-busting-type: string
  counting-method: string
  expected-url: string
  floodlight-activity-group-id: string
  floodlight-activity-group-name: string
  floodlight-activity-group-tag-string: string
  floodlight-activity-group-type: string
  floodlight-configuration-id: string
  floodlight-configuration-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  floodlight-tag-type: string
  hidden: boolean
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
  notes: string
  secure: boolean
  ssl-compliant: boolean
  ssl-required: boolean
  subaccount-id: string
  tag-format: string
  tag-string: string
  user-defined-variable-types: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=et`
    - Account ID of this floodlight activity. This is a read-only field that can be left blank.
* `advertiser-id=lorem`
    - Advertiser ID of this floodlight activity. If this field is left blank, the value will be copied over either from the activity group&#39;s advertiser or the existing activity&#39;s advertiser.
* `advertiser-id-dimension-value    dimension-name=sit`
    - The name of the dimension.
* `etag=diam`
    - The eTag of this response for caching purposes.
* `id=ipsum`
    - The ID associated with the value if available.
* `kind=sadipscing`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=erat`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=rebum.`
    - The value of the dimension.

* `..    cache-busting-type=ipsum`
    - Code type used for cache busting in the generated tag. Applicable only when floodlightActivityGroupType is COUNTER and countingMethod is STANDARD_COUNTING or UNIQUE_COUNTING.
* `counting-method=at`
    - Counting method for conversions for this floodlight activity. This is a required field.
* `expected-url=elitr`
    - URL where this tag will be deployed. If specified, must be less than 256 characters long.
* `floodlight-activity-group-id=no`
    - Floodlight activity group ID of this floodlight activity. This is a required field.
* `floodlight-activity-group-name=kasd`
    - Name of the associated floodlight activity group. This is a read-only field.
* `floodlight-activity-group-tag-string=diam`
    - Tag string of the associated floodlight activity group. This is a read-only field.
* `floodlight-activity-group-type=eos`
    - Type of the associated floodlight activity group. This is a read-only field.
* `floodlight-configuration-id=eos`
    - Floodlight configuration ID of this floodlight activity. If this field is left blank, the value will be copied over either from the activity group&#39;s floodlight configuration or from the existing activity&#39;s floodlight configuration.
* `floodlight-configuration-id-dimension-value    dimension-name=kasd`
    - The name of the dimension.
* `etag=sadipscing`
    - The eTag of this response for caching purposes.
* `id=sit`
    - The ID associated with the value if available.
* `kind=et`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=rebum.`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=rebum.`
    - The value of the dimension.

* `..    floodlight-tag-type=sit`
    - The type of Floodlight tag this activity will generate. This is a required field.
* `hidden=true`
    - Whether this activity is archived.
* `id=et`
    - ID of this floodlight activity. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=amet`
    - The name of the dimension.
* `etag=stet`
    - The eTag of this response for caching purposes.
* `id=at`
    - The ID associated with the value if available.
* `kind=sea`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=duo`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=diam`
    - The value of the dimension.

* `..    kind=sanctus`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#floodlightActivity&#34;.
* `name=diam`
    - Name of this floodlight activity. This is a required field. Must be less than 129 characters long and cannot contain quotes.
* `notes=ut`
    - General notes or implementation instructions for the tag.
* `secure=false`
    - Whether this tag should use SSL.
* `ssl-compliant=false`
    - Whether the floodlight activity is SSL-compliant. This is a read-only field, its value detected by the system from the floodlight tags.
* `ssl-required=true`
    - Whether this floodlight activity must be SSL-compliant.
* `subaccount-id=sanctus`
    - Subaccount ID of this floodlight activity. This is a read-only field that can be left blank.
* `tag-format=consetetur`
    - Tag format type for the floodlight activity. If left blank, the tag format will default to HTML.
* `tag-string=dolores`
    - Value of the cat= parameter in the floodlight tag, which the ad servers use to identify the activity. This is optional: if empty, a new tag string will be generated for you. This string must be 1 to 8 characters long, with valid characters being [a-z][A-Z][0-9][-][ _ ]. This tag string must also be unique among activities of the same activity group. This field is read-only after insertion.
* `user-defined-variable-types=diam`
    - List of the user-defined variables used by this conversion tag. These map to the &#34;u[1-100]=&#34; in the tags. Each of these can have a user defined type.
        Acceptable values are U1 to U100, inclusive.
    - Each invocation of this argument appends the given value to the array.


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
