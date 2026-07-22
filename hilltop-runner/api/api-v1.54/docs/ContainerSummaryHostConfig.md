# ContainerSummaryHostConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_mode** | Option<**String**> | Networking mode (`host`, `none`, `container:<id>`) or name of the primary network the container is using.  This field is primarily for backward compatibility. The container can be connected to multiple networks for which information can be found in the `NetworkSettings.Networks` field, which enumerates settings per network. | [optional]
**annotations** | Option<**std::collections::HashMap<String, String>**> | Arbitrary key-value metadata attached to the container. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


