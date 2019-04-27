Retrieve either deleted users or all users in a domain (paginated)
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/admin.directory.user*
* *https://www.googleapis.com/auth/admin.directory.user.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.user.readonly*.
You can set the scope for this method like this: `admin1-directory --scope <scope> users list ...`

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

* **-p view-type=string**
    - Whether to fetch the ADMIN_VIEW or DOMAIN_PUBLIC view of the user.

* **-p query=string**
    - Query string search. Should be of the form &#34;&#34;. Complete documentation is at https://developers.google.com/admin-sdk/directory/v1/guides/search-users

* **-p event=string**
    - Event on which subscription is intended (if subscribing)

* **-p sort-order=string**
    - Whether to return results in ascending or descending order.

* **-p max-results=integer**
    - Maximum number of results to return. Default is 100. Max allowed is 500

* **-p custom-field-mask=string**
    - Comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when projection=custom.

* **-p page-token=string**
    - Token to specify next page in the list

* **-p projection=string**
    - What subset of fields to fetch for this user.

* **-p show-deleted=string**
    - If set to true retrieves the list of deleted users. Default is false

* **-p customer=string**
    - Immutable ID of the G Suite account. In case of multi-domain, to fetch all users for a customer, fill this field instead of domain.

* **-p order-by=string**
    - Column to use for sorting results

* **-p domain=string**
    - Name of the domain. Fill this field to get users from only this domain. To return all users in a multi-domain fill customer field instead.

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
