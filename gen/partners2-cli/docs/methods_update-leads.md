Updates the specified lead.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Lead:
  adwords-customer-id: string
  comments: string
  create-time: string
  email: string
  family-name: string
  given-name: string
  gps-motivations: [string]
  id: string
  language-code: string
  marketing-opt-in: boolean
  min-monthly-budget:
    currency-code: string
    nanos: integer
    units: string
  phone-number: string
  state: string
  type: string
  website-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    adwords-customer-id=sit`
    - The AdWords Customer ID of the lead.
* `comments=vero`
    - Comments lead source gave.
* `create-time=diam`
    - Timestamp of when this lead was created.
* `email=rebum.`
    - Email address of lead source.
* `family-name=consetetur`
    - Last name of lead source.
* `given-name=sadipscing`
    - First name of lead source.
* `gps-motivations=vero`
    - List of reasons for using Google Partner Search and creating a lead.
    - Each invocation of this argument appends the given value to the array.
* `id=sadipscing`
    - ID of the lead.
* `language-code=invidunt`
    - Language code of the lead&#39;s language preference, as defined by
        &lt;a href=&#34;https://tools.ietf.org/html/bcp47&#34;&gt;BCP 47&lt;/a&gt;
        (IETF BCP 47, &#34;Tags for Identifying Languages&#34;).
* `marketing-opt-in=false`
    - Whether or not the lead signed up for marketing emails
* `min-monthly-budget    currency-code=dolore`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=82`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=aliquyam`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* `..    phone-number=lorem`
    - Phone number of lead source.
* `state=et`
    - The lead&#39;s state in relation to the company.
* `type=clita`
    - Type of lead.
* `website-url=consetetur`
    - Website URL of lead source.


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

* **-p request-metadata-locale=string**
    - Locale to use for the current request.

* **-p request-metadata-traffic-source-traffic-source-id=string**
    - Identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p request-metadata-user-overrides-ip-address=string**
    - IP address to use instead of the user&#39;s geo-located IP address.

* **-p request-metadata-partners-session-id=string**
    - Google Partners session ID.

* **-p request-metadata-traffic-source-traffic-sub-id=string**
    - Second level identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p request-metadata-experiment-ids=string**
    - Experiment IDs the current request belongs to.

* **-p update-mask=string**
    - Standard field mask for the set of fields to be updated.
        Required with at least 1 value in FieldMask&#39;s paths.
        Only `state` and `adwords_customer_id` are currently supported.

* **-p request-metadata-user-overrides-user-id=string**
    - Logged-in user ID to impersonate instead of the user&#39;s ID.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).