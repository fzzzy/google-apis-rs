Get high scores, and optionally ranks, in leaderboards for the currently authenticated player. For a specific time span, leaderboardId can be set to ALL to retrieve data for all leaderboards in a given time span.
NOTE: You cannot ask for &#39;ALL&#39; leaderboards and &#39;ALL&#39; timeSpans in the same request; only one parameter may be set to &#39;ALL&#39;.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/games*
* *https://www.googleapis.com/auth/plus.login*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/games*.
You can set the scope for this method like this: `games1 --scope <scope> scores get ...`
# Required Scalar Arguments
* **&lt;player-id&gt;** *(string)*
    - A player ID. A value of me may be used in place of the authenticated player&#39;s ID.
* **&lt;leaderboard-id&gt;** *(string)*
    - The ID of the leaderboard. Can be set to &#39;ALL&#39; to retrieve data for all leaderboards for this application.
* **&lt;time-span&gt;** *(string)*
    - The time span for the scores and ranks you&#39;re requesting.

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

* **-p language=string**
    - The preferred language to use for strings returned by this method.

* **-p max-results=integer**
    - The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified maxResults.

* **-p include-rank-type=string**
    - The types of ranks to return. If the parameter is omitted, no ranks will be returned.

* **-p page-token=string**
    - The token returned by the previous request.

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
