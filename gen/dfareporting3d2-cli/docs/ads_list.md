Retrieves a list of ads, possibly filtered. This method supports paging.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> ads list ...`
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

* **-p overridden-event-tag-id=string**
    - Select only ads with this event tag override ID.

* **-p audience-segment-ids=string**
    - Select only ads with these audience segment IDs.

* **-p ssl-compliant=boolean**
    - Select only ads that are SSL-compliant.

* **-p active=boolean**
    - Select only active ads.

* **-p sort-order=string**
    - Order of sorted results.

* **-p archived=boolean**
    - Select only archived ads.

* **-p advertiser-id=string**
    - Select only ads with this advertiser ID.

* **-p size-ids=string**
    - Select only ads with these size IDs.

* **-p compatibility=string**
    - Select default ads with the specified compatibility. Applicable when type is AD_SERVING_DEFAULT_AD. DISPLAY and DISPLAY_INTERSTITIAL refer to rendering either on desktop or on mobile devices for regular or interstitial ads, respectively. APP and APP_INTERSTITIAL are for rendering in mobile apps. IN_STREAM_VIDEO refers to rendering an in-stream video ads developed with the VAST standard.

* **-p search-string=string**
    - Allows searching for objects by name or ID. Wildcards (*) are allowed. For example, &#34;ad*2015&#34; will return objects with names like &#34;ad June 2015&#34;, &#34;ad April 2015&#34;, or simply &#34;ad 2015&#34;. Most of the searches also add wildcards implicitly at the start and the end of the search string. For example, a search string of &#34;ad&#34; will match objects with name &#34;my ad&#34;, &#34;ad 2015&#34;, or simply &#34;ad&#34;.

* **-p campaign-ids=string**
    - Select only ads with these campaign IDs.

* **-p ids=string**
    - Select only ads with these IDs.

* **-p dynamic-click-tracker=boolean**
    - Select only dynamic click trackers. Applicable when type is AD_SERVING_CLICK_TRACKER. If true, select dynamic click trackers. If false, select static click trackers. Leave unset to select both.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p placement-ids=string**
    - Select only ads with these placement IDs assigned.

* **-p landing-page-ids=string**
    - Select only ads with these landing page IDs.

* **-p page-token=string**
    - Value of the nextPageToken from the previous result page.

* **-p creative-optimization-configuration-ids=string**
    - Select only ads with these creative optimization configuration IDs.

* **-p ssl-required=boolean**
    - Select only ads that require SSL.

* **-p sort-field=string**
    - Field by which to sort the list.

* **-p type=string**
    - Select only ads with these types.

* **-p remarketing-list-ids=string**
    - Select only ads whose list targeting expression use these remarketing list IDs.

* **-p creative-ids=string**
    - Select only ads with these creative IDs assigned.

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