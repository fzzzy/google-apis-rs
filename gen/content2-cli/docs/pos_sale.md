Submit a sale event for the given merchant.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> pos sale ...`
# Required Scalar Arguments
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the POS or inventory data provider.
* **&lt;target-merchant-id&gt;** *(string)*
    - The ID of the target merchant.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PosSaleRequest:
  content-language: string
  gtin: string
  item-id: string
  price:
    currency: string
    value: string
  quantity: string
  sale-id: string
  store-code: string
  target-country: int64
  timestamp: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    content-language=et`
    - The two-letter ISO 639-1 language code for the item.
* `gtin=at`
    - Global Trade Item Number.
* `item-id=sit`
    - A unique identifier for the item.
* `price    currency=ut`
    - The currency of the price.
* `value=diam`
    - The price represented as a number.

* `..    quantity=tempor`
    - The relative change of the available quantity. Negative for items returned.
* `sale-id=et`
    - A unique ID to group items from the same sale event.
* `store-code=erat`
    - The identifier of the merchant&#39;s store. Either a storeCode inserted via the API or the code of the store in Google My Business.
* `target-country=-18`
    - The CLDR territory code for the item.
* `timestamp=kasd`
    - The inventory timestamp, in ISO 8601 format.


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