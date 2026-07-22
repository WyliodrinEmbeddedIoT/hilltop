# ServiceUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the service. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**task_template** | Option<[**models::TaskSpec**](TaskSpec.md)> |  | [optional]
**mode** | Option<[**models::ServiceSpecMode**](ServiceSpecMode.md)> |  | [optional]
**update_config** | Option<[**models::ServiceSpecUpdateConfig**](ServiceSpecUpdateConfig.md)> |  | [optional]
**rollback_config** | Option<[**models::ServiceSpecRollbackConfig**](ServiceSpecRollbackConfig.md)> |  | [optional]
**networks** | Option<[**Vec<models::NetworkAttachmentConfig>**](NetworkAttachmentConfig.md)> | Specifies which networks the service should attach to.  Deprecated: This field is deprecated since v1.44. The Networks field in TaskSpec should be used instead.  | [optional]
**endpoint_spec** | Option<[**models::EndpointSpec**](EndpointSpec.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


