Creates a set of partition tokens that can be used to execute a read
operation in parallel.  Each of the returned partition tokens can be used
by StreamingRead to specify a subset of the read
result to read.  The same session and read-only transaction must be used by
the PartitionReadRequest used to create the partition tokens and the
ReadRequests that use the partition tokens.  There are no ordering
guarantees on rows returned among the returned partition tokens, or even
within each individual StreamingRead call issued with a partition_token.

Partition tokens become invalid when the session used to create them
is deleted, is idle for too long, begins a new transaction, or becomes too
old.  When any of these happen, it is not possible to resume the read, and
the whole operation must be restarted from the beginning.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/spanner.data*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `spanner1 --scope <scope> projects instances-databases-sessions-partition-read ...`
# Required Scalar Argument
* **&lt;session&gt;** *(string)*
    - Required. The session used to create the partitions.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PartitionReadRequest:
  columns: [string]
  index: string
  key-set:
    all: boolean
  partition-options:
    max-partitions: string
    partition-size-bytes: string
  table: string
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

* `-r .    columns=sadipscing`
    - The columns of table to be returned for each row matching
        this request.
    - Each invocation of this argument appends the given value to the array.
* `index=invidunt`
    - If non-empty, the name of an index on table. This index is
        used instead of the table primary key when interpreting key_set
        and sorting result rows. See key_set for further information.
* `key-set    all=false`
    - For convenience `all` can be set to `true` to indicate that this
        `KeySet` matches all keys in the table or index. Note that any keys
        specified in `keys` or `ranges` are only yielded once.

* `..partition-options    max-partitions=dolore`
    - **Note:** This hint is currently ignored by PartitionQuery and
        PartitionRead requests.
        
        The desired maximum number of partitions to return.  For example, this may
        be set to the number of workers available.  The default for this option
        is currently 10,000. The maximum value is currently 200,000.  This is only
        a hint.  The actual number of partitions returned may be smaller or larger
        than this maximum count request.
* `partition-size-bytes=duo`
    - **Note:** This hint is currently ignored by PartitionQuery and
        PartitionRead requests.
        
        The desired data size for each partition generated.  The default for this
        option is currently 1 GiB.  This is only a hint. The actual size of each
        partition may be smaller or larger than this size request.

* `..    table=aliquyam`
    - Required. The name of the table in the database to be read.
* `transaction.begin.read-only    exact-staleness=lorem`
    - Executes all reads at a timestamp that is `exact_staleness`
        old. The timestamp is chosen soon after the read is started.
        
        Guarantees that all writes that have committed more than the
        specified number of seconds ago are visible. Because Cloud Spanner
        chooses the exact timestamp, this mode works even if the client&#39;s
        local clock is substantially skewed from Cloud Spanner commit
        timestamps.
        
        Useful for reading at nearby replicas without the distributed
        timestamp negotiation overhead of `max_staleness`.
* `max-staleness=et`
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
* `min-read-timestamp=clita`
    - Executes all reads at a timestamp &gt;= `min_read_timestamp`.
        
        This is useful for requesting fresher data than some previous
        read, or data that is fresh enough to observe the effects of some
        previously committed transaction whose timestamp is known.
        
        Note that this option can only be used in single-use transactions.
        
        A timestamp in RFC3339 UTC \&#34;Zulu\&#34; format, accurate to nanoseconds.
        Example: `&#34;2014-10-02T15:01:23.045123456Z&#34;`.
* `read-timestamp=consetetur`
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


* `...    id=kasd`
    - Execute the read or SQL query in a previously-started transaction.
* `single-use.read-only    exact-staleness=sanctus`
    - Executes all reads at a timestamp that is `exact_staleness`
        old. The timestamp is chosen soon after the read is started.
        
        Guarantees that all writes that have committed more than the
        specified number of seconds ago are visible. Because Cloud Spanner
        chooses the exact timestamp, this mode works even if the client&#39;s
        local clock is substantially skewed from Cloud Spanner commit
        timestamps.
        
        Useful for reading at nearby replicas without the distributed
        timestamp negotiation overhead of `max_staleness`.
* `max-staleness=takimata`
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
* `min-read-timestamp=at`
    - Executes all reads at a timestamp &gt;= `min_read_timestamp`.
        
        This is useful for requesting fresher data than some previous
        read, or data that is fresh enough to observe the effects of some
        previously committed transaction whose timestamp is known.
        
        Note that this option can only be used in single-use transactions.
        
        A timestamp in RFC3339 UTC \&#34;Zulu\&#34; format, accurate to nanoseconds.
        Example: `&#34;2014-10-02T15:01:23.045123456Z&#34;`.
* `read-timestamp=labore`
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
* `return-read-timestamp=true`
    - If true, the Cloud Spanner-selected read timestamp is included in
        the Transaction message that describes the transaction.
* `strong=false`
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
