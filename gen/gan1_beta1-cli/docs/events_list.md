Retrieves event data for a given advertiser/publisher.
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

* **-p charge-type=string**
    - Filters out all charge events that are not of the given charge type. Valid values: &#39;other&#39;, &#39;slotting_fee&#39;, &#39;monthly_minimum&#39;, &#39;tier_bonus&#39;, &#39;credit&#39;, &#39;debit&#39;. Optional.

* **-p sku=string**
    - Caret(^) delimited list of SKUs. Filters out all events that do not reference one of the given SKU. Optional.

* **-p order-id=string**
    - Caret(^) delimited list of order IDs. Filters out all events that do not reference one of the given order IDs. Optional.

* **-p max-results=integer**
    - Max number of offers to return in this page. Optional. Defaults to 20.

* **-p status=string**
    - Filters out all events that do not have the given status. Valid values: &#39;active&#39;, &#39;canceled&#39;. Optional.

* **-p advertiser-id=string**
    - Caret(^) delimited list of advertiser IDs. Filters out all events that do not reference one of the given advertiser IDs. Only used when under publishers role. Optional.

* **-p event-date-min=string**
    - Filters out all events earlier than given date. Optional. Defaults to 24 hours from current date/time.

* **-p link-id=string**
    - Caret(^) delimited list of link IDs. Filters out all events that do not reference one of the given link IDs. Optional.

* **-p modify-date-max=string**
    - Filters out all events modified later than given date. Optional. Defaults to 24 hours after modifyDateMin, if modifyDateMin is explicitly set.

* **-p event-date-max=string**
    - Filters out all events later than given date. Optional. Defaults to 24 hours after eventMin.

* **-p member-id=string**
    - Caret(^) delimited list of member IDs. Filters out all events that do not reference one of the given member IDs. Optional.

* **-p page-token=string**
    - The value of &#39;nextPageToken&#39; from the previous page. Optional.

* **-p product-category=string**
    - Caret(^) delimited list of product categories. Filters out all events that do not reference a product in one of the given product categories. Optional.

* **-p publisher-id=string**
    - Caret(^) delimited list of publisher IDs. Filters out all events that do not reference one of the given publishers IDs. Only used when under advertiser role. Optional.

* **-p modify-date-min=string**
    - Filters out all events modified earlier than given date. Optional. Defaults to 24 hours before the current modifyDateMax, if modifyDateMax is explicitly set.

* **-p type=string**
    - Filters out all events that are not of the given type. Valid values: &#39;action&#39;, &#39;transaction&#39;, &#39;charge&#39;. Optional.

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
