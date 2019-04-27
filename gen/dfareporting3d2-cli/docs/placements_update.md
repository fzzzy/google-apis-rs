Updates an existing placement.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> placements update ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Placement:
  account-id: string
  ad-blocking-opt-out: boolean
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  archived: boolean
  campaign-id: string
  campaign-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  comment: string
  compatibility: string
  content-category-id: string
  create-info:
    time: string
  directory-site-id: string
  directory-site-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  external-id: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  key-name: string
  kind: string
  last-modified-info:
    time: string
  lookback-configuration:
    click-duration: integer
    post-impression-activities-duration: integer
  name: string
  payment-approved: boolean
  payment-source: string
  placement-group-id: string
  placement-group-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  placement-strategy-id: string
  pricing-schedule:
    cap-cost-option: string
    disregard-overdelivery: boolean
    end-date: string
    flighted: boolean
    floodlight-activity-id: string
    pricing-type: string
    start-date: string
    testing-start-date: string
  primary: boolean
  publisher-update-info:
    time: string
  site-id: string
  site-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  size:
    height: integer
    iab: boolean
    id: string
    kind: string
    width: integer
  ssl-required: boolean
  status: string
  subaccount-id: string
  tag-formats: [string]
  tag-setting:
    additional-key-values: string
    include-click-through-urls: boolean
    include-click-tracking: boolean
    keyword-option: string
  video-active-view-opt-out: boolean
  video-settings:
    companion-settings:
      companions-disabled: boolean
      image-only: boolean
      kind: string
    kind: string
    orientation: string
    skippable-settings:
      kind: string
      progress-offset:
        offset-percentage: integer
        offset-seconds: integer
      skip-offset:
        offset-percentage: integer
        offset-seconds: integer
      skippable: boolean
    transcode-settings:
      enabled-video-formats: [integer]
      kind: string
  vpaid-adapter-choice: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=sed`
    - Account ID of this placement. This field can be left blank.
* `ad-blocking-opt-out=false`
    - Whether this placement opts out of ad blocking. When true, ad blocking is disabled for this placement. When false, the campaign and site settings take effect.
* `advertiser-id=vero`
    - Advertiser ID of this placement. This field can be left blank.
* `advertiser-id-dimension-value    dimension-name=eos`
    - The name of the dimension.
* `etag=labore`
    - The eTag of this response for caching purposes.
* `id=diam`
    - The ID associated with the value if available.
* `kind=consetetur`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=eirmod`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=tempor`
    - The value of the dimension.

* `..    archived=false`
    - Whether this placement is archived.
* `campaign-id=sanctus`
    - Campaign ID of this placement. This field is a required field on insertion.
* `campaign-id-dimension-value    dimension-name=labore`
    - The name of the dimension.
* `etag=dolores`
    - The eTag of this response for caching purposes.
* `id=diam`
    - The ID associated with the value if available.
* `kind=dolor`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=aliquyam`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=ipsum`
    - The value of the dimension.

* `..    comment=at`
    - Comments for this placement.
* `compatibility=dolor`
    - Placement compatibility. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering on desktop, on mobile devices or in mobile apps for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are no longer allowed for new placement insertions. Instead, use DISPLAY or DISPLAY_INTERSTITIAL. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. This field is required on insertion.
* `content-category-id=amet`
    - ID of the content category assigned to this placement.
* `create-info    time=sit`
    - Timestamp of the last change in milliseconds since epoch.

* `..    directory-site-id=no`
    - Directory site ID of this placement. On insert, you must set either this field or the siteId field to specify the site associated with this placement. This is a required field that is read-only after insertion.
* `directory-site-id-dimension-value    dimension-name=amet.`
    - The name of the dimension.
* `etag=voluptua.`
    - The eTag of this response for caching purposes.
* `id=et`
    - The ID associated with the value if available.
* `kind=ipsum`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=rebum.`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=lorem`
    - The value of the dimension.

* `..    external-id=consetetur`
    - External ID for this placement.
* `id=sea`
    - ID of this placement. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=sanctus`
    - The name of the dimension.
* `etag=consetetur`
    - The eTag of this response for caching purposes.
* `id=gubergren`
    - The ID associated with the value if available.
* `kind=sanctus`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=consetetur`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=sadipscing`
    - The value of the dimension.

* `..    key-name=clita`
    - Key name of this placement. This is a read-only, auto-generated field.
* `kind=labore`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#placement&#34;.
* `last-modified-info    time=sit`
    - Timestamp of the last change in milliseconds since epoch.

* `..lookback-configuration    click-duration=76`
    - Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
* `post-impression-activities-duration=99`
    - Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.

* `..    name=magna`
    - Name of this placement.This is a required field and must be less than 256 characters long.
* `payment-approved=true`
    - Whether payment was approved for this placement. This is a read-only field relevant only to publisher-paid placements.
* `payment-source=et`
    - Payment source for this placement. This is a required field that is read-only after insertion.
* `placement-group-id=gubergren`
    - ID of this placement&#39;s group, if applicable.
* `placement-group-id-dimension-value    dimension-name=takimata`
    - The name of the dimension.
* `etag=voluptua.`
    - The eTag of this response for caching purposes.
* `id=consetetur`
    - The ID associated with the value if available.
* `kind=ipsum`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sed`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=clita`
    - The value of the dimension.

* `..    placement-strategy-id=et`
    - ID of the placement strategy assigned to this placement.
* `pricing-schedule    cap-cost-option=amet`
    - Placement cap cost option.
* `disregard-overdelivery=false`
    - Whether cap costs are ignored by ad serving.
* `end-date=invidunt`
    - Placement end date. This date must be later than, or the same day as, the placement start date, but not later than the campaign end date. If, for example, you set 6/25/2015 as both the start and end dates, the effective placement date is just that day only, 6/25/2015. The hours, minutes, and seconds of the end date should not be set, as doing so will result in an error. This field is required on insertion.
* `flighted=false`
    - Whether this placement is flighted. If true, pricing periods will be computed automatically.
* `floodlight-activity-id=stet`
    - Floodlight activity ID associated with this placement. This field should be set when placement pricing type is set to PRICING_TYPE_CPA.
* `pricing-type=erat`
    - Placement pricing type. This field is required on insertion.
* `start-date=et`
    - Placement start date. This date must be later than, or the same day as, the campaign start date. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error. This field is required on insertion.
* `testing-start-date=ea`
    - Testing start date of this placement. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error.

* `..    primary=true`
    - Whether this placement is the primary placement of a roadblock (placement group). You cannot change this field from true to false. Setting this field to true will automatically set the primary field on the original primary placement of the roadblock to false, and it will automatically set the roadblock&#39;s primaryPlacementId field to the ID of this placement.
* `publisher-update-info    time=diam`
    - Timestamp of the last change in milliseconds since epoch.

* `..    site-id=sed`
    - Site ID associated with this placement. On insert, you must set either this field or the directorySiteId field to specify the site associated with this placement. This is a required field that is read-only after insertion.
* `site-id-dimension-value    dimension-name=gubergren`
    - The name of the dimension.
* `etag=sadipscing`
    - The eTag of this response for caching purposes.
* `id=accusam`
    - The ID associated with the value if available.
* `kind=et`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sadipscing`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=dolores`
    - The value of the dimension.

* `..size    height=33`
    - Height of this size. Acceptable values are 0 to 32767, inclusive.
* `iab=false`
    - IAB standard size. This is a read-only, auto-generated field.
* `id=ipsum`
    - ID of this size. This is a read-only, auto-generated field.
* `kind=diam`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#size&#34;.
* `width=32`
    - Width of this size. Acceptable values are 0 to 32767, inclusive.

* `..    ssl-required=false`
    - Whether creatives assigned to this placement must be SSL-compliant.
* `status=kasd`
    - Third-party placement status.
* `subaccount-id=et`
    - Subaccount ID of this placement. This field can be left blank.
* `tag-formats=sanctus`
    - Tag formats to generate for this placement. This field is required on insertion.
        Acceptable values are:
        - &#34;PLACEMENT_TAG_STANDARD&#34;
        - &#34;PLACEMENT_TAG_IFRAME_JAVASCRIPT&#34;
        - &#34;PLACEMENT_TAG_IFRAME_ILAYER&#34;
        - &#34;PLACEMENT_TAG_INTERNAL_REDIRECT&#34;
        - &#34;PLACEMENT_TAG_JAVASCRIPT&#34;
        - &#34;PLACEMENT_TAG_INTERSTITIAL_IFRAME_JAVASCRIPT&#34;
        - &#34;PLACEMENT_TAG_INTERSTITIAL_INTERNAL_REDIRECT&#34;
        - &#34;PLACEMENT_TAG_INTERSTITIAL_JAVASCRIPT&#34;
        - &#34;PLACEMENT_TAG_CLICK_COMMANDS&#34;
        - &#34;PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH&#34;
        - &#34;PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_3&#34;
        - &#34;PLACEMENT_TAG_INSTREAM_VIDEO_PREFETCH_VAST_4&#34;
        - &#34;PLACEMENT_TAG_TRACKING&#34;
        - &#34;PLACEMENT_TAG_TRACKING_IFRAME&#34;
        - &#34;PLACEMENT_TAG_TRACKING_JAVASCRIPT&#34;
    - Each invocation of this argument appends the given value to the array.
* `tag-setting    additional-key-values=rebum.`
    - Additional key-values to be included in tags. Each key-value pair must be of the form key=value, and pairs must be separated by a semicolon (;). Keys and values must not contain commas. For example, id=2;color=red is a valid value for this field.
* `include-click-through-urls=true`
    - Whether static landing page URLs should be included in the tags. This setting applies only to placements.
* `include-click-tracking=true`
    - Whether click-tracking string should be included in the tags.
* `keyword-option=duo`
    - Option specifying how keywords are embedded in ad tags. This setting can be used to specify whether keyword placeholders are inserted in placement tags for this site. Publishers can then add keywords to those placeholders.

* `..    video-active-view-opt-out=true`
    - Whether Verification and ActiveView are disabled for in-stream video creatives for this placement. The same setting videoActiveViewOptOut exists on the site level -- the opt out occurs if either of these settings are true. These settings are distinct from DirectorySites.settings.activeViewOptOut or Sites.siteSettings.activeViewOptOut which only apply to display ads. However, Accounts.activeViewOptOut opts out both video traffic, as well as display ads, from Verification and ActiveView.
* `video-settings.companion-settings    companions-disabled=true`
    - Whether companions are disabled for this placement.
* `image-only=false`
    - Whether to serve only static images as companions.
* `kind=tempor`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#companionSetting&#34;.

* `..    kind=labore`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#videoSettings&#34;.
* `orientation=elitr`
    - Orientation of a video placement. If this value is set, placement will return assets matching the specified orientation.
* `skippable-settings    kind=gubergren`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#skippableSetting&#34;.
* `progress-offset    offset-percentage=56`
    - Duration, as a percentage of video duration. Do not set when offsetSeconds is set. Acceptable values are 0 to 100, inclusive.
* `offset-seconds=93`
    - Duration, in seconds. Do not set when offsetPercentage is set. Acceptable values are 0 to 86399, inclusive.

* `..skip-offset    offset-percentage=100`
    - Duration, as a percentage of video duration. Do not set when offsetSeconds is set. Acceptable values are 0 to 100, inclusive.
* `offset-seconds=30`
    - Duration, in seconds. Do not set when offsetPercentage is set. Acceptable values are 0 to 86399, inclusive.

* `..    skippable=true`
    - Whether the user can skip creatives served to this placement.

* `..transcode-settings    enabled-video-formats=10`
    - Whitelist of video formats to be served to this placement. Set this list to null or empty to serve all video formats.
    - Each invocation of this argument appends the given value to the array.
* `kind=sanctus`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#transcodeSetting&#34;.


* `...    vpaid-adapter-choice=clita`
    - VPAID adapter setting for this placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to this placement.
        
        Note: Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH.


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
