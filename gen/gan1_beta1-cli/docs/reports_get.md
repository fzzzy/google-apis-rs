Retrieves a report of the specified type.
# Required Scalar Arguments
* **&lt;role&gt;** *(string)*
    - The role of the requester. Valid values: &#39;advertisers&#39; or &#39;publishers&#39;.
* **&lt;role-id&gt;** *(string)*
    - The ID of the requesting advertiser or publisher.
* **&lt;report-type&gt;** *(string)*
    - The type of report being requested. Valid values: &#39;order_delta&#39;. Required.

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

* **-p advertiser-id=string**
    - The IDs of the advertisers to look up, if applicable.

* **-p max-results=integer**
    - Max number of items to return in this page. Optional. Defaults to return all results.

* **-p start-index=integer**
    - Offset on which to return results when paging. Optional.

* **-p publisher-id=string**
    - The IDs of the publishers to look up, if applicable.

* **-p status=string**
    - Filters out all events that do not have the given status. Valid values: &#39;active&#39;, &#39;canceled&#39;, or &#39;invalid&#39;. Optional.

* **-p event-type=string**
    - Filters out all events that are not of the given type. Valid values: &#39;action&#39;, &#39;transaction&#39;, or &#39;charge&#39;. Optional.

* **-p calculate-totals=boolean**
    - Whether or not to calculate totals rows. Optional.

* **-p link-id=string**
    - Filters to capture one of given link IDs. Optional.

* **-p order-id=string**
    - Filters to capture one of the given order IDs. Optional.

* **-p start-date=string**
    - The start date (inclusive), in RFC 3339 format, for the report data to be returned. Defaults to one day before endDate, if that is given, or yesterday. Optional.

* **-p end-date=string**
    - The end date (exclusive), in RFC 3339 format, for the report data to be returned. Defaults to one day after startDate, if that is given, or today. Optional.

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
