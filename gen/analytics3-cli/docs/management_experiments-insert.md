Create a new experiment.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/analytics*
* *https://www.googleapis.com/auth/analytics.edit*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics*.
You can set the scope for this method like this: `analytics3 --scope <scope> management experiments-insert ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to create the experiment for.
* **&lt;web-property-id&gt;** *(string)*
    - Web property ID to create the experiment for.
* **&lt;profile-id&gt;** *(string)*
    - View (Profile) ID to create the experiment for.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Experiment:
  account-id: string
  created: string
  description: string
  editable-in-ga-ui: boolean
  end-time: string
  equal-weighting: boolean
  id: string
  internal-web-property-id: string
  kind: string
  minimum-experiment-length-in-days: integer
  name: string
  objective-metric: string
  optimization-type: string
  parent-link:
    href: string
    type: string
  profile-id: string
  reason-experiment-ended: string
  rewrite-variation-urls-as-original: boolean
  self-link: string
  serving-framework: string
  snippet: string
  start-time: string
  status: string
  traffic-coverage: number
  updated: string
  web-property-id: string
  winner-confidence-level: number
  winner-found: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=et`
    - Account ID to which this experiment belongs. This field is read-only.
* `created=dolor`
    - Time the experiment was created. This field is read-only.
* `description=diam`
    - Notes about this experiment.
* `editable-in-ga-ui=false`
    - If true, the end user will be able to edit the experiment via the Google Analytics user interface.
* `end-time=invidunt`
    - The ending time of the experiment (the time the status changed from RUNNING to ENDED). This field is present only if the experiment has ended. This field is read-only.
* `equal-weighting=true`
    - Boolean specifying whether to distribute traffic evenly across all variations. If the value is False, content experiments follows the default behavior of adjusting traffic dynamically based on variation performance. Optional -- defaults to False. This field may not be changed for an experiment whose status is ENDED.
* `id=lorem`
    - Experiment ID. Required for patch and update. Disallowed for create.
* `internal-web-property-id=clita`
    - Internal ID for the web property to which this experiment belongs. This field is read-only.
* `kind=invidunt`
    - Resource type for an Analytics experiment. This field is read-only.
* `minimum-experiment-length-in-days=11`
    - An integer number in [3, 90]. Specifies the minimum length of the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED.
* `name=at`
    - Experiment name. This field may not be changed for an experiment whose status is ENDED. This field is required when creating an experiment.
* `objective-metric=consetetur`
    - The metric that the experiment is optimizing. Valid values: &#34;ga:goal(n)Completions&#34;, &#34;ga:adsenseAdsClicks&#34;, &#34;ga:adsenseAdsViewed&#34;, &#34;ga:adsenseRevenue&#34;, &#34;ga:bounces&#34;, &#34;ga:pageviews&#34;, &#34;ga:sessionDuration&#34;, &#34;ga:transactions&#34;, &#34;ga:transactionRevenue&#34;. This field is required if status is &#34;RUNNING&#34; and servingFramework is one of &#34;REDIRECT&#34; or &#34;API&#34;.
* `optimization-type=et`
    - Whether the objectiveMetric should be minimized or maximized. Possible values: &#34;MAXIMUM&#34;, &#34;MINIMUM&#34;. Optional--defaults to &#34;MAXIMUM&#34;. Cannot be specified without objectiveMetric. Cannot be modified when status is &#34;RUNNING&#34; or &#34;ENDED&#34;.
* `parent-link    href=sed`
    - Link to the view (profile) to which this experiment belongs. This field is read-only.
* `type=sit`
    - Value is &#34;analytics#profile&#34;. This field is read-only.

* `..    profile-id=takimata`
    - View (Profile) ID to which this experiment belongs. This field is read-only.
* `reason-experiment-ended=elitr`
    - Why the experiment ended. Possible values: &#34;STOPPED_BY_USER&#34;, &#34;WINNER_FOUND&#34;, &#34;EXPERIMENT_EXPIRED&#34;, &#34;ENDED_WITH_NO_WINNER&#34;, &#34;GOAL_OBJECTIVE_CHANGED&#34;. &#34;ENDED_WITH_NO_WINNER&#34; means that the experiment didn&#39;t expire but no winner was projected to be found. If the experiment status is changed via the API to ENDED this field is set to STOPPED_BY_USER. This field is read-only.
* `rewrite-variation-urls-as-original=false`
    - Boolean specifying whether variations URLS are rewritten to match those of the original. This field may not be changed for an experiments whose status is ENDED.
* `self-link=rebum.`
    - Link for this experiment. This field is read-only.
* `serving-framework=lorem`
    - The framework used to serve the experiment variations and evaluate the results. One of:  
        - REDIRECT: Google Analytics redirects traffic to different variation pages, reports the chosen variation and evaluates the results.
        - API: Google Analytics chooses and reports the variation to serve and evaluates the results; the caller is responsible for serving the selected variation.
        - EXTERNAL: The variations will be served externally and the chosen variation reported to Google Analytics. The caller is responsible for serving the selected variation and evaluating the results.
* `snippet=lorem`
    - The snippet of code to include on the control page(s). This field is read-only.
* `start-time=diam`
    - The starting time of the experiment (the time the status changed from READY_TO_RUN to RUNNING). This field is present only if the experiment has started. This field is read-only.
* `status=ut`
    - Experiment status. Possible values: &#34;DRAFT&#34;, &#34;READY_TO_RUN&#34;, &#34;RUNNING&#34;, &#34;ENDED&#34;. Experiments can be created in the &#34;DRAFT&#34;, &#34;READY_TO_RUN&#34; or &#34;RUNNING&#34; state. This field is required when creating an experiment.
* `traffic-coverage=0.641104516092`
    - A floating-point number in (0, 1]. Specifies the fraction of the traffic that participates in the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED.
* `updated=amet.`
    - Time the experiment was last modified. This field is read-only.
* `web-property-id=ipsum`
    - Web property ID to which this experiment belongs. The web property ID is of the form UA-XXXXX-YY. This field is read-only.
* `winner-confidence-level=0.148133072748`
    - A floating-point number in (0, 1). Specifies the necessary confidence level to choose a winner. This field may not be changed for an experiments whose status is ENDED.
* `winner-found=true`
    - Boolean specifying whether a winner has been found for this experiment. This field is read-only.


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
