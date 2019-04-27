Updates a customer.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.customer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.customer*.
You can set the scope for this method like this: `admin1-directory --scope <scope> customers update ...`
# Required Scalar Argument
* **&lt;customer-key&gt;** *(string)*
    - Id of the customer to be updated
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Customer:
  alternate-email: string
  customer-creation-time: string
  customer-domain: string
  etag: string
  id: string
  kind: string
  language: string
  phone-number: string
  postal-address:
    address-line1: string
    address-line2: string
    address-line3: string
    contact-name: string
    country-code: string
    locality: string
    organization-name: string
    postal-code: string
    region: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-email=kasd`
    - The customer&#39;s secondary contact email address. This email address cannot be on the same domain as the customerDomain
* `customer-creation-time=sanctus`
    - The customer&#39;s creation time (Readonly)
* `customer-domain=takimata`
    - The customer&#39;s primary domain name string. Do not include the www prefix when creating a new customer.
* `etag=at`
    - ETag of the resource.
* `id=labore`
    - The unique ID for the customer&#39;s G Suite account. (Readonly)
* `kind=invidunt`
    - Identifies the resource as a customer. Value: admin#directory#customer
* `language=ea`
    - The customer&#39;s ISO 639-2 language code. The default value is en-US
* `phone-number=sadipscing`
    - The customer&#39;s contact phone number in E.164 format.
* `postal-address    address-line1=rebum.`
    - A customer&#39;s physical address. The address can be composed of one to three lines.
* `address-line2=dolore`
    - Address line 2 of the address.
* `address-line3=nonumy`
    - Address line 3 of the address.
* `contact-name=sed`
    - The customer contact&#39;s name.
* `country-code=aliquyam`
    - This is a required property. For countryCode information see the ISO 3166 country code elements.
* `locality=sit`
    - Name of the locality. An example of a locality value is the city of San Francisco.
* `organization-name=eirmod`
    - The company or company division name.
* `postal-code=consetetur`
    - The postal code. A postalCode example is a postal zip code such as 10009. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.
* `region=labore`
    - Name of the region. An example of a region value is NY for the state of New York.



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
