Creates a Merchant Center sub-account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> accounts insert ...`
# Required Scalar Argument
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the managing account. This must be a multi-client account.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Account:
  adult-content: boolean
  business-information:
    address:
      country: string
      locality: string
      postal-code: string
      region: string
      street-address: string
    customer-service:
      email: string
      phone-number: string
      url: string
    phone-number: string
  google-my-business-link:
    gmb-email: string
    status: string
  id: string
  kind: string
  name: string
  reviews-url: string
  seller-id: string
  website-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    adult-content=true`
    - Indicates whether the merchant sells adult content.
* `business-information.address    country=sit`
    - CLDR country code (e.g. &#34;US&#34;).
* `locality=stet`
    - City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs).
* `postal-code=sed`
    - Postal code or ZIP (e.g. &#34;94043&#34;).
* `region=et`
    - Top-level administrative subdivision of the country. For example, a state like California (&#34;CA&#34;) or a province like Quebec (&#34;QC&#34;).
* `street-address=dolores`
    - Street-level part of the address.

* `..customer-service    email=kasd`
    - Customer service email.
* `phone-number=accusam`
    - Customer service phone number.
* `url=takimata`
    - Customer service URL.

* `..    phone-number=justo`
    - The phone number of the business.

* `..google-my-business-link    gmb-email=amet.`
    - The GMB email address of which a specific account within a GMB account. A sample account within a GMB account could be a business account with set of locations, managed under the GMB account.
* `status=erat`
    - Status of the link between this Merchant Center account and the GMB account.

* `..    id=labore`
    - Merchant Center account ID.
* `kind=sea`
    - Identifies what kind of resource this is. Value: the fixed string &#34;content#account&#34;.
* `name=nonumy`
    - Display name for the account.
* `reviews-url=dolores`
    - [DEPRECATED] This field is never returned and will be ignored if provided.
* `seller-id=gubergren`
    - Client-specific, locally-unique, internal ID for the child account.
* `website-url=sadipscing`
    - The merchant&#39;s website.


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
