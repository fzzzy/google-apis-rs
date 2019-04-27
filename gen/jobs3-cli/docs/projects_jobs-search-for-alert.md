Searches for jobs using the provided SearchJobsRequest.

This API call is intended for the use case of targeting passive job
seekers (for example, job seekers who have signed up to receive email
alerts about potential job opportunities), and has different algorithmic
adjustments that are targeted to passive job seekers.

This call constrains the visibility of jobs
present in the database, and only returns jobs the caller has
permission to search against.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/jobs*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `jobs3 --scope <scope> projects jobs-search-for-alert ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The resource name of the project to search within.
        
        The format is &#34;projects/{project_id}&#34;, for example,
        &#34;projects/api-test-project&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SearchJobsRequest:
  disable-keyword-match: boolean
  enable-broadening: boolean
  histogram-facets:
    simple-histogram-facets: [string]
  job-query:
    commute-filter:
      allow-imprecise-addresses: boolean
      commute-method: string
      departure-time:
        hours: integer
        minutes: integer
        nanos: integer
        seconds: integer
      road-traffic: string
      start-coordinates:
        latitude: number
        longitude: number
      travel-duration: string
    company-display-names: [string]
    company-names: [string]
    compensation-filter:
      include-jobs-with-unspecified-compensation-range: boolean
      range:
        max-compensation:
          currency-code: string
          nanos: integer
          units: string
        min-compensation:
          currency-code: string
          nanos: integer
          units: string
      type: string
      units: [string]
    custom-attribute-filter: string
    disable-spell-check: boolean
    employment-types: [string]
    job-categories: [string]
    language-codes: [string]
    publish-time-range:
      end-time: string
      start-time: string
    query: string
  job-view: string
  offset: integer
  order-by: string
  page-size: integer
  page-token: string
  request-metadata:
    device-info:
      device-type: string
      id: string
    domain: string
    session-id: string
    user-id: string
  require-precise-result-size: boolean
  search-mode: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    disable-keyword-match=false`
    - Optional.
        
        Controls whether to disable exact keyword match on Job.job_title,
        Job.description, Job.company_display_name, Job.locations,
        Job.qualifications. When disable keyword match is turned off, a
        keyword match returns jobs that do not match given category filters when
        there are matching keywords. For example, the query &#34;program manager,&#34; a
        result is returned even if the job posting has the title &#34;software
        developer,&#34; which does not fall into &#34;program manager&#34; ontology, but does
        have &#34;program manager&#34; appearing in its description.
        
        For queries like &#34;cloud&#34; that does not contain title or
        location specific ontology, jobs with &#34;cloud&#34; keyword matches are returned
        regardless of this flag&#39;s value.
        
        Please use Company.keyword_searchable_custom_fields or
        Company.keyword_searchable_custom_attributes if company specific
        globally matched custom field/attribute string values is needed. Enabling
        keyword match improves recall of subsequent search requests.
        
        Defaults to false.
* `enable-broadening=false`
    - Optional.
        
        Controls whether to broaden the search when it produces sparse results.
        Broadened queries append results to the end of the matching results
        list.
        
        Defaults to false.
* `histogram-facets    simple-histogram-facets=sed`
    - Optional.
        
        Specifies the simple type of histogram facets, for example,
        `COMPANY_SIZE`, `EMPLOYMENT_TYPE` etc.
    - Each invocation of this argument appends the given value to the array.

* `..job-query.commute-filter    allow-imprecise-addresses=true`
    - Optional.
        If `true`, jobs without street level addresses may also be returned.
        For city level addresses, the city center is used. For state and coarser
        level addresses, text matching is used.
        If this field is set to `false` or is not specified, only jobs that include
        street level addresses will be returned by commute search.
* `commute-method=invidunt`
    - Required.
        
        The method of transportation for which to calculate the commute time.
* `departure-time    hours=86`
    - Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
        to allow the value &#34;24:00:00&#34; for scenarios like business closing time.
* `minutes=15`
    - Minutes of hour of day. Must be from 0 to 59.
* `nanos=19`
    - Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
* `seconds=7`
    - Seconds of minutes of the time. Must normally be from 0 to 59. An API may
        allow the value 60 if it allows leap-seconds.

* `..    road-traffic=consetetur`
    - Optional.
        
        Specifies the traffic density to use when caculating commute time.
* `start-coordinates    latitude=0.912030282652`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.577475425569`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    travel-duration=at`
    - Required.
        
        The maximum travel time in seconds. The maximum allowed value is `3600s`
        (one hour). Format is `123s`.

* `..    company-display-names=sea`
    - Optional.
        
        This filter specifies the exact company display
        name of the jobs to search against.
        
        If a value isn&#39;t specified, jobs within the search results are
        associated with any company.
        
        If multiple values are specified, jobs within the search results may be
        associated with any of the specified companies.
        
        At most 20 company display name filters are allowed.
    - Each invocation of this argument appends the given value to the array.
* `company-names=consetetur`
    - Optional.
        
        This filter specifies the company entities to search against.
        
        If a value isn&#39;t specified, jobs are searched for against all
        companies.
        
        If multiple values are specified, jobs are searched against the
        companies specified.
        
        The format is &#34;projects/{project_id}/companies/{company_id}&#34;, for example,
        &#34;projects/api-test-project/companies/foo&#34;.
        
        At most 20 company filters are allowed.
    - Each invocation of this argument appends the given value to the array.
* `compensation-filter    include-jobs-with-unspecified-compensation-range=false`
    - Optional.
        
        Whether to include jobs whose compensation range is unspecified.
* `range.max-compensation    currency-code=accusam`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=33`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=consetetur`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* `..min-compensation    currency-code=dolor`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=19`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=elitr`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.


* `...    type=ea`
    - Required.
        
        Type of filter.
* `units=et`
    - Required.
        
        Specify desired `base compensation entry&#39;s`
        CompensationInfo.CompensationUnit.
    - Each invocation of this argument appends the given value to the array.

* `..    custom-attribute-filter=stet`
    - Optional.
        
        This filter specifies a structured syntax to match against the
        Job.custom_attributes marked as `filterable`.
        
        The syntax for this expression is a subset of SQL syntax.
        
        Supported operators are: `=`, `!=`, `&lt;`, `&lt;=`, `&gt;`, and `&gt;=` where the
        left of the operator is a custom field key and the right of the operator
        is a number or a quoted string. You must escape backslash (\\) and
        quote (\&#34;) characters.
        
        Supported functions are `LOWER([field_name])` to
        perform a case insensitive match and `EMPTY([field_name])` to filter on the
        existence of a key.
        
        Boolean expressions (AND/OR/NOT) are supported up to 3 levels of
        nesting (for example, &#34;((A AND B AND C) OR NOT D) AND E&#34;), a maximum of 50
        comparisons or functions are allowed in the expression. The expression
        must be &lt; 3000 characters in length.
        
        Sample Query:
        `(LOWER(driving_license)=&#34;class \&#34;a\&#34;&#34; OR EMPTY(driving_license)) AND
        driving_years &gt; 10`
* `disable-spell-check=true`
    - Optional.
        
        This flag controls the spell-check feature. If false, the
        service attempts to correct a misspelled query,
        for example, &#34;enginee&#34; is corrected to &#34;engineer&#34;.
        
        Defaults to false: a spell check is performed.
* `employment-types=dolor`
    - Optional.
        
        The employment type filter specifies the employment type of jobs to
        search against, such as EmploymentType.FULL_TIME.
        
        If a value is not specified, jobs in the search results includes any
        employment type.
        
        If multiple values are specified, jobs in the search results include
        any of the specified employment types.
    - Each invocation of this argument appends the given value to the array.
* `job-categories=sanctus`
    - Optional.
        
        The category filter specifies the categories of jobs to search against.
        See Category for more information.
        
        If a value is not specified, jobs from any category are searched against.
        
        If multiple values are specified, jobs from any of the specified
        categories are searched against.
    - Each invocation of this argument appends the given value to the array.
* `language-codes=dolore`
    - Optional.
        
        This filter specifies the locale of jobs to search against,
        for example, &#34;en-US&#34;.
        
        If a value isn&#39;t specified, the search results can contain jobs in any
        locale.
        
        
        Language codes should be in BCP-47 format, such as &#34;en-US&#34; or &#34;sr-Latn&#34;.
        For more information, see
        [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
        
        At most 10 language code filters are allowed.
    - Each invocation of this argument appends the given value to the array.
* `publish-time-range    end-time=lorem`
    - End of the period.
* `start-time=consetetur`
    - Begin of the period.

* `..    query=consetetur`
    - Optional.
        
        The query string that matches against the job title, description, and
        location fields.
        
        The maximum number of allowed characters is 255.

* `..    job-view=eirmod`
    - Optional.
        
        The desired job attributes returned for jobs in the
        search response. Defaults to JobView.SMALL if no value is specified.
* `offset=65`
    - Optional.
        
        An integer that specifies the current offset (that is, starting result
        location, amongst the jobs deemed by the API as relevant) in search
        results. This field is only considered if page_token is unset.
        
        For example, 0 means to  return results starting from the first matching
        job, and 10 means to return from the 11th job. This can be used for
        pagination, (for example, pageSize = 10 and offset = 10 means to return
        from the second page).
* `order-by=gubergren`
    - Optional.
        
        The criteria determining how search results are sorted. Default is
        &#34;relevance desc&#34;.
        
        Supported options are:
        
        * &#34;relevance desc&#34;: By relevance descending, as determined by the API
        algorithms. Relevance thresholding of query results is only available
        with this ordering.
        * &#34;posting`_`publish`_`time desc&#34;: By Job.posting_publish_time descending.
        * &#34;posting`_`update`_`time desc&#34;: By Job.posting_update_time descending.
        * &#34;title&#34;: By Job.title ascending.
        * &#34;title desc&#34;: By Job.title descending.
        * &#34;annualized`_`base`_`compensation&#34;: By job&#39;s
        CompensationInfo.annualized_base_compensation_range ascending. Jobs
        whose annualized base compensation is unspecified are put at the end of
        search results.
        * &#34;annualized`_`base`_`compensation desc&#34;: By job&#39;s
        CompensationInfo.annualized_base_compensation_range descending. Jobs
        whose annualized base compensation is unspecified are put at the end of
        search results.
        * &#34;annualized`_`total`_`compensation&#34;: By job&#39;s
        CompensationInfo.annualized_total_compensation_range ascending. Jobs
        whose annualized base compensation is unspecified are put at the end of
        search results.
        * &#34;annualized`_`total`_`compensation desc&#34;: By job&#39;s
        CompensationInfo.annualized_total_compensation_range descending. Jobs
        whose annualized base compensation is unspecified are put at the end of
        search results.
* `page-size=28`
    - Optional.
        
        A limit on the number of jobs returned in the search results.
        Increasing this value above the default value of 10 can increase search
        response time. The value can be between 1 and 100.
* `page-token=sadipscing`
    - Optional.
        
        The token specifying the current offset within
        search results. See SearchJobsResponse.next_page_token for
        an explanation of how to obtain the next set of query results.
* `request-metadata.device-info    device-type=accusam`
    - Optional.
        
        Type of the device.
* `id=magna`
    - Optional.
        
        A device-specific ID. The ID must be a unique identifier that
        distinguishes the device from other devices.

* `..    domain=lorem`
    - Required.
        
        The client-defined scope or source of the service call, which typically
        is the domain on
        which the service has been implemented and is currently being run.
        
        For example, if the service is being run by client &lt;em&gt;Foo, Inc.&lt;/em&gt;, on
        job board www.foo.com and career site www.bar.com, then this field is
        set to &#34;foo.com&#34; for use on the job board, and &#34;bar.com&#34; for use on the
        career site.
        
        If this field isn&#39;t available for some reason, send &#34;UNKNOWN&#34;.
        Any improvements to the model for a particular tenant site rely on this
        field being set correctly to a domain.
        
        The maximum number of allowed characters is 255.
* `session-id=rebum.`
    - Required.
        
        A unique session identification string. A session is defined as the
        duration of an end user&#39;s interaction with the service over a certain
        period.
        Obfuscate this field for privacy concerns before
        providing it to the service.
        
        If this field is not available for some reason, send &#34;UNKNOWN&#34;. Note
        that any improvements to the model for a particular tenant
        site, rely on this field being set correctly to some unique session_id.
        
        The maximum number of allowed characters is 255.
* `user-id=et`
    - Required.
        
        A unique user identification string, as determined by the client.
        To have the strongest positive impact on search quality
        make sure the client-level is unique.
        Obfuscate this field for privacy concerns before
        providing it to the service.
        
        If this field is not available for some reason, send &#34;UNKNOWN&#34;. Note
        that any improvements to the model for a particular tenant
        site, rely on this field being set correctly to a unique user_id.
        
        The maximum number of allowed characters is 255.

* `..    require-precise-result-size=false`
    - Optional.
        
        Controls if the search job request requires the return of a precise
        count of the first 300 results. Setting this to `true` ensures
        consistency in the number of results per page. Best practice is to set this
        value to true if a client allows users to jump directly to a
        non-sequential search results page.
        
        Enabling this flag may adversely impact performance.
        
        Defaults to false.
* `search-mode=eos`
    - Optional.
        
        Mode of a search.
        
        Defaults to SearchMode.JOB_SEARCH.


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
