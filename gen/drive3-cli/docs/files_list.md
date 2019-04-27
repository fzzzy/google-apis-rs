Lists or searches files.
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
You can set the scope for this method like this: `drive3 --scope <scope> files list ...`

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
    - Whether Team Drive items should be included in results.

* **-p q=string**
    - A query for filtering the file results. See the &#34;Search for Files&#34; guide for supported syntax.

* **-p corpora=string**
    - Comma-separated list of bodies of items (files/documents) to which the query applies. Supported bodies are &#39;user&#39;, &#39;domain&#39;, &#39;teamDrive&#39; and &#39;allTeamDrives&#39;. &#39;allTeamDrives&#39; must be combined with &#39;user&#39;; all other values must be used in isolation. Prefer &#39;user&#39; or &#39;teamDrive&#39; to &#39;allTeamDrives&#39; for efficiency.

* **-p order-by=string**
    - A comma-separated list of sort keys. Valid keys are &#39;createdTime&#39;, &#39;folder&#39;, &#39;modifiedByMeTime&#39;, &#39;modifiedTime&#39;, &#39;name&#39;, &#39;name_natural&#39;, &#39;quotaBytesUsed&#39;, &#39;recency&#39;, &#39;sharedWithMeTime&#39;, &#39;starred&#39;, and &#39;viewedByMeTime&#39;. Each key sorts ascending by default, but may be reversed with the &#39;desc&#39; modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored.

* **-p page-token=string**
    - The token for continuing a previous list request on the next page. This should be set to the value of &#39;nextPageToken&#39; from the previous response.

* **-p team-drive-id=string**
    - ID of Team Drive to search.

* **-p corpus=string**
    - The source of files to list. Deprecated: use &#39;corpora&#39; instead.

* **-p spaces=string**
    - A comma-separated list of spaces to query within the corpus. Supported values are &#39;drive&#39;, &#39;appDataFolder&#39; and &#39;photos&#39;.

* **-p supports-team-drives=boolean**
    - Whether the requesting application supports Team Drives.

* **-p page-size=integer**
    - The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached.

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
