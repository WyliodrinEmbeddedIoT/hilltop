# Identity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signature** | Option<[**Vec<models::SignatureIdentity>**](SignatureIdentity.md)> | Signature contains the properties of verified signatures for the image. | [optional]
**pull** | Option<[**Vec<models::PullIdentity>**](PullIdentity.md)> | Pull contains remote location information if image was created via pull. If image was pulled via mirror, this contains the original repository location. After successful push this images also contains the pushed repository location. | [optional]
**build** | Option<[**Vec<models::BuildIdentity>**](BuildIdentity.md)> | Build contains build reference information if image was created via build. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


