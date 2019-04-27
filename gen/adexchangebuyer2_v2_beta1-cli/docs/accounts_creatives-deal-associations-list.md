List all creative-deal associations.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> accounts creatives-deal-associations-list ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - The account to list the associations from.
        Specify &#34;-&#34; to list all creatives the current user has access to.
* **&lt;creative-id&gt;** *(string)*
    - The creative ID to list the associations from.
        Specify &#34;-&#34; to list all creatives under the above account.

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

* **-p query=string**
    - An optional query string to filter deal associations. If no filter is
        specified, all associations will be returned.
        Supported queries are:
        &lt;ul&gt;
        &lt;li&gt;accountId=&lt;i&gt;account_id_string&lt;/i&gt;
        &lt;li&gt;creativeId=&lt;i&gt;creative_id_string&lt;/i&gt;
        &lt;li&gt;dealsId=&lt;i&gt;deals_id_string&lt;/i&gt;
        &lt;li&gt;dealsStatus:{approved, conditionally_approved, disapproved,
                          not_checked}
        &lt;li&gt;openAuctionStatus:{approved, conditionally_approved, disapproved,
                                 not_checked}
        &lt;/ul&gt;
        Example: &#39;dealsId=12345 AND dealsStatus:disapproved&#39;

* **-p page-token=string**
    - A token identifying a page of results the server should return.
        Typically, this is the value of
        ListDealAssociationsResponse.next_page_token
        returned from the previous call to &#39;ListDealAssociations&#39; method.

* **-p page-size=integer**
    - Requested page size. Server may return fewer associations than requested.
        If unspecified, server will pick an appropriate default.

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
