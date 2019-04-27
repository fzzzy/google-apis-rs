Creates or updates a logs-based metric.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/logging.admin*
* *https://www.googleapis.com/auth/logging.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `logging2 --scope <scope> projects metrics-update ...`
# Required Scalar Argument
* **&lt;metric-name&gt;** *(string)*
    - The resource name of the metric to update:
        &#34;projects/[PROJECT_ID]/metrics/[METRIC_ID]&#34;
        The updated metric must be provided in the request and it&#39;s name field must be the same as [METRIC_ID] If the metric does not exist in [PROJECT_ID], then a new metric is created.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LogMetric:
  bucket-options:
    explicit-buckets:
      bounds: [number]
    exponential-buckets:
      growth-factor: number
      num-finite-buckets: integer
      scale: number
    linear-buckets:
      num-finite-buckets: integer
      offset: number
      width: number
  description: string
  filter: string
  label-extractors: { string: string }
  metric-descriptor:
    description: string
    display-name: string
    metadata:
      ingest-delay: string
      launch-stage: string
      sample-period: string
    metric-kind: string
    name: string
    type: string
    unit: string
    value-type: string
  name: string
  value-extractor: string
  version: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .bucket-options.explicit-buckets    bounds=0.378784903851`
    - The values must be monotonically increasing.
    - Each invocation of this argument appends the given value to the array.

* `..exponential-buckets    growth-factor=0.639492440554`
    - Must be greater than 1.
* `num-finite-buckets=11`
    - Must be greater than 0.
* `scale=0.243025714914`
    - Must be greater than 0.

* `..linear-buckets    num-finite-buckets=5`
    - Must be greater than 0.
* `offset=0.790807870902`
    - Lower bound of the first bucket.
* `width=0.212469304768`
    - Must be greater than 0.


* `...    description=sit`
    - Optional. A description of this metric, which is used in documentation.
* `filter=takimata`
    - Required. An advanced logs filter which is used to match log entries. Example:
        &#34;resource.type=gae_app AND severity&gt;=ERROR&#34;
        The maximum length of the filter is 20000 characters.
* `label-extractors=key=elitr`
    - Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If the either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project.
    - the value will be associated with the given `key`
* `metric-descriptor    description=nonumy`
    - A detailed description of the metric, which can be used in documentation.
* `display-name=rebum.`
    - A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example &#34;Request count&#34;. This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota.
* `metadata    ingest-delay=lorem`
    - The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors.
* `launch-stage=lorem`
    - The launch stage of the metric definition.
* `sample-period=diam`
    - The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period.

* `..    metric-kind=ut`
    - Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported.
* `name=ut`
    - The resource name of the metric descriptor.
* `type=amet.`
    - The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example:
        &#34;custom.googleapis.com/invoice/paid/amount&#34;
        &#34;external.googleapis.com/prometheus/up&#34;
        &#34;appengine.googleapis.com/http/server/response_latencies&#34;
        
* `unit=ipsum`
    - The unit in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The supported units are a subset of The Unified Code for Units of Measure (http://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT)
        bit bit
        By byte
        s second
        min minute
        h hour
        d dayPrefixes (PREFIX)
        k kilo (10**3)
        M mega (10**6)
        G giga (10**9)
        T tera (10**12)
        P peta (10**15)
        E exa (10**18)
        Z zetta (10**21)
        Y yotta (10**24)
        m milli (10**-3)
        u micro (10**-6)
        n nano (10**-9)
        p pico (10**-12)
        f femto (10**-15)
        a atto (10**-18)
        z zepto (10**-21)
        y yocto (10**-24)
        Ki kibi (2**10)
        Mi mebi (2**20)
        Gi gibi (2**30)
        Ti tebi (2**40)GrammarThe grammar also includes these connectors:
        / division (as an infix operator, e.g. 1/s).
        . multiplication (as an infix operator, e.g. GBy.d)The grammar for a unit is as follows:
        Expression = Component { &#34;.&#34; Component } { &#34;/&#34; Component } ;
        
        Component = ( [ PREFIX ] UNIT | &#34;%&#34; ) [ Annotation ]
                  | Annotation
                  | &#34;1&#34;
                  ;
        
        Annotation = &#34;{&#34; NAME &#34;}&#34; ;
        Notes:
        Annotation is just a comment if it follows a UNIT and is  equivalent to 1 if it is used alone. For examples,  {requests}/s == 1/s, By{transmitted}/s == By/s.
        NAME is a sequence of non-blank printable ASCII characters not  containing &#39;{&#39; or &#39;}&#39;.
        1 represents dimensionless value 1, such as in 1/s.
        % represents dimensionless value 1/100, and annotates values giving  a percentage.
* `value-type=ut`
    - Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported.

* `..    name=dolor`
    - Required. The client-assigned metric identifier. Examples: &#34;error_count&#34;, &#34;nginx/requests&#34;.Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*&#39;,()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.The metric identifier in this field must not be URL-encoded (https://en.wikipedia.org/wiki/Percent-encoding). However, when the metric identifier appears as the [METRIC_ID] part of a metric_name API parameter, then the metric identifier must be URL-encoded. Example: &#34;projects/my-project/metrics/nginx%2Frequests&#34;.
* `value-extractor=sea`
    - Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The argument are:  1. field: The name of the log entry field from which the value is to be  extracted.  2. regex: A regular expression using the Google RE2 syntax  (https://github.com/google/re2/wiki/Syntax) with a single capture  group to extract data from the specified log entry field. The value  of the field is converted to a string before applying the regex.  It is an error to specify a regex that does not include exactly one  capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, &#34;.*quantity=(\d+).*&#34;)
* `version=ut`
    - Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed.


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
