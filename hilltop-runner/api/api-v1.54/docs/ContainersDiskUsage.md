# ContainersDiskUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_count** | Option<**i64**> | Count of active containers.  | [optional]
**total_count** | Option<**i64**> | Count of all containers.  | [optional]
**reclaimable** | Option<**i64**> | Disk space that can be reclaimed by removing inactive containers.  | [optional]
**total_size** | Option<**i64**> | Disk space in use by containers.  | [optional]
**items** | Option<**Vec<serde_json::Value>**> | List of container summaries.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


