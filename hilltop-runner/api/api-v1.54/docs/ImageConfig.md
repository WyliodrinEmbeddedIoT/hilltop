# ImageConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<**String**> | The user that commands are run as inside the container. | [optional]
**exposed_ports** | Option<**std::collections::HashMap<String, serde_json::Value>**> | An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`  | [optional]
**env** | Option<**Vec<String>**> | A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.  | [optional]
**cmd** | Option<**Vec<String>**> | Command to run specified as a string or an array of strings.  | [optional]
**healthcheck** | Option<[**models::HealthConfig**](HealthConfig.md)> |  | [optional]
**args_escaped** | Option<**bool**> | Command is already escaped (Windows only) | [optional][default to false]
**volumes** | Option<**std::collections::HashMap<String, serde_json::Value>**> | An object mapping mount point paths inside the container to empty objects.  | [optional]
**working_dir** | Option<**String**> | The working directory for commands to run in. | [optional]
**entrypoint** | Option<**Vec<String>**> | The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).  | [optional]
**on_build** | Option<**Vec<String>**> | `ONBUILD` metadata that were defined in the image's `Dockerfile`.  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | User-defined key/value metadata. | [optional]
**stop_signal** | Option<**String**> | Signal to stop a container as a string or unsigned integer.  | [optional]
**shell** | Option<**Vec<String>**> | Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


