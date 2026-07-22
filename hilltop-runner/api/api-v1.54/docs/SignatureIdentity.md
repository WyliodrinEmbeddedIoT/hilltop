# SignatureIdentity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name is a textual description summarizing the type of signature. | [optional]
**timestamps** | Option<[**Vec<models::SignatureTimestamp>**](SignatureTimestamp.md)> | Timestamps contains a list of verified signed timestamps for the signature. | [optional]
**known_signer** | Option<[**models::KnownSignerIdentity**](KnownSignerIdentity.md)> |  | [optional]
**docker_reference** | Option<**String**> | DockerReference is the Docker image reference associated with the signature. This is an optional field only present in older hashedrecord signatures. | [optional]
**signer** | Option<[**models::SignerIdentity**](SignerIdentity.md)> |  | [optional]
**signature_type** | Option<[**models::SignatureType**](SignatureType.md)> |  | [optional]
**error** | Option<**String**> | Error contains error information if signature verification failed. Other fields will be empty in this case. | [optional]
**warnings** | Option<**Vec<String>**> | Warnings contains any warnings that occurred during signature verification. For example, if there was no internet connectivity and cached trust roots were used. Warning does not indicate a failed verification but may point to configuration issues. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


