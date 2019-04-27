Lists companies.

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

* **-p gps-motivations=string**
    - List of reasons for using Google Partner Search to get companies.

* **-p min-monthly-budget-units=string**
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* **-p request-metadata-user-overrides-ip-address=string**
    - IP address to use instead of the user&#39;s geo-located IP address.

* **-p website-url=string**
    - Website URL that will help to find a better matched company.
        .

* **-p page-token=string**
    - A token identifying a page of results that the server returns.
        Typically, this is the value of `ListCompaniesResponse.next_page_token`
        returned from the previous call to
        ListCompanies.

* **-p request-metadata-locale=string**
    - Locale to use for the current request.

* **-p address=string**
    - The address to use when searching for companies.
        If not given, the geo-located address of the request is used.

* **-p request-metadata-traffic-source-traffic-source-id=string**
    - Identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p specializations=string**
    - List of specializations that the returned agencies should provide. If this
        is not empty, any returned agency must have at least one of these
        specializations, or one of the services in the &#34;services&#34; field.

* **-p max-monthly-budget-units=string**
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* **-p view=string**
    - The view of the `Company` resource to be returned. This must not be
        `COMPANY_VIEW_UNSPECIFIED`.

* **-p min-monthly-budget-currency-code=string**
    - The 3-letter currency code defined in ISO 4217.

* **-p max-monthly-budget-currency-code=string**
    - The 3-letter currency code defined in ISO 4217.

* **-p company-name=string**
    - Company name to search for.

* **-p order-by=string**
    - How to order addresses within the returned companies. Currently, only
        `address` and `address desc` is supported which will sorted by closest to
        farthest in distance from given address and farthest to closest distance
        from given address respectively.

* **-p request-metadata-user-overrides-user-id=string**
    - Logged-in user ID to impersonate instead of the user&#39;s ID.

* **-p page-size=integer**
    - Requested page size. Server may return fewer companies than requested.
        If unspecified, server picks an appropriate default.

* **-p request-metadata-partners-session-id=string**
    - Google Partners session ID.

* **-p max-monthly-budget-nanos=integer**
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.

* **-p industries=string**
    - List of industries the company can help with.

* **-p request-metadata-traffic-source-traffic-sub-id=string**
    - Second level identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p language-codes=string**
    - List of language codes that company can support. Only primary language
        subtags are accepted as defined by
        &lt;a href=&#34;https://tools.ietf.org/html/bcp47&#34;&gt;BCP 47&lt;/a&gt;
        (IETF BCP 47, &#34;Tags for Identifying Languages&#34;).

* **-p min-monthly-budget-nanos=integer**
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.

* **-p request-metadata-experiment-ids=string**
    - Experiment IDs the current request belongs to.

* **-p services=string**
    - List of services that the returned agencies should provide. If this is
        not empty, any returned agency must have at least one of these services,
        or one of the specializations in the &#34;specializations&#34; field.

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
