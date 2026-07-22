# JobGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | 
**requested_by_id** | **uuid::Uuid** |  | 
**for_runner_id** | **uuid::Uuid** |  | 
**code_source_id** | **uuid::Uuid** |  | 
**using_hardware_slug** | Option<**String**> |  | 
**using_image_slug** | Option<**String**> |  | 
**status** | [**models::JobStatusEnum**](JobStatusEnum.md) |  | 
**status_message** | Option<**String**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**artifacts** | Option<[**Vec<models::ArtifactSummary>**](ArtifactSummary.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


