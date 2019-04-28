Creates a new job.

Typically, the job becomes searchable within 10 seconds, but it may take
up to 5 minutes.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/jobs*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `jobs3 --scope <scope> projects jobs-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The resource name of the project under which the job is created.
        
        The format is &#34;projects/{project_id}&#34;, for example,
        &#34;projects/api-test-project&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateJobRequest:
  job:
    addresses: [string]
    application-info:
      emails: [string]
      instruction: string
      uris: [string]
    company-display-name: string
    company-name: string
    compensation-info:
      annualized-base-compensation-range:
        max-compensation:
          currency-code: string
          nanos: integer
          units: string
        min-compensation:
          currency-code: string
          nanos: integer
          units: string
      annualized-total-compensation-range:
        max-compensation:
          currency-code: string
          nanos: integer
          units: string
        min-compensation:
          currency-code: string
          nanos: integer
          units: string
    degree-types: [string]
    department: string
    derived-info:
      job-categories: [string]
    description: string
    employment-types: [string]
    incentives: string
    job-benefits: [string]
    job-end-time: string
    job-level: string
    job-start-time: string
    language-code: string
    name: string
    posting-create-time: string
    posting-expire-time: string
    posting-publish-time: string
    posting-region: string
    posting-update-time: string
    processing-options:
      disable-street-address-resolution: boolean
      html-sanitization: string
    promotion-value: integer
    qualifications: string
    requisition-id: string
    responsibilities: string
    title: string
    visibility: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .job    addresses=et`
    - Optional but strongly recommended for the best service experience.
        
        Location(s) where the employer is looking to hire for this job posting.
        
        Specifying the full street address(es) of the hiring location enables
        better API results, especially job searches by commute time.
        
        At most 50 locations are allowed for best search performance. If a job has
        more locations, it is suggested to split it into multiple jobs with unique
        requisition_ids (e.g. &#39;ReqA&#39; becomes &#39;ReqA-1&#39;, &#39;ReqA-2&#39;, etc.) as
        multiple jobs with the same company_name, language_code and
        requisition_id are not allowed. If the original requisition_id must
        be preserved, a custom field should be used for storage. It is also
        suggested to group the locations that close to each other in the same job
        for better search experience.
        
        The maximum number of allowed characters is 500.
    - Each invocation of this argument appends the given value to the array.
* `application-info    emails=consetetur`
    - Optional but at least one of uris,
        emails or instruction must be
        specified.
        
        Use this field to specify email address(es) to which resumes or
        applications can be sent.
        
        The maximum number of allowed characters for each entry is 255.
    - Each invocation of this argument appends the given value to the array.
* `instruction=ut`
    - Optional but at least one of uris,
        emails or instruction must be
        specified.
        
        Use this field to provide instructions, such as &#34;Mail your application
        to ...&#34;, that a candidate can follow to apply for the job.
        
        This field accepts and sanitizes HTML input, and also accepts
        bold, italic, ordered list, and unordered list markup tags.
        
        The maximum number of allowed characters is 3,000.
* `uris=ea`
    - Optional but at least one of uris,
        emails or instruction must be
        specified.
        
        Use this URI field to direct an applicant to a website, for example to
        link to an online application form.
        
        The maximum number of allowed characters for each entry is 2,000.
    - Each invocation of this argument appends the given value to the array.

* `..    company-display-name=sed`
    - Output only. Display name of the company listing the job.
* `company-name=dolor`
    - Required.
        
        The resource name of the company listing the job, such as
        &#34;projects/api-test-project/companies/foo&#34;.
* `compensation-info.annualized-base-compensation-range.max-compensation    currency-code=dolor`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=53`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=et`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* `..min-compensation    currency-code=consetetur`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=49`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=voluptua.`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.


* `...annualized-total-compensation-range.max-compensation    currency-code=lorem`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=90`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=justo`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.

* `..min-compensation    currency-code=sit`
    - The 3-letter currency code defined in ISO 4217.
* `nanos=75`
    - Number of nano (10^-9) units of the amount.
        The value must be between -999,999,999 and +999,999,999 inclusive.
        If `units` is positive, `nanos` must be positive or zero.
        If `units` is zero, `nanos` can be positive, zero, or negative.
        If `units` is negative, `nanos` must be negative or zero.
        For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
* `units=diam`
    - The whole units of the amount.
        For example if `currencyCode` is `&#34;USD&#34;`, then 1 unit is one US dollar.



* `....    degree-types=rebum.`
    - Optional.
        
        The desired education degrees for the job, such as Bachelors, Masters.
    - Each invocation of this argument appends the given value to the array.
* `department=consetetur`
    - Optional.
        
        The department or functional area within the company with the open
        position.
        
        The maximum number of allowed characters is 255.
* `derived-info    job-categories=sadipscing`
    - Job categories derived from Job.title and Job.description.
    - Each invocation of this argument appends the given value to the array.

* `..    description=vero`
    - Required.
        
        The description of the job, which typically includes a multi-paragraph
        description of the company and related information. Separate fields are
        provided on the job object for responsibilities,
        qualifications, and other job characteristics. Use of
        these separate job fields is recommended.
        
        This field accepts and sanitizes HTML input, and also accepts
        bold, italic, ordered list, and unordered list markup tags.
        
        The maximum number of allowed characters is 100,000.
* `employment-types=sadipscing`
    - Optional.
        
        The employment type(s) of a job, for example,
        full time or
        part time.
    - Each invocation of this argument appends the given value to the array.
* `incentives=invidunt`
    - Optional.
        
        A description of bonus, commission, and other compensation
        incentives associated with the job not including salary or pay.
        
        The maximum number of allowed characters is 10,000.
* `job-benefits=consetetur`
    - Optional.
        
        The benefits included with the job.
    - Each invocation of this argument appends the given value to the array.
* `job-end-time=dolore`
    - Optional.
        
        The end timestamp of the job. Typically this field is used for contracting
        engagements. Invalid timestamps are ignored.
* `job-level=duo`
    - Optional.
        
        The experience level associated with the job, such as &#34;Entry Level&#34;.
* `job-start-time=aliquyam`
    - Optional.
        
        The start timestamp of the job in UTC time zone. Typically this field
        is used for contracting engagements. Invalid timestamps are ignored.
* `language-code=lorem`
    - Optional.
        
        The language of the posting. This field is distinct from
        any requirements for fluency that are associated with the job.
        
        Language codes must be in BCP-47 format, such as &#34;en-US&#34; or &#34;sr-Latn&#34;.
        For more information, see
        [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){:
        class=&#34;external&#34; target=&#34;_blank&#34; }.
        
        If this field is unspecified and Job.description is present, detected
        language code based on Job.description is assigned, otherwise
        defaults to &#39;en_US&#39;.
* `name=et`
    - Required during job update.
        
        The resource name for the job. This is generated by the service when a
        job is created.
        
        The format is &#34;projects/{project_id}/jobs/{job_id}&#34;,
        for example, &#34;projects/api-test-project/jobs/1234&#34;.
        
        Use of this field in job queries and API calls is preferred over the use of
        requisition_id since this value is unique.
* `posting-create-time=clita`
    - Output only. The timestamp when this job posting was created.
* `posting-expire-time=consetetur`
    - Optional but strongly recommended for the best service
        experience.
        
        The expiration timestamp of the job. After this timestamp, the
        job is marked as expired, and it no longer appears in search results. The
        expired job can&#39;t be deleted or listed by the DeleteJob and
        ListJobs APIs, but it can be retrieved with the GetJob API or
        updated with the UpdateJob API. An expired job can be updated and
        opened again by using a future expiration timestamp. Updating an expired
        job fails if there is another existing open job with same company_name,
        language_code and requisition_id.
        
        The expired jobs are retained in our system for 90 days. However, the
        overall expired job count cannot exceed 3 times the maximum of open jobs
        count over the past week, otherwise jobs with earlier expire time are
        cleaned first. Expired jobs are no longer accessible after they are cleaned
        out.
        
        Invalid timestamps are ignored, and treated as expire time not provided.
        
        Timestamp before the instant request is made is considered valid, the job
        will be treated as expired immediately.
        
        If this value is not provided at the time of job creation or is invalid,
        the job posting expires after 30 days from the job&#39;s creation time. For
        example, if the job was created on 2017/01/01 13:00AM UTC with an
        unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC.
        
        If this value is not provided on job update, it depends on the field masks
        set by UpdateJobRequest.update_mask. If the field masks include
        expiry_time, or the masks are empty meaning that every field is
        updated, the job posting expires after 30 days from the job&#39;s last
        update time. Otherwise the expiration date isn&#39;t updated.
* `posting-publish-time=takimata`
    - Optional.
        
        The timestamp this job posting was most recently published. The default
        value is the time the request arrives at the server. Invalid timestamps are
        ignored.
* `posting-region=nonumy`
    - Optional.
        
        The job PostingRegion (for example, state, country) throughout which
        the job is available. If this field is set, a
        LocationFilter in a search query within the job region
        finds this job posting if an exact location match is not specified.
        If this field is set to PostingRegion.NATION_WIDE or
        [PostingRegion.ADMINISTRATIVE_AREA], setting job addresses
        to the same location level as this field is strongly recommended.
* `posting-update-time=kasd`
    - Output only. The timestamp when this job posting was last updated.
* `processing-options    disable-street-address-resolution=true`
    - Optional.
        
        If set to `true`, the service does not attempt to resolve a
        more precise address for the job.
* `html-sanitization=takimata`
    - Optional.
        
        Option for job HTML content sanitization. Applied fields are:
        
        * description
        * applicationInfo.instruction
        * incentives
        * qualifications
        * responsibilities
        
        HTML tags in these fields may be stripped if sanitiazation is not
        disabled.
        
        Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY.

* `..    promotion-value=74`
    - Optional.
        
        A promotion value of the job, as determined by the client.
        The value determines the sort order of the jobs returned when searching for
        jobs using the featured jobs search call, with higher promotional values
        being returned first and ties being resolved by relevance sort. Only the
        jobs with a promotionValue &gt;0 are returned in a FEATURED_JOB_SEARCH.
        
        Default value is 0, and negative values are treated as 0.
* `qualifications=labore`
    - Optional.
        
        A description of the qualifications required to perform the
        job. The use of this field is recommended
        as an alternative to using the more general description field.
        
        This field accepts and sanitizes HTML input, and also accepts
        bold, italic, ordered list, and unordered list markup tags.
        
        The maximum number of allowed characters is 10,000.
* `requisition-id=invidunt`
    - Required.
        
        The requisition ID, also referred to as the posting ID, assigned by the
        client to identify a job. This field is intended to be used by clients
        for client identification and tracking of postings. A job is not allowed
        to be created if there is another job with the same [company_name],
        language_code and requisition_id.
        
        The maximum number of allowed characters is 255.
* `responsibilities=ea`
    - Optional.
        
        A description of job responsibilities. The use of this field is
        recommended as an alternative to using the more general description
        field.
        
        This field accepts and sanitizes HTML input, and also accepts
        bold, italic, ordered list, and unordered list markup tags.
        
        The maximum number of allowed characters is 10,000.
* `title=sadipscing`
    - Required.
        
        The title of the job, such as &#34;Software Engineer&#34;
        
        The maximum number of allowed characters is 500.
* `visibility=rebum.`
    - Optional.
        
        The visibility of the job.
        
        Defaults to Visibility.ACCOUNT_ONLY if not specified.



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