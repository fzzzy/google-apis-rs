Inserts a new pretargeting configuration.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer1d3 --scope <scope> pretargeting-config insert ...`
# Required Scalar Argument
* **&lt;account-id&gt;** *(string)*
    - The account id to insert the pretargeting config for.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PretargetingConfig:
  billing-id: string
  config-id: string
  config-name: string
  creative-type: [string]
  excluded-content-labels: [string]
  excluded-geo-criteria-ids: [string]
  excluded-user-lists: [string]
  excluded-verticals: [string]
  geo-criteria-ids: [string]
  is-active: boolean
  kind: string
  languages: [string]
  mobile-carriers: [string]
  mobile-devices: [string]
  mobile-operating-system-versions: [string]
  platforms: [string]
  supported-creative-attributes: [string]
  user-lists: [string]
  vendor-types: [string]
  verticals: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    billing-id=consetetur`
    - The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically.
* `config-id=amet.`
    - The config id; generated automatically. Leave this field blank for insert requests.
* `config-name=voluptua.`
    - The name of the config. Must be unique. Required for all requests.
* `creative-type=lorem`
    - List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO.
    - Each invocation of this argument appends the given value to the array.
* `excluded-content-labels=gubergren`
    - Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `excluded-geo-criteria-ids=justo`
    - Requests containing any of these geo criteria ids will not match.
    - Each invocation of this argument appends the given value to the array.
* `excluded-user-lists=sit`
    - Requests containing any of these users list ids will not match.
    - Each invocation of this argument appends the given value to the array.
* `excluded-verticals=vero`
    - Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `geo-criteria-ids=diam`
    - Requests containing any of these geo criteria ids will match.
    - Each invocation of this argument appends the given value to the array.
* `is-active=false`
    - Whether this config is active. Required for all requests.
* `kind=consetetur`
    - The kind of the resource, i.e. &#34;adexchangebuyer#pretargetingConfig&#34;.
* `languages=sadipscing`
    - Request containing any of these language codes will match.
    - Each invocation of this argument appends the given value to the array.
* `mobile-carriers=vero`
    - Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `mobile-devices=sadipscing`
    - Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `mobile-operating-system-versions=invidunt`
    - Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `platforms=consetetur`
    - Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET.
    - Each invocation of this argument appends the given value to the array.
* `supported-creative-attributes=dolore`
    - Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `user-lists=duo`
    - Requests containing any of these user list ids will match.
    - Each invocation of this argument appends the given value to the array.
* `vendor-types=aliquyam`
    - Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section.
    - Each invocation of this argument appends the given value to the array.
* `verticals=lorem`
    - Requests containing any of these vertical ids will match.
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
