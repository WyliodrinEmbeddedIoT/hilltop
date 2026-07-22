# ContainerdInfoNamespaces

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**containers** | Option<**String**> | The default containerd namespace used for containers managed by the daemon.  The default namespace for containers is \"moby\", but will be suffixed with the `<uid>.<gid>` of the remapped `root` if user-namespaces are enabled and the containerd image-store is used.  | [optional][default to moby]
**plugins** | Option<**String**> | The default containerd namespace used for plugins managed by the daemon.  The default namespace for plugins is \"plugins.moby\", but will be suffixed with the `<uid>.<gid>` of the remapped `root` if user-namespaces are enabled and the containerd image-store is used.  | [optional][default to plugins.moby]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


