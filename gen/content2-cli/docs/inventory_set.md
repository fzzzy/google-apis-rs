Updates price and availability of a product in your Merchant Center account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> inventory set ...`
# Required Scalar Arguments
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the account that contains the product. This account cannot be a multi-client account.
* **&lt;store-code&gt;** *(string)*
    - The code of the store for which to update price and availability. Use online to update price and availability of an online product.
* **&lt;product-id&gt;** *(string)*
    - The REST id of the product for which to update price and availability.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
InventorySetRequest:
  availability: string
  installment:
    amount:
      currency: string
      value: string
    months: string
  loyalty-points:
    name: string
    points-value: string
    ratio: number
  pickup:
    pickup-method: string
    pickup-sla: string
  price:
    currency: string
    value: string
  quantity: integer
  sale-price:
    currency: string
    value: string
  sale-price-effective-date: string
  sell-on-google-quantity: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    availability=accusam`
    - The availability of the product.
* `installment.amount    currency=clita`
    - The currency of the price.
* `value=diam`
    - The price represented as a number.

* `..    months=justo`
    - The number of installments the buyer has to pay.

* `..loyalty-points    name=est`
    - Name of loyalty points program. It is recommended to limit the name to 12 full-width characters or 24 Roman characters.
* `points-value=clita`
    - The retailer&#39;s loyalty points in absolute value.
* `ratio=0.636549512895`
    - The ratio of a point when converted to currency. Google assumes currency based on Merchant Center settings. If ratio is left out, it defaults to 1.0.

* `..pickup    pickup-method=ut`
    - Whether store pickup is available for this offer and whether the pickup option should be shown as buy, reserve, or not supported. Only supported for local inventory. Unless the value is &#34;not supported&#34;, must be submitted together with pickupSla.
* `pickup-sla=dolores`
    - The expected date that an order will be ready for pickup, relative to when the order is placed. Only supported for local inventory. Must be submitted together with pickupMethod.

* `..price    currency=eos`
    - The currency of the price.
* `value=voluptua.`
    - The price represented as a number.

* `..    quantity=82`
    - The quantity of the product. Must be equal to or greater than zero. Supported only for local products.
* `sale-price    currency=sed`
    - The currency of the price.
* `value=aliquyam`
    - The price represented as a number.

* `..    sale-price-effective-date=ea`
    - A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as &#39;null&#39; if undecided.
* `sell-on-google-quantity=84`
    - The quantity of the product that is available for selling on Google. Supported only for online products.


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
