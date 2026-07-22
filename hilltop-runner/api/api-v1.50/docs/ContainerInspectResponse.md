# ContainerInspectResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of this container as a 128-bit (64-character) hexadecimal string (32 bytes). | [optional]
**created** | Option<**String**> | Date and time at which the container was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds. | [optional]
**path** | Option<**String**> | The path to the command being run | [optional]
**args** | Option<**Vec<String>**> | The arguments to the command being run | [optional]
**state** | Option<[**models::ContainerState**](ContainerState.md)> |  | [optional]
**image** | Option<**String**> | The ID (digest) of the image that this container was created from. | [optional]
**resolv_conf_path** | Option<**String**> | Location of the `/etc/resolv.conf` generated for the container on the host.  This file is managed through the docker daemon, and should not be accessed or modified by other tools. | [optional]
**hostname_path** | Option<**String**> | Location of the `/etc/hostname` generated for the container on the host.  This file is managed through the docker daemon, and should not be accessed or modified by other tools. | [optional]
**hosts_path** | Option<**String**> | Location of the `/etc/hosts` generated for the container on the host.  This file is managed through the docker daemon, and should not be accessed or modified by other tools. | [optional]
**log_path** | Option<**String**> | Location of the file used to buffer the container's logs. Depending on the logging-driver used for the container, this field may be omitted.  This file is managed through the docker daemon, and should not be accessed or modified by other tools. | [optional]
**name** | Option<**String**> | The name associated with this container.  For historic reasons, the name may be prefixed with a forward-slash (`/`). | [optional]
**restart_count** | Option<**i32**> | Number of times the container was restarted since it was created, or since daemon was started. | [optional]
**driver** | Option<**String**> | The storage-driver used for the container's filesystem (graph-driver or snapshotter). | [optional]
**platform** | Option<**String**> | The platform (operating system) for which the container was created.  This field was introduced for the experimental \"LCOW\" (Linux Containers On Windows) features, which has been removed. In most cases, this field is equal to the host's operating system (`linux` or `windows`). | [optional]
**image_manifest_descriptor** | Option<[**models::OciDescriptor**](OCIDescriptor.md)> |  | [optional]
**mount_label** | Option<**String**> | SELinux mount label set for the container. | [optional]
**process_label** | Option<**String**> | SELinux process label set for the container. | [optional]
**app_armor_profile** | Option<**String**> | The AppArmor profile set for the container. | [optional]
**exec_ids** | Option<**Vec<String>**> | IDs of exec instances that are running in the container. | [optional]
**host_config** | Option<[**models::HostConfig**](HostConfig.md)> |  | [optional]
**graph_driver** | Option<[**models::DriverData**](DriverData.md)> |  | [optional]
**size_rw** | Option<**i64**> | The size of files that have been created or changed by this container.  This field is omitted by default, and only set when size is requested in the API request. | [optional]
**size_root_fs** | Option<**i64**> | The total size of all files in the read-only layers from the image that the container uses. These layers can be shared between containers.  This field is omitted by default, and only set when size is requested in the API request. | [optional]
**mounts** | Option<[**Vec<models::MountPoint>**](MountPoint.md)> | List of mounts used by the container. | [optional]
**config** | Option<[**models::ContainerConfig**](ContainerConfig.md)> |  | [optional]
**network_settings** | Option<[**models::NetworkSettings**](NetworkSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


