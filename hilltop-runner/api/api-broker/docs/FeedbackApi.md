# \FeedbackApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_feedback_paged_get**](FeedbackApi.md#api_feedback_paged_get) | **GET** /api/feedback/paged | 
[**api_feedback_post**](FeedbackApi.md#api_feedback_post) | **POST** /api/feedback | 



## api_feedback_paged_get

> models::FeedbackGetRequestPagedResponse api_feedback_paged_get(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::FeedbackGetRequestPagedResponse**](FeedbackGetRequestPagedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_feedback_post

> serde_json::Value api_feedback_post(feedback_submit_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_submit_request** | Option<[**FeedbackSubmitRequest**](FeedbackSubmitRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

