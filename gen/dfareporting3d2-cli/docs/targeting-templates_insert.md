Inserts a new targeting template.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> targeting-templates insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
TargetingTemplate:
  account-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  day-part-targeting:
    days-of-week: [string]
    hours-of-day: [integer]
    user-local-time: boolean
  geo-targeting:
    exclude-countries: boolean
  id: string
  key-value-targeting-expression:
    expression: string
  kind: string
  list-targeting-expression:
    expression: string
  name: string
  subaccount-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=vero`
    - Account ID of this targeting template. This field, if left unset, will be auto-generated on insert and is read-only after insert.
* `advertiser-id=sit`
    - Advertiser ID of this targeting template. This is a required field on insert and is read-only after insert.
* `advertiser-id-dimension-value    dimension-name=ut`
    - The name of the dimension.
* `etag=takimata`
    - The eTag of this response for caching purposes.
* `id=accusam`
    - The ID associated with the value if available.
* `kind=kasd`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=consetetur`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=dolor`
    - The value of the dimension.

* `..day-part-targeting    days-of-week=rebum.`
    - Days of the week when the ad will serve.
        
        Acceptable values are:
        - &#34;SUNDAY&#34;
        - &#34;MONDAY&#34;
        - &#34;TUESDAY&#34;
        - &#34;WEDNESDAY&#34;
        - &#34;THURSDAY&#34;
        - &#34;FRIDAY&#34;
        - &#34;SATURDAY&#34;
    - Each invocation of this argument appends the given value to the array.
* `hours-of-day=12`
    - Hours of the day when the ad will serve, where 0 is midnight to 1 AM and 23 is 11 PM to midnight. Can be specified with days of week, in which case the ad would serve during these hours on the specified days. For example if Monday, Wednesday, Friday are the days of week specified and 9-10am, 3-5pm (hours 9, 15, and 16) is specified, the ad would serve Monday, Wednesdays, and Fridays at 9-10am and 3-5pm. Acceptable values are 0 to 23, inclusive.
    - Each invocation of this argument appends the given value to the array.
* `user-local-time=true`
    - Whether or not to use the user&#39;s local time. If false, the America/New York time zone applies.

* `..geo-targeting    exclude-countries=false`
    - Whether or not to exclude the countries in the countries field from targeting. If false, the countries field refers to countries which will be targeted by the ad.

* `..    id=magna`
    - ID of this targeting template. This is a read-only, auto-generated field.
* `key-value-targeting-expression    expression=sed`
    - Keyword expression being targeted by the ad.

* `..    kind=est`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#targetingTemplate&#34;.
* `list-targeting-expression    expression=lorem`
    - Expression describing which lists are being targeted by the ad.

* `..    name=aliquyam`
    - Name of this targeting template. This field is required. It must be less than 256 characters long and unique within an advertiser.
* `subaccount-id=eos`
    - Subaccount ID of this targeting template. This field, if left unset, will be auto-generated on insert and is read-only after insert.


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