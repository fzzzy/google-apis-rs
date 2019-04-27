Updates a user&#39;s profile. A user can only update their own profile and
should only be called within the context of a logged in user.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UserProfile:
  address:
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
  adwords-manager-account: string
  channels: [string]
  email-address: string
  email-opt-ins:
    market-comm: boolean
    performance-suggestions: boolean
    phone-contact: boolean
    physical-mail: boolean
    special-offers: boolean
  family-name: string
  given-name: string
  industries: [string]
  job-functions: [string]
  languages: [string]
  markets: [string]
  migrate-to-afa: boolean
  phone-number: string
  primary-country-code: int64
  profile-public: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .address    address=invidunt`
    - The single string version of the address.
* `address-line=rebum.`
    - The following address lines represent the most specific part of any
        address.
    - Each invocation of this argument appends the given value to the array.
* `administrative-area=lorem`
    - Top-level administrative subdivision of this country.
* `dependent-locality=clita`
    - Dependent locality or sublocality. Used for UK dependent localities, or
        neighborhoods or boroughs in other locations.
* `language-code=invidunt`
    - Language code of the address. Should be in BCP 47 format.
* `lat-lng    latitude=0.113683921598`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.243025714914`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    locality=consetetur`
    - Generally refers to the city/town portion of an address.
* `postal-code=et`
    - Values are frequently alphanumeric.
* `region-code=sed`
    - CLDR (Common Locale Data Repository) region code .
* `sorting-code=sit`
    - Use of this code is very country-specific, but will refer to a secondary
        classification code for sorting mail.

* `..    adwords-manager-account=takimata`
    - If the user has edit access to multiple accounts, the user can choose the
        preferred account and it is used when a personal account is needed. Can
        be empty.
* `channels=elitr`
    - A list of ids representing which channels the user selected they were in.
    - Each invocation of this argument appends the given value to the array.
* `email-address=nonumy`
    - The email address the user has selected on the Partners site as primary.
* `email-opt-ins    market-comm=true`
    - An opt-in about receiving email from Partners marketing teams. Includes
        member-only events and special promotional offers for Google products.
* `performance-suggestions=true`
    - An opt-in about receiving email with customized AdWords campaign management
        tips.
* `phone-contact=true`
    - An opt-in to allow recieivng phone calls about their Partners account.
* `physical-mail=true`
    - An opt-in to receive special promotional gifts and material in the mail.
* `special-offers=true`
    - An opt-in about receiving email regarding new features and products.

* `..    family-name=ut`
    - The user&#39;s family name.
* `given-name=amet.`
    - The user&#39;s given name.
* `industries=ipsum`
    - A list of ids representing which industries the user selected.
    - Each invocation of this argument appends the given value to the array.
* `job-functions=ut`
    - A list of ids represnting which job categories the user selected.
    - Each invocation of this argument appends the given value to the array.
* `languages=dolor`
    - The list of languages this user understands.
    - Each invocation of this argument appends the given value to the array.
* `markets=sea`
    - A list of ids representing which markets the user was interested in.
    - Each invocation of this argument appends the given value to the array.
* `migrate-to-afa=true`
    - Whether or not to migrate the user&#39;s exam data to Academy for Ads.
* `phone-number=eirmod`
    - The user&#39;s phone number.
* `primary-country-code=-8`
    - The user&#39;s primary country, an ISO 2-character code.
* `profile-public=true`
    - Whether the user&#39;s public profile is visible to anyone with the URL.


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

* **-p request-metadata-user-overrides-ip-address=string**
    - IP address to use instead of the user&#39;s geo-located IP address.

* **-p request-metadata-partners-session-id=string**
    - Google Partners session ID.

* **-p request-metadata-user-overrides-user-id=string**
    - Logged-in user ID to impersonate instead of the user&#39;s ID.

* **-p request-metadata-experiment-ids=string**
    - Experiment IDs the current request belongs to.

* **-p request-metadata-locale=string**
    - Locale to use for the current request.

* **-p request-metadata-traffic-source-traffic-source-id=string**
    - Identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p request-metadata-traffic-source-traffic-sub-id=string**
    - Second level identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

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
