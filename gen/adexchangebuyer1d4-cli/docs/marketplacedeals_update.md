Replaces all the deals in the proposal with the passed in deals
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer1d4 --scope <scope> marketplacedeals update ...`
# Required Scalar Argument
* **&lt;proposal-id&gt;** *(string)*
    - The proposalId to edit deals on.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
EditAllOrderDealsRequest:
  proposal:
    billed-buyer:
      account-id: string
    buyer:
      account-id: string
    buyer-private-data:
      reference-id: string
      reference-payload: string
    dbm-advertiser-ids: [string]
    has-buyer-signed-off: boolean
    has-seller-signed-off: boolean
    inventory-source: string
    is-renegotiating: boolean
    is-setup-complete: boolean
    kind: string
    last-updater-or-commentor-role: string
    name: string
    negotiation-id: string
    originator-role: string
    private-auction-id: string
    proposal-id: string
    proposal-state: string
    revision-number: string
    revision-time-ms: string
    seller:
      account-id: string
      sub-account-id: string
  proposal-revision-number: string
  update-action: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .proposal.billed-buyer    account-id=consetetur`
    - Adx account id of the buyer.

* `..buyer    account-id=dolore`
    - Adx account id of the buyer.

* `..buyer-private-data    reference-id=duo`
    - No description provided.
* `reference-payload=aliquyam`
    - No description provided.

* `..    dbm-advertiser-ids=lorem`
    - IDs of DBM advertisers permission to this proposal.
    - Each invocation of this argument appends the given value to the array.
* `has-buyer-signed-off=true`
    - When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly)
* `has-seller-signed-off=true`
    - When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly)
* `inventory-source=consetetur`
    - What exchange will provide this inventory (readonly, except on create).
* `is-renegotiating=false`
    - True if the proposal is being renegotiated (readonly).
* `is-setup-complete=true`
    - True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag.
* `kind=kasd`
    - Identifies what kind of resource this is. Value: the fixed string &#34;adexchangebuyer#proposal&#34;.
* `last-updater-or-commentor-role=sanctus`
    - The role of the last user that either updated the proposal or left a comment. (readonly)
* `name=takimata`
    - The name for the proposal (updatable)
* `negotiation-id=at`
    - Optional negotiation id if this proposal is a preferred deal proposal.
* `originator-role=labore`
    - Indicates whether the buyer/seller created the proposal.(readonly)
* `private-auction-id=invidunt`
    - Optional private auction id if this proposal is a private auction proposal.
* `proposal-id=ea`
    - The unique id of the proposal. (readonly).
* `proposal-state=sadipscing`
    - The current state of the proposal. (readonly)
* `revision-number=rebum.`
    - The revision number for the proposal (readonly).
* `revision-time-ms=dolore`
    - The time (ms since epoch) when the proposal was last revised (readonly).
* `seller    account-id=nonumy`
    - The unique id for the seller. The seller fills in this field. The seller account id is then available to buyer in the product.
* `sub-account-id=sed`
    - Optional sub-account id for the seller.


* `...    proposal-revision-number=aliquyam`
    - The last known revision number for the proposal.
* `update-action=sit`
    - Indicates an optional action to take on the proposal


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
