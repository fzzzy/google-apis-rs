Creates a user&#39;s company relation. Affiliates the user to a company.
# Required Scalar Argument
* **&lt;user-id&gt;** *(string)*
    - The ID of the user. Can be set to &lt;code&gt;me&lt;/code&gt; to mean
        the currently authenticated user.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CompanyRelation:
  address: string
  badge-tier: string
  company-admin: boolean
  company-id: string
  creation-time: string
  internal-company-id: string
  is-pending: boolean
  logo-url: string
  manager-account: string
  name: string
  phone-number: string
  primary-address:
    address: string
    address-line: [string]
    administrative-area: string
    dependent-locality: string
    language-code: string
    lat-lng:
      latitude: number
      longitude: number
    locality: string
    postal-code: string
    region-code: string
    sorting-code: string
  primary-country-code: int64
  primary-language-code: string
  resolved-timestamp: string
  segment: [string]
  state: string
  website: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    address=aliquyam`
    - The primary address for this company.
* `badge-tier=dolores`
    - Whether the company is a Partner.
* `company-admin=false`
    - Indicates if the user is an admin for this company.
* `company-id=diam`
    - The ID of the company. There may be no id if this is a
        pending company.5
* `creation-time=ut`
    - The timestamp of when affiliation was requested.
        @OutputOnly
* `internal-company-id=justo`
    - The internal company ID.
        Only available for a whitelisted set of api clients.
* `is-pending=true`
    - The flag that indicates if the company is pending verification.
* `logo-url=amet`
    - A URL to a profile photo, e.g. a G+ profile photo.
* `manager-account=accusam`
    - The AdWords manager account # associated this company.
* `name=clita`
    - The name (in the company&#39;s primary language) for the company.
* `phone-number=diam`
    - The phone number for the company&#39;s primary address.
* `primary-address    address=justo`
    - The single string version of the address.
* `address-line=est`
    - The following address lines represent the most specific part of any
        address.
    - Each invocation of this argument appends the given value to the array.
* `administrative-area=clita`
    - Top-level administrative subdivision of this country.
* `dependent-locality=invidunt`
    - Dependent locality or sublocality. Used for UK dependent localities, or
        neighborhoods or boroughs in other locations.
* `language-code=ut`
    - Language code of the address. Should be in BCP 47 format.
* `lat-lng    latitude=0.820349993375`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.762594370144`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    locality=voluptua.`
    - Generally refers to the city/town portion of an address.
* `postal-code=duo`
    - Values are frequently alphanumeric.
* `region-code=sed`
    - CLDR (Common Locale Data Repository) region code .
* `sorting-code=aliquyam`
    - Use of this code is very country-specific, but will refer to a secondary
        classification code for sorting mail.

* `..    primary-country-code=-67`
    - The primary country code of the company.
* `primary-language-code=ea`
    - The primary language code of the company.
* `resolved-timestamp=et`
    - The timestamp when the user was approved.
        @OutputOnly
* `segment=dolor`
    - The segment the company is classified as.
    - Each invocation of this argument appends the given value to the array.
* `state=diam`
    - The state of relationship, in terms of approvals.
* `website=kasd`
    - The website URL for this company.


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

* **-p request-metadata-traffic-source-traffic-source-id=string**
    - Identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p request-metadata-locale=string**
    - Locale to use for the current request.

* **-p request-metadata-traffic-source-traffic-sub-id=string**
    - Second level identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p request-metadata-user-overrides-ip-address=string**
    - IP address to use instead of the user&#39;s geo-located IP address.

* **-p request-metadata-partners-session-id=string**
    - Google Partners session ID.

* **-p request-metadata-experiment-ids=string**
    - Experiment IDs the current request belongs to.

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
