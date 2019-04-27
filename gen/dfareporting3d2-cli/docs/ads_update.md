Updates an existing ad.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> ads update ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Ad:
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
  archived: boolean
  audience-segment-id: string
  campaign-id: string
  campaign-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  click-through-url:
    computed-click-through-url: string
    custom-click-through-url: string
    default-landing-page: boolean
    landing-page-id: string
  click-through-url-suffix-properties:
    click-through-url-suffix: string
    override-inherited-suffix: boolean
  comments: string
  compatibility: string
  create-info:
    time: string
  creative-rotation:
    creative-optimization-configuration-id: string
    type: string
    weight-calculation-strategy: string
  day-part-targeting:
    days-of-week: [string]
    hours-of-day: [integer]
    user-local-time: boolean
  default-click-through-event-tag-properties:
    default-click-through-event-tag-id: string
    override-inherited-event-tag: boolean
  delivery-schedule:
    frequency-cap:
      duration: string
      impressions: string
    hard-cutoff: boolean
    impression-ratio: string
    priority: string
  dynamic-click-tracker: boolean
  end-time: string
  geo-targeting:
    exclude-countries: boolean
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  key-value-targeting-expression:
    expression: string
  kind: string
  last-modified-info:
    time: string
  name: string
  remarketing-list-expression:
    expression: string
  size:
    height: integer
    iab: boolean
    id: string
    kind: string
    width: integer
  ssl-compliant: boolean
  ssl-required: boolean
  start-time: string
  subaccount-id: string
  targeting-template-id: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=accusam`
    - Account ID of this ad. This is a read-only field that can be left blank.
* `active=false`
    - Whether this ad is active. When true, archived must be false.
* `advertiser-id=diam`
    - Advertiser ID of this ad. This is a required field on insertion.
* `advertiser-id-dimension-value    dimension-name=nonumy`
    - The name of the dimension.
* `etag=sed`
    - The eTag of this response for caching purposes.
* `id=diam`
    - The ID associated with the value if available.
* `kind=magna`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=dolor`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=lorem`
    - The value of the dimension.

* `..    archived=true`
    - Whether this ad is archived. When true, active must be false.
* `audience-segment-id=vero`
    - Audience segment ID that is being targeted for this ad. Applicable when type is AD_SERVING_STANDARD_AD.
* `campaign-id=nonumy`
    - Campaign ID of this ad. This is a required field on insertion.
* `campaign-id-dimension-value    dimension-name=takimata`
    - The name of the dimension.
* `etag=dolores`
    - The eTag of this response for caching purposes.
* `id=consetetur`
    - The ID associated with the value if available.
* `kind=erat`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=amet.`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=dolores`
    - The value of the dimension.

* `..click-through-url    computed-click-through-url=dolores`
    - Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: 
        - If defaultLandingPage is enabled then the campaign&#39;s default landing page URL is assigned to this field.
        - If defaultLandingPage is not enabled and a landingPageId is specified then that landing page&#39;s URL is assigned to this field.
        - If neither of the above cases apply, then the customClickThroughUrl is assigned to this field.
* `custom-click-through-url=et`
    - Custom click-through URL. Applicable if the defaultLandingPage field is set to false and the landingPageId field is left unset.
* `default-landing-page=true`
    - Whether the campaign default landing page is used.
* `landing-page-id=et`
    - ID of the landing page for the click-through URL. Applicable if the defaultLandingPage field is set to false.

* `..click-through-url-suffix-properties    click-through-url-suffix=aliquyam`
    - Click-through URL suffix to apply to all ads in this entity&#39;s scope. Must be less than 128 characters long.
* `override-inherited-suffix=false`
    - Whether this entity should override the inherited click-through URL suffix with its own defined value.

* `..    comments=sit`
    - Comments for this ad.
* `compatibility=aliquyam`
    - Compatibility of this ad. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to either rendering on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are only used for existing default ads. New mobile placements must be assigned DISPLAY or DISPLAY_INTERSTITIAL and default ads created for those placements will be limited to those compatibility types. IN_STREAM_VIDEO refers to rendering in-stream video ads developed with the VAST standard.
* `create-info    time=sadipscing`
    - Timestamp of the last change in milliseconds since epoch.

* `..creative-rotation    creative-optimization-configuration-id=magna`
    - Creative optimization configuration that is used by this ad. It should refer to one of the existing optimization configurations in the ad&#39;s campaign. If it is unset or set to 0, then the campaign&#39;s default optimization configuration will be used for this ad.
* `type=gubergren`
    - Type of creative rotation. Can be used to specify whether to use sequential or random rotation.
* `weight-calculation-strategy=sit`
    - Strategy for calculating weights. Used with CREATIVE_ROTATION_TYPE_RANDOM.

* `..day-part-targeting    days-of-week=gubergren`
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
* `hours-of-day=99`
    - Hours of the day when the ad will serve, where 0 is midnight to 1 AM and 23 is 11 PM to midnight. Can be specified with days of week, in which case the ad would serve during these hours on the specified days. For example if Monday, Wednesday, Friday are the days of week specified and 9-10am, 3-5pm (hours 9, 15, and 16) is specified, the ad would serve Monday, Wednesdays, and Fridays at 9-10am and 3-5pm. Acceptable values are 0 to 23, inclusive.
    - Each invocation of this argument appends the given value to the array.
* `user-local-time=false`
    - Whether or not to use the user&#39;s local time. If false, the America/New York time zone applies.

* `..default-click-through-event-tag-properties    default-click-through-event-tag-id=eirmod`
    - ID of the click-through event tag to apply to all ads in this entity&#39;s scope.
* `override-inherited-event-tag=false`
    - Whether this entity should override the inherited default click-through event tag with its own defined value.

* `..delivery-schedule.frequency-cap    duration=lorem`
    - Duration of time, in seconds, for this frequency cap. The maximum duration is 90 days. Acceptable values are 1 to 7776000, inclusive.
* `impressions=amet.`
    - Number of times an individual user can be served the ad within the specified duration. Acceptable values are 1 to 15, inclusive.

* `..    hard-cutoff=true`
    - Whether or not hard cutoff is enabled. If true, the ad will not serve after the end date and time. Otherwise the ad will continue to be served until it has reached its delivery goals.
* `impression-ratio=eirmod`
    - Impression ratio for this ad. This ratio determines how often each ad is served relative to the others. For example, if ad A has an impression ratio of 1 and ad B has an impression ratio of 3, then Campaign Manager will serve ad B three times as often as ad A. Acceptable values are 1 to 10, inclusive.
* `priority=sadipscing`
    - Serving priority of an ad, with respect to other ads. The lower the priority number, the greater the priority with which it is served.

* `..    dynamic-click-tracker=true`
    - Whether this ad is a dynamic click tracker. Applicable when type is AD_SERVING_CLICK_TRACKER. This is a required field on insert, and is read-only after insert.
* `end-time=sed`
    - Date and time that this ad should stop serving. Must be later than the start time. This is a required field on insertion.
* `geo-targeting    exclude-countries=true`
    - Whether or not to exclude the countries in the countries field from targeting. If false, the countries field refers to countries which will be targeted by the ad.

* `..    id=dolore`
    - ID of this ad. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=et`
    - The name of the dimension.
* `etag=at`
    - The eTag of this response for caching purposes.
* `id=sit`
    - The ID associated with the value if available.
* `kind=ut`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=diam`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=tempor`
    - The value of the dimension.

* `..key-value-targeting-expression    expression=et`
    - Keyword expression being targeted by the ad.

* `..    kind=erat`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#ad&#34;.
* `last-modified-info    time=dolores`
    - Timestamp of the last change in milliseconds since epoch.

* `..    name=kasd`
    - Name of this ad. This is a required field and must be less than 256 characters long.
* `remarketing-list-expression    expression=et`
    - Expression describing which lists are being targeted by the ad.

* `..size    height=37`
    - Height of this size. Acceptable values are 0 to 32767, inclusive.
* `iab=false`
    - IAB standard size. This is a read-only, auto-generated field.
* `id=dolores`
    - ID of this size. This is a read-only, auto-generated field.
* `kind=clita`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#size&#34;.
* `width=27`
    - Width of this size. Acceptable values are 0 to 32767, inclusive.

* `..    ssl-compliant=true`
    - Whether this ad is ssl compliant. This is a read-only field that is auto-generated when the ad is inserted or updated.
* `ssl-required=false`
    - Whether this ad requires ssl. This is a read-only field that is auto-generated when the ad is inserted or updated.
* `start-time=takimata`
    - Date and time that this ad should start serving. If creating an ad, this field must be a time in the future. This is a required field on insertion.
* `subaccount-id=sit`
    - Subaccount ID of this ad. This is a read-only field that can be left blank.
* `targeting-template-id=labore`
    - Targeting template ID, used to apply preconfigured targeting information to this ad. This cannot be set while any of dayPartTargeting, geoTargeting, keyValueTargetingExpression, languageTargeting, remarketingListExpression, or technologyTargeting are set. Applicable when type is AD_SERVING_STANDARD_AD.
* `type=nonumy`
    - Type of ad. This is a required field on insertion. Note that default ads (AD_SERVING_DEFAULT_AD) cannot be created directly (see Creative resource).


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
