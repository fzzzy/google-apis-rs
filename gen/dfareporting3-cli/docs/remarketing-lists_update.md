Updates an existing remarketing list.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> remarketing-lists update ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RemarketingList:
  account-id: string
  active: boolean
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  description: string
  id: string
  kind: string
  life-span: string
  list-population-rule:
    floodlight-activity-id: string
    floodlight-activity-name: string
  list-size: string
  list-source: string
  name: string
  subaccount-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=voluptua.`
    - Account ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests.
* `active=true`
    - Whether this remarketing list is active.
* `advertiser-id=clita`
    - Dimension value for the advertiser ID that owns this remarketing list. This is a required field.
* `advertiser-id-dimension-value    dimension-name=et`
    - The name of the dimension.
* `etag=tempor`
    - The eTag of this response for caching purposes.
* `id=ea`
    - The ID associated with the value if available.
* `kind=aliquyam`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=gubergren`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=amet`
    - The value of the dimension.

* `..    description=voluptua.`
    - Remarketing list description.
* `id=consetetur`
    - Remarketing list ID. This is a read-only, auto-generated field.
* `kind=sanctus`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#remarketingList&#34;.
* `life-span=at`
    - Number of days that a user should remain in the remarketing list without an impression. Acceptable values are 1 to 540, inclusive.
* `list-population-rule    floodlight-activity-id=labore`
    - Floodlight activity ID associated with this rule. This field can be left blank.
* `floodlight-activity-name=et`
    - Name of floodlight activity associated with this rule. This is a read-only, auto-generated field.

* `..    list-size=rebum.`
    - Number of users currently in the list. This is a read-only field.
* `list-source=amet`
    - Product from which this remarketing list was originated.
* `name=sed`
    - Name of the remarketing list. This is a required field. Must be no greater than 128 characters long.
* `subaccount-id=vero`
    - Subaccount ID of this remarketing list. This is a read-only, auto-generated field that is only returned in GET requests.


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
