Lists the changes for a user or Team Drive.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.appdata*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/drive.metadata*
* *https://www.googleapis.com/auth/drive.metadata.readonly*
* *https://www.googleapis.com/auth/drive.photos.readonly*
* *https://www.googleapis.com/auth/drive.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive.metadata.readonly*.
You can set the scope for this method like this: `drive3 --scope <scope> changes list ...`
# Required Scalar Argument
* **&lt;page-token&gt;** *(string)*
    - The token for continuing a previous list request on the next page. This should be set to the value of &#39;nextPageToken&#39; from the previous response or to the response from the getStartPageToken method.

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

* **-p include-team-drive-items=boolean**
    - Whether Team Drive files or changes should be included in results.

* **-p page-size=integer**
    - The maximum number of changes to return per page.

* **-p restrict-to-my-drive=boolean**
    - Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.

* **-p spaces=string**
    - A comma-separated list of spaces to query within the user corpus. Supported values are &#39;drive&#39;, &#39;appDataFolder&#39; and &#39;photos&#39;.

* **-p include-corpus-removals=boolean**
    - Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.

* **-p supports-team-drives=boolean**
    - Whether the requesting application supports Team Drives.

* **-p team-drive-id=string**
    - The Team Drive from which changes will be returned. If specified the change IDs will be reflective of the Team Drive; use the combined Team Drive ID and change ID as an identifier.

* **-p include-removed=boolean**
    - Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.

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
