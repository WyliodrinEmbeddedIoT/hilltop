# VolumesDiskUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_count** | Option<**i64**> | Count of active volumes.  | [optional]
**total_count** | Option<**i64**> | Count of all volumes.  | [optional]
**reclaimable** | Option<**i64**> | Disk space that can be reclaimed by removing inactive volumes.  | [optional]
**total_size** | Option<**i64**> | Disk space in use by volumes.  | [optional]
**items** | Option<**Vec<serde_json::Value>**> | List of volumes.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


