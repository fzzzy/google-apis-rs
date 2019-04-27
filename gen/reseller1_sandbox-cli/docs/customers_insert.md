Creates a customer resource if one does not already exist.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/apps.order* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/apps.order*.
You can set the scope for this method like this: `reseller1-sandbox --scope <scope> customers insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Customer:
  alternate-email: string
  customer-domain: string
  customer-domain-verified: boolean
  customer-id: string
  kind: string
  phone-number: string
  postal-address:
    address-line1: string
    address-line2: string
    address-line3: string
    contact-name: string
    country-code: string
    kind: string
    locality: string
    organization-name: string
    postal-code: string
    region: string
  resource-ui-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-email=eirmod`
    - The alternate email of the customer.
* `customer-domain=sit`
    - The domain name of the customer.
* `customer-domain-verified=false`
    - Whether the customer&#39;s primary domain has been verified.
* `customer-id=sed`
    - The id of the customer.
* `kind=et`
    - Identifies the resource as a customer.
* `phone-number=dolores`
    - The phone number of the customer.
* `postal-address    address-line1=kasd`
    - Address line 1 of the address.
* `address-line2=accusam`
    - Address line 2 of the address.
* `address-line3=takimata`
    - Address line 3 of the address.
* `contact-name=justo`
    - Name of the contact person.
* `country-code=amet.`
    - ISO 3166 country code.
* `kind=erat`
    - Identifies the resource as a customer address.
* `locality=labore`
    - Name of the locality. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.
* `organization-name=sea`
    - Name of the organization.
* `postal-code=nonumy`
    - The postal code. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.
* `region=dolores`
    - Name of the region. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.

* `..    resource-ui-url=gubergren`
    - Ui url for customer resource.


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

* **-p customer-auth-token=string**
    - An auth token needed for inserting a customer for which domain already exists. Can be generated at https://admin.google.com/TransferToken. Optional.

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
