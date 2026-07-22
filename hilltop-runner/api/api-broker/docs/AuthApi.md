# \AuthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_auth_check_runner_session_get**](AuthApi.md#api_auth_check_runner_session_get) | **GET** /api/auth/checkRunnerSession | 
[**api_auth_check_user_session_get**](AuthApi.md#api_auth_check_user_session_get) | **GET** /api/auth/checkUserSession | 
[**api_auth_login_post**](AuthApi.md#api_auth_login_post) | **POST** /api/auth/login | 
[**api_auth_me_get**](AuthApi.md#api_auth_me_get) | **GET** /api/auth/me | 
[**api_auth_new_runner_session_post**](AuthApi.md#api_auth_new_runner_session_post) | **POST** /api/auth/newRunnerSession | 
[**api_auth_new_user_session_post**](AuthApi.md#api_auth_new_user_session_post) | **POST** /api/auth/newUserSession | 
[**api_auth_register_post**](AuthApi.md#api_auth_register_post) | **POST** /api/auth/register | 



## api_auth_check_runner_session_get

> models::RunnerGetResponse api_auth_check_runner_session_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RunnerGetResponse**](RunnerGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_check_user_session_get

> models::UserGetResponse api_auth_check_user_session_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserGetResponse**](UserGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_login_post

> models::LoginResponse api_auth_login_post(login_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_request** | Option<[**LoginRequest**](LoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_me_get

> models::UserGetResponse api_auth_me_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserGetResponse**](UserGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_new_runner_session_post

> models::RunnerSessionResponse api_auth_new_runner_session_post(runner_session_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_session_request** | Option<[**RunnerSessionRequest**](RunnerSessionRequest.md)> |  |  |

### Return type

[**models::RunnerSessionResponse**](RunnerSessionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_new_user_session_post

> models::UserSessionResponse api_auth_new_user_session_post(user_session_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_session_request** | Option<[**UserSessionRequest**](UserSessionRequest.md)> |  |  |

### Return type

[**models::UserSessionResponse**](UserSessionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_register_post

> serde_json::Value api_auth_register_post(register_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_request** | Option<[**RegisterRequest**](RegisterRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

