# \RunnerArtifactApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**runner_api_artifact_job_job_id_post**](RunnerArtifactApi.md#runner_api_artifact_job_job_id_post) | **POST** /runner_api/artifact/job/{jobId} | 



## runner_api_artifact_job_job_id_post

> models::ArtifactGetResponse runner_api_artifact_job_job_id_post(job_id, display_identifier, artifact_file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |
**display_identifier** | **String** |  | [required] |
**artifact_file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::ArtifactGetResponse**](ArtifactGetResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

