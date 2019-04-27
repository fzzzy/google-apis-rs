Submit a new creative.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer1d3 --scope <scope> creatives insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Creative:
  html-snippet: string
  account-id: integer
  advertiser-id: [string]
  advertiser-name: string
  agency-id: string
  api-upload-timestamp: string
  attribute: [integer]
  buyer-creative-id: string
  click-through-url: [string]
  filtering-reasons:
    date: string
  height: integer
  impression-tracking-url: [string]
  kind: string
  native-ad:
    advertiser: string
    app-icon:
      height: integer
      url: string
      width: integer
    body: string
    call-to-action: string
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
  product-categories: [integer]
  restricted-categories: [integer]
  sensitive-categories: [integer]
  status: string
  vendor-type: [integer]
  version: integer
  video-url: string
  width: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    html-snippet=ipsum`
    - The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set.
* `account-id=96`
    - Account id.
* `advertiser-id=et`
    - Detected advertiser id, if any. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `advertiser-name=duo`
    - The name of the company being advertised in the creative.
* `agency-id=aliquyam`
    - The agency id for this creative.
* `api-upload-timestamp=sea`
    - The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp).
* `attribute=46`
    - All attributes for the ads that may be shown from this snippet.
    - Each invocation of this argument appends the given value to the array.
* `buyer-creative-id=eos`
    - A buyer-specific id identifying the creative in this ad.
* `click-through-url=erat`
    - The set of destination urls for the snippet.
    - Each invocation of this argument appends the given value to the array.
* `filtering-reasons    date=sadipscing`
    - The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST.

* `..    height=53`
    - Ad height.
* `impression-tracking-url=eirmod`
    - The set of urls to be called to record an impression.
    - Each invocation of this argument appends the given value to the array.
* `kind=elitr`
    - Resource type.
* `native-ad    advertiser=amet`
    - No description provided.
* `app-icon    height=41`
    - No description provided.
* `url=labore`
    - No description provided.
* `width=62`
    - No description provided.

* `..    body=dolore`
    - A long description of the ad.
* `call-to-action=invidunt`
    - A label for the button that the user is supposed to click.
* `click-tracking-url=aliquyam`
    - The URL to use for click tracking.
* `headline=accusam`
    - A short title for the ad.
* `image    height=45`
    - No description provided.
* `url=sea`
    - No description provided.
* `width=80`
    - No description provided.

* `..    impression-tracking-url=duo`
    - The URLs are called when the impression is rendered.
    - Each invocation of this argument appends the given value to the array.
* `logo    height=80`
    - No description provided.
* `url=eirmod`
    - No description provided.
* `width=43`
    - No description provided.

* `..    price=et`
    - The price of the promoted app including the currency info.
* `star-rating=0.549609244211`
    - The app rating in the app store. Must be in the range [0-5].
* `store=et`
    - The URL to the app store to purchase/download the promoted app.

* `..    product-categories=56`
    - Detected product categories, if any. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `restricted-categories=65`
    - All restricted categories for the ads that may be shown from this snippet.
    - Each invocation of this argument appends the given value to the array.
* `sensitive-categories=85`
    - Detected sensitive categories, if any. Read-only. This field should not be set in requests.
    - Each invocation of this argument appends the given value to the array.
* `status=sed`
    - Creative serving status. Read-only. This field should not be set in requests.
* `vendor-type=98`
    - All vendor types for the ads that may be shown from this snippet.
    - Each invocation of this argument appends the given value to the array.
* `version=53`
    - The version for this creative. Read-only. This field should not be set in requests.
* `video-url=dolor`
    - The URL to fetch a video ad. If set, HTMLSnippet and the nativeAd should not be set.
* `width=78`
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
