Retrieves data about all advertisers that the requesting advertiser/publisher has access to.
# Required Scalar Arguments
* **&lt;role&gt;** *(string)*
    - The role of the requester. Valid values: &#39;advertisers&#39; or &#39;publishers&#39;.
* **&lt;role-id&gt;** *(string)*
    - The ID of the requesting advertiser or publisher.

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

* **-p relationship-status=string**
    - Filters out all advertisers for which do not have the given relationship status with the requesting publisher.

* **-p min-seven-day-epc=number**
    - Filters out all advertisers that have a seven day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.

* **-p advertiser-category=string**
    - Caret(^) delimted list of advertiser categories. Valid categories are defined here: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&amp;answer=107581. Filters out all advertisers not in one of the given advertiser categories. Optional.

* **-p min-ninety-day-epc=number**
    - Filters out all advertisers that have a ninety day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.

* **-p page-token=string**
    - The value of &#39;nextPageToken&#39; from the previous page. Optional.

* **-p max-results=integer**
    - Max number of items to return in this page. Optional. Defaults to 20.

* **-p min-payout-rank=integer**
    - A value between 1 and 4, where 1 represents the quartile of advertisers with the lowest ranks and 4 represents the quartile of advertisers with the highest ranks. Filters out all advertisers with a lower rank than the given quartile. For example if a 2 was given only advertisers with a payout rank of 25 or higher would be included. Optional.

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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
