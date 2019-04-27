Sandbox only. Creates a test order.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> orders createtestorder ...`
# Required Scalar Argument
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the account that should manage the order. This cannot be a multi-client account.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
OrdersCreateTestOrderRequest:
  country: string
  template-name: string
  test-order:
    customer:
      email: string
      explicit-marketing-preference: boolean
      full-name: string
      marketing-rights-info:
        explicit-marketing-preference: string
        last-updated-timestamp: string
    enable-orderinvoices: boolean
    kind: string
    notification-mode: string
    payment-method:
      expiration-month: integer
      expiration-year: integer
      last-four-digits: string
      predefined-billing-address: string
      type: string
    predefined-delivery-address: string
    shipping-cost:
      currency: string
      value: string
    shipping-cost-tax:
      currency: string
      value: string
    shipping-option: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    country=sea`
    - The  CLDR territory code of the country of the test order to create. Affects the currency and addresses of orders created via template_name, or the addresses of orders created via test_order.
        
        Acceptable values are:  
        - &#34;US&#34; 
        - &#34;FR&#34;  Defaults to US.
* `template-name=elitr`
    - The test order template to use. Specify as an alternative to testOrder as a shortcut for retrieving a template and then creating an order using that template.
* `test-order.customer    email=at`
    - Deprecated.
* `explicit-marketing-preference=true`
    - Deprecated. Please use marketingRightsInfo instead.
* `full-name=consetetur`
    - Full name of the customer.
* `marketing-rights-info    explicit-marketing-preference=diam`
    - Last know user use selection regards marketing preferences. In certain cases selection might not be known, so this field would be empty.
* `last-updated-timestamp=accusam`
    - Timestamp when last time marketing preference was updated. Could be empty, if user wasn&#39;t offered a selection yet.


* `...    enable-orderinvoices=false`
    - Whether the orderinvoices service should support this order.
* `kind=consetetur`
    - Identifies what kind of resource this is. Value: the fixed string &#34;content#testOrder&#34;.
* `notification-mode=dolor`
    - Determines if test order must be pulled by merchant or pushed to merchant via push integration.
* `payment-method    expiration-month=19`
    - The card expiration month (January = 1, February = 2 etc.).
* `expiration-year=7`
    - The card expiration year (4-digit, e.g. 2015).
* `last-four-digits=ea`
    - The last four digits of the card number.
* `predefined-billing-address=et`
    - The billing address.
* `type=stet`
    - The type of instrument. Note that real orders might have different values than the four values accepted by createTestOrder.

* `..    predefined-delivery-address=sed`
    - Identifier of one of the predefined delivery addresses for the delivery.
* `shipping-cost    currency=dolor`
    - The currency of the price.
* `value=sanctus`
    - The price represented as a number.

* `..shipping-cost-tax    currency=dolore`
    - The currency of the price.
* `value=lorem`
    - The price represented as a number.

* `..    shipping-option=consetetur`
    - The requested shipping option.



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
