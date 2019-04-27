Retrieves a list of placements, possibly filtered. This method supports paging.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> placements list ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.

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

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p content-category-ids=string**
    - Select only placements that are associated with these content categories.

* **-p placement-strategy-ids=string**
    - Select only placements that are associated with these placement strategies.

* **-p site-ids=string**
    - Select only placements that are associated with these sites.

* **-p compatibilities=string**
    - Select only placements that are associated with these compatibilities. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices for regular or interstitial ads respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering in in-stream video ads developed with the VAST standard.

* **-p payment-source=string**
    - Select only placements with this payment source.

* **-p min-start-date=string**
    - Select only placements or placement groups whose start date is on or after the specified minStartDate. The date should be formatted as &#34;yyyy-MM-dd&#34;.

* **-p pricing-types=string**
    - Select only placements with these pricing types.

* **-p max-end-date=string**
    - Select only placements or placement groups whose end date is on or before the specified maxEndDate. The date should be formatted as &#34;yyyy-MM-dd&#34;.

* **-p archived=boolean**
    - Select only archived placements. Don&#39;t set this field to select both archived and non-archived placements.

* **-p min-end-date=string**
    - Select only placements or placement groups whose end date is on or after the specified minEndDate. The date should be formatted as &#34;yyyy-MM-dd&#34;.

* **-p search-string=string**
    - Allows searching for placements by name or ID. Wildcards (*) are allowed. For example, &#34;placement*2015&#34; will return placements with names like &#34;placement June 2015&#34;, &#34;placement May 2015&#34;, or simply &#34;placements 2015&#34;. Most of the searches also add wildcards implicitly at the start and the end of the search string. For example, a search string of &#34;placement&#34; will match placements with name &#34;my placement&#34;, &#34;placement 2015&#34;, or simply &#34;placement&#34;.

* **-p directory-site-ids=string**
    - Select only placements that are associated with these directory sites.

* **-p sort-field=string**
    - Field by which to sort the list.

* **-p ids=string**
    - Select only placements with these IDs.

* **-p advertiser-ids=string**
    - Select only placements that belong to these advertisers.

* **-p page-token=string**
    - Value of the nextPageToken from the previous result page.

* **-p size-ids=string**
    - Select only placements that are associated with these sizes.

* **-p max-start-date=string**
    - Select only placements or placement groups whose start date is on or before the specified maxStartDate. The date should be formatted as &#34;yyyy-MM-dd&#34;.

* **-p sort-order=string**
    - Order of sorted results.

* **-p group-ids=string**
    - Select only placements that belong to these placement groups.

* **-p campaign-ids=string**
    - Select only placements that belong to these campaigns.

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
