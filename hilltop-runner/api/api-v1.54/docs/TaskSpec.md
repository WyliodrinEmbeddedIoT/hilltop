# TaskSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plugin_spec** | Option<[**models::TaskSpecPluginSpec**](TaskSpecPluginSpec.md)> |  | [optional]
**container_spec** | Option<[**models::TaskSpecContainerSpec**](TaskSpecContainerSpec.md)> |  | [optional]
**network_attachment_spec** | Option<[**models::TaskSpecNetworkAttachmentSpec**](TaskSpecNetworkAttachmentSpec.md)> |  | [optional]
**resources** | Option<[**models::TaskSpecResources**](TaskSpecResources.md)> |  | [optional]
**restart_policy** | Option<[**models::TaskSpecRestartPolicy**](TaskSpecRestartPolicy.md)> |  | [optional]
**placement** | Option<[**models::TaskSpecPlacement**](TaskSpecPlacement.md)> |  | [optional]
**force_update** | Option<**i32**> | A counter that triggers an update even if no relevant parameters have been changed.  | [optional]
**runtime** | Option<**String**> | Runtime is the type of runtime specified for the task executor.  | [optional]
**networks** | Option<[**Vec<models::NetworkAttachmentConfig>**](NetworkAttachmentConfig.md)> | Specifies which networks the service should attach to. | [optional]
**log_driver** | Option<[**models::TaskSpecLogDriver**](TaskSpecLogDriver.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


