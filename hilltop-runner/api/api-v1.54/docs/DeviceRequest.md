# DeviceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**driver** | Option<**String**> | The name of the device driver to use for this request.  Note that if this is specified the capabilities are ignored when selecting a device driver.  | [optional]
**count** | Option<**i32**> |  | [optional]
**device_ids** | Option<**Vec<String>**> |  | [optional]
**capabilities** | Option<[**Vec<Vec<String>>**](Vec.md)> | A list of capabilities; an OR list of AND lists of capabilities.  Note that if a driver is specified the capabilities have no effect on selecting a driver as the driver name is used directly.  Note that if no driver is specified the capabilities are used to select a driver with the required capabilities.  | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | Driver-specific options, specified as a key/value pairs. These options are passed directly to the driver.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


