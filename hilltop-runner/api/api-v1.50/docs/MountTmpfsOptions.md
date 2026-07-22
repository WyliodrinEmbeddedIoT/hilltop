# MountTmpfsOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size_bytes** | Option<**i64**> | The size for the tmpfs mount in bytes. | [optional]
**mode** | Option<**i32**> | The permission mode for the tmpfs mount in an integer. The value must not be in octal format (e.g. 755) but rather the decimal representation of the octal value (e.g. 493).  | [optional]
**options** | Option<[**Vec<Vec<String>>**](Vec.md)> | The options to be passed to the tmpfs mount. An array of arrays. Flag options should be provided as 1-length arrays. Other types should be provided as as 2-length arrays, where the first item is the key and the second the value.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


