Updates an existing floodlight configuration. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> floodlight-configurations patch ...`
# Required Scalar Arguments
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
* **&lt;id&gt;** *(string)*
    - Floodlight configuration ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
FloodlightConfiguration:
  account-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  analytics-data-sharing-enabled: boolean
  exposure-to-conversion-enabled: boolean
  first-day-of-week: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  in-app-attribution-tracking-enabled: boolean
  kind: string
  lookback-configuration:
    click-duration: integer
    post-impression-activities-duration: integer
  natural-search-conversion-attribution-option: string
  omniture-settings:
    omniture-cost-data-enabled: boolean
    omniture-integration-enabled: boolean
  subaccount-id: string
  tag-settings:
    dynamic-tag-enabled: boolean
    image-tag-enabled: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=invidunt`
    - Account ID of this floodlight configuration. This is a read-only field that can be left blank.
* `advertiser-id=vero`
    - Advertiser ID of the parent advertiser of this floodlight configuration.
* `advertiser-id-dimension-value    dimension-name=rebum.`
    - The name of the dimension.
* `etag=voluptua.`
    - The eTag of this response for caching purposes.
* `id=et`
    - The ID associated with the value if available.
* `kind=consetetur`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=vero`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=ipsum`
    - The value of the dimension.

* `..    analytics-data-sharing-enabled=true`
    - Whether advertiser data is shared with Google Analytics.
* `exposure-to-conversion-enabled=true`
    - Whether the exposure-to-conversion report is enabled. This report shows detailed pathway information on up to 10 of the most recent ad exposures seen by a user before converting.
* `first-day-of-week=consetetur`
    - Day that will be counted as the first day of the week in reports. This is a required field.
* `id=dolores`
    - ID of this floodlight configuration. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=et`
    - The name of the dimension.
* `etag=tempor`
    - The eTag of this response for caching purposes.
* `id=nonumy`
    - The ID associated with the value if available.
* `kind=sanctus`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sanctus`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=sed`
    - The value of the dimension.

* `..    in-app-attribution-tracking-enabled=true`
    - Whether in-app attribution tracking is enabled.
* `kind=amet`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#floodlightConfiguration&#34;.
* `lookback-configuration    click-duration=20`
    - Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
* `post-impression-activities-duration=88`
    - Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.

* `..    natural-search-conversion-attribution-option=et`
    - Types of attribution options for natural search conversions.
* `omniture-settings    omniture-cost-data-enabled=true`
    - Whether placement cost data will be sent to Omniture. This property can be enabled only if omnitureIntegrationEnabled is true.
* `omniture-integration-enabled=true`
    - Whether Omniture integration is enabled. This property can be enabled only when the &#34;Advanced Ad Serving&#34; account setting is enabled.

* `..    subaccount-id=amet`
    - Subaccount ID of this floodlight configuration. This is a read-only field that can be left blank.
* `tag-settings    dynamic-tag-enabled=true`
    - Whether dynamic floodlight tags are enabled.
* `image-tag-enabled=true`
    - Whether image tags are enabled.



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
