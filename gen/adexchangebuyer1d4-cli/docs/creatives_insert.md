Submit a new creative.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer1d4 --scope <scope> creatives insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Creative:
  html-snippet: string
  account-id: integer
  ad-choices-destination-url: string
  advertiser-id: [string]
  advertiser-name: string
  agency-id: string
  api-upload-timestamp: string
  attribute: [integer]
  buyer-creative-id: string
  click-through-url: [string]
  creative-status-identity-type: string
  deals-status: string
  detected-domains: [string]
  filtering-reasons:
    date: string
  height: integer
  impression-tracking-url: [string]
  kind: string
  languages: [string]
  native-ad:
    advertiser: string
    app-icon:
      height: integer
      url: string
      width: integer
    body: string
    call-to-action: string
    click-link-url: string
    click-tracking-url: string
    headline: string
    image:
      height: integer
      url: string
      width: integer
    impression-tracking-url: [string]
    logo:
      height: integer
      url: string
      width: integer
    price: string
    star-rating: number
    store: string
    video-url: string
  open-auction-status: string
  product-categories: [integer]
  restricted-categories: [integer]
  sensitive-categories: [integer]
  vendor-type: [integer]
  version: integer
  video-url: string
  video-vast-xml: string
  width: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    html-snippet=et`
    - The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set.
* `account-id=31`
    - Account id.
* `ad-choices-destination-url=aliquyam`
    - The link to the Ad Preferences page. This is only supported for native ads.
* `advertiser-id=sea`
    - Detected advertiser id, if any. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `advertiser-name=lorem`
    - The name of the company being advertised in the creative. The value provided must exist in the advertisers.txt file.
* `agency-id=eos`
    - The agency id for this creative.
* `api-upload-timestamp=erat`
    - The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp).
* `attribute=6`
    - List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt.
    - Each invocation of this argument appends the given value to the array.
* `buyer-creative-id=dolor`
    - A buyer-specific id identifying the creative in this ad.
* `click-through-url=eirmod`
    - The set of destination urls for the snippet.
    - Each invocation of this argument appends the given value to the array.
* `creative-status-identity-type=elitr`
    - Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole.
* `deals-status=amet`
    - Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly.
* `detected-domains=no`
    - Detected domains for this creative. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `filtering-reasons    date=labore`
    - The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST.

* `..    height=62`
    - Ad height.
* `impression-tracking-url=dolore`
    - The set of urls to be called to record an impression.
    - Each invocation of this argument appends the given value to the array.
* `kind=invidunt`
    - Resource type.
* `languages=aliquyam`
    - Detected languages for this creative. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `native-ad    advertiser=accusam`
    - No description provided.
* `app-icon    height=45`
    - No description provided.
* `url=sea`
    - No description provided.
* `width=80`
    - No description provided.

* `..    body=duo`
    - A long description of the ad.
* `call-to-action=et`
    - A label for the button that the user is supposed to click.
* `click-link-url=eirmod`
    - The URL that the browser/SDK will load when the user clicks the ad.
* `click-tracking-url=sanctus`
    - The URL to use for click tracking.
* `headline=et`
    - A short title for the ad.
* `image    height=55`
    - No description provided.
* `url=et`
    - No description provided.
* `width=56`
    - No description provided.

* `..    impression-tracking-url=ut`
    - The URLs are called when the impression is rendered.
    - Each invocation of this argument appends the given value to the array.
* `logo    height=85`
    - No description provided.
* `url=sed`
    - No description provided.
* `width=98`
    - No description provided.

* `..    price=dolor`
    - The price of the promoted app including the currency info.
* `star-rating=0.528448098786`
    - The app rating in the app store. Must be in the range [0-5].
* `store=et`
    - The URL to the app store to purchase/download the promoted app.
* `video-url=consetetur`
    - The URL of the XML VAST for a native ad. Note this is a separate field from resource.video_url.

* `..    open-auction-status=amet.`
    - Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly.
* `product-categories=74`
    - Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `restricted-categories=45`
    - All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt.
    - Each invocation of this argument appends the given value to the array.
* `sensitive-categories=90`
    - Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `vendor-type=81`
    - List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt.
    - Each invocation of this argument appends the given value to the array.
* `version=49`
    - The version for this creative. Read-only. This field should not be set in requests.
* `video-url=vero`
    - The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above.
* `video-vast-xml=diam`
    - The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set.
* `width=35`
    - Ad width.


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
