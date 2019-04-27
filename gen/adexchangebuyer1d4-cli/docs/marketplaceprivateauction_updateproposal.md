Update a given private auction proposal
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer1d4 --scope <scope> marketplaceprivateauction updateproposal ...`
# Required Scalar Argument
* **&lt;private-auction-id&gt;** *(string)*
    - The private auction id to be updated.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UpdatePrivateAuctionProposalRequest:
  external-deal-id: string
  note:
    creator-role: string
    deal-id: string
    kind: string
    note: string
    note-id: string
    proposal-id: string
    proposal-revision-number: string
    timestamp-ms: string
  proposal-revision-number: string
  update-action: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    external-deal-id=eirmod`
    - The externalDealId of the deal to be updated.
* `note    creator-role=consetetur`
    - The role of the person (buyer/seller) creating the note. (readonly)
* `deal-id=labore`
    - Notes can optionally be associated with a deal. (readonly, except on create)
* `kind=sed`
    - Identifies what kind of resource this is. Value: the fixed string &#34;adexchangebuyer#marketplaceNote&#34;.
* `note=ea`
    - The actual note to attach. (readonly, except on create)
* `note-id=gubergren`
    - The unique id for the note. (readonly)
* `proposal-id=aliquyam`
    - The proposalId that a note is attached to. (readonly)
* `proposal-revision-number=eos`
    - If the note is associated with a proposal revision number, then store that here. (readonly, except on create)
* `timestamp-ms=tempor`
    - The timestamp (ms since epoch) that this note was created. (readonly)

* `..    proposal-revision-number=sea`
    - The current revision number of the proposal to be updated.
* `update-action=labore`
    - The proposed action on the private auction proposal.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

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
