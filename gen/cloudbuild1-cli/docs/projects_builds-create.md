Starts a build with the specified configuration.

This method returns a long-running `Operation`, which includes the build
ID. Pass the build ID to `GetBuild` to determine the build status (such as
`SUCCESS` or `FAILURE`).
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudbuild1 --scope <scope> projects builds-create ...`
# Required Scalar Argument
* **&lt;project-id&gt;** *(string)*
    - ID of the project.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Build:
  artifacts:
    images: [string]
    objects:
      location: string
      paths: [string]
      timing:
        end-time: string
        start-time: string
  build-trigger-id: string
  create-time: string
  finish-time: string
  id: string
  images: [string]
  log-url: string
  logs-bucket: string
  options:
    disk-size-gb: string
    env: [string]
    log-streaming-option: string
    logging: string
    machine-type: string
    requested-verify-option: string
    secret-env: [string]
    source-provenance-hash: [string]
    substitution-option: string
  project-id: string
  results:
    artifact-manifest: string
    build-step-images: [string]
    build-step-outputs: [string]
    num-artifacts: string
  source:
    repo-source:
      branch-name: string
      commit-sha: string
      dir: string
      project-id: string
      repo-name: string
      tag-name: string
    storage-source:
      bucket: string
      generation: string
      object: string
  source-provenance:
    resolved-repo-source:
      branch-name: string
      commit-sha: string
      dir: string
      project-id: string
      repo-name: string
      tag-name: string
    resolved-storage-source:
      bucket: string
      generation: string
      object: string
  start-time: string
  status: string
  status-detail: string
  substitutions: { string: string }
  tags: [string]
  timeout: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .artifacts    images=eirmod`
    - A list of images to be pushed upon the successful completion of all build
        steps.
        
        The images will be pushed using the builder service account&#39;s credentials.
        
        The digests of the pushed images will be stored in the Build resource&#39;s
        results field.
        
        If any of the images fail to be pushed, the build is marked FAILURE.
    - Each invocation of this argument appends the given value to the array.
* `objects    location=sit`
    - Cloud Storage bucket and optional object path, in the form
        &#34;gs://bucket/path/to/somewhere/&#34;. (see [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
        
        Files in the workspace matching any path pattern will be uploaded to
        Cloud Storage with this location as a prefix.
* `paths=stet`
    - Path globs used to match files in the build&#39;s workspace.
    - Each invocation of this argument appends the given value to the array.
* `timing    end-time=sed`
    - End of time span.
* `start-time=et`
    - Start of time span.



* `....    build-trigger-id=dolores`
    - Output only. The ID of the `BuildTrigger` that triggered this build, if it
        was triggered automatically.
* `create-time=kasd`
    - Output only. Time at which the request to create the build was received.
* `finish-time=accusam`
    - Output only. Time at which execution of the build was finished.
        
        The difference between finish_time and start_time is the duration of the
        build&#39;s execution.
* `id=takimata`
    - Output only. Unique identifier of the build.
* `images=justo`
    - A list of images to be pushed upon the successful completion of all build
        steps.
        
        The images are pushed using the builder service account&#39;s credentials.
        
        The digests of the pushed images will be stored in the `Build` resource&#39;s
        results field.
        
        If any of the images fail to be pushed, the build status is marked
        `FAILURE`.
    - Each invocation of this argument appends the given value to the array.
* `log-url=amet.`
    - Output only. URL to logs for this build in Google Cloud Console.
* `logs-bucket=erat`
    - Google Cloud Storage bucket where logs should be written (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
        Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`.
* `options    disk-size-gb=labore`
    - Requested disk size for the VM that runs the build. Note that this is *NOT*
        &#34;disk free&#34;; some of the space will be used by the operating system and
        build utilities. Also note that this is the minimum disk size that will be
        allocated for the build -- the build may run with a larger disk than
        requested. At present, the maximum disk size is 1000GB; builds that request
        more than the maximum are rejected with an error.
* `env=sea`
    - A list of global environment variable definitions that will exist for all
        build steps in this build. If a variable is defined in both globally and in
        a build step, the variable will use the build step value.
        
        The elements are of the form &#34;KEY=VALUE&#34; for the environment variable &#34;KEY&#34;
        being given the value &#34;VALUE&#34;.
    - Each invocation of this argument appends the given value to the array.
* `log-streaming-option=nonumy`
    - Option to define build log streaming behavior to Google Cloud
        Storage.
* `logging=dolores`
    - Option to specify the logging mode, which determines where the logs are
        stored.
* `machine-type=gubergren`
    - Compute Engine machine type on which to run the build.
* `requested-verify-option=sadipscing`
    - Requested verifiability options.
* `secret-env=aliquyam`
    - A list of global environment variables, which are encrypted using a Cloud
        Key Management Service crypto key. These values must be specified in the
        build&#39;s `Secret`. These variables will be available to all build steps
        in this build.
    - Each invocation of this argument appends the given value to the array.
* `source-provenance-hash=ea`
    - Requested hash for SourceProvenance.
    - Each invocation of this argument appends the given value to the array.
* `substitution-option=no`
    - Option to specify behavior when there is an error in the substitution
        checks.

* `..    project-id=justo`
    - Output only. ID of the project.
* `results    artifact-manifest=justo`
    - Path to the artifact manifest. Only populated when artifacts are uploaded.
* `build-step-images=et`
    - List of build step digests, in the order corresponding to build step
        indices.
    - Each invocation of this argument appends the given value to the array.
* `build-step-outputs=et`
    - List of build step outputs, produced by builder images, in the order
        corresponding to build step indices.
        
        [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders)
        can produce this output by writing to `$BUILDER_OUTPUT/output`.
        Only the first 4KB of data is stored.
    - Each invocation of this argument appends the given value to the array.
* `num-artifacts=diam`
    - Number of artifacts uploaded. Only populated when artifacts are uploaded.

* `..source.repo-source    branch-name=ipsum`
    - Name of the branch to build.
* `commit-sha=lorem`
    - Explicit commit SHA to build.
* `dir=et`
    - Directory, relative to the source root, in which to run the build.
        
        This must be a relative path. If a step&#39;s `dir` is specified and is an
        absolute path, this value is ignored for that step&#39;s execution.
* `project-id=duo`
    - ID of the project that owns the Cloud Source Repository. If omitted, the
        project ID requesting the build is assumed.
* `repo-name=aliquyam`
    - Name of the Cloud Source Repository. If omitted, the name &#34;default&#34; is
        assumed.
* `tag-name=sea`
    - Name of the tag to build.

* `..storage-source    bucket=lorem`
    - Google Cloud Storage bucket containing the source (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
* `generation=eos`
    - Google Cloud Storage generation for the object. If the generation is
        omitted, the latest generation will be used.
* `object=erat`
    - Google Cloud Storage object containing the source.
        
        This object must be a gzipped archive file (`.tar.gz`) containing source to
        build.


* `...source-provenance.resolved-repo-source    branch-name=sadipscing`
    - Name of the branch to build.
* `commit-sha=dolor`
    - Explicit commit SHA to build.
* `dir=eirmod`
    - Directory, relative to the source root, in which to run the build.
        
        This must be a relative path. If a step&#39;s `dir` is specified and is an
        absolute path, this value is ignored for that step&#39;s execution.
* `project-id=elitr`
    - ID of the project that owns the Cloud Source Repository. If omitted, the
        project ID requesting the build is assumed.
* `repo-name=amet`
    - Name of the Cloud Source Repository. If omitted, the name &#34;default&#34; is
        assumed.
* `tag-name=no`
    - Name of the tag to build.

* `..resolved-storage-source    bucket=labore`
    - Google Cloud Storage bucket containing the source (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
* `generation=eirmod`
    - Google Cloud Storage generation for the object. If the generation is
        omitted, the latest generation will be used.
* `object=dolore`
    - Google Cloud Storage object containing the source.
        
        This object must be a gzipped archive file (`.tar.gz`) containing source to
        build.


* `...    start-time=invidunt`
    - Output only. Time at which execution of the build was started.
* `status=aliquyam`
    - Output only. Status of the build.
* `status-detail=accusam`
    - Output only. Customer-readable message about the current status.
* `substitutions=key=lorem`
    - Substitutions data for `Build` resource.
    - the value will be associated with the given `key`
* `tags=sea`
    - Tags for annotation of a `Build`. These are not docker tags.
    - Each invocation of this argument appends the given value to the array.
* `timeout=et`
    - Amount of time that this build should be allowed to run, to second
        granularity. If this amount of time elapses, work on the build will cease
        and the build status will be `TIMEOUT`.
        
        Default time is ten minutes.


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
