Inserts a new creative.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> creatives insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Creative:
  account-id: string
  active: boolean
  ad-parameters: string
  ad-tag-keys: [string]
  advertiser-id: string
  allow-script-access: boolean
  archived: boolean
  artwork-type: string
  authoring-source: string
  authoring-tool: string
  auto-advance-images: boolean
  background-color: string
  backup-image-click-through-url:
    computed-click-through-url: string
    custom-click-through-url: string
    landing-page-id: string
  backup-image-features: [string]
  backup-image-reporting-label: string
  backup-image-target-window:
    custom-html: string
    target-window-option: string
  commercial-id: string
  companion-creatives: [string]
  compatibility: [string]
  convert-flash-to-html5: boolean
  creative-asset-selection:
    default-asset-id: string
  custom-key-values: [string]
  dynamic-asset-selection: boolean
  fs-command:
    left: integer
    position-option: string
    top: integer
    window-height: integer
    window-width: integer
  html-code: string
  html-code-locked: boolean
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
  latest-trafficked-creative-id: string
  media-description: string
  media-duration: number
  name: string
  override-css: string
  polite-load-asset-id: string
  progress-offset:
    offset-percentage: integer
    offset-seconds: integer
  redirect-url: string
  rendering-id: string
  rendering-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  required-flash-plugin-version: string
  required-flash-version: integer
  size:
    height: integer
    iab: boolean
    id: string
    kind: string
    width: integer
  skip-offset:
    offset-percentage: integer
    offset-seconds: integer
  skippable: boolean
  ssl-compliant: boolean
  ssl-override: boolean
  studio-advertiser-id: string
  studio-creative-id: string
  studio-trafficked-creative-id: string
  subaccount-id: string
  third-party-backup-image-impressions-url: string
  third-party-rich-media-impressions-url: string
  total-file-size: string
  type: string
  universal-ad-id:
    registry: string
    value: string
  version: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=no`
    - Account ID of this creative. This field, if left unset, will be auto-generated for both insert and update operations. Applicable to all creative types.
* `active=true`
    - Whether the creative is active. Applicable to all creative types.
* `ad-parameters=dolor`
    - Ad parameters user for VPAID creative. This is a read-only field. Applicable to the following creative types: all VPAID.
* `ad-tag-keys=et`
    - Keywords for a Rich Media creative. Keywords let you customize the creative settings of a Rich Media ad running on your site without having to contact the advertiser. You can use keywords to dynamically change the look or functionality of a creative. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    - Each invocation of this argument appends the given value to the array.
* `advertiser-id=ipsum`
    - Advertiser ID of this creative. This is a required field. Applicable to all creative types.
* `allow-script-access=true`
    - Whether script access is allowed for this creative. This is a read-only and deprecated field which will automatically be set to true on update. Applicable to the following creative types: FLASH_INPAGE.
* `archived=true`
    - Whether the creative is archived. Applicable to all creative types.
* `artwork-type=lorem`
    - Type of artwork used for the creative. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `authoring-source=dolores`
    - Source application where creative was authored. Presently, only DBM authored creatives will have this field set. Applicable to all creative types.
* `authoring-tool=dolores`
    - Authoring tool for HTML5 banner creatives. This is a read-only field. Applicable to the following creative types: HTML5_BANNER.
* `auto-advance-images=false`
    - Whether images are automatically advanced for image gallery creatives. Applicable to the following creative types: DISPLAY_IMAGE_GALLERY.
* `background-color=amet`
    - The 6-character HTML color code, beginning with #, for the background of the window area where the Flash file is displayed. Default is white. Applicable to the following creative types: FLASH_INPAGE.
* `backup-image-click-through-url    computed-click-through-url=duo`
    - Read-only convenience field representing the actual URL that will be used for this click-through. The URL is computed as follows: 
        - If landingPageId is specified then that landing page&#39;s URL is assigned to this field.
        - Otherwise, the customClickThroughUrl is assigned to this field.
* `custom-click-through-url=diam`
    - Custom click-through URL. Applicable if the landingPageId field is left unset.
* `landing-page-id=et`
    - ID of the landing page for the click-through URL.

* `..    backup-image-features=sit`
    - List of feature dependencies that will cause a backup image to be served if the browser that serves the ad does not support them. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative asset correctly. This field is initially auto-generated to contain all features detected by Campaign Manager for all the assets of this creative and can then be modified by the client. To reset this field, copy over all the creativeAssets&#39; detected features. Applicable to the following creative types: HTML5_BANNER. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
    - Each invocation of this argument appends the given value to the array.
* `backup-image-reporting-label=tempor`
    - Reporting label used for HTML5 banner backup image. Applicable to the following creative types: DISPLAY when the primary asset type is not HTML_IMAGE.
* `backup-image-target-window    custom-html=rebum.`
    - User-entered value.
* `target-window-option=sed`
    - Type of browser window for which the backup image of the flash creative can be displayed.

* `..    commercial-id=et`
    - Industry standard ID assigned to creative for reach and frequency. Applicable to INSTREAM_VIDEO_REDIRECT creatives.
* `companion-creatives=rebum.`
    - List of companion creatives assigned to an in-Stream video creative. Acceptable values include IDs of existing flash and image creatives. Applicable to the following creative types: all VPAID, all INSTREAM_AUDIO and all INSTREAM_VIDEO with dynamicAssetSelection set to false.
    - Each invocation of this argument appends the given value to the array.
* `compatibility=eos`
    - Compatibilities associated with this creative. This is a read-only field. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices or in mobile apps for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. Only pre-existing creatives may have these compatibilities since new creatives will either be assigned DISPLAY or DISPLAY_INTERSTITIAL instead. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard. IN_STREAM_AUDIO refers to rendering in in-stream audio ads developed with the VAST standard. Applicable to all creative types.
        
        Acceptable values are:
        - &#34;APP&#34;
        - &#34;APP_INTERSTITIAL&#34;
        - &#34;IN_STREAM_VIDEO&#34;
        - &#34;IN_STREAM_AUDIO&#34;
        - &#34;DISPLAY&#34;
        - &#34;DISPLAY_INTERSTITIAL&#34;
    - Each invocation of this argument appends the given value to the array.
* `convert-flash-to-html5=true`
    - Whether Flash assets associated with the creative need to be automatically converted to HTML5. This flag is enabled by default and users can choose to disable it if they don&#39;t want the system to generate and use HTML5 asset for this creative. Applicable to the following creative type: FLASH_INPAGE. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
* `creative-asset-selection    default-asset-id=dolores`
    - A creativeAssets[].id. This should refer to one of the parent assets in this creative, and will be served if none of the rules match. This is a required field.

* `..    custom-key-values=ut`
    - Custom key-values for a Rich Media creative. Key-values let you customize the creative settings of a Rich Media ad running on your site without having to contact the advertiser. You can use key-values to dynamically change the look or functionality of a creative. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
    - Each invocation of this argument appends the given value to the array.
* `dynamic-asset-selection=false`
    - Set this to true to enable the use of rules to target individual assets in this creative. When set to true creativeAssetSelection must be set. This also controls asset-level companions. When this is true, companion creatives should be assigned to creative assets. Learn more. Applicable to INSTREAM_VIDEO creatives.
* `fs-command    left=26`
    - Distance from the left of the browser.Applicable when positionOption is DISTANCE_FROM_TOP_LEFT_CORNER.
* `position-option=voluptua.`
    - Position in the browser where the window will open.
* `top=43`
    - Distance from the top of the browser. Applicable when positionOption is DISTANCE_FROM_TOP_LEFT_CORNER.
* `window-height=98`
    - Height of the window.
* `window-width=1`
    - Width of the window.

* `..    html-code=lorem`
    - HTML code for the creative. This is a required field when applicable. This field is ignored if htmlCodeLocked is true. Applicable to the following creative types: all CUSTOM, FLASH_INPAGE, and HTML5_BANNER, and all RICH_MEDIA.
* `html-code-locked=true`
    - Whether HTML code is generated by Campaign Manager or manually entered. Set to true to ignore changes to htmlCode. Applicable to the following creative types: FLASH_INPAGE and HTML5_BANNER.
* `id=sanctus`
    - ID of this creative. This is a read-only, auto-generated field. Applicable to all creative types.
* `id-dimension-value    dimension-name=rebum.`
    - The name of the dimension.
* `etag=invidunt`
    - The eTag of this response for caching purposes.
* `id=no`
    - The ID associated with the value if available.
* `kind=et`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=consetetur`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=aliquyam`
    - The value of the dimension.

* `..    kind=diam`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#creative&#34;.
* `last-modified-info    time=magna`
    - Timestamp of the last change in milliseconds since epoch.

* `..    latest-trafficked-creative-id=dolores`
    - Latest Studio trafficked creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `media-description=eirmod`
    - Description of the audio or video ad. Applicable to the following creative types: all INSTREAM_VIDEO, INSTREAM_AUDIO, and all VPAID.
* `media-duration=0.0967763212736`
    - Creative audio or video duration in seconds. This is a read-only field. Applicable to the following creative types: INSTREAM_VIDEO, INSTREAM_AUDIO, all RICH_MEDIA, and all VPAID.
* `name=sea`
    - Name of the creative. This is a required field and must be less than 256 characters long. Applicable to all creative types.
* `override-css=labore`
    - Override CSS value for rich media creatives. Applicable to the following creative types: all RICH_MEDIA.
* `polite-load-asset-id=sed`
    - The asset ID of the polite load image asset. Applicable to the creative type: DISPLAY.
* `progress-offset    offset-percentage=100`
    - Duration, as a percentage of video duration. Do not set when offsetSeconds is set. Acceptable values are 0 to 100, inclusive.
* `offset-seconds=32`
    - Duration, in seconds. Do not set when offsetPercentage is set. Acceptable values are 0 to 86399, inclusive.

* `..    redirect-url=sit`
    - URL of hosted image or hosted video or another ad tag. For INSTREAM_VIDEO_REDIRECT creatives this is the in-stream video redirect URL. The standard for a VAST (Video Ad Serving Template) ad response allows for a redirect link to another VAST 2.0 or 3.0 call. This is a required field when applicable. Applicable to the following creative types: DISPLAY_REDIRECT, INTERNAL_REDIRECT, INTERSTITIAL_INTERNAL_REDIRECT, and INSTREAM_VIDEO_REDIRECT
* `rendering-id=at`
    - ID of current rendering version. This is a read-only field. Applicable to all creative types.
* `rendering-id-dimension-value    dimension-name=voluptua.`
    - The name of the dimension.
* `etag=sed`
    - The eTag of this response for caching purposes.
* `id=sea`
    - The ID associated with the value if available.
* `kind=dolore`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=kasd`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=at`
    - The value of the dimension.

* `..    required-flash-plugin-version=diam`
    - The minimum required Flash plugin version for this creative. For example, 11.2.202.235. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `required-flash-version=93`
    - The internal Flash version for this creative as calculated by Studio. This is a read-only field. Applicable to the following creative types: FLASH_INPAGE all RICH_MEDIA, and all VPAID. Applicable to DISPLAY when the primary asset type is not HTML_IMAGE.
* `size    height=62`
    - Height of this size. Acceptable values are 0 to 32767, inclusive.
* `iab=true`
    - IAB standard size. This is a read-only, auto-generated field.
* `id=justo`
    - ID of this size. This is a read-only, auto-generated field.
* `kind=et`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#size&#34;.
* `width=83`
    - Width of this size. Acceptable values are 0 to 32767, inclusive.

* `..skip-offset    offset-percentage=32`
    - Duration, as a percentage of video duration. Do not set when offsetSeconds is set. Acceptable values are 0 to 100, inclusive.
* `offset-seconds=8`
    - Duration, in seconds. Do not set when offsetPercentage is set. Acceptable values are 0 to 86399, inclusive.

* `..    skippable=false`
    - Whether the user can choose to skip the creative. Applicable to the following creative types: all INSTREAM_VIDEO and all VPAID.
* `ssl-compliant=false`
    - Whether the creative is SSL-compliant. This is a read-only field. Applicable to all creative types.
* `ssl-override=false`
    - Whether creative should be treated as SSL compliant even if the system scan shows it&#39;s not. Applicable to all creative types.
* `studio-advertiser-id=est`
    - Studio advertiser ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `studio-creative-id=invidunt`
    - Studio creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `studio-trafficked-creative-id=invidunt`
    - Studio trafficked creative ID associated with rich media and VPAID creatives. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `subaccount-id=ipsum`
    - Subaccount ID of this creative. This field, if left unset, will be auto-generated for both insert and update operations. Applicable to all creative types.
* `third-party-backup-image-impressions-url=eirmod`
    - Third-party URL used to record backup image impressions. Applicable to the following creative types: all RICH_MEDIA.
* `third-party-rich-media-impressions-url=tempor`
    - Third-party URL used to record rich media impressions. Applicable to the following creative types: all RICH_MEDIA.
* `total-file-size=clita`
    - Combined size of all creative assets. This is a read-only field. Applicable to the following creative types: all RICH_MEDIA, and all VPAID.
* `type=stet`
    - Type of this creative. This is a required field. Applicable to all creative types.
        
        Note: FLASH_INPAGE, HTML5_BANNER, and IMAGE are only used for existing creatives. New creatives should use DISPLAY as a replacement for these types.
* `universal-ad-id    registry=et`
    - Registry used for the Ad ID value.
* `value=est`
    - ID value for this creative. Only alphanumeric characters and the following symbols are valid: &#34;_/\-&#34;. Maximum length is 64 characters. Read only when registry is DCM.

* `..    version=46`
    - The version number helps you keep track of multiple versions of your creative in your reports. The version number will always be auto-generated during insert operations to start at 1. For tracking creatives the version cannot be incremented and will always remain at 1. For all other creative types the version can be incremented only by 1 during update operations. In addition, the version will be automatically incremented by 1 when undergoing Rich Media creative merging. Applicable to all creative types.


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
