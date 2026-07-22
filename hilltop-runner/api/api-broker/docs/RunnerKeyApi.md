# \RunnerKeyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_runner_runner_id_key_get**](RunnerKeyApi.md#api_runner_runner_id_key_get) | **GET** /api/runner/{runnerId}/key | 
[**api_runner_runner_id_key_post**](RunnerKeyApi.md#api_runner_runner_id_key_post) | **POST** /api/runner/{runnerId}/key | 



## api_runner_runner_id_key_get

> Vec<models::RunnerKeyGetResponse> api_runner_runner_id_key_get(runner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::RunnerKeyGetResponse>**](RunnerKeyGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_key_post

> models::RunnerKeyCreateResponse api_runner_runner_id_key_post(runner_id, runner_key_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**runner_key_create_request** | Option<[**RunnerKeyCreateRequest**](RunnerKeyCreateRequest.md)> |  |  |

### Return type

[**models::RunnerKeyCreateResponse**](RunnerKeyCreateResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

