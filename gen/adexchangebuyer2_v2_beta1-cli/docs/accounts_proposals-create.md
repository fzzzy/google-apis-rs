Create the given proposal. Each created proposal and any deals it contains
are assigned a unique ID by the server.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> accounts proposals-create ...`
# Required Scalar Argument
* **&lt;account-id&gt;** *(string)*
    - Account ID of the buyer.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Proposal:
  billed-buyer:
    account-id: string
  buyer:
    account-id: string
  buyer-private-data:
    reference-id: string
  display-name: string
  is-renegotiating: boolean
  is-setup-complete: boolean
  last-updater-or-commentor-role: string
  originator-role: string
  private-auction-id: string
  proposal-id: string
  proposal-revision: string
  proposal-state: string
  seller:
    account-id: string
    sub-account-id: string
  update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .billed-buyer    account-id=dolores`
    - Authorized Buyers account ID of the buyer.

* `..buyer    account-id=eos`
    - Authorized Buyers account ID of the buyer.

* `..buyer-private-data    reference-id=voluptua.`
    - A buyer or seller specified reference ID. This can be queried in the list
        operations (max-length: 1024 unicode code units).

* `..    display-name=duo`
    - The name for the proposal.
* `is-renegotiating=false`
    - True if the proposal is being renegotiated.
        @OutputOnly
* `is-setup-complete=true`
    - True, if the buyside inventory setup is complete for this proposal.
        @OutputOnly
* `last-updater-or-commentor-role=ea`
    - The role of the last user that either updated the proposal or left a
        comment.
        @OutputOnly
* `originator-role=ea`
    - Indicates whether the buyer/seller created the proposal.
        @OutputOnly
* `private-auction-id=et`
    - Private auction ID if this proposal is a private auction proposal.
        @OutputOnly
* `proposal-id=dolor`
    - The unique ID of the proposal.
        @OutputOnly
* `proposal-revision=diam`
    - The revision number for the proposal.
        Each update to the proposal or the deal causes the proposal revision number
        to auto-increment. The buyer keeps track of the last revision number they
        know of and pass it in when making an update. If the head revision number
        on the server has since incremented, then an ABORTED error is returned
        during the update operation to let the buyer know that a subsequent update
        was made.
        @OutputOnly
* `proposal-state=kasd`
    - The current state of the proposal.
        @OutputOnly
* `seller    account-id=invidunt`
    - The unique ID for the seller. The seller fills in this field.
        The seller account ID is then available to buyer in the product.
* `sub-account-id=rebum.`
    - Optional sub-account ID for the seller.

* `..    update-time=lorem`
    - The time when the proposal was last revised.
        @OutputOnly


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
