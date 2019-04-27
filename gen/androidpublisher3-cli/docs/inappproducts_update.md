Updates the details of an in-app product.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `androidpublisher3 --scope <scope> inappproducts update ...`
# Required Scalar Arguments
* **&lt;package-name&gt;** *(string)*
    - Unique identifier for the Android app with the in-app product; for example, &#34;com.spiffygame&#34;.
* **&lt;sku&gt;** *(string)*
    - Unique identifier for the in-app product.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
InAppProduct:
  default-language: string
  default-price:
    currency: string
    price-micros: string
  grace-period: string
  package-name: string
  purchase-type: string
  season:
    end:
      day: integer
      month: integer
    start:
      day: integer
      month: integer
  sku: string
  status: string
  subscription-period: string
  trial-period: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    default-language=vero`
    - The default language of the localized data, as defined by BCP 47. e.g. &#34;en-US&#34;, &#34;en-GB&#34;.
* `default-price    currency=diam`
    - 3 letter Currency code, as defined by ISO 4217.
* `price-micros=rebum.`
    - The price in millionths of the currency base unit represented as a string.

* `..    grace-period=consetetur`
    - Grace period of the subscription, specified in ISO 8601 format. It will allow developers to give their subscribers a grace period when the payment for the new recurrence period is declined. Acceptable values = &#34;P3D&#34; (three days) and &#34;P7D&#34; (seven days)
* `package-name=sadipscing`
    - The package name of the parent app.
* `purchase-type=vero`
    - Purchase type enum value. Unmodifiable after creation.
* `season.end    day=6`
    - Day of a month, value in [1, 31] range. Valid range depends on the specified month.
* `month=13`
    - Month of a year. e.g. 1 = JAN, 2 = FEB etc.

* `..start    day=5`
    - Day of a month, value in [1, 31] range. Valid range depends on the specified month.
* `month=17`
    - Month of a year. e.g. 1 = JAN, 2 = FEB etc.


* `...    sku=duo`
    - The stock-keeping-unit (SKU) of the product, unique within an app.
* `status=aliquyam`
    - No description provided.
* `subscription-period=lorem`
    - Subscription period, specified in ISO 8601 format. Acceptable values are &#34;P1W&#34; (one week), &#34;P1M&#34; (one month), &#34;P3M&#34; (three months), &#34;P6M&#34; (six months), and &#34;P1Y&#34; (one year).
* `trial-period=et`
    - Trial period, specified in ISO 8601 format. Acceptable values are anything between &#34;P7D&#34; (seven days) and &#34;P999D&#34; (999 days). Seasonal subscriptions cannot have a trial period.


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

* **-p auto-convert-missing-prices=boolean**
    - If true the prices for all regions targeted by the parent app that don&#39;t have a price specified for this in-app product will be auto converted to the target currency based on the default price. Defaults to false.

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
