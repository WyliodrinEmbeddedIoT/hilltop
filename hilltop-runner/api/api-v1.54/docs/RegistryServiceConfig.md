# RegistryServiceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**insecure_registry_cidrs** | Option<**Vec<String>**> | List of IP ranges of insecure registries, using the CIDR syntax ([RFC 4632](https://tools.ietf.org/html/4632)). Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  By default, local registries (`::1/128` and `127.0.0.0/8`) are configured as insecure. All other registries are secure. Communicating with an insecure registry is not possible if the daemon assumes that registry is secure.  This configuration override this behavior, insecure communication with registries whose resolved IP address is within the subnet described by the CIDR syntax.  Registries can also be marked insecure by hostname. Those registries are listed under `IndexConfigs` and have their `Secure` field set to `false`.  > **Warning**: Using this option can be useful when running a local > registry, but introduces security vulnerabilities. This option > should therefore ONLY be used for testing purposes. For increased > security, users should add their CA to their system's list of trusted > CAs instead of enabling this option.  | [optional]
**index_configs** | Option<[**std::collections::HashMap<String, models::IndexInfo>**](IndexInfo.md)> |  | [optional]
**mirrors** | Option<**Vec<String>**> | List of registry URLs that act as a mirror for the official (`docker.io`) registry.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


