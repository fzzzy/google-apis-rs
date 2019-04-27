Inserts a new event tag.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> event-tags insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
EventTag:
  account-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  campaign-id: string
  campaign-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  enabled-by-default: boolean
  exclude-from-adx-requests: boolean
  id: string
  kind: string
  name: string
  site-filter-type: string
  site-ids: [string]
  ssl-compliant: boolean
  status: string
  subaccount-id: string
  type: string
  url: string
  url-escape-levels: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=justo`
    - Account ID of this event tag. This is a read-only field that can be left blank.
* `advertiser-id=rebum.`
    - Advertiser ID of this event tag. This field or the campaignId field is required on insertion.
* `advertiser-id-dimension-value    dimension-name=ea`
    - The name of the dimension.
* `etag=sit`
    - The eTag of this response for caching purposes.
* `id=lorem`
    - The ID associated with the value if available.
* `kind=sadipscing`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=voluptua.`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=justo`
    - The value of the dimension.

* `..    campaign-id=accusam`
    - Campaign ID of this event tag. This field or the advertiserId field is required on insertion.
* `campaign-id-dimension-value    dimension-name=at`
    - The name of the dimension.
* `etag=vero`
    - The eTag of this response for caching purposes.
* `id=sed`
    - The ID associated with the value if available.
* `kind=dolore`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=diam`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=sit`
    - The value of the dimension.

* `..    enabled-by-default=false`
    - Whether this event tag should be automatically enabled for all of the advertiser&#39;s campaigns and ads.
* `exclude-from-adx-requests=true`
    - Whether to remove this event tag from ads that are trafficked through Display &amp; Video 360 to Ad Exchange. This may be useful if the event tag uses a pixel that is unapproved for Ad Exchange bids on one or more networks, such as the Google Display Network.
* `id=aliquyam`
    - ID of this event tag. This is a read-only, auto-generated field.
* `kind=est`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#eventTag&#34;.
* `name=nonumy`
    - Name of this event tag. This is a required field and must be less than 256 characters long.
* `site-filter-type=amet.`
    - Site filter type for this event tag. If no type is specified then the event tag will be applied to all sites.
* `site-ids=gubergren`
    - Filter list of site IDs associated with this event tag. The siteFilterType determines whether this is a whitelist or blacklist filter.
    - Each invocation of this argument appends the given value to the array.
* `ssl-compliant=true`
    - Whether this tag is SSL-compliant or not. This is a read-only field.
* `status=kasd`
    - Status of this event tag. Must be ENABLED for this event tag to fire. This is a required field.
* `subaccount-id=eos`
    - Subaccount ID of this event tag. This is a read-only field that can be left blank.
* `type=lorem`
    - Event tag type. Can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking. This is a required field.
* `url=kasd`
    - Payload URL for this event tag. The URL on a click-through event tag should have a landing page URL appended to the end of it. This field is required on insertion.
* `url-escape-levels=75`
    - Number of times the landing page URL should be URL-escaped before being appended to the click-through event tag URL. Only applies to click-through event tags as specified by the event tag type.


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
