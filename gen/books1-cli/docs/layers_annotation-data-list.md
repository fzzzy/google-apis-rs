Gets the annotation data for a volume and layer.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/books* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/books*.
You can set the scope for this method like this: `books1 --scope <scope> layers annotation-data-list ...`
# Required Scalar Arguments
* **&lt;volume-id&gt;** *(string)*
    - The volume to retrieve annotation data for.
* **&lt;layer-id&gt;** *(string)*
    - The ID for the layer to get the annotation data.
* **&lt;content-version&gt;** *(string)*
    - The content version for the requested volume.

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

* **-p updated-max=string**
    - RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).

* **-p scale=integer**
    - The requested scale for the image.

* **-p page-token=string**
    - The value of the nextToken from the previous page.

* **-p locale=string**
    - The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: &#39;en_US&#39;.

* **-p h=integer**
    - The requested pixel height for any images. If height is provided width must also be provided.

* **-p max-results=integer**
    - Maximum number of results to return

* **-p annotation-data-id=string**
    - The list of Annotation Data Ids to retrieve. Pagination is ignored if this is set.

* **-p source=string**
    - String to identify the originator of this request.

* **-p w=integer**
    - The requested pixel width for any images. If width is provided height must also be provided.

* **-p updated-min=string**
    - RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).

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