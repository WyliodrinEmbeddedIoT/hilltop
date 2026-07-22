# ContainerStatsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the container for which the stats were collected.  | [optional]
**name** | Option<**String**> | Name of the container for which the stats were collected.  | [optional]
**os_type** | Option<**String**> | OSType is the OS of the container (\"linux\" or \"windows\") to allow platform-specific handling of stats.  | [optional]
**read** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date and time at which this sample was collected. The value is formatted as [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) with nano-seconds.  | [optional]
**cpu_stats** | Option<[**models::ContainerCpuStats**](ContainerCPUStats.md)> |  | [optional]
**memory_stats** | Option<[**models::ContainerMemoryStats**](ContainerMemoryStats.md)> |  | [optional]
**networks** | Option<**serde_json::Value**> | Network statistics for the container per interface.  This field is omitted if the container has no networking enabled.  | [optional]
**pids_stats** | Option<[**models::ContainerPidsStats**](ContainerPidsStats.md)> |  | [optional]
**blkio_stats** | Option<[**models::ContainerBlkioStats**](ContainerBlkioStats.md)> |  | [optional]
**num_procs** | Option<**i32**> | The number of processors on the system.  This field is Windows-specific and always zero for Linux containers.  | [optional]
**storage_stats** | Option<[**models::ContainerStorageStats**](ContainerStorageStats.md)> |  | [optional]
**preread** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date and time at which this first sample was collected. This field is not propagated if the \"one-shot\" option is set. If the \"one-shot\" option is set, this field may be omitted, empty, or set to a default date (`0001-01-01T00:00:00Z`).  The value is formatted as [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) with nano-seconds.  | [optional]
**precpu_stats** | Option<[**models::ContainerCpuStats**](ContainerCPUStats.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


