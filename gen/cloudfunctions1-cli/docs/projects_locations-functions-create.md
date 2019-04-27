Creates a new function. If a function with the given name already exists in
the specified project, the long running operation will return
`ALREADY_EXISTS` error.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudfunctions1 --scope <scope> projects locations-functions-create ...`
# Required Scalar Argument
* **&lt;location&gt;** *(string)*
    - The project and location in which the function should be created, specified
        in the format `projects/*/locations/*`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CloudFunction:
  available-memory-mb: integer
  description: string
  entry-point: string
  environment-variables: { string: string }
  event-trigger:
    event-type: string
    resource: string
    service: string
  https-trigger:
    url: string
  labels: { string: string }
  max-instances: integer
  name: string
  network: string
  runtime: string
  service-account-email: string
  source-archive-url: string
  source-repository:
    deployed-url: string
    url: string
  source-upload-url: string
  status: string
  timeout: string
  update-time: string
  version-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    available-memory-mb=53`
    - The amount of memory in MB available for a function.
        Defaults to 256MB.
* `description=stet`
    - User-provided description of a function.
* `entry-point=sed`
    - The name of the function (as defined in source code) that will be
        executed. Defaults to the resource name suffix, if not specified. For
        backward compatibility, if function with given name is not found, then the
        system will try to use function named &#34;function&#34;.
        For Node.js this is name of a function exported by the module specified
        in `source_location`.
* `environment-variables=key=et`
    - **Beta Feature**
        
        Environment variables that shall be available during function execution.
    - the value will be associated with the given `key`
* `event-trigger    event-type=dolores`
    - Required. The type of event to observe. For example:
        `providers/cloud.storage/eventTypes/object.change` and
        `providers/cloud.pubsub/eventTypes/topic.publish`.
        
        Event types match pattern `providers/*/eventTypes/*.*`.
        The pattern contains:
        
        1. namespace: For example, `cloud.storage` and
           `google.firebase.analytics`.
        2. resource type: The type of resource on which event occurs. For
           example, the Google Cloud Storage API includes the type `object`.
        3. action: The action that generates the event. For example, action for
           a Google Cloud Storage Object is &#39;change&#39;.
        These parts are lower case.
* `resource=kasd`
    - Required. The resource(s) from which to observe events, for example,
        `projects/_/buckets/myBucket`.
        
        Not all syntactically correct values are accepted by all services. For
        example:
        
        1. The authorization model must support it. Google Cloud Functions
           only allows EventTriggers to be deployed that observe resources in the
           same project as the `CloudFunction`.
        2. The resource type must match the pattern expected for an
           `event_type`. For example, an `EventTrigger` that has an
           `event_type` of &#34;google.pubsub.topic.publish&#34; should have a resource
           that matches Google Cloud Pub/Sub topics.
        
        Additionally, some services may support short names when creating an
        `EventTrigger`. These will always be returned in the normalized &#34;long&#34;
        format.
        
        See each *service&#39;s* documentation for supported formats.
* `service=accusam`
    - The hostname of the service that should be observed.
        
        If no string is provided, the default service implementing the API will
        be used. For example, `storage.googleapis.com` is the default for all
        event types in the `google.storage` namespace.

* `..https-trigger    url=takimata`
    - Output only. The deployed url for the function.

* `..    labels=key=justo`
    - Labels associated with this Cloud Function.
    - the value will be associated with the given `key`
* `max-instances=100`
    - The limit on the maximum number of function instances that may coexist at a
        given time. This feature is currently in alpha, available only for
        whitelisted users.
* `name=erat`
    - A user-defined name of the function. Function names must be unique
        globally and match pattern `projects/*/locations/*/functions/*`
* `network=labore`
    - The VPC Network that this cloud function can connect to. It can be
        either the fully-qualified URI, or the short name of the network resource.
        If the short network name is used, the network must belong to the same
        project. Otherwise, it must belong to a project within the same
        organization. The format of this field is either
        `projects/{project}/global/networks/{network}` or `{network}`, where
        {project} is a project id where the network is defined, and {network} is
        the short name of the network.
        
        See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
        more information on connecting Cloud projects.
        
        This feature is currently in alpha, available only for whitelisted users.
* `runtime=sea`
    - The runtime in which the function is going to run. If empty, defaults to
        Node.js 6.
* `service-account-email=nonumy`
    - Output only. The email of the function&#39;s service account.
* `source-archive-url=dolores`
    - The Google Cloud Storage URL, starting with gs://, pointing to the zip
        archive which contains the function.
* `source-repository    deployed-url=gubergren`
    - Output only. The URL pointing to the hosted repository where the function
        were defined at the time of deployment. It always points to a specific
        commit in the format described above.
* `url=sadipscing`
    - The URL pointing to the hosted repository where the function is defined.
        There are supported Cloud Source Repository URLs in the following
        formats:
        
        To refer to a specific commit:
        `https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`
        To refer to a moveable alias (branch):
        `https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`
        In particular, to refer to HEAD use `master` moveable alias.
        To refer to a specific fixed alias (tag):
        `https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`
        
        You may omit `paths/*` if you want to use the main directory.

* `..    source-upload-url=aliquyam`
    - The Google Cloud Storage signed URL used for source uploading, generated
        by google.cloud.functions.v1.GenerateUploadUrl
* `status=ea`
    - Output only. Status of the function deployment.
* `timeout=no`
    - The function execution timeout. Execution is considered failed and
        can be terminated if the function is not completed at the end of the
        timeout period. Defaults to 60 seconds.
* `update-time=justo`
    - Output only. The last update timestamp of a Cloud Function.
* `version-id=justo`
    - Output only.
        The version identifier of the Cloud Function. Each deployment attempt
        results in a new version of a function being created.


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
