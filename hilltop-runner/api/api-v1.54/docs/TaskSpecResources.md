# TaskSpecResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limits** | Option<[**models::Limit**](Limit.md)> |  | [optional]
**reservations** | Option<[**models::ResourceObject**](ResourceObject.md)> |  | [optional]
**swap_bytes** | Option<**i64**> | Amount of swap in bytes - can only be used together with a memory limit. If not specified, the default behaviour is to grant a swap space twice as big as the memory limit. Set to -1 to enable unlimited swap.  | [optional]
**memory_swappiness** | Option<**i64**> | Tune the service's containers' memory swappiness (0 to 100). If not specified, defaults to the containers' OS' default, generally 60, or whatever value was predefined in the image. Set to -1 to unset a previously set value.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


