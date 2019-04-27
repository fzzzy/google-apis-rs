Provides a list of the authenticated user&#39;s contacts merged with any
connected profiles.
&lt;br&gt;
The request throws a 400 error if &#39;personFields&#39; is not specified.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/contacts*
* *https://www.googleapis.com/auth/contacts.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/contacts.readonly*.
You can set the scope for this method like this: `people1 --scope <scope> people connections-list ...`
# Required Scalar Argument
* **&lt;resource-name&gt;** *(string)*
    - The resource name to return connections for. Only `people/me` is valid.

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

* **-p page-size=integer**
    - The number of connections to include in the response. Valid values are
        between 1 and 2000, inclusive. Defaults to 100.

* **-p sort-order=string**
    - The order in which the connections should be sorted. Defaults to
        `LAST_MODIFIED_ASCENDING`.

* **-p person-fields=string**
    - **Required.** A field mask to restrict which fields on each person are
        returned. Multiple fields can be specified by separating them with commas.
        Valid values are:
        
        * addresses
        * ageRanges
        * biographies
        * birthdays
        * braggingRights
        * coverPhotos
        * emailAddresses
        * events
        * genders
        * imClients
        * interests
        * locales
        * memberships
        * metadata
        * names
        * nicknames
        * occupations
        * organizations
        * phoneNumbers
        * photos
        * relations
        * relationshipInterests
        * relationshipStatuses
        * residences
        * sipAddresses
        * skills
        * taglines
        * urls
        * userDefined

* **-p request-sync-token=boolean**
    - Whether the response should include a sync token, which can be used to get
        all changes since the last request. For subsequent sync requests use the
        `sync_token` param instead. Initial sync requests that specify
        `request_sync_token` have an additional rate limit.

* **-p page-token=string**
    - The token of the page to be returned.

* **-p sync-token=string**
    - A sync token returned by a previous call to `people.connections.list`.
        Only resources changed since the sync token was created will be returned.
        Sync requests that specify `sync_token` have an additional rate limit.

* **-p request-mask-include-field=string**
    - **Required.** Comma-separated list of person fields to be included in the
        response. Each path should start with `person.`: for example,
        `person.names` or `person.photos`.

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
