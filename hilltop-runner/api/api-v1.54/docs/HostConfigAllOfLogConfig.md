# HostConfigAllOfLogConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**Type**> | Name of the logging driver used for the container or \"none\" if logging is disabled. (enum: local, json-file, syslog, journald, gelf, fluentd, awslogs, splunk, etwlogs, none) | [optional]
**config** | Option<**std::collections::HashMap<String, String>**> | Driver-specific configuration options for the logging driver. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


