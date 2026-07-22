# ContainerBlkioStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**io_service_bytes_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> |  | [optional]
**io_serviced_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]
**io_queue_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]
**io_service_time_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]
**io_wait_time_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]
**io_merged_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]
**io_time_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]
**sectors_recursive** | Option<[**Vec<models::ContainerBlkioStatEntry>**](ContainerBlkioStatEntry.md)> | This field is only available when using Linux containers with cgroups v1. It is omitted or `null` when using cgroups v2.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


