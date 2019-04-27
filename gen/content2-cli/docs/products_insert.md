Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> products insert ...`
# Required Scalar Argument
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the account that contains the product. This account cannot be a multi-client account.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Product:
  additional-image-links: [string]
  additional-product-types: [string]
  adult: boolean
  adwords-grouping: string
  adwords-labels: [string]
  adwords-redirect: string
  age-group: string
  availability: string
  availability-date: string
  brand: string
  channel: string
  color: string
  condition: string
  content-language: string
  cost-of-goods-sold:
    currency: string
    value: string
  custom-label0: string
  custom-label1: string
  custom-label2: string
  custom-label3: string
  custom-label4: string
  description: string
  display-ads-id: string
  display-ads-link: string
  display-ads-similar-ids: [string]
  display-ads-title: string
  display-ads-value: number
  energy-efficiency-class: string
  expiration-date: string
  gender: string
  google-product-category: string
  gtin: string
  id: string
  identifier-exists: boolean
  image-link: string
  installment:
    amount:
      currency: string
      value: string
    months: string
  is-bundle: boolean
  item-group-id: string
  kind: string
  link: string
  loyalty-points:
    name: string
    points-value: string
    ratio: number
  material: string
  max-energy-efficiency-class: string
  max-handling-time: string
  min-energy-efficiency-class: string
  min-handling-time: string
  mobile-link: string
  mpn: string
  multipack: string
  offer-id: string
  online-only: boolean
  pattern: string
  price:
    currency: string
    value: string
  product-type: string
  promotion-ids: [string]
  sale-price:
    currency: string
    value: string
  sale-price-effective-date: string
  sell-on-google-quantity: string
  shipping-height:
    unit: string
    value: number
  shipping-label: string
  shipping-length:
    unit: string
    value: number
  shipping-weight:
    unit: string
    value: number
  shipping-width:
    unit: string
    value: number
  size-system: string
  size-type: string
  sizes: [string]
  source: string
  target-country: int64
  title: string
  unit-pricing-base-measure:
    unit: string
    value: string
  unit-pricing-measure:
    unit: string
    value: number
  validated-destinations: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    additional-image-links=et`
    - Additional URLs of images of the item.
    - Each invocation of this argument appends the given value to the array.
* `additional-product-types=clita`
    - Additional categories of the item (formatted as in products feed specification).
    - Each invocation of this argument appends the given value to the array.
* `adult=false`
    - Set to true if the item is targeted towards adults.
* `adwords-grouping=dolores`
    - Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise.
* `adwords-labels=clita`
    - Similar to adwords_grouping, but only works on CPC.
    - Each invocation of this argument appends the given value to the array.
* `adwords-redirect=eos`
    - Allows advertisers to override the item URL when the product is shown within the context of Product Ads.
* `age-group=amet`
    - Target age group of the item.
* `availability=sed`
    - Availability status of the item.
* `availability-date=takimata`
    - The day a pre-ordered product becomes available for delivery, in ISO 8601 format.
* `brand=sit`
    - Brand of the item.
* `channel=labore`
    - The item&#39;s channel (online or local).
* `color=nonumy`
    - Color of the item.
* `condition=erat`
    - Condition or state of the item.
* `content-language=gubergren`
    - The two-letter ISO 639-1 language code for the item.
* `cost-of-goods-sold    currency=erat`
    - The currency of the price.
* `value=et`
    - The price represented as a number.

* `..    custom-label0=amet`
    - Custom label 0 for custom grouping of items in a Shopping campaign.
* `custom-label1=lorem`
    - Custom label 1 for custom grouping of items in a Shopping campaign.
* `custom-label2=voluptua.`
    - Custom label 2 for custom grouping of items in a Shopping campaign.
* `custom-label3=rebum.`
    - Custom label 3 for custom grouping of items in a Shopping campaign.
* `custom-label4=justo`
    - Custom label 4 for custom grouping of items in a Shopping campaign.
* `description=labore`
    - Description of the item.
* `display-ads-id=voluptua.`
    - An identifier for an item for dynamic remarketing campaigns.
* `display-ads-link=takimata`
    - URL directly to your item&#39;s landing page for dynamic remarketing campaigns.
* `display-ads-similar-ids=dolor`
    - Advertiser-specified recommendations.
    - Each invocation of this argument appends the given value to the array.
* `display-ads-title=takimata`
    - Title of an item for dynamic remarketing campaigns.
* `display-ads-value=0.730502575286`
    - Offer margin for dynamic remarketing campaigns.
* `energy-efficiency-class=no`
    - The energy efficiency class as defined in EU directive 2010/30/EU.
* `expiration-date=aliquyam`
    - Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in productstatuses as googleExpirationDate and might be earlier if expirationDate is too far in the future.
* `gender=magna`
    - Target gender of the item.
* `google-product-category=et`
    - Google&#39;s category of the item (see Google product taxonomy).
* `gtin=sed`
    - Global Trade Item Number (GTIN) of the item.
* `id=est`
    - The REST id of the product. Content API methods that operate on products take this as their productId parameter.
        The REST id for a product is of the form channel:contentLanguage:targetCountry:offerId.
* `identifier-exists=true`
    - False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada.
* `image-link=et`
    - URL of an image of the item.
* `installment.amount    currency=consetetur`
    - The currency of the price.
* `value=sea`
    - The price represented as a number.

* `..    months=nonumy`
    - The number of installments the buyer has to pay.

* `..    is-bundle=true`
    - Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price.
* `item-group-id=accusam`
    - Shared identifier for all variants of the same product.
* `kind=clita`
    - Identifies what kind of resource this is. Value: the fixed string &#34;content#product&#34;.
* `link=sea`
    - URL directly linking to your item&#39;s page on your website.
* `loyalty-points    name=vero`
    - Name of loyalty points program. It is recommended to limit the name to 12 full-width characters or 24 Roman characters.
* `points-value=dolores`
    - The retailer&#39;s loyalty points in absolute value.
* `ratio=0.688763581879`
    - The ratio of a point when converted to currency. Google assumes currency based on Merchant Center settings. If ratio is left out, it defaults to 1.0.

* `..    material=ut`
    - The material of which the item is made.
* `max-energy-efficiency-class=amet`
    - The energy efficiency class as defined in EU directive 2010/30/EU.
* `max-handling-time=sed`
    - Maximal product handling time (in business days).
* `min-energy-efficiency-class=sit`
    - The energy efficiency class as defined in EU directive 2010/30/EU.
* `min-handling-time=sit`
    - Minimal product handling time (in business days).
* `mobile-link=dolores`
    - Link to a mobile-optimized version of the landing page.
* `mpn=et`
    - Manufacturer Part Number (MPN) of the item.
* `multipack=sanctus`
    - The number of identical products in a merchant-defined multipack.
* `offer-id=takimata`
    - A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details.
        Note: Content API methods that operate on products take the REST id of the product, not this identifier.
* `online-only=true`
    - Deprecated. Whether an item is available for purchase only online.
* `pattern=ut`
    - The item&#39;s pattern (e.g. polka dots).
* `price    currency=sadipscing`
    - The currency of the price.
* `value=et`
    - The price represented as a number.

* `..    product-type=clita`
    - Your category of the item (formatted as in products feed specification).
* `promotion-ids=ipsum`
    - The unique ID of a promotion.
    - Each invocation of this argument appends the given value to the array.
* `sale-price    currency=dolor`
    - The currency of the price.
* `value=elitr`
    - The price represented as a number.

* `..    sale-price-effective-date=magna`
    - Date range during which the item is on sale (see products feed specification).
* `sell-on-google-quantity=aliquyam`
    - The quantity of the product that is available for selling on Google. Supported only for online products.
* `shipping-height    unit=kasd`
    - The unit of value.
        
        Acceptable values are:  
        - &#34;cm&#34; 
        - &#34;in&#34;
* `value=0.310373270569`
    - The dimension of the product used to calculate the shipping cost of the item.

* `..    shipping-label=et`
    - The shipping label of the product, used to group product in account-level shipping rules.
* `shipping-length    unit=sit`
    - The unit of value.
        
        Acceptable values are:  
        - &#34;cm&#34; 
        - &#34;in&#34;
* `value=0.613007959864`
    - The dimension of the product used to calculate the shipping cost of the item.

* `..shipping-weight    unit=no`
    - The unit of value.
* `value=0.452909313083`
    - The weight of the product used to calculate the shipping cost of the item.

* `..shipping-width    unit=sed`
    - The unit of value.
        
        Acceptable values are:  
        - &#34;cm&#34; 
        - &#34;in&#34;
* `value=0.12771503919`
    - The dimension of the product used to calculate the shipping cost of the item.

* `..    size-system=clita`
    - System in which the size is specified. Recommended for apparel items.
* `size-type=kasd`
    - The cut of the item. Recommended for apparel items.
* `sizes=elitr`
    - Size of the item.
    - Each invocation of this argument appends the given value to the array.
* `source=et`
    - The source of the offer, i.e., how the offer was created.
* `target-country=-76`
    - The CLDR territory code for the item.
* `title=sadipscing`
    - Title of the item.
* `unit-pricing-base-measure    unit=sed`
    - The unit of the denominator.
* `value=vero`
    - The denominator of the unit price.

* `..unit-pricing-measure    unit=nonumy`
    - The unit of the measure.
* `value=0.185693294784`
    - The measure of an item.

* `..    validated-destinations=ea`
    - Deprecated. The read-only list of intended destinations which passed validation.
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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p dry-run=boolean**
    - Flag to run the request in dry-run mode.

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
