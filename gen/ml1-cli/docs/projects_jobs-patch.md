Updates a specific job resource.

Currently the only supported fields to update are `labels`.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `ml1 --scope <scope> projects jobs-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required. The job name.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudMlV1__Job:
  create-time: string
  end-time: string
  error-message: string
  etag: string
  job-id: string
  labels: { string: string }
  prediction-input:
    accelerator:
      count: string
      type: string
    batch-size: string
    data-format: string
    input-paths: [string]
    max-worker-count: int64
    model-name: string
    output-data-format: string
    output-path: string
    region: string
    runtime-version: string
    signature-name: string
    uri: string
    version-name: string
  prediction-output:
    error-count: int64
    node-hours: number
    output-path: string
    prediction-count: int64
  start-time: string
  state: string
  training-input:
    args: [string]
    hyperparameters:
      algorithm: string
      enable-trial-early-stopping: boolean
      goal: string
      hyperparameter-metric-tag: string
      max-parallel-trials: integer
      max-trials: integer
      resume-previous-job-id: string
    job-dir: string
    master-type: string
    package-uris: [string]
    parameter-server-count: int64
    parameter-server-type: string
    python-module: string
    python-version: string
    region: string
    runtime-version: string
    scale-tier: string
    worker-count: int64
    worker-type: string
  training-output:
    completed-trial-count: int64
    consumed-ml-units: number
    is-hyperparameter-tuning-job: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=et`
    - Output only. When the job was created.
* `end-time=duo`
    - Output only. When the job processing was completed.
* `error-message=et`
    - Output only. The details of a failure or a cancellation.
* `etag=eirmod`
    - `etag` is used for optimistic concurrency control as a way to help
        prevent simultaneous updates of a job from overwriting each other.
        It is strongly suggested that systems make use of the `etag` in the
        read-modify-write cycle to perform job updates in order to avoid race
        conditions: An `etag` is returned in the response to `GetJob`, and
        systems are expected to put that etag in the request to `UpdateJob` to
        ensure that their change will be applied to the same version of the job.
* `job-id=sanctus`
    - Required. The user-specified id of the job.
* `labels=key=et`
    - Optional. One or more labels that you can add, to organize your jobs.
        Each label is a key-value pair, where both the key and the value are
        arbitrary strings that you supply.
        For more information, see the documentation on
        &lt;a href=&#34;/ml-engine/docs/tensorflow/resource-labels&#34;&gt;using labels&lt;/a&gt;.
    - the value will be associated with the given `key`
* `prediction-input.accelerator    count=amet`
    - The number of accelerators to attach to each machine running the job.
* `type=et`
    - The available types of accelerators.

* `..    batch-size=consetetur`
    - Optional. Number of records per batch, defaults to 64.
        The service will buffer batch_size number of records in memory before
        invoking one Tensorflow prediction call internally. So take the record
        size and memory available into consideration when setting this parameter.
* `data-format=ut`
    - Required. The format of the input data files.
* `input-paths=ea`
    - Required. The Google Cloud Storage location of the input data files.
        May contain wildcards.
    - Each invocation of this argument appends the given value to the array.
* `max-worker-count=-80`
    - Optional. The maximum number of workers to be used for parallel processing.
        Defaults to 10 if not specified.
* `model-name=dolor`
    - Use this field if you want to use the default version for the specified
        model. The string must use the following format:
        
        `&#34;projects/YOUR_PROJECT/models/YOUR_MODEL&#34;`
* `output-data-format=dolor`
    - Optional. Format of the output data files, defaults to JSON.
* `output-path=dolor`
    - Required. The output Google Cloud Storage location.
* `region=et`
    - Required. The Google Compute Engine region to run the prediction job in.
        See the &lt;a href=&#34;/ml-engine/docs/tensorflow/regions&#34;&gt;available regions&lt;/a&gt;
        for ML Engine services.
* `runtime-version=consetetur`
    - Optional. The Google Cloud ML runtime version to use for this batch
        prediction. If not set, Google Cloud ML will pick the runtime version used
        during the CreateVersion request for this model version, or choose the
        latest stable version when model version information is not available
        such as when the model is specified by uri.
* `signature-name=amet.`
    - Optional. The name of the signature defined in the SavedModel to use for
        this job. Please refer to
        [SavedModel](https://tensorflow.github.io/serving/serving_basic.html)
        for information about how to use signatures.
        
        Defaults to
        [DEFAULT_SERVING_SIGNATURE_DEF_KEY](https://www.tensorflow.org/api_docs/python/tf/saved_model/signature_constants)
        , which is &#34;serving_default&#34;.
* `uri=voluptua.`
    - Use this field if you want to specify a Google Cloud Storage path for
        the model to use.
* `version-name=lorem`
    - Use this field if you want to specify a version of the model to use. The
        string is formatted the same way as `model_version`, with the addition
        of the version information:
        
        `&#34;projects/YOUR_PROJECT/models/YOUR_MODEL/versions/YOUR_VERSION&#34;`

* `..prediction-output    error-count=-11`
    - The number of data instances which resulted in errors.
* `node-hours=0.804989036339`
    - Node hours used by the batch prediction job.
* `output-path=sit`
    - The output Google Cloud Storage location provided at the job creation time.
* `prediction-count=-26`
    - The number of generated predictions.

* `..    start-time=diam`
    - Output only. When the job processing was started.
* `state=rebum.`
    - Output only. The detailed state of a job.
* `training-input    args=consetetur`
    - Optional. Command line arguments to pass to the program.
    - Each invocation of this argument appends the given value to the array.
* `hyperparameters    algorithm=sadipscing`
    - Optional. The search algorithm specified for the hyperparameter
        tuning job.
        Uses the default CloudML Engine hyperparameter tuning
        algorithm if unspecified.
* `enable-trial-early-stopping=false`
    - Optional. Indicates if the hyperparameter tuning job enables auto trial
        early stopping.
* `goal=sadipscing`
    - Required. The type of goal to use for tuning. Available types are
        `MAXIMIZE` and `MINIMIZE`.
        
        Defaults to `MAXIMIZE`.
* `hyperparameter-metric-tag=invidunt`
    - Optional. The Tensorflow summary tag name to use for optimizing trials. For
        current versions of Tensorflow, this tag name should exactly match what is
        shown in Tensorboard, including all scopes.  For versions of Tensorflow
        prior to 0.12, this should be only the tag passed to tf.Summary.
        By default, &#34;training/hptuning/metric&#34; will be used.
* `max-parallel-trials=5`
    - Optional. The number of training trials to run concurrently.
        You can reduce the time it takes to perform hyperparameter tuning by adding
        trials in parallel. However, each trail only benefits from the information
        gained in completed trials. That means that a trial does not get access to
        the results of trials running at the same time, which could reduce the
        quality of the overall optimization.
        
        Each trial will use the same scale tier and machine types.
        
        Defaults to one.
* `max-trials=17`
    - Optional. How many training trials should be attempted to optimize
        the specified hyperparameters.
        
        Defaults to one.
* `resume-previous-job-id=duo`
    - Optional. The prior hyperparameter tuning job id that users hope to
        continue with. The job id will be used to find the corresponding vizier
        study guid and resume the study.

* `..    job-dir=aliquyam`
    - Optional. A Google Cloud Storage path in which to store training outputs
        and other data needed for training. This path is passed to your TensorFlow
        program as the &#39;--job-dir&#39; command-line argument. The benefit of specifying
        this field is that Cloud ML validates the path for use in training.
* `master-type=lorem`
    - Optional. Specifies the type of virtual machine to use for your training
        job&#39;s master worker.
        
        The following types are supported:
        
        &lt;dl&gt;
          &lt;dt&gt;standard&lt;/dt&gt;
          &lt;dd&gt;
          A basic machine configuration suitable for training simple models with
          small to moderate datasets.
          &lt;/dd&gt;
          &lt;dt&gt;large_model&lt;/dt&gt;
          &lt;dd&gt;
          A machine with a lot of memory, specially suited for parameter servers
          when your model is large (having many hidden layers or layers with very
          large numbers of nodes).
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_s&lt;/dt&gt;
          &lt;dd&gt;
          A machine suitable for the master and workers of the cluster when your
          model requires more computation than the standard machine can handle
          satisfactorily.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_m&lt;/dt&gt;
          &lt;dd&gt;
          A machine with roughly twice the number of cores and roughly double the
          memory of &lt;i&gt;complex_model_s&lt;/i&gt;.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_l&lt;/dt&gt;
          &lt;dd&gt;
          A machine with roughly twice the number of cores and roughly double the
          memory of &lt;i&gt;complex_model_m&lt;/i&gt;.
          &lt;/dd&gt;
          &lt;dt&gt;standard_gpu&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;standard&lt;/i&gt; that
          also includes a single NVIDIA Tesla K80 GPU. See more about
          &lt;a href=&#34;/ml-engine/docs/tensorflow/using-gpus&#34;&gt;using GPUs to
          train your model&lt;/a&gt;.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_m_gpu&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;complex_model_m&lt;/i&gt; that also includes
          four NVIDIA Tesla K80 GPUs.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_l_gpu&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;complex_model_l&lt;/i&gt; that also includes
          eight NVIDIA Tesla K80 GPUs.
          &lt;/dd&gt;
          &lt;dt&gt;standard_p100&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;standard&lt;/i&gt; that
          also includes a single NVIDIA Tesla P100 GPU.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_m_p100&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;complex_model_m&lt;/i&gt; that also includes
          four NVIDIA Tesla P100 GPUs.
          &lt;/dd&gt;
          &lt;dt&gt;standard_v100&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;standard&lt;/i&gt; that
          also includes a single NVIDIA Tesla V100 GPU. The availability of these
          GPUs is in the &lt;i&gt;Beta&lt;/i&gt; launch stage.
          &lt;/dd&gt;
          &lt;dt&gt;large_model_v100&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;large_model&lt;/i&gt; that
          also includes a single NVIDIA Tesla V100 GPU. The availability of these
          GPUs is in the &lt;i&gt;Beta&lt;/i&gt; launch stage.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_m_v100&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;complex_model_m&lt;/i&gt; that
          also includes four NVIDIA Tesla V100 GPUs. The availability of these
          GPUs is in the &lt;i&gt;Beta&lt;/i&gt; launch stage.
          &lt;/dd&gt;
          &lt;dt&gt;complex_model_l_v100&lt;/dt&gt;
          &lt;dd&gt;
          A machine equivalent to &lt;i&gt;complex_model_l&lt;/i&gt; that
          also includes eight NVIDIA Tesla V100 GPUs. The availability of these
          GPUs is in the &lt;i&gt;Beta&lt;/i&gt; launch stage.
          &lt;/dd&gt;
          &lt;dt&gt;cloud_tpu&lt;/dt&gt;
          &lt;dd&gt;
          A TPU VM including one Cloud TPU. See more about
          &lt;a href=&#34;/ml-engine/docs/tensorflow/using-tpus&#34;&gt;using TPUs to train
          your model&lt;/a&gt;.
          &lt;/dd&gt;
        &lt;/dl&gt;
        
        You must set this value when `scaleTier` is set to `CUSTOM`.
* `package-uris=et`
    - Required. The Google Cloud Storage location of the packages with
        the training program and any additional dependencies.
        The maximum number of package URIs is 100.
    - Each invocation of this argument appends the given value to the array.
* `parameter-server-count=-13`
    - Optional. The number of parameter server replicas to use for the training
        job. Each replica in the cluster will be of the type specified in
        `parameter_server_type`.
        
        This value can only be used when `scale_tier` is set to `CUSTOM`.If you
        set this value, you must also set `parameter_server_type`.
* `parameter-server-type=consetetur`
    - Optional. Specifies the type of virtual machine to use for your training
        job&#39;s parameter server.
        
        The supported values are the same as those described in the entry for
        `master_type`.
        
        This value must be present when `scaleTier` is set to `CUSTOM` and
        `parameter_server_count` is greater than zero.
* `python-module=takimata`
    - Required. The Python module name to run after installing the packages.
* `python-version=nonumy`
    - Optional. The version of Python used in training. If not set, the default
        version is &#39;2.7&#39;. Python &#39;3.5&#39; is available when `runtime_version` is set
        to &#39;1.4&#39; and above. Python &#39;2.7&#39; works with all supported runtime versions.
* `region=kasd`
    - Required. The Google Compute Engine region to run the training job in.
        See the &lt;a href=&#34;/ml-engine/docs/tensorflow/regions&#34;&gt;available regions&lt;/a&gt;
        for ML Engine services.
* `runtime-version=sanctus`
    - Optional. The Google Cloud ML runtime version to use for training.  If not
        set, Google Cloud ML will choose a stable version, which is defined in the
        documentation of runtime version list.
* `scale-tier=takimata`
    - Required. Specifies the machine types, the number of replicas for workers
        and parameter servers.
* `worker-count=-27`
    - Optional. The number of worker replicas to use for the training job. Each
        replica in the cluster will be of the type specified in `worker_type`.
        
        This value can only be used when `scale_tier` is set to `CUSTOM`. If you
        set this value, you must also set `worker_type`.
* `worker-type=labore`
    - Optional. Specifies the type of virtual machine to use for your training
        job&#39;s worker nodes.
        
        The supported values are the same as those described in the entry for
        `masterType`.
        
        This value must be present when `scaleTier` is set to `CUSTOM` and
        `workerCount` is greater than zero.

* `..training-output    completed-trial-count=-37`
    - The number of hyperparameter tuning trials that completed successfully.
        Only set for hyperparameter tuning jobs.
* `consumed-ml-units=0.347300286445`
    - The amount of ML units consumed by the job.
* `is-hyperparameter-tuning-job=false`
    - Whether this job is a hyperparameter tuning job.



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
    - Required. Specifies the path, relative to `Job`, of the field to update.
        To adopt etag mechanism, include `etag` field in the mask, and include the
        `etag` value in your job resource.
        
        For example, to change the labels of a job, the `update_mask` parameter
        would be specified as `labels`, `etag`, and the
        `PATCH` request body would specify the new value, as follows:
            {
              &#34;labels&#34;: {
                 &#34;owner&#34;: &#34;Google&#34;,
                 &#34;color&#34;: &#34;Blue&#34;
              }
              &#34;etag&#34;: &#34;33a64df551425fcc55e4d42a148795d9f25f89d4&#34;
            }
        If `etag` matches the one on the server, the labels of the job will be
        replaced with the given ones, and the server end `etag` will be
        recalculated.
        
        Currently the only supported update masks are `labels` and `etag`.

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
