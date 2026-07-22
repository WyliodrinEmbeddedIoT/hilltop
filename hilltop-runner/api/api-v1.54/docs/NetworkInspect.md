# NetworkInspect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the network.  | [optional]
**id** | Option<**String**> | ID that uniquely identifies a network on a single machine.  | [optional]
**created** | Option<**String**> | Date and time at which the network was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**scope** | Option<**String**> | The level at which the network exists (e.g. `swarm` for cluster-wide or `local` for machine level)  | [optional]
**driver** | Option<**String**> | The name of the driver used to create the network (e.g. `bridge`, `overlay`).  | [optional]
**enable_ipv4** | Option<**bool**> | Whether the network was created with IPv4 enabled.  | [optional]
**enable_ipv6** | Option<**bool**> | Whether the network was created with IPv6 enabled.  | [optional]
**ipam** | Option<[**models::Ipam**](IPAM.md)> |  | [optional]
**internal** | Option<**bool**> | Whether the network is created to only allow internal networking connectivity.  | [optional][default to false]
**attachable** | Option<**bool**> | Whether a global / swarm scope network is manually attachable by regular containers from workers in swarm mode.  | [optional][default to false]
**ingress** | Option<**bool**> | Whether the network is providing the routing-mesh for the swarm cluster.  | [optional][default to false]
**config_from** | Option<[**models::ConfigReference**](ConfigReference.md)> |  | [optional]
**config_only** | Option<**bool**> | Whether the network is a config-only network. Config-only networks are placeholder networks for network configurations to be used by other networks. Config-only networks cannot be used directly to run containers or services.  | [optional][default to false]
**options** | Option<**std::collections::HashMap<String, String>**> | Network-specific options uses when creating the network.  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Metadata specific to the network being created.  | [optional]
**peers** | Option<[**Vec<models::PeerInfo>**](PeerInfo.md)> | List of peer nodes for an overlay network. This field is only present for overlay networks, and omitted for other network types.  | [optional]
**containers** | Option<[**std::collections::HashMap<String, models::EndpointResource>**](EndpointResource.md)> | Contains endpoints attached to the network.  | [optional]
**services** | Option<**std::collections::HashMap<String, serde_json::Value>**> | List of services using the network. This field is only present for swarm scope networks, and omitted for local scope networks.  | [optional]
**status** | Option<[**models::NetworkStatus**](NetworkStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


