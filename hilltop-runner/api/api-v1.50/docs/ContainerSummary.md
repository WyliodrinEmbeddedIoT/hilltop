# ContainerSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of this container as a 128-bit (64-character) hexadecimal string (32 bytes). | [optional]
**names** | Option<**Vec<String>**> | The names associated with this container. Most containers have a single name, but when using legacy \"links\", the container can have multiple names.  For historic reasons, names are prefixed with a forward-slash (`/`). | [optional]
**image** | Option<**String**> | The name or ID of the image used to create the container.  This field shows the image reference as was specified when creating the container, which can be in its canonical form (e.g., `docker.io/library/ubuntu:latest` or `docker.io/library/ubuntu@sha256:72297848456d5d37d1262630108ab308d3e9ec7ed1c3286a32fe09856619a782`), short form (e.g., `ubuntu:latest`)), or the ID(-prefix) of the image (e.g., `72297848456d`).  The content of this field can be updated at runtime if the image used to create the container is untagged, in which case the field is updated to contain the the image ID (digest) it was resolved to in its canonical, non-truncated form (e.g., `sha256:72297848456d5d37d1262630108ab308d3e9ec7ed1c3286a32fe09856619a782`). | [optional]
**image_id** | Option<**String**> | The ID (digest) of the image that this container was created from. | [optional]
**image_manifest_descriptor** | Option<[**models::OciDescriptor**](OCIDescriptor.md)> |  | [optional]
**command** | Option<**String**> | Command to run when starting the container | [optional]
**created** | Option<**i64**> | Date and time at which the container was created as a Unix timestamp (number of seconds since EPOCH). | [optional]
**ports** | Option<[**Vec<models::Port>**](Port.md)> | Port-mappings for the container. | [optional]
**size_rw** | Option<**i64**> | The size of files that have been created or changed by this container.  This field is omitted by default, and only set when size is requested in the API request. | [optional]
**size_root_fs** | Option<**i64**> | The total size of all files in the read-only layers from the image that the container uses. These layers can be shared between containers.  This field is omitted by default, and only set when size is requested in the API request. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**state** | Option<**State**> | The state of this container.  (enum: created, running, paused, restarting, exited, removing, dead) | [optional]
**status** | Option<**String**> | Additional human-readable status of this container (e.g. `Exit 0`) | [optional]
**host_config** | Option<[**models::ContainerSummaryHostConfig**](ContainerSummaryHostConfig.md)> |  | [optional]
**network_settings** | Option<[**models::ContainerSummaryNetworkSettings**](ContainerSummaryNetworkSettings.md)> |  | [optional]
**mounts** | Option<[**Vec<models::MountPoint>**](MountPoint.md)> | List of mounts used by the container. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


