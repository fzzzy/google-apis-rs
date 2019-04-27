Update the given proposal
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer1d4 --scope <scope> proposals update ...`
# Required Scalar Arguments
* **&lt;proposal-id&gt;** *(string)*
    - The proposal id to update.
* **&lt;revision-number&gt;** *(string)*
    - The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the latest proposal at head revision and retry the update at that revision.
* **&lt;update-action&gt;** *(string)*
    - The proposed action to take on the proposal. This field is required and it must be set when updating a proposal.
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

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .billed-buyer    account-id=dolores`
    - Adx account id of the buyer.

* `..buyer    account-id=consetetur`
    - Adx account id of the buyer.

* `..buyer-private-data    reference-id=dolor`
    - No description provided.
* `reference-payload=aliquyam`
    - No description provided.

* `..    dbm-advertiser-ids=elitr`
    - IDs of DBM advertisers permission to this proposal.
    - Each invocation of this argument appends the given value to the array.
* `has-buyer-signed-off=true`
    - When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly)
* `has-seller-signed-off=true`
    - When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly)
* `inventory-source=stet`
    - What exchange will provide this inventory (readonly, except on create).
* `is-renegotiating=true`
    - True if the proposal is being renegotiated (readonly).
* `is-setup-complete=false`
    - True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag.
* `kind=sanctus`
    - Identifies what kind of resource this is. Value: the fixed string &#34;adexchangebuyer#proposal&#34;.
* `last-updater-or-commentor-role=dolore`
    - The role of the last user that either updated the proposal or left a comment. (readonly)
* `name=lorem`
    - The name for the proposal (updatable)
* `negotiation-id=consetetur`
    - Optional negotiation id if this proposal is a preferred deal proposal.
* `originator-role=consetetur`
    - Indicates whether the buyer/seller created the proposal.(readonly)
* `private-auction-id=eirmod`
    - Optional private auction id if this proposal is a private auction proposal.
* `proposal-id=labore`
    - The unique id of the proposal. (readonly).
* `proposal-state=gubergren`
    - The current state of the proposal. (readonly)
* `revision-number=et`
    - The revision number for the proposal (readonly).
* `revision-time-ms=sadipscing`
    - The time (ms since epoch) when the proposal was last revised (readonly).
* `seller    account-id=accusam`
    - The unique id for the seller. The seller fills in this field. The seller account id is then available to buyer in the product.
* `sub-account-id=magna`
    - Optional sub-account id for the seller.



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
