Updates a transfer job. Updating a job&#39;s transfer spec does not affect
transfer operations that are running already. Updating the scheduling
of a job is not allowed.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `storagetransfer1 --scope <scope> transfer-jobs patch ...`
# Required Scalar Argument
* **&lt;job-name&gt;** *(string)*
    - The name of job to update.
        Required.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UpdateTransferJobRequest:
  project-id: string
  transfer-job:
    creation-time: string
    deletion-time: string
    description: string
    last-modification-time: string
    name: string
    project-id: string
    schedule:
      schedule-end-date:
        day: integer
        month: integer
        year: integer
      schedule-start-date:
        day: integer
        month: integer
        year: integer
      start-time-of-day:
        hours: integer
        minutes: integer
        nanos: integer
        seconds: integer
    status: string
    transfer-spec:
      aws-s3-data-source:
        aws-access-key:
          access-key-id: string
          secret-access-key: string
        bucket-name: string
      gcs-data-sink:
        bucket-name: string
      gcs-data-source:
        bucket-name: string
      http-data-source:
        list-url: string
      object-conditions:
        exclude-prefixes: [string]
        include-prefixes: [string]
        max-time-elapsed-since-last-modification: string
        min-time-elapsed-since-last-modification: string
      transfer-options:
        delete-objects-from-source-after-transfer: boolean
        delete-objects-unique-in-sink: boolean
        overwrite-objects-already-existing-in-sink: boolean
  update-transfer-job-field-mask: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    project-id=aliquyam`
    - The ID of the Google Cloud Platform Console project that owns the job.
        Required.
* `transfer-job    creation-time=sea`
    - This field cannot be changed by user requests.
* `deletion-time=lorem`
    - This field cannot be changed by user requests.
* `description=eos`
    - A description provided by the user for the job. Its max length is 1024
        bytes when Unicode-encoded.
* `last-modification-time=erat`
    - This field cannot be changed by user requests.
* `name=sadipscing`
    - A globally unique name assigned by Storage Transfer Service when the
        job is created. This field should be left empty in requests to create a new
        transfer job; otherwise, the requests result in an `INVALID_ARGUMENT`
        error.
* `project-id=dolor`
    - The ID of the Google Cloud Platform Console project that owns the job.
* `schedule.schedule-end-date    day=62`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=58`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=4`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.

* `..schedule-start-date    day=41`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=65`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=62`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.

* `..start-time-of-day    hours=68`
    - Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
        to allow the value &#34;24:00:00&#34; for scenarios like business closing time.
* `minutes=64`
    - Minutes of hour of day. Must be from 0 to 59.
* `nanos=19`
    - Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
* `seconds=28`
    - Seconds of minutes of the time. Must normally be from 0 to 59. An API may
        allow the value 60 if it allows leap-seconds.


* `...    status=lorem`
    - Status of the job. This value MUST be specified for
        `CreateTransferJobRequests`.
        
        NOTE: The effect of the new job status takes place during a subsequent job
        run. For example, if you change the job status from `ENABLED` to
        `DISABLED`, and an operation spawned by the transfer is running, the status
        change would not affect the current operation.
* `transfer-spec.aws-s3-data-source.aws-access-key    access-key-id=sea`
    - AWS access key ID.
        Required.
* `secret-access-key=et`
    - AWS secret access key. This field is not returned in RPC responses.
        Required.

* `..    bucket-name=duo`
    - S3 Bucket name (see
        [Creating a bucket](http://docs.aws.amazon.com/AmazonS3/latest/dev/create-bucket-get-location-example.html)).
        Required.

* `..gcs-data-sink    bucket-name=et`
    - Google Cloud Storage bucket name (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/naming#requirements)).
        Required.

* `..gcs-data-source    bucket-name=eirmod`
    - Google Cloud Storage bucket name (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/naming#requirements)).
        Required.

* `..http-data-source    list-url=sanctus`
    - The URL that points to the file that stores the object list entries.
        This file must allow public access.  Currently, only URLs with HTTP and
        HTTPS schemes are supported.
        Required.

* `..object-conditions    exclude-prefixes=et`
    - `excludePrefixes` must follow the requirements described for
        `includePrefixes`.
        
        The max size of `excludePrefixes` is 1000.
    - Each invocation of this argument appends the given value to the array.
* `include-prefixes=amet`
    - If `includePrefixes` is specified, objects that satisfy the object
        conditions must have names that start with one of the `includePrefixes`
        and that do not start with any of the `excludePrefixes`. If `includePrefixes`
        is not specified, all objects except those that have names starting with
        one of the `excludePrefixes` must satisfy the object conditions.
        
        Requirements:
        
          * Each include-prefix and exclude-prefix can contain any sequence of
            Unicode characters, of max length 1024 bytes when UTF8-encoded, and
            must not contain Carriage Return or Line Feed characters.  Wildcard
            matching and regular expression matching are not supported.
        
          * Each include-prefix and exclude-prefix must omit the leading slash.
            For example, to include the `requests.gz` object in a transfer from
            `s3://my-aws-bucket/logs/y=2015/requests.gz`, specify the include
            prefix as `logs/y=2015/requests.gz`.
        
          * None of the include-prefix or the exclude-prefix values can be empty,
            if specified.
        
          * Each include-prefix must include a distinct portion of the object
            namespace, i.e., no include-prefix may be a prefix of another
            include-prefix.
        
          * Each exclude-prefix must exclude a distinct portion of the object
            namespace, i.e., no exclude-prefix may be a prefix of another
            exclude-prefix.
        
          * If `includePrefixes` is specified, then each exclude-prefix must start
            with the value of a path explicitly included by `includePrefixes`.
        
        The max size of `includePrefixes` is 1000.
    - Each invocation of this argument appends the given value to the array.
* `max-time-elapsed-since-last-modification=et`
    - `maxTimeElapsedSinceLastModification` is the complement to
        `minTimeElapsedSinceLastModification`.
* `min-time-elapsed-since-last-modification=consetetur`
    - If unspecified, `minTimeElapsedSinceLastModification` takes a zero value
        and `maxTimeElapsedSinceLastModification` takes the maximum possible
        value of Duration. Objects that satisfy the object conditions
        must either have a `lastModificationTime` greater or equal to
        `NOW` - `maxTimeElapsedSinceLastModification` and less than
        `NOW` - `minTimeElapsedSinceLastModification`, or not have a
        `lastModificationTime`.

* `..transfer-options    delete-objects-from-source-after-transfer=true`
    - Whether objects should be deleted from the source after they are
        transferred to the sink.  Note that this option and
        `deleteObjectsUniqueInSink` are mutually exclusive.
* `delete-objects-unique-in-sink=true`
    - Whether objects that exist only in the sink should be deleted.  Note that
        this option and `deleteObjectsFromSourceAfterTransfer` are mutually
        exclusive.
* `overwrite-objects-already-existing-in-sink=false`
    - Whether overwriting objects that already exist in the sink is allowed.



* `....    update-transfer-job-field-mask=dolor`
    - The field mask of the fields in `transferJob` that are to be updated in
        this request.  Fields in `transferJob` that can be updated are:
        `description`, `transferSpec`, and `status`.  To update the `transferSpec`
        of the job, a complete transfer specification has to be provided. An
        incomplete specification which misses any required fields will be rejected
        with the error `INVALID_ARGUMENT`.


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
