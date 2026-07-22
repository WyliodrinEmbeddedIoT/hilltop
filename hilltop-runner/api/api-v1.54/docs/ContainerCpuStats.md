# ContainerCpuStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_usage** | Option<[**models::ContainerCpuUsage**](ContainerCPUUsage.md)> |  | [optional]
**system_cpu_usage** | Option<**i32**> | System Usage.  This field is Linux-specific and omitted for Windows containers.  | [optional]
**online_cpus** | Option<**i32**> | Number of online CPUs.  This field is Linux-specific and omitted for Windows containers.  | [optional]
**throttling_data** | Option<[**models::ContainerThrottlingData**](ContainerThrottlingData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


