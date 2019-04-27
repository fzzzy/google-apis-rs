Returns a dataset containing all data points whose start and end times overlap with the specified range of the dataset minimum start time and maximum end time. Specifically, any data point whose start time is less than or equal to the dataset end time and whose end time is greater than or equal to the dataset start time.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/fitness.activity.read*
* *https://www.googleapis.com/auth/fitness.activity.write*
* *https://www.googleapis.com/auth/fitness.blood_glucose.read*
* *https://www.googleapis.com/auth/fitness.blood_glucose.write*
* *https://www.googleapis.com/auth/fitness.blood_pressure.read*
* *https://www.googleapis.com/auth/fitness.blood_pressure.write*
* *https://www.googleapis.com/auth/fitness.body.read*
* *https://www.googleapis.com/auth/fitness.body.write*
* *https://www.googleapis.com/auth/fitness.body_temperature.read*
* *https://www.googleapis.com/auth/fitness.body_temperature.write*
* *https://www.googleapis.com/auth/fitness.location.read*
* *https://www.googleapis.com/auth/fitness.location.write*
* *https://www.googleapis.com/auth/fitness.nutrition.read*
* *https://www.googleapis.com/auth/fitness.nutrition.write*
* *https://www.googleapis.com/auth/fitness.oxygen_saturation.read*
* *https://www.googleapis.com/auth/fitness.oxygen_saturation.write*
* *https://www.googleapis.com/auth/fitness.reproductive_health.read*
* *https://www.googleapis.com/auth/fitness.reproductive_health.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/fitness.activity.read*.
You can set the scope for this method like this: `fitness1 --scope <scope> users data-sources-datasets-get ...`
# Required Scalar Arguments
* **&lt;user-id&gt;** *(string)*
    - Retrieve a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
* **&lt;data-source-id&gt;** *(string)*
    - The data stream ID of the data source that created the dataset.
* **&lt;dataset-id&gt;** *(string)*
    - Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: &#34;startTime-endTime&#34; where startTime and endTime are 64 bit integers.

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

* **-p limit=integer**
    - If specified, no more than this many data points will be included in the dataset. If there are more data points in the dataset, nextPageToken will be set in the dataset response.

* **-p page-token=string**
    - The continuation token, which is used to page through large datasets. To get the next page of a dataset, set this parameter to the value of nextPageToken from the previous response. Each subsequent call will yield a partial dataset with data point end timestamps that are strictly smaller than those in the previous partial response.

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
