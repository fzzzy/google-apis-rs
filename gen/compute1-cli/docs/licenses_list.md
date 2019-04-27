Retrieves the list of licenses available in the specified project. This method does not get any licenses that belong to other projects, including licenses attached to publicly-available images, like Debian 9. If you want to get a list of publicly-available licenses, use this method to make a request to the respective image project, such as debian-cloud or windows-cloud.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*
* *https://www.googleapis.com/auth/compute.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/compute.readonly*.
You can set the scope for this method like this: `compute1 --scope <scope> licenses list ...`
# Required Scalar Argument
* **&lt;project&gt;** *(string)*
    - Project ID for this request.

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

* **-p order-by=string**
    - Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name.
        
        You can also sort results in descending order based on the creation timestamp using orderBy=&#34;creationTimestamp desc&#34;. This sorts results based on the creationTimestamp field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first.
        
        Currently, only sorting by name or creationTimestamp desc is supported.

* **-p filter=string**
    - A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either =, !=, &gt;, or &lt;.
        
        For example, if you are filtering Compute Engine instances, you can exclude instances named example-instance by specifying name != example-instance.
        
        You can also filter nested fields. For example, you could specify scheduling.automaticRestart = false to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels.
        
        To filter on multiple expressions, provide each separate expression within parentheses. For example, (scheduling.automaticRestart = true) (cpuPlatform = &#34;Intel Skylake&#34;). By default, each expression is an AND expression. However, you can include AND and OR expressions explicitly. For example, (cpuPlatform = &#34;Intel Skylake&#34;) OR (cpuPlatform = &#34;Intel Broadwell&#34;) AND (scheduling.automaticRestart = true).

* **-p page-token=string**
    - Specifies a page token to use. Set pageToken to the nextPageToken returned by a previous list request to get the next page of results.

* **-p max-results=integer**
    - The maximum number of results per page that should be returned. If the number of available results is larger than maxResults, Compute Engine returns a nextPageToken that can be used to get the next page of results in subsequent list requests. Acceptable values are 0 to 500, inclusive. (Default: 500)

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
