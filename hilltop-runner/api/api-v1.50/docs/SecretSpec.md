# SecretSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | User-defined name of the secret. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**data** | Option<**String**> | Data is the data to store as a secret, formatted as a standard base64-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-4)) string. It must be empty if the Driver field is set, in which case the data is loaded from an external secret store. The maximum allowed size is 500KB, as defined in [MaxSecretSize](https://pkg.go.dev/github.com/moby/swarmkit/v2@v2.0.0/api/validation#MaxSecretSize).  This field is only used to _create_ a secret, and is not returned by other endpoints.  | [optional]
**driver** | Option<[**models::Driver**](Driver.md)> |  | [optional]
**templating** | Option<[**models::Driver**](Driver.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


