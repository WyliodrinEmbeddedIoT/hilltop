# BuildInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**stream** | Option<**String**> |  | [optional]
**error** | Option<**String**> | errors encountered during the operation.   > **Deprecated**: This field is deprecated since API v1.4, and will be omitted in a future API version. Use the information in errorDetail instead. | [optional]
**error_detail** | Option<[**models::ErrorDetail**](ErrorDetail.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**progress** | Option<**String**> | Progress is a pre-formatted presentation of progressDetail.   > **Deprecated**: This field is deprecated since API v1.8, and will be omitted in a future API version. Use the information in progressDetail instead. | [optional]
**progress_detail** | Option<[**models::ProgressDetail**](ProgressDetail.md)> |  | [optional]
**aux** | Option<[**models::ImageId**](ImageID.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


