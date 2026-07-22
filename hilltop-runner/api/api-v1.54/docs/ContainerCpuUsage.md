# ContainerCpuUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_usage** | Option<**i32**> | Total CPU time consumed in nanoseconds (Linux) or 100's of nanoseconds (Windows).  | [optional]
**percpu_usage** | Option<**Vec<i32>**> | Total CPU time (in nanoseconds) consumed per core (Linux).  This field is Linux-specific when using cgroups v1. It is omitted when using cgroups v2 and Windows containers.  | [optional]
**usage_in_kernelmode** | Option<**i32**> | Time (in nanoseconds) spent by tasks of the cgroup in kernel mode (Linux), or time spent (in 100's of nanoseconds) by all container processes in kernel mode (Windows).  Not populated for Windows containers using Hyper-V isolation.  | [optional]
**usage_in_usermode** | Option<**i32**> | Time (in nanoseconds) spent by tasks of the cgroup in user mode (Linux), or time spent (in 100's of nanoseconds) by all container processes in kernel mode (Windows).  Not populated for Windows containers using Hyper-V isolation.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


