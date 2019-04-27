List StoreInfos owned or managed by the partner.

See _Authentication and Authorization rules_ and
_List methods rules_ for more information about this method.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/playmovies_partner.readonly* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/playmovies_partner.readonly*.
You can set the scope for this method like this: `playmoviespartner1 --scope <scope> accounts store-infos-list ...`
# Required Scalar Argument
* **&lt;account-id&gt;** *(string)*
    - REQUIRED. See _General rules_ for more information about this field.

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

* **-p name=string**
    - Filter that matches StoreInfos with a `name` or `show_name`
        that contains the given case-insensitive name.

* **-p page-token=string**
    - See _List methods rules_ for info about this field.

* **-p pph-names=string**
    - See _List methods rules_ for info about this field.

* **-p page-size=integer**
    - See _List methods rules_ for info about this field.

* **-p studio-names=string**
    - See _List methods rules_ for info about this field.

* **-p video-ids=string**
    - Filter StoreInfos that match any of the given `video_id`s.

* **-p season-ids=string**
    - Filter StoreInfos that match any of the given `season_id`s.

* **-p countries=string**
    - Filter StoreInfos that match (case-insensitive) any of the given country
        codes, using the &#34;ISO 3166-1 alpha-2&#34; format (examples: &#34;US&#34;, &#34;us&#34;, &#34;Us&#34;).

* **-p video-id=string**
    - Filter StoreInfos that match a given `video_id`.
        NOTE: this field is deprecated and will be removed on V2; `video_ids`
        should be used instead.

* **-p mids=string**
    - Filter StoreInfos that match any of the given `mid`s.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p bearer-token=string**
    - OAuth bearer token.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pp=boolean**
    - Pretty-print response.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
