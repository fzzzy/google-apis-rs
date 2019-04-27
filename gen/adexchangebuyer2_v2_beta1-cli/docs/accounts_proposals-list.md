List proposals. A filter expression (PQL query) may be specified to
filter the results. To retrieve all finalized proposals, regardless if a
proposal is being renegotiated, see the FinalizedProposals resource.
Note that Bidder/ChildSeat relationships differ from the usual behavior.
A Bidder account can only see its child seats&#39; proposals by specifying
the ChildSeat&#39;s accountId in the request path.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> accounts proposals-list ...`
# Required Scalar Argument
* **&lt;account-id&gt;** *(string)*
    - Account ID of the buyer.

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

* **-p filter-syntax=string**
    - Syntax the filter is written in. Current implementation defaults to PQL
        but in the future it will be LIST_FILTER.

* **-p filter=string**
    - An optional PQL filter query used to query for proposals.
        
        Nested repeated fields, such as proposal.deals.targetingCriterion,
        cannot be filtered.

* **-p page-token=string**
    - The page token as returned from ListProposalsResponse.

* **-p page-size=integer**
    - Requested page size. The server may return fewer results than requested.
        If unspecified, the server will pick an appropriate default.

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
