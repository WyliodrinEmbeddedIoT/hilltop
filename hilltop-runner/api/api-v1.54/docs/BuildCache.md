# BuildCache

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique ID of the build cache record.  | [optional]
**parents** | Option<**Vec<String>**> | List of parent build cache record IDs.  | [optional]
**r#type** | Option<**Type**> | Cache record type.  (enum: internal, frontend, source.local, source.git.checkout, exec.cachemount, regular) | [optional]
**description** | Option<**String**> | Description of the build-step that produced the build cache.  | [optional]
**in_use** | Option<**bool**> | Indicates if the build cache is in use.  | [optional]
**shared** | Option<**bool**> | Indicates if the build cache is shared.  | [optional]
**size** | Option<**i32**> | Amount of disk space used by the build cache (in bytes).  | [optional]
**created_at** | Option<**String**> | Date and time at which the build cache was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**last_used_at** | Option<**String**> | Date and time at which the build cache was last used in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  | [optional]
**usage_count** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


