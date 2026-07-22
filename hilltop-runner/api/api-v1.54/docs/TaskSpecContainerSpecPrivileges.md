# TaskSpecContainerSpecPrivileges

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credential_spec** | Option<[**models::TaskSpecContainerSpecPrivilegesCredentialSpec**](TaskSpecContainerSpecPrivilegesCredentialSpec.md)> |  | [optional]
**se_linux_context** | Option<[**models::TaskSpecContainerSpecPrivilegesSeLinuxContext**](TaskSpecContainerSpecPrivilegesSELinuxContext.md)> |  | [optional]
**seccomp** | Option<[**models::TaskSpecContainerSpecPrivilegesSeccomp**](TaskSpecContainerSpecPrivilegesSeccomp.md)> |  | [optional]
**app_armor** | Option<[**models::TaskSpecContainerSpecPrivilegesAppArmor**](TaskSpecContainerSpecPrivilegesAppArmor.md)> |  | [optional]
**no_new_privileges** | Option<**bool**> | Configuration of the no_new_privs bit in the container | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


