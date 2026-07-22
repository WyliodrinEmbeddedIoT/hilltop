# ImageManifestSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID is the content-addressable ID of an image and is the same as the digest of the image manifest.  | 
**descriptor** | [**models::OciDescriptor**](OCIDescriptor.md) |  | 
**available** | **bool** | Indicates whether all the child content (image config, layers) is fully available locally. | 
**size** | [**models::ImageManifestSummarySize**](ImageManifestSummarySize.md) |  | 
**kind** | **Kind** | The kind of the manifest.  kind         | description -------------|----------------------------------------------------------- image        | Image manifest that can be used to start a container. attestation  | Attestation manifest produced by the Buildkit builder for a specific image manifest.  (enum: image, attestation, unknown) | 
**image_data** | Option<[**models::ImageManifestSummaryImageData**](ImageManifestSummaryImageData.md)> |  | [optional]
**attestation_data** | Option<[**models::ImageManifestSummaryAttestationData**](ImageManifestSummaryAttestationData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


