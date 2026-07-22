# NetworkCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The network's name. | 
**driver** | Option<**String**> | Name of the network driver plugin to use. | [optional][default to bridge]
**scope** | Option<**String**> | The level at which the network exists (e.g. `swarm` for cluster-wide or `local` for machine level).  | [optional]
**internal** | Option<**bool**> | Restrict external access to the network. | [optional]
**attachable** | Option<**bool**> | Globally scoped network is manually attachable by regular containers from workers in swarm mode.  | [optional]
**ingress** | Option<**bool**> | Ingress network is the network which provides the routing-mesh in swarm mode.  | [optional]
**config_only** | Option<**bool**> | Creates a config-only network. Config-only networks are placeholder networks for network configurations to be used by other networks. Config-only networks cannot be used directly to run containers or services.  | [optional][default to false]
**config_from** | Option<[**models::ConfigReference**](ConfigReference.md)> |  | [optional]
**ipam** | Option<[**models::Ipam**](IPAM.md)> |  | [optional]
**enable_ipv4** | Option<**bool**> | Enable IPv4 on the network. | [optional]
**enable_ipv6** | Option<**bool**> | Enable IPv6 on the network. | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | Network specific options to be used by the drivers. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


