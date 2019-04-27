Demotes the stand-alone instance to be a Cloud SQL read replica for an external database server.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/sqlservice.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `sqladmin1-beta4 --scope <scope> instances demote-master ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - ID of the project that contains the instance.
* **&lt;instance&gt;** *(string)*
    - Cloud SQL instance name.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
InstancesDemoteMasterRequest:
  demote-master-context:
    kind: string
    master-instance-name: string
    replica-configuration:
      kind: string
      mysql-replica-configuration:
        ca-certificate: string
        client-certificate: string
        client-key: string
        kind: string
        password: string
        username: string
    verify-gtid-consistency: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .demote-master-context    kind=invidunt`
    - This is always sql#demoteMasterContext.
* `master-instance-name=aliquyam`
    - The name of the instance which will act as on-premises master in the replication setup.
* `replica-configuration    kind=accusam`
    - This is always sql#demoteMasterConfiguration.
* `mysql-replica-configuration    ca-certificate=lorem`
    - PEM representation of the trusted CA&#39;s x509 certificate.
* `client-certificate=sea`
    - PEM representation of the slave&#39;s x509 certificate.
* `client-key=et`
    - PEM representation of the slave&#39;s private key. The corresponsing public key is encoded in the client&#39;s certificate. The format of the slave&#39;s private key can be either PKCS #1 or PKCS #8.
* `kind=duo`
    - This is always sql#demoteMasterMysqlReplicaConfiguration.
* `password=et`
    - The password for the replication connection.
* `username=eirmod`
    - The username for the replication connection.


* `...    verify-gtid-consistency=false`
    - Verify GTID consistency for demote operation. Default value: True. Second Generation instances only. Setting this flag to false enables you to bypass GTID consistency check between on-premises master and Cloud SQL instance during the demotion operation but also exposes you to the risk of future replication failures. Change the value only if you know the reason for the GTID divergence and are confident that doing so will not cause any replication issues.



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
