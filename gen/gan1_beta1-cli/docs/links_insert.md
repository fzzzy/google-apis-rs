Inserts a new link.
# Required Scalar Arguments
* **&lt;role&gt;** *(string)*
    - The role of the requester. Valid values: &#39;advertisers&#39; or &#39;publishers&#39;.
* **&lt;role-id&gt;** *(string)*
    - The ID of the requesting advertiser or publisher.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Link:
  advertiser-id: string
  authorship: string
  availability: string
  click-tracking-url: string
  create-date: string
  description: string
  destination-url: string
  duration: string
  end-date: string
  epc-ninety-day-average:
    amount: number
    currency-code: string
  epc-seven-day-average:
    amount: number
    currency-code: string
  id: string
  image-alt-text: string
  impression-tracking-url: string
  is-active: boolean
  kind: string
  link-type: string
  name: string
  promotion-type: string
  special-offers:
    free-gift: boolean
    free-shipping: boolean
    free-shipping-min:
      amount: number
      currency-code: string
    percent-off: number
    percent-off-min:
      amount: number
      currency-code: string
    price-cut:
      amount: number
      currency-code: string
    price-cut-min:
      amount: number
      currency-code: string
    promotion-codes: [string]
  start-date: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    advertiser-id=eirmod`
    - The advertiser id for the advertiser who owns this link.
* `authorship=sit`
    - Authorship
* `availability=stet`
    - Availability.
* `click-tracking-url=sed`
    - Tracking url for clicks.
* `create-date=et`
    - Date that this link was created.
* `description=dolores`
    - Description.
* `destination-url=kasd`
    - The destination URL for the link.
* `duration=accusam`
    - Duration
* `end-date=takimata`
    - Date that this link becomes inactive.
* `epc-ninety-day-average    amount=0.30763338798`
    - The amount of money.
* `currency-code=amet.`
    - The 3-letter code of the currency in question.

* `..epc-seven-day-average    amount=0.204946002078`
    - The amount of money.
* `currency-code=labore`
    - The 3-letter code of the currency in question.

* `..    id=sea`
    - The ID of this link.
* `image-alt-text=nonumy`
    - image alt text.
* `impression-tracking-url=dolores`
    - Tracking url for impressions.
* `is-active=false`
    - Flag for if this link is active.
* `kind=sadipscing`
    - The kind for one entity.
* `link-type=aliquyam`
    - The link type.
* `name=ea`
    - The logical name for this link.
* `promotion-type=no`
    - Promotion Type
* `special-offers    free-gift=true`
    - Whether there is a free gift
* `free-shipping=true`
    - Whether there is free shipping
* `free-shipping-min    amount=0.663748882844`
    - The amount of money.
* `currency-code=et`
    - The 3-letter code of the currency in question.

* `..    percent-off=0.594456807662`
    - Percent off on the purchase
* `percent-off-min    amount=0.460933679688`
    - The amount of money.
* `currency-code=lorem`
    - The 3-letter code of the currency in question.

* `..price-cut    amount=0.795720263212`
    - The amount of money.
* `currency-code=duo`
    - The 3-letter code of the currency in question.

* `..price-cut-min    amount=0.69054137112`
    - The amount of money.
* `currency-code=sea`
    - The 3-letter code of the currency in question.

* `..    promotion-codes=lorem`
    - List of promotion code associated with the link
    - Each invocation of this argument appends the given value to the array.

* `..    start-date=eos`
    - Date that this link becomes active.


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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
