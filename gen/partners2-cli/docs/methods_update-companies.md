Update company.
Should only be called within the context of an authorized logged in user.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Company:
  additional-websites: [string]
  auto-approval-email-domains: [string]
  badge-authority-in-awn: boolean
  badge-tier: string
  company-types: [string]
  converted-min-monthly-budget:
    currency-code: string
    nanos: integer
    units: string
  id: string
  industries: [string]
  name: string
  original-min-monthly-budget:
    currency-code: string
    nanos: integer
    units: string
  primary-adwords-manager-account-id: string
  primary-language-code: string
  primary-location:
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
  profile-status: string
  public-profile:
    display-image-url: string
    display-name: string
    id: string
    profile-image: string
    url: string
  services: [string]
  website-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    additional-websites=dolor`
    - URL of the company&#39;s additional websites used to verify the dynamic badges.
        These are stored as full URLs as entered by the user, but only the TLD will
        be used for the actual verification.
    - Each invocation of this argument appends the given value to the array.
* `auto-approval-email-domains=eirmod`
    - Email domains that allow users with a matching email address to get
        auto-approved for associating with this company.
    - Each invocation of this argument appends the given value to the array.
* `badge-authority-in-awn=true`
    - Whether the company&#39;s badge authority is in AWN
* `badge-tier=amet`
    - Partner badge tier
* `company-types=no`
    - Company type labels listed on the company&#39;s profile.
    - Each invocation of this argument appends the given value to the array.
* `converted-min-monthly-budget    currency-code=labore`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=62`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=dolore`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* `..    id=invidunt`
    - The ID of the company.
* `industries=aliquyam`
    - Industries the company can help with.
    - Each invocation of this argument appends the given value to the array.
* `name=accusam`
    - The name of the company.
* `original-min-monthly-budget    currency-code=lorem`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=92`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=et`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* `..    primary-adwords-manager-account-id=duo`
    - The Primary AdWords Manager Account id.
* `primary-language-code=et`
    - The primary language code of the company, as defined by
        &lt;a href=&#34;https://tools.ietf.org/html/bcp47&#34;&gt;BCP 47&lt;/a&gt;
        (IETF BCP 47, &#34;Tags for Identifying Languages&#34;).
* `primary-location    address=eirmod`
    - The single string version of the address.
* `address-line=sanctus`
    - The following address lines represent the most specific part of any
        address.
    - Each invocation of this argument appends the given value to the array.
* `administrative-area=et`
    - Top-level administrative subdivision of this country.
* `dependent-locality=amet`
    - Dependent locality or sublocality. Used for UK dependent localities, or
        neighborhoods or boroughs in other locations.
* `language-code=et`
    - Language code of the address. Should be in BCP 47 format.
* `lat-lng    latitude=0.555410189379`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.645756686533`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    locality=ea`
    - Generally refers to the city/town portion of an address.
* `postal-code=sed`
    - Values are frequently alphanumeric.
* `region-code=dolor`
    - CLDR (Common Locale Data Repository) region code .
* `sorting-code=dolor`
    - Use of this code is very country-specific, but will refer to a secondary
        classification code for sorting mail.

* `..    profile-status=dolor`
    - The public viewability status of the company&#39;s profile.
* `public-profile    display-image-url=et`
    - The URL to the main display image of the public profile. Being deprecated.
* `display-name=consetetur`
    - The display name of the public profile.
* `id=amet.`
    - The ID which can be used to retrieve more details about the public profile.
* `profile-image=voluptua.`
    - The URL to the main profile image of the public profile.
* `url=lorem`
    - The URL of the public profile.

* `..    services=gubergren`
    - Services the company can help with.
    - Each invocation of this argument appends the given value to the array.
* `website-url=justo`
    - URL of the company&#39;s website.


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

* **-p request-metadata-user-overrides-ip-address=string**
    - IP address to use instead of the user&#39;s geo-located IP address.

* **-p request-metadata-user-overrides-user-id=string**
    - Logged-in user ID to impersonate instead of the user&#39;s ID.

* **-p request-metadata-partners-session-id=string**
    - Google Partners session ID.

* **-p request-metadata-locale=string**
    - Locale to use for the current request.

* **-p update-mask=string**
    - Standard field mask for the set of fields to be updated.
        Required with at least 1 value in FieldMask&#39;s paths.

* **-p request-metadata-traffic-source-traffic-sub-id=string**
    - Second level identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p request-metadata-experiment-ids=string**
    - Experiment IDs the current request belongs to.

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
