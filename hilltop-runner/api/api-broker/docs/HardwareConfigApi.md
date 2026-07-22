# \HardwareConfigApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_runner_runner_id_hardware_get**](HardwareConfigApi.md#api_runner_runner_id_hardware_get) | **GET** /api/runner/{runnerId}/hardware | 
[**api_runner_runner_id_hardware_hw_slug_delete**](HardwareConfigApi.md#api_runner_runner_id_hardware_hw_slug_delete) | **DELETE** /api/runner/{runnerId}/hardware/{hwSlug} | 
[**api_runner_runner_id_hardware_hw_slug_get**](HardwareConfigApi.md#api_runner_runner_id_hardware_hw_slug_get) | **GET** /api/runner/{runnerId}/hardware/{hwSlug} | 
[**api_runner_runner_id_hardware_hw_slug_put**](HardwareConfigApi.md#api_runner_runner_id_hardware_hw_slug_put) | **PUT** /api/runner/{runnerId}/hardware/{hwSlug} | 
[**api_runner_runner_id_hardware_post**](HardwareConfigApi.md#api_runner_runner_id_hardware_post) | **POST** /api/runner/{runnerId}/hardware | 



## api_runner_runner_id_hardware_get

> models::HardwareConfigGetResponsePagedResponse api_runner_runner_id_hardware_get(runner_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::HardwareConfigGetResponsePagedResponse**](HardwareConfigGetResponsePagedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_hardware_hw_slug_delete

> serde_json::Value api_runner_runner_id_hardware_hw_slug_delete(runner_id, hw_slug)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**hw_slug** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_hardware_hw_slug_get

> models::HardwareConfigGetResponse api_runner_runner_id_hardware_hw_slug_get(runner_id, hw_slug, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**hw_slug** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::HardwareConfigGetResponse**](HardwareConfigGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_hardware_hw_slug_put

> models::HardwareConfigGetResponse api_runner_runner_id_hardware_hw_slug_put(runner_id, hw_slug, hardware_config_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**hw_slug** | **String** |  | [required] |
**hardware_config_update_request** | Option<[**HardwareConfigUpdateRequest**](HardwareConfigUpdateRequest.md)> |  |  |

### Return type

[**models::HardwareConfigGetResponse**](HardwareConfigGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_runner_runner_id_hardware_post

> serde_json::Value api_runner_runner_id_hardware_post(runner_id, hardware_config_add_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **uuid::Uuid** |  | [required] |
**hardware_config_add_request** | Option<[**HardwareConfigAddRequest**](HardwareConfigAddRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

