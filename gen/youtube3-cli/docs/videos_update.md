Updates a video&#39;s metadata.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> videos update ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Video:
  age-gating:
    alcohol-content: boolean
    restricted: boolean
    video-game-rating: string
  content-details:
    caption: string
    content-rating:
      acb-rating: string
      agcom-rating: string
      anatel-rating: string
      bbfc-rating: string
      bfvc-rating: string
      bmukk-rating: string
      catv-rating: string
      catvfr-rating: string
      cbfc-rating: string
      ccc-rating: string
      cce-rating: string
      chfilm-rating: string
      chvrs-rating: string
      cicf-rating: string
      cna-rating: string
      cnc-rating: string
      csa-rating: string
      cscf-rating: string
      czfilm-rating: string
      djctq-rating: string
      djctq-rating-reasons: [string]
      ecbmct-rating: string
      eefilm-rating: string
      egfilm-rating: string
      eirin-rating: string
      fcbm-rating: string
      fco-rating: string
      fmoc-rating: string
      fpb-rating: string
      fpb-rating-reasons: [string]
      fsk-rating: string
      grfilm-rating: string
      icaa-rating: string
      ifco-rating: string
      ilfilm-rating: string
      incaa-rating: string
      kfcb-rating: string
      kijkwijzer-rating: string
      kmrb-rating: string
      lsf-rating: string
      mccaa-rating: string
      mccyp-rating: string
      mcst-rating: string
      mda-rating: string
      medietilsynet-rating: string
      meku-rating: string
      mena-mpaa-rating: string
      mibac-rating: string
      moc-rating: string
      moctw-rating: string
      mpaa-rating: string
      mpaat-rating: string
      mtrcb-rating: string
      nbc-rating: string
      nbcpl-rating: string
      nfrc-rating: string
      nfvcb-rating: string
      nkclv-rating: string
      oflc-rating: string
      pefilm-rating: string
      rcnof-rating: string
      resorteviolencia-rating: string
      rtc-rating: string
      rte-rating: string
      russia-rating: string
      skfilm-rating: string
      smais-rating: string
      smsa-rating: string
      tvpg-rating: string
      yt-rating: string
    country-restriction:
      allowed: boolean
      exception: [string]
    definition: string
    dimension: string
    duration: string
    has-custom-thumbnail: boolean
    licensed-content: boolean
    projection: string
    region-restriction:
      allowed: [string]
      blocked: [string]
  etag: string
  file-details:
    bitrate-bps: string
    container: string
    creation-time: string
    duration-ms: string
    file-name: string
    file-size: string
    file-type: string
  id: string
  kind: string
  live-streaming-details:
    active-live-chat-id: string
    actual-end-time: string
    actual-start-time: string
    concurrent-viewers: string
    scheduled-end-time: string
    scheduled-start-time: string
  monetization-details:
    access:
      allowed: boolean
      exception: [string]
  player:
    embed-height: string
    embed-html: string
    embed-width: string
  processing-details:
    editor-suggestions-availability: string
    file-details-availability: string
    processing-failure-reason: string
    processing-issues-availability: string
    processing-progress:
      parts-processed: string
      parts-total: string
      time-left-ms: string
    processing-status: string
    tag-suggestions-availability: string
    thumbnails-availability: string
  project-details:
    tags: [string]
  recording-details:
    location:
      altitude: number
      latitude: number
      longitude: number
    location-description: string
    recording-date: string
  snippet:
    category-id: string
    channel-id: string
    channel-title: string
    default-audio-language: string
    default-language: string
    description: string
    live-broadcast-content: string
    localized:
      description: string
      title: string
    published-at: string
    tags: [string]
    thumbnails:
      default:
        height: integer
        url: string
        width: integer
      high:
        height: integer
        url: string
        width: integer
      maxres:
        height: integer
        url: string
        width: integer
      medium:
        height: integer
        url: string
        width: integer
      standard:
        height: integer
        url: string
        width: integer
    title: string
  statistics:
    comment-count: int64
    dislike-count: int64
    favorite-count: int64
    like-count: int64
    view-count: int64
  status:
    embeddable: boolean
    failure-reason: string
    license: string
    privacy-status: string
    public-stats-viewable: boolean
    publish-at: string
    rejection-reason: string
    upload-status: string
  suggestions:
    editor-suggestions: [string]
    processing-errors: [string]
    processing-hints: [string]
    processing-warnings: [string]
  topic-details:
    relevant-topic-ids: [string]
    topic-categories: [string]
    topic-ids: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .age-gating    alcohol-content=true`
    - Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content.
* `restricted=true`
    - Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won&#39;t be present.
* `video-game-rating=voluptua.`
    - Video game rating, if any.

* `..content-details    caption=labore`
    - The value of captions indicates whether the video has captions or not.
* `content-rating    acb-rating=et`
    - The video&#39;s Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children&#39;s television programming.
* `agcom-rating=no`
    - The video&#39;s rating from Italy&#39;s Autorità per le Garanzie nelle Comunicazioni (AGCOM).
* `anatel-rating=sed`
    - The video&#39;s Anatel (Asociación Nacional de Televisión) rating for Chilean television.
* `bbfc-rating=stet`
    - The video&#39;s British Board of Film Classification (BBFC) rating.
* `bfvc-rating=diam`
    - The video&#39;s rating from Thailand&#39;s Board of Film and Video Censors.
* `bmukk-rating=lorem`
    - The video&#39;s rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur).
* `catv-rating=amet.`
    - Rating system for Canadian TV - Canadian TV Classification System The video&#39;s rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
* `catvfr-rating=sed`
    - The video&#39;s rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
* `cbfc-rating=eos`
    - The video&#39;s Central Board of Film Certification (CBFC - India) rating.
* `ccc-rating=sadipscing`
    - The video&#39;s Consejo de Calificación Cinematográfica (Chile) rating.
* `cce-rating=erat`
    - The video&#39;s rating from Portugal&#39;s Comissão de Classificação de Espect´culos.
* `chfilm-rating=dolores`
    - The video&#39;s rating in Switzerland.
* `chvrs-rating=invidunt`
    - The video&#39;s Canadian Home Video Rating System (CHVRS) rating.
* `cicf-rating=ipsum`
    - The video&#39;s rating from the Commission de Contrôle des Films (Belgium).
* `cna-rating=rebum.`
    - The video&#39;s rating from Romania&#39;s CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA).
* `cnc-rating=dolor`
    - Rating system in France - Commission de classification cinematographique
* `csa-rating=nonumy`
    - The video&#39;s rating from France&#39;s Conseil supérieur de l?audiovisuel, which rates broadcast content.
* `cscf-rating=takimata`
    - The video&#39;s rating from Luxembourg&#39;s Commission de surveillance de la classification des films (CSCF).
* `czfilm-rating=et`
    - The video&#39;s rating in the Czech Republic.
* `djctq-rating=sanctus`
    - The video&#39;s Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating.
* `djctq-rating-reasons=sit`
    - Reasons that explain why the video received its DJCQT (Brazil) rating.
    - Each invocation of this argument appends the given value to the array.
* `ecbmct-rating=ea`
    - Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism
* `eefilm-rating=lorem`
    - The video&#39;s rating in Estonia.
* `egfilm-rating=voluptua.`
    - The video&#39;s rating in Egypt.
* `eirin-rating=duo`
    - The video&#39;s Eirin (映倫) rating. Eirin is the Japanese rating system.
* `fcbm-rating=dolor`
    - The video&#39;s rating from Malaysia&#39;s Film Censorship Board.
* `fco-rating=tempor`
    - The video&#39;s rating from Hong Kong&#39;s Office for Film, Newspaper and Article Administration.
* `fmoc-rating=dolor`
    - This property has been deprecated. Use the contentDetails.contentRating.cncRating instead.
* `fpb-rating=ea`
    - The video&#39;s rating from South Africa&#39;s Film and Publication Board.
* `fpb-rating-reasons=elitr`
    - Reasons that explain why the video received its FPB (South Africa) rating.
    - Each invocation of this argument appends the given value to the array.
* `fsk-rating=voluptua.`
    - The video&#39;s Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating.
* `grfilm-rating=et`
    - The video&#39;s rating in Greece.
* `icaa-rating=diam`
    - The video&#39;s Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating.
* `ifco-rating=amet.`
    - The video&#39;s Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information.
* `ilfilm-rating=justo`
    - The video&#39;s rating in Israel.
* `incaa-rating=et`
    - The video&#39;s INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating.
* `kfcb-rating=rebum.`
    - The video&#39;s rating from the Kenya Film Classification Board.
* `kijkwijzer-rating=eos`
    - voor de Classificatie van Audiovisuele Media (Netherlands).
* `kmrb-rating=diam`
    - The video&#39;s Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea.
* `lsf-rating=eos`
    - The video&#39;s rating from Indonesia&#39;s Lembaga Sensor Film.
* `mccaa-rating=dolore`
    - The video&#39;s rating from Malta&#39;s Film Age-Classification Board.
* `mccyp-rating=sea`
    - The video&#39;s rating from the Danish Film Institute&#39;s (Det Danske Filminstitut) Media Council for Children and Young People.
* `mcst-rating=lorem`
    - The video&#39;s rating system for Vietnam - MCST
* `mda-rating=et`
    - The video&#39;s rating from Singapore&#39;s Media Development Authority (MDA) and, specifically, it&#39;s Board of Film Censors (BFC).
* `medietilsynet-rating=tempor`
    - The video&#39;s rating from Medietilsynet, the Norwegian Media Authority.
* `meku-rating=erat`
    - The video&#39;s rating from Finland&#39;s Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute).
* `mena-mpaa-rating=eirmod`
    - The rating system for MENA countries, a clone of MPAA. It is needed to
* `mibac-rating=kasd`
    - The video&#39;s rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy).
* `moc-rating=invidunt`
    - The video&#39;s Ministerio de Cultura (Colombia) rating.
* `moctw-rating=sadipscing`
    - The video&#39;s rating from Taiwan&#39;s Ministry of Culture (文化部).
* `mpaa-rating=labore`
    - The video&#39;s Motion Picture Association of America (MPAA) rating.
* `mpaat-rating=nonumy`
    - The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html.
* `mtrcb-rating=voluptua.`
    - The video&#39;s rating from the Movie and Television Review and Classification Board (Philippines).
* `nbc-rating=justo`
    - The video&#39;s rating from the Maldives National Bureau of Classification.
* `nbcpl-rating=elitr`
    - The video&#39;s rating in Poland.
* `nfrc-rating=takimata`
    - The video&#39;s rating from the Bulgarian National Film Center.
* `nfvcb-rating=rebum.`
    - The video&#39;s rating from Nigeria&#39;s National Film and Video Censors Board.
* `nkclv-rating=dolore`
    - The video&#39;s rating from the Nacionãlais Kino centrs (National Film Centre of Latvia).
* `oflc-rating=nonumy`
    - The video&#39;s Office of Film and Literature Classification (OFLC - New Zealand) rating.
* `pefilm-rating=erat`
    - The video&#39;s rating in Peru.
* `rcnof-rating=eos`
    - The video&#39;s rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film.
* `resorteviolencia-rating=justo`
    - The video&#39;s rating in Venezuela.
* `rtc-rating=justo`
    - The video&#39;s General Directorate of Radio, Television and Cinematography (Mexico) rating.
* `rte-rating=rebum.`
    - The video&#39;s rating from Ireland&#39;s Raidió Teilifís Éireann.
* `russia-rating=ea`
    - The video&#39;s National Film Registry of the Russian Federation (MKRF - Russia) rating.
* `skfilm-rating=sit`
    - The video&#39;s rating in Slovakia.
* `smais-rating=lorem`
    - The video&#39;s rating in Iceland.
* `smsa-rating=sadipscing`
    - The video&#39;s rating from Statens medieråd (Sweden&#39;s National Media Council).
* `tvpg-rating=voluptua.`
    - The video&#39;s TV Parental Guidelines (TVPG) rating.
* `yt-rating=justo`
    - A rating that YouTube uses to identify age-restricted content.

* `..country-restriction    allowed=false`
    - The value of allowed indicates whether the access to the policy is allowed or denied by default.
* `exception=at`
    - A list of region codes that identify countries where the default policy do not apply.
    - Each invocation of this argument appends the given value to the array.

* `..    definition=vero`
    - The value of definition indicates whether the video is available in high definition or only in standard definition.
* `dimension=sed`
    - The value of dimension indicates whether the video is available in 3D or in 2D.
* `duration=dolore`
    - The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.
* `has-custom-thumbnail=true`
    - Indicates whether the video uploader has provided a custom thumbnail image for the video. This property is only visible to the video uploader.
* `licensed-content=true`
    - The value of is_license_content indicates whether the video is licensed content.
* `projection=justo`
    - Specifies the projection format of the video.
* `region-restriction    allowed=ipsum`
    - A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries.
    - Each invocation of this argument appends the given value to the array.
* `blocked=aliquyam`
    - A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries.
    - Each invocation of this argument appends the given value to the array.


* `...    etag=est`
    - Etag of this resource.
* `file-details    bitrate-bps=nonumy`
    - The uploaded video file&#39;s combined (video and audio) bitrate in bits per second.
* `container=amet.`
    - The uploaded video file&#39;s container format.
* `creation-time=gubergren`
    - The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported:  
        - Date only: YYYY-MM-DD 
        - Naive time: YYYY-MM-DDTHH:MM:SS 
        - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM
* `duration-ms=ipsum`
    - The length of the uploaded video in milliseconds.
* `file-name=kasd`
    - The uploaded file&#39;s name. This field is present whether a video file or another type of file was uploaded.
* `file-size=eos`
    - The uploaded file&#39;s size in bytes. This field is present whether a video file or another type of file was uploaded.
* `file-type=lorem`
    - The uploaded file&#39;s type as detected by YouTube&#39;s video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.

* `..    id=kasd`
    - The ID that YouTube uses to uniquely identify the video.
* `kind=at`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#video&#34;.
* `live-streaming-details    active-live-chat-id=magna`
    - The ID of the currently active live chat attached to this video. This field is filled only if the video is a currently live broadcast that has live chat. Once the broadcast transitions to complete this field will be removed and the live chat closed down. For persistent broadcasts that live chat id will no longer be tied to this video but rather to the new video being displayed at the persistent page.
* `actual-end-time=amet`
    - The time that the broadcast actually ended. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast is over.
* `actual-start-time=rebum.`
    - The time that the broadcast actually started. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast begins.
* `concurrent-viewers=sed`
    - The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
* `scheduled-end-time=dolore`
    - The time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.
* `scheduled-start-time=est`
    - The time that the broadcast is scheduled to begin. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.

* `..monetization-details.access    allowed=false`
    - The value of allowed indicates whether the access to the policy is allowed or denied by default.
* `exception=no`
    - A list of region codes that identify countries where the default policy do not apply.
    - Each invocation of this argument appends the given value to the array.


* `...player    embed-height=rebum.`
    - No description provided.
* `embed-html=et`
    - An &lt;iframe&gt; tag that embeds a player that will play the video.
* `embed-width=et`
    - The embed width

* `..processing-details    editor-suggestions-availability=sea`
    - This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
* `file-details-availability=consetetur`
    - This value indicates whether file details are available for the uploaded video. You can retrieve a video&#39;s file details by requesting the fileDetails part in your videos.list() request.
* `processing-failure-reason=amet.`
    - The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property&#39;s value is failed.
* `processing-issues-availability=amet.`
    - This value indicates whether the video processing engine has generated suggestions that might improve YouTube&#39;s ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
* `processing-progress    parts-processed=et`
    - The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating:
        100 * parts_processed / parts_total
        
        Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.
* `parts-total=dolor`
    - An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.
* `time-left-ms=dolor`
    - An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video.

* `..    processing-status=dolore`
    - The video&#39;s processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
* `tag-suggestions-availability=sit`
    - This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video&#39;s metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
* `thumbnails-availability=ipsum`
    - This value indicates whether thumbnail images have been generated for the video.

* `..project-details    tags=ut`
    - A list of project tags associated with the video during the upload.
    - Each invocation of this argument appends the given value to the array.

* `..recording-details.location    altitude=0.346502284116`
    - Altitude above the reference ellipsoid, in meters.
* `latitude=0.360562764272`
    - Latitude in degrees.
* `longitude=0.274993978236`
    - Longitude in degrees.

* `..    location-description=lorem`
    - The text description of the location where the video was recorded.
* `recording-date=et`
    - The date and time when the video was recorded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sssZ) format.

* `..snippet    category-id=justo`
    - The YouTube video category associated with the video.
* `channel-id=sed`
    - The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.
* `channel-title=lorem`
    - Channel title for the channel that the video belongs to.
* `default-audio-language=sed`
    - The default_audio_language property specifies the language spoken in the video&#39;s default audio track.
* `default-language=dolores`
    - The language of the videos&#39;s default snippet.
* `description=et`
    - The video&#39;s description.
* `live-broadcast-content=elitr`
    - Indicates if the video is an upcoming/active live broadcast. Or it&#39;s &#34;none&#34; if the video is not an upcoming/active live broadcast.
* `localized    description=vero`
    - Localized version of the video&#39;s description.
* `title=duo`
    - Localized version of the video&#39;s title.

* `..    published-at=consetetur`
    - The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `tags=at`
    - A list of keyword tags associated with the video. Tags may contain spaces.
    - Each invocation of this argument appends the given value to the array.
* `thumbnails.default    height=81`
    - (Optional) Height of the thumbnail image.
* `url=tempor`
    - The thumbnail image&#39;s URL.
* `width=44`
    - (Optional) Width of the thumbnail image.

* `..high    height=40`
    - (Optional) Height of the thumbnail image.
* `url=dolore`
    - The thumbnail image&#39;s URL.
* `width=22`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=71`
    - (Optional) Height of the thumbnail image.
* `url=sadipscing`
    - The thumbnail image&#39;s URL.
* `width=80`
    - (Optional) Width of the thumbnail image.

* `..medium    height=0`
    - (Optional) Height of the thumbnail image.
* `url=sit`
    - The thumbnail image&#39;s URL.
* `width=4`
    - (Optional) Width of the thumbnail image.

* `..standard    height=12`
    - (Optional) Height of the thumbnail image.
* `url=aliquyam`
    - The thumbnail image&#39;s URL.
* `width=4`
    - (Optional) Width of the thumbnail image.


* `...    title=amet.`
    - The video&#39;s title.

* `..statistics    comment-count=-44`
    - The number of comments for the video.
* `dislike-count=-67`
    - The number of users who have indicated that they disliked the video by giving it a negative rating.
* `favorite-count=-40`
    - The number of users who currently have the video marked as a favorite video.
* `like-count=-40`
    - The number of users who have indicated that they liked the video by giving it a positive rating.
* `view-count=-13`
    - The number of times the video has been viewed.

* `..status    embeddable=false`
    - This value indicates if the video can be embedded on another website.
* `failure-reason=invidunt`
    - This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
* `license=nonumy`
    - The video&#39;s license.
* `privacy-status=at`
    - The video&#39;s privacy status.
* `public-stats-viewable=false`
    - This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled.
* `publish-at=eirmod`
    - The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `rejection-reason=sit`
    - This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
* `upload-status=sed`
    - The status of the uploaded video.

* `..suggestions    editor-suggestions=diam`
    - A list of video editing operations that might improve the video quality or playback experience of the uploaded video.
    - Each invocation of this argument appends the given value to the array.
* `processing-errors=at`
    - A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video&#39;s current processing status, eventually, that status will almost certainly be failed.
    - Each invocation of this argument appends the given value to the array.
* `processing-hints=takimata`
    - A list of suggestions that may improve YouTube&#39;s ability to process the video.
    - Each invocation of this argument appends the given value to the array.
* `processing-warnings=et`
    - A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.
    - Each invocation of this argument appends the given value to the array.

* `..topic-details    relevant-topic-ids=sed`
    - Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.
    - Each invocation of this argument appends the given value to the array.
* `topic-categories=kasd`
    - A list of Wikipedia URLs that provide a high-level description of the video&#39;s content.
    - Each invocation of this argument appends the given value to the array.
* `topic-ids=aliquyam`
    - A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the Freebase Topic API.
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

* **-p on-behalf-of-content-owner=string**
    - Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwner parameter indicates that the request&#39;s authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

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
