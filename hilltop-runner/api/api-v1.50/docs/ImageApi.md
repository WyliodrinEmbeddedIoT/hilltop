# \ImageApi

All URIs are relative to */v1.50*

Method | HTTP request | Description
------------- | ------------- | -------------
[**build_prune**](ImageApi.md#build_prune) | **POST** /build/prune | Delete builder cache
[**image_build**](ImageApi.md#image_build) | **POST** /build | Build an image
[**image_commit**](ImageApi.md#image_commit) | **POST** /commit | Create a new image from a container
[**image_create**](ImageApi.md#image_create) | **POST** /images/create | Create an image
[**image_delete**](ImageApi.md#image_delete) | **DELETE** /images/{name} | Remove an image
[**image_get**](ImageApi.md#image_get) | **GET** /images/{name}/get | Export an image
[**image_get_all**](ImageApi.md#image_get_all) | **GET** /images/get | Export several images
[**image_history**](ImageApi.md#image_history) | **GET** /images/{name}/history | Get the history of an image
[**image_inspect**](ImageApi.md#image_inspect) | **GET** /images/{name}/json | Inspect an image
[**image_list**](ImageApi.md#image_list) | **GET** /images/json | List Images
[**image_load**](ImageApi.md#image_load) | **POST** /images/load | Import images
[**image_prune**](ImageApi.md#image_prune) | **POST** /images/prune | Delete unused images
[**image_push**](ImageApi.md#image_push) | **POST** /images/{name}/push | Push an image
[**image_search**](ImageApi.md#image_search) | **GET** /images/search | Search images
[**image_tag**](ImageApi.md#image_tag) | **POST** /images/{name}/tag | Tag an image



## build_prune

> models::BuildPruneResponse build_prune(keep_storage, reserved_space, max_used_space, min_free_space, all, filters)
Delete builder cache

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keep_storage** | Option<**i64**> | Amount of disk space in bytes to keep for cache  > **Deprecated**: This parameter is deprecated and has been renamed to \"reserved-space\". > It is kept for backward compatibility and will be removed in API v1.52.  |  |
**reserved_space** | Option<**i64**> | Amount of disk space in bytes to keep for cache |  |
**max_used_space** | Option<**i64**> | Maximum amount of disk space allowed to keep for cache |  |
**min_free_space** | Option<**i64**> | Target amount of free disk space after pruning |  |
**all** | Option<**bool**> | Remove all types of build cache |  |
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the list of build cache objects.  Available filters:  - `until=<timestamp>` remove cache older than `<timestamp>`. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon's local time. - `id=<id>` - `parent=<id>` - `type=<string>` - `description=<string>` - `inuse` - `shared` - `private`  |  |

### Return type

[**models::BuildPruneResponse**](BuildPruneResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_build

> image_build(dockerfile, t, extrahosts, remote, q, nocache, cachefrom, pull, rm, forcerm, memory, memswap, cpushares, cpusetcpus, cpuperiod, cpuquota, buildargs, shmsize, squash, labels, networkmode, content_type, x_registry_config, platform, target, outputs, version, input_stream)
Build an image

Build an image from a tar archive with a `Dockerfile` in it.  The `Dockerfile` specifies how the image is built from the tar archive. It is typically in the archive's root, but can be at a different path or have a different name by specifying the `dockerfile` parameter. [See the `Dockerfile` reference for more information](https://docs.docker.com/engine/reference/builder/).  The Docker daemon performs a preliminary validation of the `Dockerfile` before starting the build, and returns an error if the syntax is incorrect. After that, each instruction is run one-by-one until the ID of the new image is output.  The build is canceled if the client drops the connection by quitting or being killed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dockerfile** | Option<**String**> | Path within the build context to the `Dockerfile`. This is ignored if `remote` is specified and points to an external `Dockerfile`. |  |[default to Dockerfile]
**t** | Option<**String**> | A name and optional tag to apply to the image in the `name:tag` format. If you omit the tag the default `latest` value is assumed. You can provide several `t` parameters. |  |
**extrahosts** | Option<**String**> | Extra hosts to add to /etc/hosts |  |
**remote** | Option<**String**> | A Git repository URI or HTTP/HTTPS context URI. If the URI points to a single text file, the file’s contents are placed into a file called `Dockerfile` and the image is built from that file. If the URI points to a tarball, the file is downloaded by the daemon and the contents therein used as the context for the build. If the URI points to a tarball and the `dockerfile` parameter is also specified, there must be a file with the corresponding path inside the tarball. |  |
**q** | Option<**bool**> | Suppress verbose build output. |  |[default to false]
**nocache** | Option<**bool**> | Do not use the cache when building the image. |  |[default to false]
**cachefrom** | Option<**String**> | JSON array of images used for build cache resolution. |  |
**pull** | Option<**String**> | Attempt to pull the image even if an older image exists locally. |  |
**rm** | Option<**bool**> | Remove intermediate containers after a successful build. |  |[default to true]
**forcerm** | Option<**bool**> | Always remove intermediate containers, even upon failure. |  |[default to false]
**memory** | Option<**i32**> | Set memory limit for build. |  |
**memswap** | Option<**i32**> | Total memory (memory + swap). Set as `-1` to disable swap. |  |
**cpushares** | Option<**i32**> | CPU shares (relative weight). |  |
**cpusetcpus** | Option<**String**> | CPUs in which to allow execution (e.g., `0-3`, `0,1`). |  |
**cpuperiod** | Option<**i32**> | The length of a CPU period in microseconds. |  |
**cpuquota** | Option<**i32**> | Microseconds of CPU time that the container can get in a CPU period. |  |
**buildargs** | Option<**String**> | JSON map of string pairs for build-time variables. Users pass these values at build-time. Docker uses the buildargs as the environment context for commands run via the `Dockerfile` RUN instruction, or for variable expansion in other `Dockerfile` instructions. This is not meant for passing secret values.  For example, the build arg `FOO=bar` would become `{\"FOO\":\"bar\"}` in JSON. This would result in the query parameter `buildargs={\"FOO\":\"bar\"}`. Note that `{\"FOO\":\"bar\"}` should be URI component encoded.  [Read more about the buildargs instruction.](https://docs.docker.com/engine/reference/builder/#arg)  |  |
**shmsize** | Option<**i32**> | Size of `/dev/shm` in bytes. The size must be greater than 0. If omitted the system uses 64MB. |  |
**squash** | Option<**bool**> | Squash the resulting images layers into a single layer. *(Experimental release only.)* |  |
**labels** | Option<**String**> | Arbitrary key/value labels to set on the image, as a JSON map of string pairs. |  |
**networkmode** | Option<**String**> | Sets the networking mode for the run commands during build. Supported standard values are: `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's name or ID to which this container should connect to.  |  |
**content_type** | Option<**String**> |  |  |[default to application/x-tar]
**x_registry_config** | Option<**String**> | This is a base64-encoded JSON object with auth configurations for multiple registries that a build may refer to.  The key is a registry URL, and the value is an auth configuration object, [as described in the authentication section](#section/Authentication). For example:  ``` {   \"docker.example.com\": {     \"username\": \"janedoe\",     \"password\": \"hunter2\"   },   \"https://index.docker.io/v1/\": {     \"username\": \"mobydock\",     \"password\": \"conta1n3rize14\"   } } ```  Only the registry domain name (and port if not the default 443) are required. However, for legacy reasons, the Docker Hub registry must be specified with both a `https://` prefix and a `/v1/` suffix even though Docker will prefer to use the v2 registry API.  |  |
**platform** | Option<**String**> | Platform in the format os[/arch[/variant]] |  |
**target** | Option<**String**> | Target build stage |  |
**outputs** | Option<**String**> | BuildKit output configuration in the format of a stringified JSON array of objects. Each object must have two top-level properties: `Type` and `Attrs`. The `Type` property must be set to 'moby'. The `Attrs` property is a map of attributes for the BuildKit output configuration. See https://docs.docker.com/build/exporters/oci-docker/ for more information.  Example:  ``` [{\"Type\":\"moby\",\"Attrs\":{\"type\":\"image\",\"force-compression\":\"true\",\"compression\":\"zstd\"}}] ```  |  |
**version** | Option<**String**> | Version of the builder backend to use.  - `1` is the first generation classic (deprecated) builder in the Docker daemon (default) - `2` is [BuildKit](https://github.com/moby/buildkit)  |  |[default to 1]
**input_stream** | Option<**std::path::PathBuf**> | A tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_commit

> models::IdResponse image_commit(container, repo, tag, comment, author, pause, changes, container_config)
Create a new image from a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container** | Option<**String**> | The ID or name of the container to commit |  |
**repo** | Option<**String**> | Repository name for the created image |  |
**tag** | Option<**String**> | Tag name for the create image |  |
**comment** | Option<**String**> | Commit message |  |
**author** | Option<**String**> | Author of the image (e.g., `John Hannibal Smith <hannibal@a-team.com>`) |  |
**pause** | Option<**bool**> | Whether to pause the container before committing |  |[default to true]
**changes** | Option<**String**> | `Dockerfile` instructions to apply while committing |  |
**container_config** | Option<[**ContainerConfig**](ContainerConfig.md)> | The container configuration |  |

### Return type

[**models::IdResponse**](IDResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_create

> image_create(from_image, from_src, repo, tag, message, x_registry_auth, changes, platform, input_image)
Create an image

Pull or import an image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_image** | Option<**String**> | Name of the image to pull. If the name includes a tag or digest, specific behavior applies:  - If only `fromImage` includes a tag, that tag is used. - If both `fromImage` and `tag` are provided, `tag` takes precedence. - If `fromImage` includes a digest, the image is pulled by digest, and `tag` is ignored. - If neither a tag nor digest is specified, all tags are pulled.  |  |
**from_src** | Option<**String**> | Source to import. The value may be a URL from which the image can be retrieved or `-` to read the image from the request body. This parameter may only be used when importing an image. |  |
**repo** | Option<**String**> | Repository name given to an image when it is imported. The repo may include a tag. This parameter may only be used when importing an image. |  |
**tag** | Option<**String**> | Tag or digest. If empty when pulling an image, this causes all tags for the given image to be pulled. |  |
**message** | Option<**String**> | Set commit message for imported image. |  |
**x_registry_auth** | Option<**String**> | A base64url-encoded auth configuration.  Refer to the [authentication section](#section/Authentication) for details.  |  |
**changes** | Option<[**Vec<String>**](String.md)> | Apply `Dockerfile` instructions to the image that is created, for example: `changes=ENV DEBUG=true`. Note that `ENV DEBUG=true` should be URI component encoded.  Supported `Dockerfile` instructions: `CMD`|`ENTRYPOINT`|`ENV`|`EXPOSE`|`ONBUILD`|`USER`|`VOLUME`|`WORKDIR`  |  |
**platform** | Option<**String**> | Platform in the format os[/arch[/variant]].  When used in combination with the `fromImage` option, the daemon checks if the given image is present in the local image cache with the given OS and Architecture, and otherwise attempts to pull the image. If the option is not set, the host's native OS and Architecture are used. If the given image does not exist in the local image cache, the daemon attempts to pull the image with the host's native OS and Architecture. If the given image does exists in the local image cache, but its OS or architecture does not match, a warning is produced.  When used with the `fromSrc` option to import an image from an archive, this option sets the platform information for the imported image. If the option is not set, the host's native OS and Architecture are used for the imported image.  |  |
**input_image** | Option<**String**> | Image content if the value `-` has been specified in fromSrc query parameter |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain, application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_delete

> Vec<models::ImageDeleteResponseItem> image_delete(name, force, noprune, platforms)
Remove an image

Remove an image, along with any untagged parent images that were referenced by that image.  Images can't be removed if they have descendant images, are being used by a running container or are being used by a build. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Image name or ID | [required] |
**force** | Option<**bool**> | Remove the image even if it is being used by stopped containers or has other tags |  |[default to false]
**noprune** | Option<**bool**> | Do not delete untagged parent images |  |[default to false]
**platforms** | Option<[**Vec<String>**](String.md)> | Select platform-specific content to delete. Multiple values are accepted. Each platform is a OCI platform encoded as a JSON string.  |  |

### Return type

[**Vec<models::ImageDeleteResponseItem>**](ImageDeleteResponseItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_get

> std::path::PathBuf image_get(name, platform)
Export an image

Get a tarball containing all images and metadata for a repository.  If `name` is a specific name and tag (e.g. `ubuntu:latest`), then only that image (and its parents) are returned. If `name` is an image ID, similarly only that image (and its parents) are returned, but with the exclusion of the `repositories` file in the tarball, as there were no image names referenced.  ### Image tarball format  An image tarball contains [Content as defined in the OCI Image Layout Specification](https://github.com/opencontainers/image-spec/blob/v1.1.1/image-layout.md#content).  Additionally, includes the manifest.json file associated with a backwards compatible docker save format.  If the tarball defines a repository, the tarball should also include a `repositories` file at the root that contains a list of repository and tag names mapped to layer IDs.  ```json {   \"hello-world\": {     \"latest\": \"565a9d68a73f6706862bfe8409a7f659776d4d60a8d096eb4a3cbce6999cc2a1\"   } } ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Image name or ID | [required] |
**platform** | Option<**String**> | JSON encoded OCI platform describing a platform which will be used to select a platform-specific image to be saved if the image is multi-platform. If not provided, the full multi-platform image will be saved.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}`  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-tar

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_get_all

> std::path::PathBuf image_get_all(names, platform)
Export several images

Get a tarball containing all images and metadata for several image repositories.  For each value of the `names` parameter: if it is a specific name and tag (e.g. `ubuntu:latest`), then only that image (and its parents) are returned; if it is an image ID, similarly only that image (and its parents) are returned and there would be no names referenced in the 'repositories' file for this image ID.  For details on the format, see the [export image endpoint](#operation/ImageGet). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**names** | Option<[**Vec<String>**](String.md)> | Image names to filter by |  |
**platform** | Option<**String**> | JSON encoded OCI platform describing a platform which will be used to select a platform-specific image to be saved if the image is multi-platform. If not provided, the full multi-platform image will be saved.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}`  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-tar

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_history

> Vec<models::ImageHistoryResponseItem> image_history(name, platform)
Get the history of an image

Return parent layers of an image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Image name or ID | [required] |
**platform** | Option<**String**> | JSON-encoded OCI platform to select the platform-variant. If omitted, it defaults to any locally available platform, prioritizing the daemon's host platform.  If the daemon provides a multi-platform image store, this selects the platform-variant to show the history for. If the image is a single-platform image, or if the multi-platform image does not provide a variant matching the given platform, an error is returned.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}`  |  |

### Return type

[**Vec<models::ImageHistoryResponseItem>**](ImageHistoryResponseItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_inspect

> models::ImageInspect image_inspect(name, manifests, platform)
Inspect an image

Return low-level information about an image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Image name or id | [required] |
**manifests** | Option<**bool**> | Include Manifests in the image summary.  The `manifests` and `platform` options are mutually exclusive, and an error is produced if both are set. |  |[default to false]
**platform** | Option<**String**> | JSON-encoded OCI platform to select the platform-variant. If omitted, it defaults to any locally available platform, prioritizing the daemon's host platform.  If the daemon provides a multi-platform image store, this selects the platform-variant to show inspect. If the image is a single-platform image, or if the multi-platform image does not provide a variant matching the given platform, an error is returned.  The `platform` and `manifests` options are mutually exclusive, and an error is produced if both are set.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}` |  |

### Return type

[**models::ImageInspect**](ImageInspect.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_list

> Vec<models::ImageSummary> image_list(all, filters, shared_size, digests, manifests)
List Images

Returns a list of images on the server. Note that it uses a different, smaller representation of an image than inspecting a single image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Show all images. Only images from a final layer (no children) are shown by default. |  |[default to false]
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the images list.  Available filters:  - `before`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`) - `dangling=true` - `label=key` or `label=\"key=value\"` of an image label - `reference`=(`<image-name>[:<tag>]`) - `since`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`) - `until=<timestamp>`  |  |
**shared_size** | Option<**bool**> | Compute and show shared size as a `SharedSize` field on each image. |  |[default to false]
**digests** | Option<**bool**> | Show digest information as a `RepoDigests` field on each image. |  |[default to false]
**manifests** | Option<**bool**> | Include `Manifests` in the image summary. |  |[default to false]

### Return type

[**Vec<models::ImageSummary>**](ImageSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_load

> image_load(quiet, platform, images_tarball)
Import images

Load a set of images and tags into a repository.  For details on the format, see the [export image endpoint](#operation/ImageGet). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quiet** | Option<**bool**> | Suppress progress details during load. |  |[default to false]
**platform** | Option<**String**> | JSON encoded OCI platform describing a platform which will be used to select a platform-specific image to be load if the image is multi-platform. If not provided, the full multi-platform image will be loaded.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}`  |  |
**images_tarball** | Option<**std::path::PathBuf**> | Tar archive containing images |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-tar
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_prune

> models::ImagePruneResponse image_prune(filters)
Delete unused images

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filters to process on the prune list, encoded as JSON (a `map[string][]string`). Available filters:  - `dangling=<boolean>` When set to `true` (or `1`), prune only    unused *and* untagged images. When set to `false`    (or `0`), all unused images are pruned. - `until=<string>` Prune images created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time. - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune images with (or without, in case `label!=...` is used) the specified labels.  |  |

### Return type

[**models::ImagePruneResponse**](ImagePruneResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_push

> image_push(name, x_registry_auth, tag, platform)
Push an image

Push an image to a registry.  If you wish to push an image on to a private registry, that image must already have a tag which references the registry. For example, `registry.example.com/myimage:latest`.  The push is cancelled if the HTTP connection is closed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the image to push. For example, `registry.example.com/myimage`. The image must be present in the local image store with the same name.  The name should be provided without tag; if a tag is provided, it is ignored. For example, `registry.example.com/myimage:latest` is considered equivalent to `registry.example.com/myimage`.  Use the `tag` parameter to specify the tag to push.  | [required] |
**x_registry_auth** | **String** | A base64url-encoded auth configuration.  Refer to the [authentication section](#section/Authentication) for details.  | [required] |
**tag** | Option<**String**> | Tag of the image to push. For example, `latest`. If no tag is provided, all tags of the given image that are present in the local image store are pushed.  |  |
**platform** | Option<**String**> | JSON-encoded OCI platform to select the platform-variant to push. If not provided, all available variants will attempt to be pushed.  If the daemon provides a multi-platform image store, this selects the platform-variant to push to the registry. If the image is a single-platform image, or if the multi-platform image does not provide a variant matching the given platform, an error is returned.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}`  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_search

> Vec<models::ImageSearchResponseItem> image_search(term, limit, filters)
Search images

Search for an image on Docker Hub.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | Term to search | [required] |
**limit** | Option<**i32**> | Maximum number of results to return |  |
**filters** | Option<**String**> | A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters:  - `is-official=(true|false)` - `stars=<number>` Matches images that has at least 'number' stars.  |  |

### Return type

[**Vec<models::ImageSearchResponseItem>**](ImageSearchResponseItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_tag

> image_tag(name, repo, tag)
Tag an image

Create a tag that refers to a source image.  This creates an additional reference (tag) to the source image. The tag can include a different repository name and/or tag. If the repository or tag already exists, it will be overwritten. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Image name or ID to tag. | [required] |
**repo** | Option<**String**> | The repository to tag in. For example, `someuser/someimage`. |  |
**tag** | Option<**String**> | The name of the new tag. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

