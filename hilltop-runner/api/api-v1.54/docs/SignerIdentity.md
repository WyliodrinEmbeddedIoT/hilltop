# SignerIdentity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**certificate_issuer** | Option<**String**> | CertificateIssuer is the certificate issuer. | [optional]
**subject_alternative_name** | Option<**String**> | SubjectAlternativeName is the certificate subject alternative name. | [optional]
**issuer** | Option<**String**> | The OIDC issuer. Should match `iss` claim of ID token or, in the case of a federated login like Dex it should match the issuer URL of the upstream issuer. The issuer is not set the extensions are invalid and will fail to render. | [optional]
**build_signer_uri** | Option<**String**> | Reference to specific build instructions that are responsible for signing. | [optional]
**build_signer_digest** | Option<**String**> | Immutable reference to the specific version of the build instructions that is responsible for signing. | [optional]
**runner_environment** | Option<**String**> | Specifies whether the build took place in platform-hosted cloud infrastructure or customer/self-hosted infrastructure. | [optional]
**source_repository_uri** | Option<**String**> | Source repository URL that the build was based on. | [optional]
**source_repository_digest** | Option<**String**> | Immutable reference to a specific version of the source code that the build was based upon. | [optional]
**source_repository_ref** | Option<**String**> | Source Repository Ref that the build run was based upon. | [optional]
**source_repository_identifier** | Option<**String**> | Immutable identifier for the source repository the workflow was based upon. | [optional]
**source_repository_owner_uri** | Option<**String**> | Source repository owner URL of the owner of the source repository that the build was based on. | [optional]
**source_repository_owner_identifier** | Option<**String**> | Immutable identifier for the owner of the source repository that the workflow was based upon. | [optional]
**build_config_uri** | Option<**String**> | Build Config URL to the top-level/initiating build instructions. | [optional]
**build_config_digest** | Option<**String**> | Immutable reference to the specific version of the top-level/initiating build instructions. | [optional]
**build_trigger** | Option<**String**> | Event or action that initiated the build. | [optional]
**run_invocation_uri** | Option<**String**> | Run Invocation URL to uniquely identify the build execution. | [optional]
**source_repository_visibility_at_signing** | Option<**String**> | Source repository visibility at the time of signing the certificate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


