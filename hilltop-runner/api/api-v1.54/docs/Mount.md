# Mount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | Option<**String**> | Container path. | [optional]
**source** | Option<**String**> | Mount source (e.g. a volume name, a host path). The source cannot be specified when using `Type=tmpfs`. For `Type=bind`, the source path must either exist, or the `CreateMountpoint` must be set to `true` to create the source path on the host if missing.  For `Type=npipe`, the pipe must exist prior to creating the container. | [optional]
**r#type** | Option<[**models::MountType**](MountType.md)> | The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. The `Source` must exist prior to creating the container. - `cluster` a Swarm cluster volume - `image` Mounts an image. - `npipe` Mounts a named pipe from the host into the container. The `Source` must exist prior to creating the container. - `tmpfs` Create a tmpfs with the given options. The mount `Source` cannot be specified for tmpfs. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed.  | [optional]
**read_only** | Option<**bool**> | Whether the mount should be read-only. | [optional]
**consistency** | Option<**String**> | The consistency requirement for the mount: `default`, `consistent`, `cached`, or `delegated`. | [optional]
**bind_options** | Option<[**models::MountBindOptions**](MountBindOptions.md)> |  | [optional]
**volume_options** | Option<[**models::MountVolumeOptions**](MountVolumeOptions.md)> |  | [optional]
**image_options** | Option<[**models::MountImageOptions**](MountImageOptions.md)> |  | [optional]
**tmpfs_options** | Option<[**models::MountTmpfsOptions**](MountTmpfsOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


