# ContainerMemoryStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**usage** | Option<**i32**> | Current `res_counter` usage for memory.  This field is Linux-specific and omitted for Windows containers.  | [optional]
**max_usage** | Option<**i32**> | Maximum usage ever recorded.  This field is Linux-specific and only supported on cgroups v1. It is omitted when using cgroups v2 and for Windows containers.  | [optional]
**stats** | Option<**std::collections::HashMap<String, i32>**> | All the stats exported via memory.stat.  The fields in this object differ between cgroups v1 and v2. On cgroups v1, fields such as `cache`, `rss`, `mapped_file` are available. On cgroups v2, fields such as `file`, `anon`, `inactive_file` are available.  This field is Linux-specific and omitted for Windows containers.  | [optional]
**failcnt** | Option<**i32**> | Number of times memory usage hits limits.  This field is Linux-specific and only supported on cgroups v1. It is omitted when using cgroups v2 and for Windows containers.  | [optional]
**limit** | Option<**i32**> | This field is Linux-specific and omitted for Windows containers.  | [optional]
**commitbytes** | Option<**i32**> | Committed bytes.  This field is Windows-specific and omitted for Linux containers.  | [optional]
**commitpeakbytes** | Option<**i32**> | Peak committed bytes.  This field is Windows-specific and omitted for Linux containers.  | [optional]
**privateworkingset** | Option<**i32**> | Private working set.  This field is Windows-specific and omitted for Linux containers.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


