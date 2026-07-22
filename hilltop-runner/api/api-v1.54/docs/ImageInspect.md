# ImageInspect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.  | [optional]
**descriptor** | Option<[**models::OciDescriptor**](OCIDescriptor.md)> |  | [optional]
**identity** | Option<[**models::Identity**](Identity.md)> |  | [optional]
**manifests** | Option<[**Vec<models::ImageManifestSummary>**](ImageManifestSummary.md)> | Manifests is a list of image manifests available in this image. It provides a more detailed view of the platform-specific image manifests or other image-attached data like build attestations.  Only available if the daemon provides a multi-platform image store and the `manifests` option is set in the inspect request.  WARNING: This is experimental and may change at any time without any backward compatibility.  | [optional]
**repo_tags** | Option<**Vec<String>**> | List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.  | [optional]
**repo_digests** | Option<**Vec<String>**> | List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.  | [optional]
**comment** | Option<**String**> | Optional message that was set when committing or importing the image.  | [optional]
**created** | Option<**String**> | Date and time at which the image was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if present in the image, and omitted otherwise.  | [optional]
**author** | Option<**String**> | Name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile.  | [optional]
**config** | Option<[**models::ImageConfig**](ImageConfig.md)> |  | [optional]
**architecture** | Option<**String**> | Hardware CPU architecture that the image runs on.  | [optional]
**variant** | Option<**String**> | CPU architecture variant (presently ARM-only).  | [optional]
**os** | Option<**String**> | Operating System the image is built to run on.  | [optional]
**os_version** | Option<**String**> | Operating System version the image is built to run on (especially for Windows).  | [optional]
**size** | Option<**i64**> | Total size of the image including all layers it is composed of.  | [optional]
**graph_driver** | Option<[**models::DriverData**](DriverData.md)> |  | [optional]
**root_fs** | Option<[**models::ImageInspectRootFs**](ImageInspectRootFS.md)> |  | [optional]
**metadata** | Option<[**models::ImageInspectMetadata**](ImageInspectMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


