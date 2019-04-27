Updates a creative.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> accounts creatives-update ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - The account that this creative belongs to.
        Can be used to filter the response of the
        creatives.list
        method.
* **&lt;creative-id&gt;** *(string)*
    - The buyer-defined creative ID of this creative.
        Can be used to filter the response of the
        creatives.list
        method.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Creative:
  account-id: string
  ad-choices-destination-url: string
  advertiser-name: string
  agency-id: string
  api-update-time: string
  attributes: [string]
  click-through-urls: [string]
  creative-id: string
  deals-status: string
  declared-click-through-urls: [string]
  detected-advertiser-ids: [string]
  detected-domains: [string]
  detected-languages: [string]
  detected-product-categories: [integer]
  detected-sensitive-categories: [integer]
  filtering-stats:
    date:
      day: integer
      month: integer
      year: integer
  html:
    height: integer
    snippet: string
    width: integer
  impression-tracking-urls: [string]
  native:
    advertiser-name: string
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
    logo:
      height: integer
      url: string
      width: integer
    price-display-text: string
    star-rating: number
    store-url: string
    video-url: string
  open-auction-status: string
  restricted-categories: [string]
  vendor-ids: [integer]
  version: integer
  video:
    video-url: string
    video-vast-xml: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=sadipscing`
    - The account that this creative belongs to.
        Can be used to filter the response of the
        creatives.list
        method.
* `ad-choices-destination-url=invidunt`
    - The link to AdChoices destination page.
* `advertiser-name=consetetur`
    - The name of the company being advertised in the creative.
* `agency-id=dolore`
    - The agency ID for this creative.
* `api-update-time=duo`
    - @OutputOnly The last update timestamp of the creative via API.
* `attributes=aliquyam`
    - All attributes for the ads that may be shown from this creative.
        Can be used to filter the response of the
        creatives.list
        method.
    - Each invocation of this argument appends the given value to the array.
* `click-through-urls=lorem`
    - The set of destination URLs for the creative.
    - Each invocation of this argument appends the given value to the array.
* `creative-id=et`
    - The buyer-defined creative ID of this creative.
        Can be used to filter the response of the
        creatives.list
        method.
* `deals-status=clita`
    - @OutputOnly The top-level deals status of this creative.
        If disapproved, an entry for &#39;auctionType=DIRECT_DEALS&#39; (or &#39;ALL&#39;) in
        serving_restrictions will also exist. Note
        that this may be nuanced with other contextual restrictions, in which case,
        it may be preferable to read from serving_restrictions directly.
        Can be used to filter the response of the
        creatives.list
        method.
* `declared-click-through-urls=consetetur`
    - The set of declared destination URLs for the creative.
    - Each invocation of this argument appends the given value to the array.
* `detected-advertiser-ids=takimata`
    - @OutputOnly Detected advertiser IDs, if any.
    - Each invocation of this argument appends the given value to the array.
* `detected-domains=nonumy`
    - @OutputOnly
        The detected domains for this creative.
    - Each invocation of this argument appends the given value to the array.
* `detected-languages=kasd`
    - @OutputOnly
        The detected languages for this creative. The order is arbitrary. The codes
        are 2 or 5 characters and are documented at
        https://developers.google.com/adwords/api/docs/appendix/languagecodes.
    - Each invocation of this argument appends the given value to the array.
* `detected-product-categories=94`
    - @OutputOnly Detected product categories, if any.
        See the ad-product-categories.txt file in the technical documentation
        for a list of IDs.
    - Each invocation of this argument appends the given value to the array.
* `detected-sensitive-categories=43`
    - @OutputOnly Detected sensitive categories, if any.
        See the ad-sensitive-categories.txt file in the technical documentation for
        a list of IDs. You should use these IDs along with the
        excluded-sensitive-category field in the bid request to filter your bids.
    - Each invocation of this argument appends the given value to the array.
* `filtering-stats.date    day=74`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=15`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=64`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.


* `...html    height=35`
    - The height of the HTML snippet in pixels.
* `snippet=sadipscing`
    - The HTML snippet that displays the ad when inserted in the web page.
* `width=35`
    - The width of the HTML snippet in pixels.

* `..    impression-tracking-urls=dolore`
    - The set of URLs to be called to record an impression.
    - Each invocation of this argument appends the given value to the array.
* `native    advertiser-name=nonumy`
    - The name of the advertiser or sponsor, to be displayed in the ad creative.
* `app-icon    height=72`
    - Image height in pixels.
* `url=aliquyam`
    - The URL of the image.
* `width=48`
    - Image width in pixels.

* `..    body=eirmod`
    - A long description of the ad.
* `call-to-action=consetetur`
    - A label for the button that the user is supposed to click.
* `click-link-url=labore`
    - The URL that the browser/SDK will load when the user clicks the ad.
* `click-tracking-url=sed`
    - The URL to use for click tracking.
* `headline=ea`
    - A short title for the ad.
* `image    height=39`
    - Image height in pixels.
* `url=aliquyam`
    - The URL of the image.
* `width=77`
    - Image width in pixels.

* `..logo    height=63`
    - Image height in pixels.
* `url=sea`
    - The URL of the image.
* `width=16`
    - Image width in pixels.

* `..    price-display-text=ipsum`
    - The price of the promoted app including currency info.
* `star-rating=0.697654944876`
    - The app rating in the app store. Must be in the range [0-5].
* `store-url=dolores`
    - The URL to the app store to purchase/download the promoted app.
* `video-url=sit`
    - The URL to fetch a native video ad.

* `..    open-auction-status=diam`
    - @OutputOnly The top-level open auction status of this creative.
        If disapproved, an entry for &#39;auctionType = OPEN_AUCTION&#39; (or &#39;ALL&#39;) in
        serving_restrictions will also exist. Note
        that this may be nuanced with other contextual restrictions, in which case,
        it may be preferable to read from serving_restrictions directly.
        Can be used to filter the response of the
        creatives.list
        method.
* `restricted-categories=ut`
    - All restricted categories for the ads that may be shown from this creative.
    - Each invocation of this argument appends the given value to the array.
* `vendor-ids=31`
    - All vendor IDs for the ads that may be shown from this creative.
        See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt
        for possible values.
    - Each invocation of this argument appends the given value to the array.
* `version=95`
    - @OutputOnly The version of this creative.
* `video    video-url=amet`
    - The URL to fetch a video ad.
* `video-vast-xml=accusam`
    - The contents of a VAST document for a video ad.
        This document should conform to the VAST 2.0 or 3.0 standard.



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
