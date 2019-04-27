Updates a specific model resource.

Currently the only supported fields to update are `description` and
`default_version.name`.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `ml1 --scope <scope> projects models-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required. The project name.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudMlV1__Model:
  default-version:
    auto-scaling:
      min-nodes: integer
    create-time: string
    deployment-uri: string
    description: string
    error-message: string
    etag: string
    framework: string
    is-default: boolean
    labels: { string: string }
    last-use-time: string
    machine-type: string
    manual-scaling:
      nodes: integer
    name: string
    python-version: string
    runtime-version: string
    state: string
  description: string
  etag: string
  labels: { string: string }
  name: string
  online-prediction-logging: boolean
  regions: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .default-version.auto-scaling    min-nodes=78`
    - Optional. The minimum number of nodes to allocate for this model. These
        nodes are always up, starting from the time the model is deployed.
        Therefore, the cost of operating this model will be at least
        `rate` * `min_nodes` * number of hours since last billing cycle,
        where `rate` is the cost per node-hour as documented in the
        [pricing guide](/ml-engine/docs/pricing),
        even if no predictions are performed. There is additional cost for each
        prediction performed.
        
        Unlike manual scaling, if the load gets too heavy for the nodes
        that are up, the service will automatically add nodes to handle the
        increased load as well as scale back as traffic drops, always maintaining
        at least `min_nodes`. You will be charged for the time in which additional
        nodes are used.
        
        If not specified, `min_nodes` defaults to 0, in which case, when traffic
        to a model stops (and after a cool-down period), nodes will be shut down
        and no charges will be incurred until traffic to the model resumes.
        
        You can set `min_nodes` when creating the model version, and you can also
        update `min_nodes` for an existing version:
        &lt;pre&gt;
        update_body.json:
        {
          &#39;autoScaling&#39;: {
            &#39;minNodes&#39;: 5
          }
        }
        &lt;/pre&gt;
        HTTP request:
        &lt;pre&gt;
        PATCH https://ml.googleapis.com/v1/{name=projects/*/models/*/versions/*}?update_mask=autoScaling.minNodes -d @./update_body.json
        &lt;/pre&gt;

* `..    create-time=clita`
    - Output only. The time the version was created.
* `deployment-uri=diam`
    - Required. The Google Cloud Storage location of the trained model used to
        create the version. See the
        [guide to model
        deployment](/ml-engine/docs/tensorflow/deploying-models) for more
        information.
        
        When passing Version to
        [projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create)
        the model service uses the specified location as the source of the model.
        Once deployed, the model version is hosted by the prediction service, so
        this location is useful only as a historical record.
        The total number of model files can&#39;t exceed 1000.
* `description=justo`
    - Optional. The description specified for the version when it was created.
* `error-message=est`
    - Output only. The details of a failure or a cancellation.
* `etag=clita`
    - `etag` is used for optimistic concurrency control as a way to help
        prevent simultaneous updates of a model from overwriting each other.
        It is strongly suggested that systems make use of the `etag` in the
        read-modify-write cycle to perform model updates in order to avoid race
        conditions: An `etag` is returned in the response to `GetVersion`, and
        systems are expected to put that etag in the request to `UpdateVersion` to
        ensure that their change will be applied to the model as intended.
* `framework=invidunt`
    - Optional. The machine learning framework Cloud ML Engine uses to train
        this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`,
        `XGBOOST`. If you do not specify a framework, Cloud ML Engine
        will analyze files in the deployment_uri to determine a framework. If you
        choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version
        of the model to 1.4 or greater.
* `is-default=false`
    - Output only. If true, this version will be used to handle prediction
        requests that do not specify a version.
        
        You can change the default version by calling
        [projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault).
* `labels=key=dolores`
    - Optional. One or more labels that you can add, to organize your model
        versions. Each label is a key-value pair, where both the key and the value
        are arbitrary strings that you supply.
        For more information, see the documentation on
        &lt;a href=&#34;/ml-engine/docs/tensorflow/resource-labels&#34;&gt;using labels&lt;/a&gt;.
    - the value will be associated with the given `key`
* `last-use-time=eos`
    - Output only. The time the version was last used for prediction.
* `machine-type=voluptua.`
    - Optional. The type of machine on which to serve the model. Currently only
        applies to online prediction service.
        The following are currently supported and will be deprecated in
        Beta release.
          mls1-highmem-1    1 core    2 Gb RAM
          mls1-highcpu-4    4 core    2 Gb RAM
        The following are available in Beta:
          mls1-c1-m2        1 core    2 Gb RAM   Default
          mls1-c4-m2        4 core    2 Gb RAM
* `manual-scaling    nodes=82`
    - The number of nodes to allocate for this model. These nodes are always up,
        starting from the time the model is deployed, so the cost of operating
        this model will be proportional to `nodes` * number of hours since
        last billing cycle plus the cost for each prediction performed.

* `..    name=sed`
    - Required.The name specified for the version when it was created.
        
        The version name must be unique within the model it is created in.
* `python-version=aliquyam`
    - Optional. The version of Python used in prediction. If not set, the default
        version is &#39;2.7&#39;. Python &#39;3.5&#39; is available when `runtime_version` is set
        to &#39;1.4&#39; and above. Python &#39;2.7&#39; works with all supported runtime versions.
* `runtime-version=ea`
    - Optional. The Google Cloud ML runtime version to use for this deployment.
        If not set, Google Cloud ML will choose a version.
* `state=ea`
    - Output only. The state of a version.

* `..    description=et`
    - Optional. The description specified for the model when it was created.
* `etag=dolor`
    - `etag` is used for optimistic concurrency control as a way to help
        prevent simultaneous updates of a model from overwriting each other.
        It is strongly suggested that systems make use of the `etag` in the
        read-modify-write cycle to perform model updates in order to avoid race
        conditions: An `etag` is returned in the response to `GetModel`, and
        systems are expected to put that etag in the request to `UpdateModel` to
        ensure that their change will be applied to the model as intended.
* `labels=key=diam`
    - Optional. One or more labels that you can add, to organize your models.
        Each label is a key-value pair, where both the key and the value are
        arbitrary strings that you supply.
        For more information, see the documentation on
        &lt;a href=&#34;/ml-engine/docs/tensorflow/resource-labels&#34;&gt;using labels&lt;/a&gt;.
    - the value will be associated with the given `key`
* `name=kasd`
    - Required. The name specified for the model when it was created.
        
        The model name must be unique within the project it is created in.
* `online-prediction-logging=false`
    - Optional. If true, enables StackDriver Logging for online prediction.
        Default is false.
* `regions=rebum.`
    - Optional. The list of regions where the model is going to be deployed.
        Currently only one region per model is supported.
        Defaults to &#39;us-central1&#39; if nothing is set.
        See the &lt;a href=&#34;/ml-engine/docs/tensorflow/regions&#34;&gt;available regions&lt;/a&gt;
        for ML Engine services.
        Note:
        *   No matter where a model is deployed, it can always be accessed by
            users from anywhere, both for online and batch prediction.
        *   The region for a batch prediction job is set by the region field when
            submitting the batch prediction job and does not take its value from
            this field.
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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p update-mask=string**
    - Required. Specifies the path, relative to `Model`, of the field to update.
        
        For example, to change the description of a model to &#34;foo&#34; and set its
        default version to &#34;version_1&#34;, the `update_mask` parameter would be
        specified as `description`, `default_version.name`, and the `PATCH`
        request body would specify the new value, as follows:
            {
              &#34;description&#34;: &#34;foo&#34;,
              &#34;defaultVersion&#34;: {
                &#34;name&#34;:&#34;version_1&#34;
              }
            }
        
        Currently the supported update masks are `description` and
        `default_version.name`.

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
