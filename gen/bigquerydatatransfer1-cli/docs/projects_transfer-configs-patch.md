Updates a data transfer configuration.
All fields must be set, even if they are not updated.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `bigquerydatatransfer1 --scope <scope> projects transfer-configs-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The resource name of the transfer config.
        Transfer config names have the form of
        `projects/{project_id}/location/{region}/transferConfigs/{config_id}`.
        The name is automatically generated based on the config_id specified in
        CreateTransferConfigRequest along with project_id and region. If config_id
        is not provided, usually a uuid, even though it is not guaranteed or
        required, will be generated for config_id.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
TransferConfig:
  data-refresh-window-days: integer
  data-source-id: string
  dataset-region: string
  destination-dataset-id: string
  disabled: boolean
  display-name: string
  name: string
  next-run-time: string
  schedule: string
  state: string
  update-time: string
  user-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    data-refresh-window-days=58`
    - The number of days to look back to automatically refresh the data.
        For example, if `data_refresh_window_days = 10`, then every day
        BigQuery reingests data for [today-10, today-1], rather than ingesting data
        for just [today-1].
        Only valid if the data source supports the feature. Set the value to  0
        to use the default value.
* `data-source-id=amet`
    - Data source id. Cannot be changed once data transfer is created.
* `dataset-region=no`
    - Output only. Region in which BigQuery dataset is located.
* `destination-dataset-id=labore`
    - The BigQuery target dataset id.
* `disabled=true`
    - Is this config disabled. When set to true, no runs are scheduled
        for a given transfer.
* `display-name=dolore`
    - User specified display name for the data transfer.
* `name=invidunt`
    - The resource name of the transfer config.
        Transfer config names have the form of
        `projects/{project_id}/location/{region}/transferConfigs/{config_id}`.
        The name is automatically generated based on the config_id specified in
        CreateTransferConfigRequest along with project_id and region. If config_id
        is not provided, usually a uuid, even though it is not guaranteed or
        required, will be generated for config_id.
* `next-run-time=aliquyam`
    - Output only. Next time when data transfer will run.
* `schedule=accusam`
    - Data transfer schedule.
        If the data source does not support a custom schedule, this should be
        empty. If it is empty, the default value for the data source will be
        used.
        The specified times are in UTC.
        Examples of valid format:
        `1st,3rd monday of month 15:30`,
        `every wed,fri of jan,jun 13:15`, and
        `first sunday of quarter 00:00`.
        See more explanation about the format here:
        https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format
        NOTE: the granularity should be at least 8 hours, or less frequent.
* `state=lorem`
    - Output only. State of the most recently updated transfer run.
* `update-time=sea`
    - Output only. Data transfer modification time. Ignored by server on input.
* `user-id=et`
    - Output only. Unique ID of the user on whose behalf transfer is done.
        Applicable only to data sources that do not support service accounts.
        When set to 0, the data source service account credentials are used.
        May be negative. Note, that this identifier is not stable.
        It may change over time even for the same user.


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

* **-p update-mask=string**
    - Required list of fields to be updated in this request.

* **-p authorization-code=string**
    - Optional OAuth2 authorization code to use with this transfer configuration.
        If it is provided, the transfer configuration will be associated with the
        authorizing user.
        In order to obtain authorization_code, please make a
        request to
        https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=&lt;datatransferapiclientid&gt;&amp;scope=&lt;data_source_scopes&gt;&amp;redirect_uri=&lt;redirect_uri&gt;
        
        * client_id should be OAuth client_id of BigQuery DTS API for the given
          data source returned by ListDataSources method.
        * data_source_scopes are the scopes returned by ListDataSources method.
        * redirect_uri is an optional parameter. If not specified, then
          authorization code is posted to the opener of authorization flow window.
          Otherwise it will be sent to the redirect uri. A special value of
          urn:ietf:wg:oauth:2.0:oob means that authorization code should be
          returned in the title bar of the browser, with the page text prompting
          the user to copy the code and paste it in the application.

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
