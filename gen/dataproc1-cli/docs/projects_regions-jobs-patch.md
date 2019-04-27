Updates a job in a project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dataproc1 --scope <scope> projects regions-jobs-patch ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - Required. The ID of the Google Cloud Platform project that the job belongs to.
* **&lt;region&gt;** *(string)*
    - Required. The Cloud Dataproc region in which to handle the request.
* **&lt;job-id&gt;** *(string)*
    - Required. The job ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Job:
  driver-control-files-uri: string
  driver-output-resource-uri: string
  hadoop-job:
    archive-uris: [string]
    args: [string]
    file-uris: [string]
    jar-file-uris: [string]
    logging-config:
      driver-log-levels: { string: string }
    main-class: string
    main-jar-file-uri: string
    properties: { string: string }
  hive-job:
    continue-on-failure: boolean
    jar-file-uris: [string]
    properties: { string: string }
    query-file-uri: string
    query-list:
      queries: [string]
    script-variables: { string: string }
  labels: { string: string }
  pig-job:
    continue-on-failure: boolean
    jar-file-uris: [string]
    logging-config:
      driver-log-levels: { string: string }
    properties: { string: string }
    query-file-uri: string
    query-list:
      queries: [string]
    script-variables: { string: string }
  placement:
    cluster-name: string
    cluster-uuid: string
  pyspark-job:
    archive-uris: [string]
    args: [string]
    file-uris: [string]
    jar-file-uris: [string]
    logging-config:
      driver-log-levels: { string: string }
    main-python-file-uri: string
    properties: { string: string }
    python-file-uris: [string]
  reference:
    job-id: string
    project-id: string
  scheduling:
    max-failures-per-hour: integer
  spark-job:
    archive-uris: [string]
    args: [string]
    file-uris: [string]
    jar-file-uris: [string]
    logging-config:
      driver-log-levels: { string: string }
    main-class: string
    main-jar-file-uri: string
    properties: { string: string }
  spark-sql-job:
    jar-file-uris: [string]
    logging-config:
      driver-log-levels: { string: string }
    properties: { string: string }
    query-file-uri: string
    query-list:
      queries: [string]
    script-variables: { string: string }
  status:
    details: string
    state: string
    state-start-time: string
    substate: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    driver-control-files-uri=et`
    - Output only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri.
* `driver-output-resource-uri=aliquyam`
    - Output only. A URI pointing to the location of the stdout of the job&#39;s driver program.
* `hadoop-job    archive-uris=nonumy`
    - Optional. HCFS URIs of archives to be extracted in the working directory of Hadoop drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, or .zip.
    - Each invocation of this argument appends the given value to the array.
* `args=sit`
    - Optional. The arguments to pass to the driver. Do not include arguments, such as -libjars or -Dfoo=bar, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    - Each invocation of this argument appends the given value to the array.
* `file-uris=aliquyam`
    - Optional. HCFS (Hadoop Compatible Filesystem) URIs of files to be copied to the working directory of Hadoop drivers and distributed tasks. Useful for naively parallel tasks.
    - Each invocation of this argument appends the given value to the array.
* `jar-file-uris=sadipscing`
    - Optional. Jar file URIs to add to the CLASSPATHs of the Hadoop driver and tasks.
    - Each invocation of this argument appends the given value to the array.
* `logging-config    driver-log-levels=key=magna`
    - The per-package log levels for the driver. This may include &#34;root&#34; package name to configure rootLogger. Examples:  &#39;com.google = FATAL&#39;, &#39;root = INFO&#39;, &#39;org.apache = DEBUG&#39;
    - the value will be associated with the given `key`

* `..    main-class=gubergren`
    - The name of the driver&#39;s main class. The jar file containing the class must be in the default CLASSPATH or specified in jar_file_uris.
* `main-jar-file-uri=sit`
    - The HCFS URI of the jar file containing the main class. Examples:  &#39;gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar&#39;  &#39;hdfs:/tmp/test-samples/custom-wordcount.jar&#39;  &#39;file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar&#39;
* `properties=key=gubergren`
    - Optional. A mapping of property names to values, used to configure Hadoop. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site and classes in user code.
    - the value will be associated with the given `key`

* `..hive-job    continue-on-failure=true`
    - Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries.
* `jar-file-uris=amet`
    - Optional. HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs.
    - Each invocation of this argument appends the given value to the array.
* `properties=key=eirmod`
    - Optional. A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code.
    - the value will be associated with the given `key`
* `query-file-uri=sanctus`
    - The HCFS URI of the script that contains Hive queries.
* `query-list    queries=lorem`
    - Required. The queries to execute. You do not need to terminate a query with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of an Cloud Dataproc API snippet that uses a QueryList to specify a HiveJob:
        &#34;hiveJob&#34;: {
          &#34;queryList&#34;: {
            &#34;queries&#34;: [
              &#34;query1&#34;,
              &#34;query2&#34;,
              &#34;query3;query4&#34;,
            ]
          }
        }
        
    - Each invocation of this argument appends the given value to the array.

* `..    script-variables=key=amet.`
    - Optional. Mapping of query variable names to values (equivalent to the Hive command: SET name=&#34;value&#34;;).
    - the value will be associated with the given `key`

* `..    labels=key=diam`
    - Optional. The labels to associate with this job. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a job.
    - the value will be associated with the given `key`
* `pig-job    continue-on-failure=true`
    - Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries.
* `jar-file-uris=sadipscing`
    - Optional. HCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs.
    - Each invocation of this argument appends the given value to the array.
* `logging-config    driver-log-levels=key=lorem`
    - The per-package log levels for the driver. This may include &#34;root&#34; package name to configure rootLogger. Examples:  &#39;com.google = FATAL&#39;, &#39;root = INFO&#39;, &#39;org.apache = DEBUG&#39;
    - the value will be associated with the given `key`

* `..    properties=key=sed`
    - Optional. A mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code.
    - the value will be associated with the given `key`
* `query-file-uri=sit`
    - The HCFS URI of the script that contains the Pig queries.
* `query-list    queries=dolore`
    - Required. The queries to execute. You do not need to terminate a query with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of an Cloud Dataproc API snippet that uses a QueryList to specify a HiveJob:
        &#34;hiveJob&#34;: {
          &#34;queryList&#34;: {
            &#34;queries&#34;: [
              &#34;query1&#34;,
              &#34;query2&#34;,
              &#34;query3;query4&#34;,
            ]
          }
        }
        
    - Each invocation of this argument appends the given value to the array.

* `..    script-variables=key=et`
    - Optional. Mapping of query variable names to values (equivalent to the Pig command: name=[value]).
    - the value will be associated with the given `key`

* `..placement    cluster-name=at`
    - Required. The name of the cluster where the job will be submitted.
* `cluster-uuid=sit`
    - Output only. A cluster UUID generated by the Cloud Dataproc service when the job is submitted.

* `..pyspark-job    archive-uris=ut`
    - Optional. HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip.
    - Each invocation of this argument appends the given value to the array.
* `args=diam`
    - Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    - Each invocation of this argument appends the given value to the array.
* `file-uris=tempor`
    - Optional. HCFS URIs of files to be copied to the working directory of Python drivers and distributed tasks. Useful for naively parallel tasks.
    - Each invocation of this argument appends the given value to the array.
* `jar-file-uris=et`
    - Optional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks.
    - Each invocation of this argument appends the given value to the array.
* `logging-config    driver-log-levels=key=erat`
    - The per-package log levels for the driver. This may include &#34;root&#34; package name to configure rootLogger. Examples:  &#39;com.google = FATAL&#39;, &#39;root = INFO&#39;, &#39;org.apache = DEBUG&#39;
    - the value will be associated with the given `key`

* `..    main-python-file-uri=dolores`
    - Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file.
* `properties=key=kasd`
    - Optional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    - the value will be associated with the given `key`
* `python-file-uris=et`
    - Optional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip.
    - Each invocation of this argument appends the given value to the array.

* `..reference    job-id=clita`
    - Optional. The job ID, which must be unique within the project. The job ID is generated by the server upon job submission or provided by the user as a means to perform retries without creating duplicate jobs. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or hyphens (-). The maximum length is 100 characters.
* `project-id=sed`
    - Required. The ID of the Google Cloud Platform project that the job belongs to.

* `..scheduling    max-failures-per-hour=83`
    - Optional. Maximum number of times per hour a driver may be restarted as a result of driver terminating with non-zero code before job is reported failed.A job may be reported as thrashing if driver exits with non-zero code 4 times within 10 minute window.Maximum value is 10.

* `..spark-job    archive-uris=clita`
    - Optional. HCFS URIs of archives to be extracted in the working directory of Spark drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    - Each invocation of this argument appends the given value to the array.
* `args=eos`
    - Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    - Each invocation of this argument appends the given value to the array.
* `file-uris=amet`
    - Optional. HCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks.
    - Each invocation of this argument appends the given value to the array.
* `jar-file-uris=sed`
    - Optional. HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks.
    - Each invocation of this argument appends the given value to the array.
* `logging-config    driver-log-levels=key=takimata`
    - The per-package log levels for the driver. This may include &#34;root&#34; package name to configure rootLogger. Examples:  &#39;com.google = FATAL&#39;, &#39;root = INFO&#39;, &#39;org.apache = DEBUG&#39;
    - the value will be associated with the given `key`

* `..    main-class=sit`
    - The name of the driver&#39;s main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris.
* `main-jar-file-uri=labore`
    - The HCFS URI of the jar file that contains the main class.
* `properties=key=nonumy`
    - Optional. A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    - the value will be associated with the given `key`

* `..spark-sql-job    jar-file-uris=erat`
    - Optional. HCFS URIs of jar files to be added to the Spark CLASSPATH.
    - Each invocation of this argument appends the given value to the array.
* `logging-config    driver-log-levels=key=gubergren`
    - The per-package log levels for the driver. This may include &#34;root&#34; package name to configure rootLogger. Examples:  &#39;com.google = FATAL&#39;, &#39;root = INFO&#39;, &#39;org.apache = DEBUG&#39;
    - the value will be associated with the given `key`

* `..    properties=key=erat`
    - Optional. A mapping of property names to values, used to configure Spark SQL&#39;s SparkConf. Properties that conflict with values set by the Cloud Dataproc API may be overwritten.
    - the value will be associated with the given `key`
* `query-file-uri=et`
    - The HCFS URI of the script that contains SQL queries.
* `query-list    queries=amet`
    - Required. The queries to execute. You do not need to terminate a query with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of an Cloud Dataproc API snippet that uses a QueryList to specify a HiveJob:
        &#34;hiveJob&#34;: {
          &#34;queryList&#34;: {
            &#34;queries&#34;: [
              &#34;query1&#34;,
              &#34;query2&#34;,
              &#34;query3;query4&#34;,
            ]
          }
        }
        
    - Each invocation of this argument appends the given value to the array.

* `..    script-variables=key=lorem`
    - Optional. Mapping of query variable names to values (equivalent to the Spark SQL command: SET name=&#34;value&#34;;).
    - the value will be associated with the given `key`

* `..status    details=voluptua.`
    - Output only. Optional job state details, such as an error description if the state is &lt;code&gt;ERROR&lt;/code&gt;.
* `state=rebum.`
    - Output only. A state message specifying the overall job state.
* `state-start-time=justo`
    - Output only. The time when this state was entered.
* `substate=labore`
    - Output only. Additional state information, which includes status reported by the agent.



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
    - Required. Specifies the path, relative to &lt;code&gt;Job&lt;/code&gt;, of the field to update. For example, to update the labels of a Job the &lt;code&gt;update_mask&lt;/code&gt; parameter would be specified as &lt;code&gt;labels&lt;/code&gt;, and the PATCH request body would specify the new value. &lt;strong&gt;Note:&lt;/strong&gt; Currently, &lt;code&gt;labels&lt;/code&gt; is the only field that can be updated.

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
