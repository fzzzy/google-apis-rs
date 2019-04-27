Creates a customer for zero-touch enrollment. After the method returns
successfully, admin and owner roles can manage devices and EMM configs
by calling API methods or using their zero-touch enrollment portal. The API
doesn&#39;t notify the customer that they have access.
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The parent resource ID in the format `partners/[PARTNER_ID]` that
        identifies the reseller.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateCustomerRequest:
  customer:
    admin-emails: [string]
    company-id: string
    company-name: string
    name: string
    owner-emails: [string]
    terms-status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .customer    admin-emails=amet`
    - Input only. Optional. Email address of customer&#39;s users in the admin role.
        Each email address must be associated with a Google Account.
    - Each invocation of this argument appends the given value to the array.
* `company-id=no`
    - Output only. The ID of the company. Assigned by the server.
* `company-name=labore`
    - Required. The name of the company. For example _XYZ Corp_. Displayed to the
        company&#39;s employees in the zero-touch enrollment portal.
* `name=eirmod`
    - Output only. The API resource name of the company. The resource name is one
        of the following formats:
        
        * `partners/[PARTNER_ID]/customers/[CUSTOMER_ID]`
        * `partners/[PARTNER_ID]/vendors/[VENDOR_ID]`
        * `partners/[PARTNER_ID]/vendors/[VENDOR_ID]/customers/[CUSTOMER_ID]`
        
        Assigned by the server.
* `owner-emails=dolore`
    - Input only. Email address of customer&#39;s users in the owner role. At least
        one `owner_email` is required. Each email address must be associated with a
        Google Account. Owners share the same access as admins but can also add,
        delete, and edit your organization&#39;s portal users.
    - Each invocation of this argument appends the given value to the array.
* `terms-status=invidunt`
    - Output only. Whether any user from the company has accepted the latest
        Terms of Service (ToS). See
        TermsStatus.



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
