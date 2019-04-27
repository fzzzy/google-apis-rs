Updates a `BuildTrigger` by its project ID and trigger ID.

This API is experimental.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudbuild1 --scope <scope> projects triggers-patch ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - ID of the project that owns the trigger.
* **&lt;trigger-id&gt;** *(string)*
    - ID of the `BuildTrigger` to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
BuildTrigger:
  build:
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
  create-time: string
  description: string
  disabled: boolean
  filename: string
  id: string
  ignored-files: [string]
  included-files: [string]
  substitutions: { string: string }
  trigger-template:
    branch-name: string
    commit-sha: string
    dir: string
    project-id: string
    repo-name: string
    tag-name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .build.artifacts    images=labore`
    - A list of images to be pushed upon the successful completion of all build
        steps.
        
        The images will be pushed using the builder service account&#39;s credentials.
        
        The digests of the pushed images will be stored in the Build resource&#39;s
        results field.
        
        If any of the images fail to be pushed, the build is marked FAILURE.
    - Each invocation of this argument appends the given value to the array.
* `objects    location=ipsum`
    - Cloud Storage bucket and optional object path, in the form
        &#34;gs://bucket/path/to/somewhere/&#34;. (see [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
        
        Files in the workspace matching any path pattern will be uploaded to
        Cloud Storage with this location as a prefix.
* `paths=aliquyam`
    - Path globs used to match files in the build&#39;s workspace.
    - Each invocation of this argument appends the given value to the array.
* `timing    end-time=dolores`
    - End of time span.
* `start-time=sit`
    - Start of time span.



* `....    build-trigger-id=diam`
    - Output only. The ID of the `BuildTrigger` that triggered this build, if it
        was triggered automatically.
* `create-time=ut`
    - Output only. Time at which the request to create the build was received.
* `finish-time=justo`
    - Output only. Time at which execution of the build was finished.
        
        The difference between finish_time and start_time is the duration of the
        build&#39;s execution.
* `id=est`
    - Output only. Unique identifier of the build.
* `images=amet`
    - A list of images to be pushed upon the successful completion of all build
        steps.
        
        The images are pushed using the builder service account&#39;s credentials.
        
        The digests of the pushed images will be stored in the `Build` resource&#39;s
        results field.
        
        If any of the images fail to be pushed, the build status is marked
        `FAILURE`.
    - Each invocation of this argument appends the given value to the array.
* `log-url=accusam`
    - Output only. URL to logs for this build in Google Cloud Console.
* `logs-bucket=clita`
    - Google Cloud Storage bucket where logs should be written (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
        Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`.
* `options    disk-size-gb=diam`
    - Requested disk size for the VM that runs the build. Note that this is *NOT*
        &#34;disk free&#34;; some of the space will be used by the operating system and
        build utilities. Also note that this is the minimum disk size that will be
        allocated for the build -- the build may run with a larger disk than
        requested. At present, the maximum disk size is 1000GB; builds that request
        more than the maximum are rejected with an error.
* `env=justo`
    - A list of global environment variable definitions that will exist for all
        build steps in this build. If a variable is defined in both globally and in
        a build step, the variable will use the build step value.
        
        The elements are of the form &#34;KEY=VALUE&#34; for the environment variable &#34;KEY&#34;
        being given the value &#34;VALUE&#34;.
    - Each invocation of this argument appends the given value to the array.
* `log-streaming-option=est`
    - Option to define build log streaming behavior to Google Cloud
        Storage.
* `logging=clita`
    - Option to specify the logging mode, which determines where the logs are
        stored.
* `machine-type=invidunt`
    - Compute Engine machine type on which to run the build.
* `requested-verify-option=ut`
    - Requested verifiability options.
* `secret-env=dolores`
    - A list of global environment variables, which are encrypted using a Cloud
        Key Management Service crypto key. These values must be specified in the
        build&#39;s `Secret`. These variables will be available to all build steps
        in this build.
    - Each invocation of this argument appends the given value to the array.
* `source-provenance-hash=eos`
    - Requested hash for SourceProvenance.
    - Each invocation of this argument appends the given value to the array.
* `substitution-option=voluptua.`
    - Option to specify behavior when there is an error in the substitution
        checks.

* `..    project-id=duo`
    - Output only. ID of the project.
* `results    artifact-manifest=sed`
    - Path to the artifact manifest. Only populated when artifacts are uploaded.
* `build-step-images=aliquyam`
    - List of build step digests, in the order corresponding to build step
        indices.
    - Each invocation of this argument appends the given value to the array.
* `build-step-outputs=ea`
    - List of build step outputs, produced by builder images, in the order
        corresponding to build step indices.
        
        [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders)
        can produce this output by writing to `$BUILDER_OUTPUT/output`.
        Only the first 4KB of data is stored.
    - Each invocation of this argument appends the given value to the array.
* `num-artifacts=ea`
    - Number of artifacts uploaded. Only populated when artifacts are uploaded.

* `..source.repo-source    branch-name=et`
    - Name of the branch to build.
* `commit-sha=dolor`
    - Explicit commit SHA to build.
* `dir=diam`
    - Directory, relative to the source root, in which to run the build.
        
        This must be a relative path. If a step&#39;s `dir` is specified and is an
        absolute path, this value is ignored for that step&#39;s execution.
* `project-id=kasd`
    - ID of the project that owns the Cloud Source Repository. If omitted, the
        project ID requesting the build is assumed.
* `repo-name=invidunt`
    - Name of the Cloud Source Repository. If omitted, the name &#34;default&#34; is
        assumed.
* `tag-name=rebum.`
    - Name of the tag to build.

* `..storage-source    bucket=lorem`
    - Google Cloud Storage bucket containing the source (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
* `generation=clita`
    - Google Cloud Storage generation for the object. If the generation is
        omitted, the latest generation will be used.
* `object=invidunt`
    - Google Cloud Storage object containing the source.
        
        This object must be a gzipped archive file (`.tar.gz`) containing source to
        build.


* `...source-provenance.resolved-repo-source    branch-name=eirmod`
    - Name of the branch to build.
* `commit-sha=at`
    - Explicit commit SHA to build.
* `dir=consetetur`
    - Directory, relative to the source root, in which to run the build.
        
        This must be a relative path. If a step&#39;s `dir` is specified and is an
        absolute path, this value is ignored for that step&#39;s execution.
* `project-id=et`
    - ID of the project that owns the Cloud Source Repository. If omitted, the
        project ID requesting the build is assumed.
* `repo-name=sed`
    - Name of the Cloud Source Repository. If omitted, the name &#34;default&#34; is
        assumed.
* `tag-name=sit`
    - Name of the tag to build.

* `..resolved-storage-source    bucket=takimata`
    - Google Cloud Storage bucket containing the source (see
        [Bucket Name
        Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
* `generation=elitr`
    - Google Cloud Storage generation for the object. If the generation is
        omitted, the latest generation will be used.
* `object=nonumy`
    - Google Cloud Storage object containing the source.
        
        This object must be a gzipped archive file (`.tar.gz`) containing source to
        build.


* `...    start-time=rebum.`
    - Output only. Time at which execution of the build was started.
* `status=lorem`
    - Output only. Status of the build.
* `status-detail=lorem`
    - Output only. Customer-readable message about the current status.
* `substitutions=key=diam`
    - Substitutions data for `Build` resource.
    - the value will be associated with the given `key`
* `tags=ut`
    - Tags for annotation of a `Build`. These are not docker tags.
    - Each invocation of this argument appends the given value to the array.
* `timeout=ut`
    - Amount of time that this build should be allowed to run, to second
        granularity. If this amount of time elapses, work on the build will cease
        and the build status will be `TIMEOUT`.
        
        Default time is ten minutes.

* `..    create-time=amet.`
    - Output only. Time when the trigger was created.
* `description=ipsum`
    - Human-readable description of this trigger.
* `disabled=false`
    - If true, the trigger will never result in a build.
* `filename=dolor`
    - Path, from the source root, to a file whose contents is used for the
        template.
* `id=sea`
    - Output only. Unique identifier of the trigger.
* `ignored-files=ut`
    - ignored_files and included_files are file glob matches using
        http://godoc/pkg/path/filepath#Match extended with support for &#34;**&#34;.
        
        If ignored_files and changed files are both empty, then they are
        not used to determine whether or not to trigger a build.
        
        If ignored_files is not empty, then we ignore any files that match
        any of the ignored_file globs. If the change has no files that are
        outside of the ignored_files globs, then we do not trigger a build.
    - Each invocation of this argument appends the given value to the array.
* `included-files=eirmod`
    - If any of the files altered in the commit pass the ignored_files
        filter and included_files is empty, then as far as this filter is
        concerned, we should trigger the build.
        
        If any of the files altered in the commit pass the ignored_files
        filter and included_files is not empty, then we make sure that at
        least one of those files matches a included_files glob. If not,
        then we do not trigger a build.
    - Each invocation of this argument appends the given value to the array.
* `substitutions=key=sanctus`
    - Substitutions data for Build resource.
    - the value will be associated with the given `key`
* `trigger-template    branch-name=voluptua.`
    - Name of the branch to build.
* `commit-sha=dolor`
    - Explicit commit SHA to build.
* `dir=et`
    - Directory, relative to the source root, in which to run the build.
        
        This must be a relative path. If a step&#39;s `dir` is specified and is an
        absolute path, this value is ignored for that step&#39;s execution.
* `project-id=et`
    - ID of the project that owns the Cloud Source Repository. If omitted, the
        project ID requesting the build is assumed.
* `repo-name=vero`
    - Name of the Cloud Source Repository. If omitted, the name &#34;default&#34; is
        assumed.
* `tag-name=ut`
    - Name of the tag to build.



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
