# BuildCacheDiskUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_count** | Option<**i64**> | Count of active build cache records.  | [optional]
**total_count** | Option<**i64**> | Count of all build cache records.  | [optional]
**reclaimable** | Option<**i64**> | Disk space that can be reclaimed by removing inactive build cache records.  | [optional]
**total_size** | Option<**i64**> | Disk space in use by build cache records.  | [optional]
**items** | Option<**Vec<serde_json::Value>**> | List of build cache records.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


