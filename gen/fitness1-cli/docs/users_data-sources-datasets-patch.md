Adds data points to a dataset. The dataset need not be previously created. All points within the given dataset will be returned with subsquent calls to retrieve this dataset. Data points can belong to more than one dataset. This method does not use patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/fitness.activity.write*
* *https://www.googleapis.com/auth/fitness.blood_glucose.write*
* *https://www.googleapis.com/auth/fitness.blood_pressure.write*
* *https://www.googleapis.com/auth/fitness.body.write*
* *https://www.googleapis.com/auth/fitness.body_temperature.write*
* *https://www.googleapis.com/auth/fitness.location.write*
* *https://www.googleapis.com/auth/fitness.nutrition.write*
* *https://www.googleapis.com/auth/fitness.oxygen_saturation.write*
* *https://www.googleapis.com/auth/fitness.reproductive_health.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/fitness.activity.write*.
You can set the scope for this method like this: `fitness1 --scope <scope> users data-sources-datasets-patch ...`
# Required Scalar Arguments
* **&lt;user-id&gt;** *(string)*
    - Patch a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
* **&lt;data-source-id&gt;** *(string)*
    - The data stream ID of the data source that created the dataset.
* **&lt;dataset-id&gt;** *(string)*
    - Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: &#34;startTime-endTime&#34; where startTime and endTime are 64 bit integers.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Dataset:
  data-source-id: string
  max-end-time-ns: string
  min-start-time-ns: string
  next-page-token: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    data-source-id=dolores`
    - The data stream ID of the data source that created the points in this dataset.
* `max-end-time-ns=gubergren`
    - The largest end time of all data points in this possibly partial representation of the dataset. Time is in nanoseconds from epoch. This should also match the first part of the dataset identifier.
* `min-start-time-ns=sadipscing`
    - The smallest start time of all data points in this possibly partial representation of the dataset. Time is in nanoseconds from epoch. This should also match the first part of the dataset identifier.
* `next-page-token=aliquyam`
    - This token will be set when a dataset is received in response to a GET request and the dataset is too large to be included in a single response. Provide this value in a subsequent GET request to return the next page of data points within this dataset.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


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

* **-p current-time-millis=string**
    - The client&#39;s current time in milliseconds since epoch. Note that the minStartTimeNs and maxEndTimeNs properties in the request body are in nanoseconds instead of milliseconds.

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
