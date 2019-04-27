Creates a new configuration. Once created, a customer can apply the
configuration to devices.
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The customer that manages the configuration. An API resource name
        in the format `customers/[CUSTOMER_ID]`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Configuration:
  company-name: string
  configuration-id: string
  configuration-name: string
  contact-email: string
  contact-phone: string
  custom-message: string
  dpc-extras: string
  dpc-resource-path: string
  is-default: boolean
  name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    company-name=eirmod`
    - Required. The name of the organization. Zero-touch enrollment shows this
        organization name to device users during device provisioning.
* `configuration-id=sit`
    - Output only. The ID of the configuration. Assigned by the server.
* `configuration-name=stet`
    - Required. A short name that describes the configuration&#39;s purpose. For
        example, _Sales team_ or _Temporary employees_. The zero-touch enrollment
        portal displays this name to IT admins.
* `contact-email=sed`
    - Required. The email address that device users can contact to get help.
        Zero-touch enrollment shows this email address to device users before
        device provisioning. The value is validated on input.
* `contact-phone=et`
    - Required. The telephone number that device users can call, using another
        device, to get help. Zero-touch enrollment shows this number to device
        users before device provisioning. Accepts numerals, spaces, the plus sign,
        hyphens, and parentheses.
* `custom-message=dolores`
    - A message, containing one or two sentences, to help device users get help
        or give them more details about what’s happening to their device.
        Zero-touch enrollment shows this message before the device is provisioned.
* `dpc-extras=kasd`
    - The JSON-formatted EMM provisioning extras that are passed to the DPC.
* `dpc-resource-path=accusam`
    - Required. The resource name of the selected DPC (device policy controller)
        in the format `customers/[CUSTOMER_ID]/dpcs/*`. To list the supported DPCs,
        call
        `customers.dpcs.list`.
* `is-default=true`
    - Required. Whether this is the default configuration that zero-touch
        enrollment applies to any new devices the organization purchases in the
        future. Only one customer configuration can be the default. Setting this
        value to `true`, changes the previous default configuration&#39;s `isDefault`
        value to `false`.
* `name=justo`
    - Output only. The API resource name in the format
        `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. Assigned by
        the server.


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
