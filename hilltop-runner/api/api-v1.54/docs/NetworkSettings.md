# NetworkSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sandbox_id** | Option<**String**> | SandboxID uniquely represents a container's network stack. | [optional]
**sandbox_key** | Option<**String**> | SandboxKey is the full path of the netns handle | [optional]
**ports** | Option<[**std::collections::HashMap<String, Vec<models::PortBinding>>**](Vec.md)> | PortMap describes the mapping of container ports to host ports, using the container's port-number and protocol as key in the format `<port>/<protocol>`, for example, `80/udp`.  If a container's port is mapped for multiple protocols, separate entries are added to the mapping table.  | [optional]
**networks** | Option<[**std::collections::HashMap<String, models::EndpointSettings>**](EndpointSettings.md)> | Information about all networks that the container is connected to.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


