List all details associated with a specific reason for which bids were
filtered, with the number of bids filtered for each detail.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> bidders accounts-filter-sets-filtered-bids-details-list ...`
# Required Scalar Arguments
* **&lt;filter-set-name&gt;** *(string)*
    - Name of the filter set that should be applied to the requested metrics.
        For example:
        
        - For a bidder-level filter set for bidder 123:
          `bidders/123/filterSets/abc`
        
        - For an account-level filter set for the buyer account representing bidder
          123: `bidders/123/accounts/123/filterSets/abc`
        
        - For an account-level filter set for the child seat buyer account 456
          whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
* **&lt;creative-status-id&gt;** *(integer)*
    - The ID of the creative status for which to retrieve a breakdown by detail.
        See
        [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes).
        Details are only available for statuses 10, 14, 15, 17, 18, 19, 86, and 87.

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

* **-p page-size=integer**
    - Requested page size. The server may return fewer results than requested.
        If unspecified, the server will pick an appropriate default.

* **-p page-token=string**
    - A token identifying a page of results the server should return.
        Typically, this is the value of
        ListCreativeStatusBreakdownByDetailResponse.nextPageToken
        returned from the previous call to the filteredBids.details.list
        method.

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
