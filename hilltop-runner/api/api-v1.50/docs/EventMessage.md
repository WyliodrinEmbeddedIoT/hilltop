# EventMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**Type**> | The type of object emitting the event (enum: builder, config, container, daemon, image, network, node, plugin, secret, service, volume) | [optional]
**action** | Option<**String**> | The type of event | [optional]
**actor** | Option<[**models::EventActor**](EventActor.md)> |  | [optional]
**scope** | Option<**Scope**> | Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.  (enum: local, swarm) | [optional]
**time** | Option<**i64**> | Timestamp of event | [optional]
**time_nano** | Option<**i64**> | Timestamp of event, with nanosecond accuracy | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


