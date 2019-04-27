Updates settings of a Cloud SQL instance. Caution: This is not a partial update, so you must include values for all the settings that you want to retain. For partial updates, use patch.. This method supports patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/sqlservice.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `sqladmin1-beta4 --scope <scope> instances patch ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID of the project that contains the instance.
* **&lt;instance&gt;** *(string)*
    - Cloud SQL instance ID. This does not include the project ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
DatabaseInstance:
  backend-type: string
  connection-name: string
  current-disk-size: string
  database-version: string
  etag: string
  failover-replica:
    available: boolean
    name: string
  gce-zone: string
  instance-type: string
  ipv6-address: string
  kind: string
  master-instance-name: string
  max-disk-size: string
  name: string
  on-premises-configuration:
    host-port: string
    kind: string
  project: string
  region: string
  replica-configuration:
    failover-target: boolean
    kind: string
    mysql-replica-configuration:
      ca-certificate: string
      client-certificate: string
      client-key: string
      connect-retry-interval: integer
      dump-file-path: string
      kind: string
      master-heartbeat-period: string
      password: string
      ssl-cipher: string
      username: string
      verify-server-certificate: boolean
  replica-names: [string]
  self-link: string
  server-ca-cert:
    cert: string
    cert-serial-number: string
    common-name: string
    create-time: string
    expiration-time: string
    instance: string
    kind: string
    self-link: string
    sha1-fingerprint: string
  service-account-email-address: string
  settings:
    activation-policy: string
    authorized-gae-applications: [string]
    availability-type: string
    backup-configuration:
      binary-log-enabled: boolean
      enabled: boolean
      kind: string
      replication-log-archiving-enabled: boolean
      start-time: string
    crash-safe-replication-enabled: boolean
    data-disk-size-gb: string
    data-disk-type: string
    database-replication-enabled: boolean
    ip-configuration:
      ipv4-enabled: boolean
      private-network: string
      require-ssl: boolean
    kind: string
    location-preference:
      follow-gae-application: string
      kind: string
      zone: string
    maintenance-window:
      day: integer
      hour: integer
      kind: string
      update-track: string
    pricing-plan: string
    replication-type: string
    settings-version: string
    storage-auto-resize: boolean
    storage-auto-resize-limit: string
    tier: string
    user-labels: { string: string }
  state: string
  suspension-reason: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    backend-type=rebum.`
    - FIRST_GEN: First Generation instance. MySQL only.
        SECOND_GEN: Second Generation instance or PostgreSQL instance.
        EXTERNAL: A database server that is not managed by Google.
        This property is read-only; use the tier property in the settings object to determine the database type and Second or First Generation.
* `connection-name=lorem`
    - Connection name of the Cloud SQL instance used in connection strings.
* `current-disk-size=clita`
    - The current disk usage of the instance in bytes. This property has been deprecated. Users should use the &#34;cloudsql.googleapis.com/database/disk/bytes_used&#34; metric in Cloud Monitoring API instead. Please see this announcement for details.
* `database-version=invidunt`
    - The database engine type and version. The databaseVersion field can not be changed after instance creation. MySQL Second Generation instances: MYSQL_5_7 (default) or MYSQL_5_6. PostgreSQL instances: POSTGRES_9_6 MySQL First Generation instances: MYSQL_5_6 (default) or MYSQL_5_5
* `etag=eirmod`
    - This field is deprecated and will be removed from a future version of the API. Use the settings.settingsVersion field instead.
* `failover-replica    available=false`
    - The availability status of the failover replica. A false status indicates that the failover replica is out of sync. The master can only failover to the falover replica when the status is true.
* `name=consetetur`
    - The name of the failover replica. If specified at instance creation, a failover replica is created for the instance. The name doesn&#39;t include the project ID. This property is applicable only to Second Generation instances.

* `..    gce-zone=et`
    - The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone.
* `instance-type=sed`
    - The instance type. This can be one of the following.
        CLOUD_SQL_INSTANCE: A Cloud SQL instance that is not replicating from a master.
        ON_PREMISES_INSTANCE: An instance running on the customer&#39;s premises.
        READ_REPLICA_INSTANCE: A Cloud SQL instance configured as a read-replica.
* `ipv6-address=sit`
    - The IPv6 address assigned to the instance. This property is applicable only to First Generation instances.
* `kind=takimata`
    - This is always sql#instance.
* `master-instance-name=elitr`
    - The name of the instance which will act as master in the replication setup.
* `max-disk-size=nonumy`
    - The maximum disk size of the instance in bytes.
* `name=rebum.`
    - Name of the Cloud SQL instance. This does not include the project ID.
* `on-premises-configuration    host-port=lorem`
    - The host and port of the on-premises instance in host:port format
* `kind=lorem`
    - This is always sql#onPremisesConfiguration.

* `..    project=diam`
    - The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable.
* `region=ut`
    - The geographical region. Can be us-central (FIRST_GEN instances only), us-central1 (SECOND_GEN instances only), asia-east1 or europe-west1. Defaults to us-central or us-central1 depending on the instance type (First Generation or Second Generation). The region can not be changed after instance creation.
* `replica-configuration    failover-target=true`
    - Specifies if the replica is the failover target. If the field is set to true the replica will be designated as a failover replica. In case the master instance fails, the replica instance will be promoted as the new master instance.
        Only one replica can be specified as failover target, and the replica has to be in different zone with the master instance.
* `kind=amet.`
    - This is always sql#replicaConfiguration.
* `mysql-replica-configuration    ca-certificate=ipsum`
    - PEM representation of the trusted CA&#39;s x509 certificate.
* `client-certificate=ut`
    - PEM representation of the slave&#39;s x509 certificate.
* `client-key=dolor`
    - PEM representation of the slave&#39;s private key. The corresponsing public key is encoded in the client&#39;s certificate.
* `connect-retry-interval=92`
    - Seconds to wait between connect retries. MySQL&#39;s default is 60 seconds.
* `dump-file-path=ut`
    - Path to a SQL dump file in Google Cloud Storage from which the slave instance is to be created. The URI is in the form gs://bucketName/fileName. Compressed gzip files (.gz) are also supported. Dumps should have the binlog co-ordinates from which replication should begin. This can be accomplished by setting --master-data to 1 when using mysqldump.
* `kind=eirmod`
    - This is always sql#mysqlReplicaConfiguration.
* `master-heartbeat-period=sanctus`
    - Interval in milliseconds between replication heartbeats.
* `password=voluptua.`
    - The password for the replication connection.
* `ssl-cipher=dolor`
    - A list of permissible ciphers to use for SSL encryption.
* `username=et`
    - The username for the replication connection.
* `verify-server-certificate=false`
    - Whether or not to check the master&#39;s Common Name value in the certificate that it sends during the SSL handshake.


* `...    replica-names=vero`
    - The replicas of the instance.
    - Each invocation of this argument appends the given value to the array.
* `self-link=ut`
    - The URI of this resource.
* `server-ca-cert    cert=sed`
    - PEM representation.
* `cert-serial-number=et`
    - Serial number, as extracted from the certificate.
* `common-name=ipsum`
    - User supplied name. Constrained to [a-zA-Z.-_ ]+.
* `create-time=justo`
    - The time when the certificate was created in RFC 3339 format, for example 2012-11-15T16:19:00.094Z
* `expiration-time=dolore`
    - The time when the certificate expires in RFC 3339 format, for example 2012-11-15T16:19:00.094Z.
* `instance=vero`
    - Name of the database instance.
* `kind=dolor`
    - This is always sql#sslCert.
* `self-link=takimata`
    - The URI of this resource.
* `sha1-fingerprint=et`
    - Sha1 Fingerprint.

* `..    service-account-email-address=nonumy`
    - The service account email address assigned to the instance. This property is applicable only to Second Generation instances.
* `settings    activation-policy=et`
    - The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values:
        ALWAYS: The instance is on, and remains so even in the absence of connection requests.
        NEVER: The instance is off; it is not activated, even if a connection request arrives.
        ON_DEMAND: First Generation instances only. The instance responds to incoming requests, and turns itself off when not in use. Instances with PER_USE pricing turn off after 15 minutes of inactivity. Instances with PER_PACKAGE pricing turn off after 12 hours of inactivity.
* `authorized-gae-applications=sed`
    - The App Engine app IDs that can access this instance. First Generation instances only.
    - Each invocation of this argument appends the given value to the array.
* `availability-type=no`
    - Availability type (PostgreSQL instances only). Potential values:
        ZONAL: The instance serves data from only one zone. Outages in that zone affect data accessibility.
        REGIONAL: The instance can serve data from more than one zone in a region (it is highly available).
        For more information, see Overview of the High Availability Configuration.
* `backup-configuration    binary-log-enabled=true`
    - Whether binary log is enabled. If backup configuration is disabled, binary log must be disabled as well.
* `enabled=true`
    - Whether this configuration is enabled.
* `kind=labore`
    - This is always sql#backupConfiguration.
* `replication-log-archiving-enabled=false`
    - Reserved for future use.
* `start-time=elitr`
    - Start time for the daily backup configuration in UTC timezone in the 24 hour format - HH:MM.

* `..    crash-safe-replication-enabled=true`
    - Configuration specific to read replica instances. Indicates whether database flags for crash-safe replication are enabled. This property is only applicable to First Generation instances.
* `data-disk-size-gb=sea`
    - The size of data disk, in GB. The data disk size minimum is 10GB. Not used for First Generation instances.
* `data-disk-type=elitr`
    - The type of data disk: PD_SSD (default) or PD_HDD. Not used for First Generation instances.
* `database-replication-enabled=false`
    - Configuration specific to read replica instances. Indicates whether replication is enabled or not.
* `ip-configuration    ipv4-enabled=true`
    - Whether the instance should be assigned an IP address or not.
* `private-network=consetetur`
    - Reserved for future use.
* `require-ssl=false`
    - Whether SSL connections over IP should be enforced or not.

* `..    kind=accusam`
    - This is always sql#settings.
* `location-preference    follow-gae-application=dolores`
    - The AppEngine application to follow, it must be in the same region as the Cloud SQL instance.
* `kind=consetetur`
    - This is always sql#locationPreference.
* `zone=dolor`
    - The preferred Compute Engine zone (e.g. us-central1-a, us-central1-b, etc.).

* `..maintenance-window    day=19`
    - day of week (1-7), starting on Monday.
* `hour=7`
    - hour of day - 0 to 23.
* `kind=ea`
    - This is always sql#maintenanceWindow.
* `update-track=et`
    - Maintenance timing setting: canary (Earlier) or stable (Later).
         Learn more.

* `..    pricing-plan=stet`
    - The pricing plan for this instance. This can be either PER_USE or PACKAGE. Only PER_USE is supported for Second Generation instances.
* `replication-type=sed`
    - The type of replication this instance uses. This can be either ASYNCHRONOUS or SYNCHRONOUS. This property is only applicable to First Generation instances.
* `settings-version=dolor`
    - The version of instance settings. This is a required field for update method to make sure concurrent updates are handled properly. During update, use the most recent settingsVersion value for this instance and do not try to update this value.
* `storage-auto-resize=true`
    - Configuration to increase storage size automatically. The default value is true. Not used for First Generation instances.
* `storage-auto-resize-limit=dolore`
    - The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit. Not used for First Generation instances.
* `tier=lorem`
    - The tier (or machine type) for this instance, for example db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances). For MySQL instances, this property determines whether the instance is First or Second Generation. For more information, see Instance Settings.
* `user-labels=key=consetetur`
    - User-provided labels, represented as a dictionary where each label is a single key value pair.
    - the value will be associated with the given `key`

* `..    state=consetetur`
    - The current serving state of the Cloud SQL instance. This can be one of the following.
        RUNNABLE: The instance is running, or is ready to run when accessed.
        SUSPENDED: The instance is not available, for example due to problems with billing.
        PENDING_CREATE: The instance is being created.
        MAINTENANCE: The instance is down for maintenance.
        FAILED: The instance creation failed.
        UNKNOWN_STATE: The state of the instance is unknown.
* `suspension-reason=eirmod`
    - If the instance state is SUSPENDED, the reason for the suspension.
    - Each invocation of this argument appends the given value to the array.


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
