Aggregates data of a certain type or stream into buckets divided by a given type of boundary. Multiple data sets of multiple types and from multiple sources can be aggreated into exactly one bucket type per request.
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
You can set the scope for this method like this: `fitness1 --scope <scope> users dataset-aggregate ...`
# Required Scalar Argument
* **&lt;user-id&gt;** *(string)*
    - Aggregate data for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AggregateRequest:
  bucket-by-activity-segment:
    activity-data-source-id: string
    min-duration-millis: string
  bucket-by-activity-type:
    activity-data-source-id: string
    min-duration-millis: string
  bucket-by-session:
    min-duration-millis: string
  bucket-by-time:
    duration-millis: string
    period:
      time-zone-id: string
      type: string
      value: integer
  end-time-millis: string
  filtered-data-quality-standard: [string]
  start-time-millis: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .bucket-by-activity-segment    activity-data-source-id=et`
    - The default activity stream will be used if a specific activityDataSourceId is not specified.
* `min-duration-millis=duo`
    - Specifies that only activity segments of duration longer than minDurationMillis are considered and used as a container for aggregated data.

* `..bucket-by-activity-type    activity-data-source-id=et`
    - The default activity stream will be used if a specific activityDataSourceId is not specified.
* `min-duration-millis=eirmod`
    - Specifies that only activity segments of duration longer than minDurationMillis are considered and used as a container for aggregated data.

* `..bucket-by-session    min-duration-millis=sanctus`
    - Specifies that only sessions of duration longer than minDurationMillis are considered and used as a container for aggregated data.

* `..bucket-by-time    duration-millis=et`
    - Specifies that result buckets aggregate data by exactly durationMillis time frames. Time frames that contain no data will be included in the response with an empty dataset.
* `period    time-zone-id=amet`
    - org.joda.timezone.DateTimeZone
* `type=et`
    - No description provided.
* `value=56`
    - No description provided.


* `...    end-time-millis=ut`
    - The end of a window of time. Data that intersects with this time window will be aggregated. The time is in milliseconds since epoch, inclusive.
* `filtered-data-quality-standard=ea`
    - A list of acceptable data quality standards. Only data points which conform to at least one of the specified data quality standards will be returned. If the list is empty, all data points are returned.
    - Each invocation of this argument appends the given value to the array.
* `start-time-millis=sed`
    - The start of a window of time. Data that intersects with this time window will be aggregated. The time is in milliseconds since epoch, inclusive.


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
