# MountVolumeOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**no_copy** | Option<**bool**> | Populate volume with data from the target. | [optional][default to false]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**driver_config** | Option<[**models::MountVolumeOptionsDriverConfig**](MountVolumeOptionsDriverConfig.md)> |  | [optional]
**subpath** | Option<**String**> | Source path inside the volume. Must be relative without any back traversals. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


