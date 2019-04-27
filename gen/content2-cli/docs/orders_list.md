Lists the orders in your Merchant Center account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> orders list ...`
# Required Scalar Argument
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the account that manages the order. This cannot be a multi-client account.

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

* **-p acknowledged=boolean**
    - Obtains orders that match the acknowledgement status. When set to true, obtains orders that have been acknowledged. When false, obtains orders that have not been acknowledged.
        We recommend using this filter set to false, in conjunction with the acknowledge call, such that only un-acknowledged orders are returned.

* **-p page-token=string**
    - The token returned by the previous request.

* **-p max-results=integer**
    - The maximum number of orders to return in the response, used for paging. The default value is 25 orders per page, and the maximum allowed value is 250 orders per page.
        Known issue: All List calls will return all Orders without limit regardless of the value of this field.

* **-p placed-date-start=string**
    - Obtains orders placed after this date (inclusively), in ISO 8601 format.

* **-p order-by=string**
    - The ordering of the returned list. The only supported value are placedDate desc and placedDate asc for now, which returns orders sorted by placement date. &#34;placedDate desc&#34; stands for listing orders by placement date, from oldest to most recent. &#34;placedDate asc&#34; stands for listing orders by placement date, from most recent to oldest. In future releases we&#39;ll support other sorting criteria.

* **-p statuses=string**
    - Obtains orders that match any of the specified statuses. Multiple values can be specified with comma separation. Additionally, please note that active is a shortcut for pendingShipment and partiallyShipped, and completed is a shortcut for shipped , partiallyDelivered, delivered, partiallyReturned, returned, and canceled.

* **-p placed-date-end=string**
    - Obtains orders placed before this date (exclusively), in ISO 8601 format.

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
