# PluginConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**docker_version** | Option<**String**> | Docker Version used to create the plugin | [optional]
**description** | **String** |  | 
**documentation** | **String** |  | 
**interface** | [**models::PluginConfigInterface**](PluginConfigInterface.md) |  | 
**entrypoint** | **Vec<String>** |  | 
**work_dir** | **String** |  | 
**user** | Option<[**models::PluginConfigUser**](PluginConfigUser.md)> |  | [optional]
**network** | [**models::PluginConfigNetwork**](PluginConfigNetwork.md) |  | 
**linux** | [**models::PluginConfigLinux**](PluginConfigLinux.md) |  | 
**propagated_mount** | **String** |  | 
**ipc_host** | **bool** |  | 
**pid_host** | **bool** |  | 
**mounts** | [**Vec<models::PluginMount>**](PluginMount.md) |  | 
**env** | [**Vec<models::PluginEnv>**](PluginEnv.md) |  | 
**args** | [**models::PluginConfigArgs**](PluginConfigArgs.md) |  | 
**rootfs** | Option<[**models::PluginConfigRootfs**](PluginConfigRootfs.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


