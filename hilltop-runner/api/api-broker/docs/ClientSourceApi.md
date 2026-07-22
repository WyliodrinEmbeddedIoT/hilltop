# \ClientSourceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_api_source_id_download_get**](ClientSourceApi.md#client_api_source_id_download_get) | **GET** /client_api/source/{id}/download | 
[**client_api_source_post**](ClientSourceApi.md#client_api_source_post) | **POST** /client_api/source | 



## client_api_source_id_download_get

> std::path::PathBuf client_api_source_id_download_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_api_source_post

> models::SourceGetResponse client_api_source_post(runner_slug, source_file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_slug** | **String** |  | [required] |
**source_file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::SourceGetResponse**](SourceGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

