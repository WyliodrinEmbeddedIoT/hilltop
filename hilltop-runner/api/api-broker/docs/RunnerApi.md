# \RunnerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_runner_get**](RunnerApi.md#api_runner_get) | **GET** /api/runner | 
[**api_runner_id_delete**](RunnerApi.md#api_runner_id_delete) | **DELETE** /api/runner/{id} | 
[**api_runner_id_get**](RunnerApi.md#api_runner_id_get) | **GET** /api/runner/{id} | 
[**api_runner_id_put**](RunnerApi.md#api_runner_id_put) | **PUT** /api/runner/{id} | 
[**api_runner_owned_get**](RunnerApi.md#api_runner_owned_get) | **GET** /api/runner/owned | 
[**api_runner_post**](RunnerApi.md#api_runner_post) | **POST** /api/runner | 
[**api_runner_runner_id_add_user_post**](RunnerApi.md#api_runner_runner_id_add_user_post) | **POST** /api/runner/{runnerId}/addUser | 
[**api_runner_runner_id_list_users_get**](RunnerApi.md#api_runner_runner_id_list_users_get) | **GET** /api/runner/{runnerId}/listUsers | 
[**api_runner_runner_id_remove_user_delete**](RunnerApi.md#api_runner_runner_id_remove_user_delete) | **DELETE** /api/runner/{runnerId}/removeUser | 



## api_runner_get

> models::RunnerGetResponsePagedResponse api_runner_get(runner_name_includes, runner_description_includes, runner_slug_includes, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_name_includes** | Option<**String**> |  |  |
**runner_description_includes** | Option<**String**> |  |  |
**runner_slug_includes** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::RunnerGetResponsePagedResponse**](RunnerGetResponsePagedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_id_delete

> serde_json::Value api_runner_id_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_id_get

> models::RunnerGetResponse api_runner_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RunnerGetResponse**](RunnerGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_id_put

> serde_json::Value api_runner_id_put(id, runner_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**runner_update_request** | Option<[**RunnerUpdateRequest**](RunnerUpdateRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_owned_get

> models::RunnerGetResponsePagedResponse api_runner_owned_get(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::RunnerGetResponsePagedResponse**](RunnerGetResponsePagedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_post

> serde_json::Value api_runner_post(runner_add_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_add_request** | Option<[**RunnerAddRequest**](RunnerAddRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_add_user_post

> serde_json::Value api_runner_runner_id_add_user_post(runner_id, runner_user_add_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**runner_user_add_request** | Option<[**RunnerUserAddRequest**](RunnerUserAddRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_list_users_get

> models::UserGetResponsePagedResponse api_runner_runner_id_list_users_get(runner_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::UserGetResponsePagedResponse**](UserGetResponsePagedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_remove_user_delete

> serde_json::Value api_runner_runner_id_remove_user_delete(runner_id, runner_user_delete_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**runner_user_delete_request** | Option<[**RunnerUserDeleteRequest**](RunnerUserDeleteRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

