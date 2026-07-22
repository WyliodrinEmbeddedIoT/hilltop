# OciDescriptor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**media_type** | Option<**String**> | The media type of the object this schema refers to.  | [optional]
**digest** | Option<**String**> | The digest of the targeted content.  | [optional]
**size** | Option<**i64**> | The size in bytes of the blob.  | [optional]
**urls** | Option<**Vec<String>**> | List of URLs from which this object MAY be downloaded. | [optional]
**annotations** | Option<**std::collections::HashMap<String, String>**> | Arbitrary metadata relating to the targeted content. | [optional]
**data** | Option<**String**> | Data is an embedding of the targeted content. This is encoded as a base64 string when marshalled to JSON (automatically, by encoding/json). If present, Data can be used directly to avoid fetching the targeted content. | [optional]
**platform** | Option<[**models::OciPlatform**](OCIPlatform.md)> |  | [optional]
**artifact_type** | Option<**String**> | ArtifactType is the IANA media type of this artifact. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


