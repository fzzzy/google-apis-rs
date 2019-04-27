Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results. Uses a small set of url patterns.
# Required Scalar Argument
* **&lt;q&gt;** *(string)*
    - Query

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

* **-p img-type=string**
    - Returns images of a type, which can be one of: clipart, face, lineart, news, and photo.

* **-p num=integer**
    - Number of search results to return

* **-p cr=string**
    - Country restrict(s).

* **-p img-color-type=string**
    - Returns black and white, grayscale, or color images: mono, gray, and color.

* **-p low-range=string**
    - Creates a range in form as_nlo value..as_nhi value and attempts to append it to query

* **-p img-size=string**
    - Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge.

* **-p rights=string**
    - Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these.

* **-p search-type=string**
    - Specifies the search type: image.

* **-p link-site=string**
    - Specifies that all search results should contain a link to a particular URL

* **-p file-type=string**
    - Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ...

* **-p related-site=string**
    - Specifies that all search results should be pages that are related to the specified URL

* **-p hl=string**
    - Sets the user interface language.

* **-p filter=string**
    - Controls turning on or off the duplicate content filter.

* **-p gl=string**
    - Geolocation of end user.

* **-p exclude-terms=string**
    - Identifies a word or phrase that should not appear in any documents in the search results

* **-p img-dominant-color=string**
    - Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown.

* **-p start=integer**
    - The index of the first result to return

* **-p site-search=string**
    - Specifies all search results should be pages from a given site

* **-p googlehost=string**
    - The local Google domain to use to perform the search.

* **-p cx=string**
    - The custom search engine ID to scope this search query

* **-p lr=string**
    - The language restriction for the search results

* **-p site-search-filter=string**
    - Controls whether to include or exclude results from the site named in the as_sitesearch parameter

* **-p date-restrict=string**
    - Specifies all search results are from a time period

* **-p sort=string**
    - The sort expression to apply to the results

* **-p exact-terms=string**
    - Identifies a phrase that all documents in the search results must contain

* **-p safe=string**
    - Search safety level

* **-p hq=string**
    - Appends the extra query terms to the query.

* **-p c2coff=string**
    - Turns off the translation between zh-CN and zh-TW.

* **-p or-terms=string**
    - Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms

* **-p high-range=string**
    - Creates a range in form as_nlo value..as_nhi value and attempts to append it to query

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
