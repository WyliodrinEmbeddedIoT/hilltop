# ContainerNetworkStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rx_bytes** | Option<**i32**> | Bytes received. Windows and Linux.  | [optional]
**rx_packets** | Option<**i32**> | Packets received. Windows and Linux.  | [optional]
**rx_errors** | Option<**i32**> | Received errors. Not used on Windows.  This field is Linux-specific and always zero for Windows containers.  | [optional]
**rx_dropped** | Option<**i32**> | Incoming packets dropped. Windows and Linux.  | [optional]
**tx_bytes** | Option<**i32**> | Bytes sent. Windows and Linux.  | [optional]
**tx_packets** | Option<**i32**> | Packets sent. Windows and Linux.  | [optional]
**tx_errors** | Option<**i32**> | Sent errors. Not used on Windows.  This field is Linux-specific and always zero for Windows containers.  | [optional]
**tx_dropped** | Option<**i32**> | Outgoing packets dropped. Windows and Linux.  | [optional]
**endpoint_id** | Option<**String**> | Endpoint ID. Not used on Linux.  This field is Windows-specific and omitted for Linux containers.  | [optional]
**instance_id** | Option<**String**> | Instance ID. Not used on Linux.  This field is Windows-specific and omitted for Linux containers.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


