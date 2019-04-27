Updates an existing campaign. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> campaigns patch ...`
# Required Scalar Arguments
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
* **&lt;id&gt;** *(string)*
    - Campaign ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Campaign:
  account-id: string
  ad-blocking-configuration:
    click-through-url: string
    creative-bundle-id: string
    enabled: boolean
    override-click-through-url: boolean
  advertiser-group-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  archived: boolean
  billing-invoice-code: string
  click-through-url-suffix-properties:
    click-through-url-suffix: string
    override-inherited-suffix: boolean
  comment: string
  create-info:
    time: string
  creative-group-ids: [string]
  creative-optimization-configuration:
    id: string
    name: string
    optimization-model: string
  default-click-through-event-tag-properties:
    default-click-through-event-tag-id: string
    override-inherited-event-tag: boolean
  default-landing-page-id: string
  end-date: string
  external-id: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  kind: string
  last-modified-info:
    time: string
  lookback-configuration:
    click-duration: integer
    post-impression-activities-duration: integer
  name: string
  nielsen-ocr-enabled: boolean
  start-date: string
  subaccount-id: string
  trafficker-emails: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=tempor`
    - Account ID of this campaign. This is a read-only field that can be left blank.
* `ad-blocking-configuration    click-through-url=justo`
    - Click-through URL used by brand-neutral ads. This is a required field when overrideClickThroughUrl is set to true.
* `creative-bundle-id=takimata`
    - ID of a creative bundle to use for this campaign. If set, brand-neutral ads will select creatives from this bundle. Otherwise, a default transparent pixel will be used.
* `enabled=true`
    - Whether this campaign has enabled ad blocking. When true, ad blocking is enabled for placements in the campaign, but this may be overridden by site and placement settings. When false, ad blocking is disabled for all placements under the campaign, regardless of site and placement settings.
* `override-click-through-url=true`
    - Whether the brand-neutral ad&#39;s click-through URL comes from the campaign&#39;s creative bundle or the override URL. Must be set to true if ad blocking is enabled and no creative bundle is configured.

* `..    advertiser-group-id=dolor`
    - Advertiser group ID of the associated advertiser.
* `advertiser-id=voluptua.`
    - Advertiser ID of this campaign. This is a required field.
* `advertiser-id-dimension-value    dimension-name=et`
    - The name of the dimension.
* `etag=elitr`
    - The eTag of this response for caching purposes.
* `id=kasd`
    - The ID associated with the value if available.
* `kind=sit`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=justo`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=gubergren`
    - The value of the dimension.

* `..    archived=false`
    - Whether this campaign has been archived.
* `billing-invoice-code=amet.`
    - Billing invoice code included in the Campaign Manager client billing invoices associated with the campaign.
* `click-through-url-suffix-properties    click-through-url-suffix=dolor`
    - Click-through URL suffix to apply to all ads in this entity&#39;s scope. Must be less than 128 characters long.
* `override-inherited-suffix=false`
    - Whether this entity should override the inherited click-through URL suffix with its own defined value.

* `..    comment=ut`
    - Arbitrary comments about this campaign. Must be less than 256 characters long.
* `create-info    time=magna`
    - Timestamp of the last change in milliseconds since epoch.

* `..    creative-group-ids=amet.`
    - List of creative group IDs that are assigned to the campaign.
    - Each invocation of this argument appends the given value to the array.
* `creative-optimization-configuration    id=eirmod`
    - ID of this creative optimization config. This field is auto-generated when the campaign is inserted or updated. It can be null for existing campaigns.
* `name=ea`
    - Name of this creative optimization config. This is a required field and must be less than 129 characters long.
* `optimization-model=ea`
    - Optimization model for this configuration.

* `..default-click-through-event-tag-properties    default-click-through-event-tag-id=et`
    - ID of the click-through event tag to apply to all ads in this entity&#39;s scope.
* `override-inherited-event-tag=true`
    - Whether this entity should override the inherited default click-through event tag with its own defined value.

* `..    default-landing-page-id=diam`
    - The default landing page ID for this campaign.
* `end-date=eos`
    - Date on which the campaign will stop running. On insert, the end date must be today or a future date. The end date must be later than or be the same as the start date. If, for example, you set 6/25/2015 as both the start and end dates, the effective campaign run date is just that day only, 6/25/2015. The hours, minutes, and seconds of the end date should not be set, as doing so will result in an error. This is a required field.
* `external-id=tempor`
    - External ID for this campaign.
* `id=sea`
    - ID of this campaign. This is a read-only auto-generated field.
* `id-dimension-value    dimension-name=ea`
    - The name of the dimension.
* `etag=nonumy`
    - The eTag of this response for caching purposes.
* `id=duo`
    - The ID associated with the value if available.
* `kind=tempor`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=consetetur`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=eirmod`
    - The value of the dimension.

* `..    kind=aliquyam`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#campaign&#34;.
* `last-modified-info    time=no`
    - Timestamp of the last change in milliseconds since epoch.

* `..lookback-configuration    click-duration=19`
    - Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
* `post-impression-activities-duration=79`
    - Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.

* `..    name=sed`
    - Name of this campaign. This is a required field and must be less than 256 characters long and unique among campaigns of the same advertiser.
* `nielsen-ocr-enabled=false`
    - Whether Nielsen reports are enabled for this campaign.
* `start-date=amet.`
    - Date on which the campaign starts running. The start date can be any date. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error. This is a required field.
* `subaccount-id=clita`
    - Subaccount ID of this campaign. This is a read-only field that can be left blank.
* `trafficker-emails=clita`
    - Campaign trafficker contact emails.
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
