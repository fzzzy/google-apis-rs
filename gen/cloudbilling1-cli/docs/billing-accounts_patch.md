Updates a billing account&#39;s fields.
Currently the only field that can be edited is `display_name`.
The current authenticated user must have the `billing.accounts.update`
IAM permission, which is typically given to the
[administrator](https://cloud.google.com/billing/docs/how-to/billing-access)
of the billing account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudbilling1 --scope <scope> billing-accounts patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the billing account resource to be updated.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
BillingAccount:
  display-name: string
  master-billing-account: string
  name: string
  open: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    display-name=et`
    - The display name given to the billing account, such as `My Billing
        Account`. This name is displayed in the GCP Console.
* `master-billing-account=dolores`
    - If this account is a
        [subaccount](https://cloud.google.com/billing/docs/concepts), then this
        will be the resource name of the master billing account that it is being
        resold through.
        Otherwise this will be empty.
* `name=kasd`
    - The resource name of the billing account. The resource name has the form
        `billingAccounts/{billing_account_id}`. For example,
        `billingAccounts/012345-567890-ABCDEF` would be the resource name for
        billing account `012345-567890-ABCDEF`.
* `open=true`
    - True if the billing account is open, and will therefore be charged for any
        usage on associated projects. False if the billing account is closed, and
        therefore projects associated with it will be unable to use paid services.


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

* **-p update-mask=string**
    - The update mask applied to the resource.
        Only &#34;display_name&#34; is currently supported.

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
