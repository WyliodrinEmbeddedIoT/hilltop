# \ClientKeyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_client_key_get**](ClientKeyApi.md#api_client_key_get) | **GET** /api/client/key | 
[**api_client_key_post**](ClientKeyApi.md#api_client_key_post) | **POST** /api/client/key | 



## api_client_key_get

> Vec<models::GetClientKeyResponse> api_client_key_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetClientKeyResponse>**](GetClientKeyResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_client_key_post

> models::CreateClientKeyResponse api_client_key_post(create_client_key_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_client_key_request** | Option<[**CreateClientKeyRequest**](CreateClientKeyRequest.md)> |  |  |

### Return type

[**models::CreateClientKeyResponse**](CreateClientKeyResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

