Like ExecuteSql, except returns the result
set as a stream. Unlike ExecuteSql, there
is no limit on the size of the returned result set. However, no
individual row in the result set can exceed 100 MiB, and no
column value can exceed 10 MiB.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/spanner.data*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `spanner1 --scope <scope> projects instances-databases-sessions-execute-streaming-sql ...`
# Required Scalar Argument
* **&lt;session&gt;** *(string)*
    - Required. The session in which the SQL query should be performed.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ExecuteSqlRequest:
  partition-token: string
  query-mode: string
  resume-token: string
  seqno: string
  sql: string
  transaction:
    begin:
      read-only:
        exact-staleness: string
        max-staleness: string
        min-read-timestamp: string
        read-timestamp: string
        return-read-timestamp: boolean
        strong: boolean
    id: string
    single-use:
      read-only:
        exact-staleness: string
        max-staleness: string
        min-read-timestamp: string
        read-timestamp: string
        return-read-timestamp: boolean
        strong: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    partition-token=invidunt`
    - If present, results will be restricted to the specified partition
        previously created using PartitionQuery().  There must be an exact
        match for the values of fields common to this message and the
        PartitionQueryRequest message used to create this partition_token.
* `query-mode=aliquyam`
    - Used to control the amount of debugging information returned in
        ResultSetStats. If partition_token is set, query_mode can only
        be set to QueryMode.NORMAL.
* `resume-token=accusam`
    - If this request is resuming a previously interrupted SQL statement
        execution, `resume_token` should be copied from the last
        PartialResultSet yielded before the interruption. Doing this
        enables the new SQL statement execution to resume where the last one left
        off. The rest of the request parameters must exactly match the
        request that yielded this token.
* `seqno=lorem`
    - A per-transaction sequence number used to identify this request. This
        makes each request idempotent such that if the request is received multiple
        times, at most one will succeed.
        
        The sequence number must be monotonically increasing within the
        transaction. If a request arrives for the first time with an out-of-order
        sequence number, the transaction may be aborted. Replays of previously
        handled requests will yield the same response as the first execution.
        
        Required for DML statements. Ignored for queries.
* `sql=sea`
    - Required. The SQL string.
* `transaction.begin.read-only    exact-staleness=et`
    - Executes all reads at a timestamp that is `exact_staleness`
        old. The timestamp is chosen soon after the read is started.
        
        Guarantees that all writes that have committed more than the
        specified number of seconds ago are visible. Because Cloud Spanner
        chooses the exact timestamp, this mode works even if the client&#39;s
        local clock is substantially skewed from Cloud Spanner commit
        timestamps.
        
        Useful for reading at nearby replicas without the distributed
        timestamp negotiation overhead of `max_staleness`.
* `max-staleness=duo`
    - Read data at a timestamp &gt;= `NOW - max_staleness`
        seconds. Guarantees that all writes that have committed more
        than the specified number of seconds ago are visible. Because
        Cloud Spanner chooses the exact timestamp, this mode works even if
        the client&#39;s local clock is substantially skewed from Cloud Spanner
        commit timestamps.
        
        Useful for reading the freshest data available at a nearby
        replica, while bounding the possible staleness if the local
        replica has fallen behind.
        
        Note that this option can only be used in single-use
        transactions.
* `min-read-timestamp=et`
    - Executes all reads at a timestamp &gt;= `min_read_timestamp`.
        
        This is useful for requesting fresher data than some previous
        read, or data that is fresh enough to observe the effects of some
        previously committed transaction whose timestamp is known.
        
        Note that this option can only be used in single-use transactions.
        
        A timestamp in RFC3339 UTC \&#34;Zulu\&#34; format, accurate to nanoseconds.
        Example: `&#34;2014-10-02T15:01:23.045123456Z&#34;`.
* `read-timestamp=eirmod`
    - Executes all reads at the given timestamp. Unlike other modes,
        reads at a specific timestamp are repeatable; the same read at
        the same timestamp always returns the same data. If the
        timestamp is in the future, the read will block until the
        specified timestamp, modulo the read&#39;s deadline.
        
        Useful for large scale consistent reads such as mapreduces, or
        for coordinating many reads against a consistent snapshot of the
        data.
        
        A timestamp in RFC3339 UTC \&#34;Zulu\&#34; format, accurate to nanoseconds.
        Example: `&#34;2014-10-02T15:01:23.045123456Z&#34;`.
* `return-read-timestamp=false`
    - If true, the Cloud Spanner-selected read timestamp is included in
        the Transaction message that describes the transaction.
* `strong=true`
    - Read at a timestamp where all previously committed transactions
        are visible.


* `...    id=amet`
    - Execute the read or SQL query in a previously-started transaction.
* `single-use.read-only    exact-staleness=et`
    - Executes all reads at a timestamp that is `exact_staleness`
        old. The timestamp is chosen soon after the read is started.
        
        Guarantees that all writes that have committed more than the
        specified number of seconds ago are visible. Because Cloud Spanner
        chooses the exact timestamp, this mode works even if the client&#39;s
        local clock is substantially skewed from Cloud Spanner commit
        timestamps.
        
        Useful for reading at nearby replicas without the distributed
        timestamp negotiation overhead of `max_staleness`.
* `max-staleness=consetetur`
    - Read data at a timestamp &gt;= `NOW - max_staleness`
        seconds. Guarantees that all writes that have committed more
        than the specified number of seconds ago are visible. Because
        Cloud Spanner chooses the exact timestamp, this mode works even if
        the client&#39;s local clock is substantially skewed from Cloud Spanner
        commit timestamps.
        
        Useful for reading the freshest data available at a nearby
        replica, while bounding the possible staleness if the local
        replica has fallen behind.
        
        Note that this option can only be used in single-use
        transactions.
* `min-read-timestamp=ut`
    - Executes all reads at a timestamp &gt;= `min_read_timestamp`.
        
        This is useful for requesting fresher data than some previous
        read, or data that is fresh enough to observe the effects of some
        previously committed transaction whose timestamp is known.
        
        Note that this option can only be used in single-use transactions.
        
        A timestamp in RFC3339 UTC \&#34;Zulu\&#34; format, accurate to nanoseconds.
        Example: `&#34;2014-10-02T15:01:23.045123456Z&#34;`.
* `read-timestamp=ea`
    - Executes all reads at the given timestamp. Unlike other modes,
        reads at a specific timestamp are repeatable; the same read at
        the same timestamp always returns the same data. If the
        timestamp is in the future, the read will block until the
        specified timestamp, modulo the read&#39;s deadline.
        
        Useful for large scale consistent reads such as mapreduces, or
        for coordinating many reads against a consistent snapshot of the
        data.
        
        A timestamp in RFC3339 UTC \&#34;Zulu\&#34; format, accurate to nanoseconds.
        Example: `&#34;2014-10-02T15:01:23.045123456Z&#34;`.
* `return-read-timestamp=false`
    - If true, the Cloud Spanner-selected read timestamp is included in
        the Transaction message that describes the transaction.
* `strong=true`
    - Read at a timestamp where all previously committed transactions
        are visible.





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
