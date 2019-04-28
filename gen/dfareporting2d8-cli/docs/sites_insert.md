Inserts a new site.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting2d8 --scope <scope> sites insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Site:
  account-id: string
  approved: boolean
  directory-site-id: string
  directory-site-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
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
  name: string
  site-settings:
    active-view-opt-out: boolean
    ad-blocking-opt-out: boolean
    creative-settings:
      i-frame-footer: string
      i-frame-header: string
    disable-new-cookie: boolean
    lookback-configuration:
      click-duration: integer
      post-impression-activities-duration: integer
    tag-setting:
      additional-key-values: string
      include-click-through-urls: boolean
      include-click-tracking: boolean
      keyword-option: string
    video-active-view-opt-out-template: boolean
    vpaid-adapter-choice-template: string
  subaccount-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=rebum.`
    - Account ID of this site. This is a read-only field that can be left blank.
* `approved=true`
    - Whether this site is approved.
* `directory-site-id=nonumy`
    - Directory site associated with this site. This is a required field that is read-only after insertion.
* `directory-site-id-dimension-value    dimension-name=ipsum`
    - The name of the dimension.
* `etag=et`
    - The eTag of this response for caching purposes.
* `id=eos`
    - The ID associated with the value if available.
* `kind=amet`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=consetetur`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=dolor`
    - The value of the dimension.

* `..    id=est`
    - ID of this site. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=vero`
    - The name of the dimension.
* `etag=eos`
    - The eTag of this response for caching purposes.
* `id=stet`
    - The ID associated with the value if available.
* `kind=erat`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=eirmod`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=takimata`
    - The value of the dimension.

* `..    key-name=stet`
    - Key name of this site. This is a read-only, auto-generated field.
* `kind=no`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#site&#34;.
* `name=sadipscing`
    - Name of this site.This is a required field. Must be less than 128 characters long. If this site is under a subaccount, the name must be unique among sites of the same subaccount. Otherwise, this site is a top-level site, and the name must be unique among top-level sites of the same account.
* `site-settings    active-view-opt-out=false`
    - Whether active view creatives are disabled for this site.
* `ad-blocking-opt-out=false`
    - Whether this site opts out of ad blocking. When true, ad blocking is disabled for all placements under the site, regardless of the individual placement settings. When false, the campaign and placement settings take effect.
* `creative-settings    i-frame-footer=labore`
    - Header text for iFrames for this site. Must be less than or equal to 2000 characters long.
* `i-frame-header=clita`
    - Header text for iFrames for this site. Must be less than or equal to 2000 characters long.

* `..    disable-new-cookie=false`
    - Whether new cookies are disabled for this site.
* `lookback-configuration    click-duration=41`
    - Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
* `post-impression-activities-duration=18`
    - Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.

* `..tag-setting    additional-key-values=amet`
    - Additional key-values to be included in tags. Each key-value pair must be of the form key=value, and pairs must be separated by a semicolon (;). Keys and values must not contain commas. For example, id=2;color=red is a valid value for this field.
* `include-click-through-urls=true`
    - Whether static landing page URLs should be included in the tags. This setting applies only to placements.
* `include-click-tracking=true`
    - Whether click-tracking string should be included in the tags.
* `keyword-option=aliquyam`
    - Option specifying how keywords are embedded in ad tags. This setting can be used to specify whether keyword placeholders are inserted in placement tags for this site. Publishers can then add keywords to those placeholders.

* `..    video-active-view-opt-out-template=true`
    - Whether Verification and ActiveView for in-stream video creatives are disabled by default for new placements created under this site. This value will be used to populate the placement.videoActiveViewOptOut field, when no value is specified for the new placement.
* `vpaid-adapter-choice-template=amet.`
    - Default VPAID adapter setting for new placements created under this site. This value will be used to populate the placements.vpaidAdapterChoice field, when no value is specified for the new placement. Controls which VPAID format the measurement adapter will use for in-stream video creatives assigned to the placement. The publisher&#39;s specifications will typically determine this setting. For VPAID creatives, the adapter format will match the VPAID format (HTML5 VPAID creatives use the HTML5 adapter).
        
        Note: Flash is no longer supported. This field now defaults to HTML5 when the following values are provided: FLASH, BOTH.

* `..    subaccount-id=kasd`
    - Subaccount ID of this site. This is a read-only field that can be left blank.


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