Request to run a matrix of tests according to the given specifications.
Unsupported environments will be returned in the state UNSUPPORTED.
Matrices are limited to at most 200 supported executions.

May return any of the following canonical error codes:

- PERMISSION_DENIED - if the user is not authorized to write to project
- INVALID_ARGUMENT - if the request is malformed or if the matrix expands
                     to more than 200 supported executions
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `testing1 --scope <scope> projects test-matrices-create ...`
# Required Scalar Argument
* **&lt;project-id&gt;** *(string)*
    - The GCE project under which this job will run.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
TestMatrix:
  client-info:
    name: string
  environment-matrix:
    android-matrix:
      android-model-ids: [string]
      android-version-ids: [string]
      locales: [string]
      orientations: [string]
  invalid-matrix-details: string
  project-id: string
  result-storage:
    google-cloud-storage:
      gcs-path: string
    tool-results-execution:
      execution-id: string
      history-id: string
      project-id: string
    tool-results-history:
      history-id: string
      project-id: string
  state: string
  test-matrix-id: string
  test-specification:
    android-instrumentation-test:
      app-apk:
        gcs-path: string
      app-package-id: string
      orchestrator-option: string
      test-apk:
        gcs-path: string
      test-package-id: string
      test-runner-class: string
      test-targets: [string]
    android-robo-test:
      app-apk:
        gcs-path: string
      app-initial-activity: string
      app-package-id: string
      max-depth: integer
      max-steps: integer
      robo-script:
        gcs-path: string
    android-test-loop:
      app-apk:
        gcs-path: string
      app-package-id: string
      scenario-labels: [string]
      scenarios: [integer]
    auto-google-login: boolean
    disable-performance-metrics: boolean
    disable-video-recording: boolean
    ios-test-setup:
      network-profile: string
    ios-xc-test:
      tests-zip:
        gcs-path: string
      xcode-version: string
      xctestrun:
        gcs-path: string
    test-setup:
      directories-to-pull: [string]
      network-profile: string
    test-timeout: string
  timestamp: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .client-info    name=sit`
    - Client name, such as gcloud.
        Required

* `..environment-matrix.android-matrix    android-model-ids=stet`
    - The ids of the set of Android device to be used.
        Use the EnvironmentDiscoveryService to get supported options.
        Required
    - Each invocation of this argument appends the given value to the array.
* `android-version-ids=sed`
    - The ids of the set of Android OS version to be used.
        Use the EnvironmentDiscoveryService to get supported options.
        Required
    - Each invocation of this argument appends the given value to the array.
* `locales=et`
    - The set of locales the test device will enable for testing.
        Use the EnvironmentDiscoveryService to get supported options.
        Required
    - Each invocation of this argument appends the given value to the array.
* `orientations=dolores`
    - The set of orientations to test with.
        Use the EnvironmentDiscoveryService to get supported options.
        Required
    - Each invocation of this argument appends the given value to the array.


* `...    invalid-matrix-details=kasd`
    - Describes why the matrix is considered invalid.
        Only useful for matrices in the INVALID state.
        @OutputOnly
* `project-id=accusam`
    - The cloud project that owns the test matrix.
        @OutputOnly
* `result-storage.google-cloud-storage    gcs-path=takimata`
    - The path to a directory in GCS that will
        eventually contain the results for this test.
        The requesting user must have write access on the bucket in the supplied
        path.
        Required

* `..tool-results-execution    execution-id=justo`
    - A tool results execution ID.
        @OutputOnly
* `history-id=amet.`
    - A tool results history ID.
        @OutputOnly
* `project-id=erat`
    - The cloud project that owns the tool results execution.
        @OutputOnly

* `..tool-results-history    history-id=labore`
    - A tool results history ID.
        Required
* `project-id=sea`
    - The cloud project that owns the tool results history.
        Required


* `...    state=nonumy`
    - Indicates the current progress of the test matrix (e.g., FINISHED)
        @OutputOnly
* `test-matrix-id=dolores`
    - Unique id set by the service.
        @OutputOnly
* `test-specification.android-instrumentation-test.app-apk    gcs-path=gubergren`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk

* `..    app-package-id=sadipscing`
    - The java package for the application under test.
        Optional, default is determined by examining the application&#39;s manifest.
* `orchestrator-option=aliquyam`
    - The option of whether running each test within its own invocation of
        instrumentation with Android Test Orchestrator or not.
        ** Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or
        higher! **
        Orchestrator offers the following benefits:
         - No shared state
         - Crashes are isolated
         - Logs are scoped per test
        
        See
        &lt;https://developer.android.com/training/testing/junit-runner.html#using-android-test-orchestrator&gt;
        for more information about Android Test Orchestrator.
        
        Optional. If not set, the test will be run without the orchestrator.
* `test-apk    gcs-path=ea`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk

* `..    test-package-id=no`
    - The java package for the test to be executed.
        Optional, default is determined by examining the application&#39;s manifest.
* `test-runner-class=justo`
    - The InstrumentationTestRunner class.
        Optional, default is determined by examining the application&#39;s manifest.
* `test-targets=justo`
    - Each target must be fully qualified with the package name or class name,
        in one of these formats:
         - &#34;package package_name&#34;
         - &#34;class package_name.class_name&#34;
         - &#34;class package_name.class_name#method_name&#34;
        
        Optional, if empty, all targets in the module will be run.
    - Each invocation of this argument appends the given value to the array.

* `..android-robo-test.app-apk    gcs-path=et`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk

* `..    app-initial-activity=et`
    - The initial activity that should be used to start the app.
        Optional
* `app-package-id=diam`
    - The java package for the application under test.
        Optional, default is determined by examining the application&#39;s manifest.
* `max-depth=46`
    - The max depth of the traversal stack Robo can explore. Needs to be at least
        2 to make Robo explore the app beyond the first activity.
        Default is 50.
        Optional
* `max-steps=96`
    - The max number of steps Robo can execute.
        Default is no limit.
        Optional
* `robo-script    gcs-path=et`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk


* `...android-test-loop.app-apk    gcs-path=duo`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk

* `..    app-package-id=aliquyam`
    - The java package for the application under test.
        Optional, default is determined by examining the application&#39;s manifest.
* `scenario-labels=sea`
    - The list of scenario labels that should be run during the test.
        The scenario labels should map to labels defined in the application&#39;s
        manifest. For example, player_experience and
        com.google.test.loops.player_experience add all of the loops labeled in the
        manifest with the com.google.test.loops.player_experience name to the
        execution.
        Optional. Scenarios can also be specified in the scenarios field.
    - Each invocation of this argument appends the given value to the array.
* `scenarios=46`
    - The list of scenarios that should be run during the test.
        Optional, default is all test loops, derived from the application&#39;s
        manifest.
    - Each invocation of this argument appends the given value to the array.

* `..    auto-google-login=false`
    - Enables automatic Google account login.
        If set, the service will automatically generate a Google test account and
        add it to the device, before executing the test. Note that test accounts
        might be reused.
        Many applications show their full set of functionalities when an account is
        present on the device. Logging into the device with these generated
        accounts allows testing more functionalities.
        Default is false.
        Optional
* `disable-performance-metrics=false`
    - Disables performance metrics recording; may reduce test latency.
* `disable-video-recording=false`
    - Disables video recording; may reduce test latency.
* `ios-test-setup    network-profile=dolor`
    - Optional. The network traffic profile used for running the test.
        Available network profiles can be queried by using the
        NETWORK_CONFIGURATION environment type when calling
        TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.

* `..ios-xc-test.tests-zip    gcs-path=eirmod`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk

* `..    xcode-version=elitr`
    - Optional. The Xcode version that should be used for the test.
        Use the EnvironmentDiscoveryService to get supported options.
        Defaults to the latest Xcode version Firebase Test Lab supports.
* `xctestrun    gcs-path=amet`
    - A path to a file in Google Cloud Storage.
        Example: gs://build-app-1414623860166/app-debug-unaligned.apk


* `...test-setup    directories-to-pull=no`
    - List of directories on the device to upload to GCS at the end of the test;
        they must be absolute paths under /sdcard or /data/local/tmp.
        Path names are restricted to characters a-z A-Z 0-9 _ - . + and /
        
        Note: The paths /sdcard and /data will be made available and treated as
        implicit path substitutions. E.g. if /sdcard on a particular device does
        not map to external storage, the system will replace it with the external
        storage path prefix for that device.
        
        Optional
    - Each invocation of this argument appends the given value to the array.
* `network-profile=labore`
    - Optional. The network traffic profile used for running the test.
        Available network profiles can be queried by using the
        NETWORK_CONFIGURATION environment type when calling
        TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.

* `..    test-timeout=eirmod`
    - Max time a test execution is allowed to run before it is
        automatically cancelled.
        Optional, default is 5 min.

* `..    timestamp=dolore`
    - The time this test matrix was initially created.
        @OutputOnly


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

* **-p request-id=string**
    - A string id used to detect duplicated requests.
        Ids are automatically scoped to a project, so
        users should ensure the ID is unique per-project.
        A UUID is recommended.
        
        Optional, but strongly recommended.

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
