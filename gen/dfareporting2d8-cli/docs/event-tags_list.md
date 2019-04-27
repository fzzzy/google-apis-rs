Retrieves a list of event tags, possibly filtered.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting2d8 --scope <scope> event-tags list ...`
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

* **-p campaign-id=string**
    - Select only event tags that belong to this campaign.

* **-p ad-id=string**
    - Select only event tags that belong to this ad.

* **-p sort-field=string**
    - Field by which to sort the list.

* **-p event-tag-types=string**
    - Select only event tags with the specified event tag types. Event tag types can be used to specify whether to use a third-party pixel, a third-party JavaScript URL, or a third-party click-through URL for either impression or click tracking.

* **-p enabled=boolean**
    - Select only enabled event tags. What is considered enabled or disabled depends on the definitionsOnly parameter. When definitionsOnly is set to true, only the specified advertiser or campaign&#39;s event tags&#39; enabledByDefault field is examined. When definitionsOnly is set to false, the specified ad or specified campaign&#39;s parent advertiser&#39;s or parent campaign&#39;s event tags&#39; enabledByDefault and status fields are examined as well.

* **-p sort-order=string**
    - Order of sorted results.

* **-p definitions-only=boolean**
    - Examine only the specified campaign or advertiser&#39;s event tags for matching selector criteria. When set to false, the parent advertiser and parent campaign of the specified ad or campaign is examined as well. In addition, when set to false, the status field is examined as well, along with the enabledByDefault field. This parameter can not be set to true when adId is specified as ads do not define their own even tags.

* **-p ids=string**
    - Select only event tags with these IDs.

* **-p advertiser-id=string**
    - Select only event tags that belong to this advertiser.

* **-p search-string=string**
    - Allows searching for objects by name or ID. Wildcards (*) are allowed. For example, &#34;eventtag*2015&#34; will return objects with names like &#34;eventtag June 2015&#34;, &#34;eventtag April 2015&#34;, or simply &#34;eventtag 2015&#34;. Most of the searches also add wildcards implicitly at the start and the end of the search string. For example, a search string of &#34;eventtag&#34; will match objects with name &#34;my eventtag&#34;, &#34;eventtag 2015&#34;, or simply &#34;eventtag&#34;.

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
