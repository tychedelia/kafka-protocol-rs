//! Messages used by the Kafka protocol.
//!
//! These messages are generated programmatically. See the [Kafka's protocol documentation](https://kafka.apache.org/protocol.html) for more information about a given message type.
// WARNING: the items of this module are generated and should not be edited directly.

use crate::protocol::{HeaderVersion, NewType, Request, StrBytes};
use std::convert::TryFrom;

pub mod consumer_protocol_assignment;
pub use consumer_protocol_assignment::ConsumerProtocolAssignment;

pub mod consumer_protocol_subscription;
pub use consumer_protocol_subscription::ConsumerProtocolSubscription;

pub mod default_principal_data;
pub use default_principal_data::DefaultPrincipalData;

pub mod leader_change_message;
pub use leader_change_message::LeaderChangeMessage;

pub mod request_header;
pub use request_header::RequestHeader;

pub mod response_header;
pub use response_header::ResponseHeader;

pub mod snapshot_footer_record;
pub use snapshot_footer_record::SnapshotFooterRecord;

pub mod snapshot_header_record;
pub use snapshot_header_record::SnapshotHeaderRecord;

pub mod produce_request;
pub use produce_request::ProduceRequest;

pub mod fetch_request;
pub use fetch_request::FetchRequest;

pub mod list_offsets_request;
pub use list_offsets_request::ListOffsetsRequest;

pub mod metadata_request;
pub use metadata_request::MetadataRequest;

pub mod leader_and_isr_request;
pub use leader_and_isr_request::LeaderAndIsrRequest;

pub mod stop_replica_request;
pub use stop_replica_request::StopReplicaRequest;

pub mod update_metadata_request;
pub use update_metadata_request::UpdateMetadataRequest;

pub mod controlled_shutdown_request;
pub use controlled_shutdown_request::ControlledShutdownRequest;

pub mod offset_commit_request;
pub use offset_commit_request::OffsetCommitRequest;

pub mod offset_fetch_request;
pub use offset_fetch_request::OffsetFetchRequest;

pub mod find_coordinator_request;
pub use find_coordinator_request::FindCoordinatorRequest;

pub mod join_group_request;
pub use join_group_request::JoinGroupRequest;

pub mod heartbeat_request;
pub use heartbeat_request::HeartbeatRequest;

pub mod leave_group_request;
pub use leave_group_request::LeaveGroupRequest;

pub mod sync_group_request;
pub use sync_group_request::SyncGroupRequest;

pub mod describe_groups_request;
pub use describe_groups_request::DescribeGroupsRequest;

pub mod list_groups_request;
pub use list_groups_request::ListGroupsRequest;

pub mod sasl_handshake_request;
pub use sasl_handshake_request::SaslHandshakeRequest;

pub mod api_versions_request;
pub use api_versions_request::ApiVersionsRequest;

pub mod create_topics_request;
pub use create_topics_request::CreateTopicsRequest;

pub mod delete_topics_request;
pub use delete_topics_request::DeleteTopicsRequest;

pub mod delete_records_request;
pub use delete_records_request::DeleteRecordsRequest;

pub mod init_producer_id_request;
pub use init_producer_id_request::InitProducerIdRequest;

pub mod offset_for_leader_epoch_request;
pub use offset_for_leader_epoch_request::OffsetForLeaderEpochRequest;

pub mod add_partitions_to_txn_request;
pub use add_partitions_to_txn_request::AddPartitionsToTxnRequest;

pub mod add_offsets_to_txn_request;
pub use add_offsets_to_txn_request::AddOffsetsToTxnRequest;

pub mod end_txn_request;
pub use end_txn_request::EndTxnRequest;

pub mod write_txn_markers_request;
pub use write_txn_markers_request::WriteTxnMarkersRequest;

pub mod txn_offset_commit_request;
pub use txn_offset_commit_request::TxnOffsetCommitRequest;

pub mod describe_acls_request;
pub use describe_acls_request::DescribeAclsRequest;

pub mod create_acls_request;
pub use create_acls_request::CreateAclsRequest;

pub mod delete_acls_request;
pub use delete_acls_request::DeleteAclsRequest;

pub mod describe_configs_request;
pub use describe_configs_request::DescribeConfigsRequest;

pub mod alter_configs_request;
pub use alter_configs_request::AlterConfigsRequest;

pub mod alter_replica_log_dirs_request;
pub use alter_replica_log_dirs_request::AlterReplicaLogDirsRequest;

pub mod describe_log_dirs_request;
pub use describe_log_dirs_request::DescribeLogDirsRequest;

pub mod sasl_authenticate_request;
pub use sasl_authenticate_request::SaslAuthenticateRequest;

pub mod create_partitions_request;
pub use create_partitions_request::CreatePartitionsRequest;

pub mod create_delegation_token_request;
pub use create_delegation_token_request::CreateDelegationTokenRequest;

pub mod renew_delegation_token_request;
pub use renew_delegation_token_request::RenewDelegationTokenRequest;

pub mod expire_delegation_token_request;
pub use expire_delegation_token_request::ExpireDelegationTokenRequest;

pub mod describe_delegation_token_request;
pub use describe_delegation_token_request::DescribeDelegationTokenRequest;

pub mod delete_groups_request;
pub use delete_groups_request::DeleteGroupsRequest;

pub mod elect_leaders_request;
pub use elect_leaders_request::ElectLeadersRequest;

pub mod incremental_alter_configs_request;
pub use incremental_alter_configs_request::IncrementalAlterConfigsRequest;

pub mod alter_partition_reassignments_request;
pub use alter_partition_reassignments_request::AlterPartitionReassignmentsRequest;

pub mod list_partition_reassignments_request;
pub use list_partition_reassignments_request::ListPartitionReassignmentsRequest;

pub mod offset_delete_request;
pub use offset_delete_request::OffsetDeleteRequest;

pub mod describe_client_quotas_request;
pub use describe_client_quotas_request::DescribeClientQuotasRequest;

pub mod alter_client_quotas_request;
pub use alter_client_quotas_request::AlterClientQuotasRequest;

pub mod describe_user_scram_credentials_request;
pub use describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest;

pub mod alter_user_scram_credentials_request;
pub use alter_user_scram_credentials_request::AlterUserScramCredentialsRequest;

pub mod vote_request;
pub use vote_request::VoteRequest;

pub mod begin_quorum_epoch_request;
pub use begin_quorum_epoch_request::BeginQuorumEpochRequest;

pub mod end_quorum_epoch_request;
pub use end_quorum_epoch_request::EndQuorumEpochRequest;

pub mod describe_quorum_request;
pub use describe_quorum_request::DescribeQuorumRequest;

pub mod alter_partition_request;
pub use alter_partition_request::AlterPartitionRequest;

pub mod update_features_request;
pub use update_features_request::UpdateFeaturesRequest;

pub mod envelope_request;
pub use envelope_request::EnvelopeRequest;

pub mod fetch_snapshot_request;
pub use fetch_snapshot_request::FetchSnapshotRequest;

pub mod describe_cluster_request;
pub use describe_cluster_request::DescribeClusterRequest;

pub mod describe_producers_request;
pub use describe_producers_request::DescribeProducersRequest;

pub mod broker_registration_request;
pub use broker_registration_request::BrokerRegistrationRequest;

pub mod broker_heartbeat_request;
pub use broker_heartbeat_request::BrokerHeartbeatRequest;

pub mod unregister_broker_request;
pub use unregister_broker_request::UnregisterBrokerRequest;

pub mod describe_transactions_request;
pub use describe_transactions_request::DescribeTransactionsRequest;

pub mod list_transactions_request;
pub use list_transactions_request::ListTransactionsRequest;

pub mod allocate_producer_ids_request;
pub use allocate_producer_ids_request::AllocateProducerIdsRequest;

pub mod consumer_group_heartbeat_request;
pub use consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest;

pub mod controller_registration_request;
pub use controller_registration_request::ControllerRegistrationRequest;

pub mod get_telemetry_subscriptions_request;
pub use get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest;

pub mod push_telemetry_request;
pub use push_telemetry_request::PushTelemetryRequest;

pub mod assign_replicas_to_dirs_request;
pub use assign_replicas_to_dirs_request::AssignReplicasToDirsRequest;

pub mod list_client_metrics_resources_request;
pub use list_client_metrics_resources_request::ListClientMetricsResourcesRequest;

pub mod produce_response;
pub use produce_response::ProduceResponse;

pub mod fetch_response;
pub use fetch_response::FetchResponse;

pub mod list_offsets_response;
pub use list_offsets_response::ListOffsetsResponse;

pub mod metadata_response;
pub use metadata_response::MetadataResponse;

pub mod leader_and_isr_response;
pub use leader_and_isr_response::LeaderAndIsrResponse;

pub mod stop_replica_response;
pub use stop_replica_response::StopReplicaResponse;

pub mod update_metadata_response;
pub use update_metadata_response::UpdateMetadataResponse;

pub mod controlled_shutdown_response;
pub use controlled_shutdown_response::ControlledShutdownResponse;

pub mod offset_commit_response;
pub use offset_commit_response::OffsetCommitResponse;

pub mod offset_fetch_response;
pub use offset_fetch_response::OffsetFetchResponse;

pub mod find_coordinator_response;
pub use find_coordinator_response::FindCoordinatorResponse;

pub mod join_group_response;
pub use join_group_response::JoinGroupResponse;

pub mod heartbeat_response;
pub use heartbeat_response::HeartbeatResponse;

pub mod leave_group_response;
pub use leave_group_response::LeaveGroupResponse;

pub mod sync_group_response;
pub use sync_group_response::SyncGroupResponse;

pub mod describe_groups_response;
pub use describe_groups_response::DescribeGroupsResponse;

pub mod list_groups_response;
pub use list_groups_response::ListGroupsResponse;

pub mod sasl_handshake_response;
pub use sasl_handshake_response::SaslHandshakeResponse;

pub mod api_versions_response;
pub use api_versions_response::ApiVersionsResponse;

pub mod create_topics_response;
pub use create_topics_response::CreateTopicsResponse;

pub mod delete_topics_response;
pub use delete_topics_response::DeleteTopicsResponse;

pub mod delete_records_response;
pub use delete_records_response::DeleteRecordsResponse;

pub mod init_producer_id_response;
pub use init_producer_id_response::InitProducerIdResponse;

pub mod offset_for_leader_epoch_response;
pub use offset_for_leader_epoch_response::OffsetForLeaderEpochResponse;

pub mod add_partitions_to_txn_response;
pub use add_partitions_to_txn_response::AddPartitionsToTxnResponse;

pub mod add_offsets_to_txn_response;
pub use add_offsets_to_txn_response::AddOffsetsToTxnResponse;

pub mod end_txn_response;
pub use end_txn_response::EndTxnResponse;

pub mod write_txn_markers_response;
pub use write_txn_markers_response::WriteTxnMarkersResponse;

pub mod txn_offset_commit_response;
pub use txn_offset_commit_response::TxnOffsetCommitResponse;

pub mod describe_acls_response;
pub use describe_acls_response::DescribeAclsResponse;

pub mod create_acls_response;
pub use create_acls_response::CreateAclsResponse;

pub mod delete_acls_response;
pub use delete_acls_response::DeleteAclsResponse;

pub mod describe_configs_response;
pub use describe_configs_response::DescribeConfigsResponse;

pub mod alter_configs_response;
pub use alter_configs_response::AlterConfigsResponse;

pub mod alter_replica_log_dirs_response;
pub use alter_replica_log_dirs_response::AlterReplicaLogDirsResponse;

pub mod describe_log_dirs_response;
pub use describe_log_dirs_response::DescribeLogDirsResponse;

pub mod sasl_authenticate_response;
pub use sasl_authenticate_response::SaslAuthenticateResponse;

pub mod create_partitions_response;
pub use create_partitions_response::CreatePartitionsResponse;

pub mod create_delegation_token_response;
pub use create_delegation_token_response::CreateDelegationTokenResponse;

pub mod renew_delegation_token_response;
pub use renew_delegation_token_response::RenewDelegationTokenResponse;

pub mod expire_delegation_token_response;
pub use expire_delegation_token_response::ExpireDelegationTokenResponse;

pub mod describe_delegation_token_response;
pub use describe_delegation_token_response::DescribeDelegationTokenResponse;

pub mod delete_groups_response;
pub use delete_groups_response::DeleteGroupsResponse;

pub mod elect_leaders_response;
pub use elect_leaders_response::ElectLeadersResponse;

pub mod incremental_alter_configs_response;
pub use incremental_alter_configs_response::IncrementalAlterConfigsResponse;

pub mod alter_partition_reassignments_response;
pub use alter_partition_reassignments_response::AlterPartitionReassignmentsResponse;

pub mod list_partition_reassignments_response;
pub use list_partition_reassignments_response::ListPartitionReassignmentsResponse;

pub mod offset_delete_response;
pub use offset_delete_response::OffsetDeleteResponse;

pub mod describe_client_quotas_response;
pub use describe_client_quotas_response::DescribeClientQuotasResponse;

pub mod alter_client_quotas_response;
pub use alter_client_quotas_response::AlterClientQuotasResponse;

pub mod describe_user_scram_credentials_response;
pub use describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse;

pub mod alter_user_scram_credentials_response;
pub use alter_user_scram_credentials_response::AlterUserScramCredentialsResponse;

pub mod vote_response;
pub use vote_response::VoteResponse;

pub mod begin_quorum_epoch_response;
pub use begin_quorum_epoch_response::BeginQuorumEpochResponse;

pub mod end_quorum_epoch_response;
pub use end_quorum_epoch_response::EndQuorumEpochResponse;

pub mod describe_quorum_response;
pub use describe_quorum_response::DescribeQuorumResponse;

pub mod alter_partition_response;
pub use alter_partition_response::AlterPartitionResponse;

pub mod update_features_response;
pub use update_features_response::UpdateFeaturesResponse;

pub mod envelope_response;
pub use envelope_response::EnvelopeResponse;

pub mod fetch_snapshot_response;
pub use fetch_snapshot_response::FetchSnapshotResponse;

pub mod describe_cluster_response;
pub use describe_cluster_response::DescribeClusterResponse;

pub mod describe_producers_response;
pub use describe_producers_response::DescribeProducersResponse;

pub mod broker_registration_response;
pub use broker_registration_response::BrokerRegistrationResponse;

pub mod broker_heartbeat_response;
pub use broker_heartbeat_response::BrokerHeartbeatResponse;

pub mod unregister_broker_response;
pub use unregister_broker_response::UnregisterBrokerResponse;

pub mod describe_transactions_response;
pub use describe_transactions_response::DescribeTransactionsResponse;

pub mod list_transactions_response;
pub use list_transactions_response::ListTransactionsResponse;

pub mod allocate_producer_ids_response;
pub use allocate_producer_ids_response::AllocateProducerIdsResponse;

pub mod consumer_group_heartbeat_response;
pub use consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse;

pub mod controller_registration_response;
pub use controller_registration_response::ControllerRegistrationResponse;

pub mod get_telemetry_subscriptions_response;
pub use get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse;

pub mod push_telemetry_response;
pub use push_telemetry_response::PushTelemetryResponse;

pub mod assign_replicas_to_dirs_response;
pub use assign_replicas_to_dirs_response::AssignReplicasToDirsResponse;

pub mod list_client_metrics_resources_response;
pub use list_client_metrics_resources_response::ListClientMetricsResourcesResponse;

impl Request for ProduceRequest {
    const KEY: i16 = 0;
    type Response = ProduceResponse;
}

impl Request for FetchRequest {
    const KEY: i16 = 1;
    type Response = FetchResponse;
}

impl Request for ListOffsetsRequest {
    const KEY: i16 = 2;
    type Response = ListOffsetsResponse;
}

impl Request for MetadataRequest {
    const KEY: i16 = 3;
    type Response = MetadataResponse;
}

impl Request for LeaderAndIsrRequest {
    const KEY: i16 = 4;
    type Response = LeaderAndIsrResponse;
}

impl Request for StopReplicaRequest {
    const KEY: i16 = 5;
    type Response = StopReplicaResponse;
}

impl Request for UpdateMetadataRequest {
    const KEY: i16 = 6;
    type Response = UpdateMetadataResponse;
}

impl Request for ControlledShutdownRequest {
    const KEY: i16 = 7;
    type Response = ControlledShutdownResponse;
}

impl Request for OffsetCommitRequest {
    const KEY: i16 = 8;
    type Response = OffsetCommitResponse;
}

impl Request for OffsetFetchRequest {
    const KEY: i16 = 9;
    type Response = OffsetFetchResponse;
}

impl Request for FindCoordinatorRequest {
    const KEY: i16 = 10;
    type Response = FindCoordinatorResponse;
}

impl Request for JoinGroupRequest {
    const KEY: i16 = 11;
    type Response = JoinGroupResponse;
}

impl Request for HeartbeatRequest {
    const KEY: i16 = 12;
    type Response = HeartbeatResponse;
}

impl Request for LeaveGroupRequest {
    const KEY: i16 = 13;
    type Response = LeaveGroupResponse;
}

impl Request for SyncGroupRequest {
    const KEY: i16 = 14;
    type Response = SyncGroupResponse;
}

impl Request for DescribeGroupsRequest {
    const KEY: i16 = 15;
    type Response = DescribeGroupsResponse;
}

impl Request for ListGroupsRequest {
    const KEY: i16 = 16;
    type Response = ListGroupsResponse;
}

impl Request for SaslHandshakeRequest {
    const KEY: i16 = 17;
    type Response = SaslHandshakeResponse;
}

impl Request for ApiVersionsRequest {
    const KEY: i16 = 18;
    type Response = ApiVersionsResponse;
}

impl Request for CreateTopicsRequest {
    const KEY: i16 = 19;
    type Response = CreateTopicsResponse;
}

impl Request for DeleteTopicsRequest {
    const KEY: i16 = 20;
    type Response = DeleteTopicsResponse;
}

impl Request for DeleteRecordsRequest {
    const KEY: i16 = 21;
    type Response = DeleteRecordsResponse;
}

impl Request for InitProducerIdRequest {
    const KEY: i16 = 22;
    type Response = InitProducerIdResponse;
}

impl Request for OffsetForLeaderEpochRequest {
    const KEY: i16 = 23;
    type Response = OffsetForLeaderEpochResponse;
}

impl Request for AddPartitionsToTxnRequest {
    const KEY: i16 = 24;
    type Response = AddPartitionsToTxnResponse;
}

impl Request for AddOffsetsToTxnRequest {
    const KEY: i16 = 25;
    type Response = AddOffsetsToTxnResponse;
}

impl Request for EndTxnRequest {
    const KEY: i16 = 26;
    type Response = EndTxnResponse;
}

impl Request for WriteTxnMarkersRequest {
    const KEY: i16 = 27;
    type Response = WriteTxnMarkersResponse;
}

impl Request for TxnOffsetCommitRequest {
    const KEY: i16 = 28;
    type Response = TxnOffsetCommitResponse;
}

impl Request for DescribeAclsRequest {
    const KEY: i16 = 29;
    type Response = DescribeAclsResponse;
}

impl Request for CreateAclsRequest {
    const KEY: i16 = 30;
    type Response = CreateAclsResponse;
}

impl Request for DeleteAclsRequest {
    const KEY: i16 = 31;
    type Response = DeleteAclsResponse;
}

impl Request for DescribeConfigsRequest {
    const KEY: i16 = 32;
    type Response = DescribeConfigsResponse;
}

impl Request for AlterConfigsRequest {
    const KEY: i16 = 33;
    type Response = AlterConfigsResponse;
}

impl Request for AlterReplicaLogDirsRequest {
    const KEY: i16 = 34;
    type Response = AlterReplicaLogDirsResponse;
}

impl Request for DescribeLogDirsRequest {
    const KEY: i16 = 35;
    type Response = DescribeLogDirsResponse;
}

impl Request for SaslAuthenticateRequest {
    const KEY: i16 = 36;
    type Response = SaslAuthenticateResponse;
}

impl Request for CreatePartitionsRequest {
    const KEY: i16 = 37;
    type Response = CreatePartitionsResponse;
}

impl Request for CreateDelegationTokenRequest {
    const KEY: i16 = 38;
    type Response = CreateDelegationTokenResponse;
}

impl Request for RenewDelegationTokenRequest {
    const KEY: i16 = 39;
    type Response = RenewDelegationTokenResponse;
}

impl Request for ExpireDelegationTokenRequest {
    const KEY: i16 = 40;
    type Response = ExpireDelegationTokenResponse;
}

impl Request for DescribeDelegationTokenRequest {
    const KEY: i16 = 41;
    type Response = DescribeDelegationTokenResponse;
}

impl Request for DeleteGroupsRequest {
    const KEY: i16 = 42;
    type Response = DeleteGroupsResponse;
}

impl Request for ElectLeadersRequest {
    const KEY: i16 = 43;
    type Response = ElectLeadersResponse;
}

impl Request for IncrementalAlterConfigsRequest {
    const KEY: i16 = 44;
    type Response = IncrementalAlterConfigsResponse;
}

impl Request for AlterPartitionReassignmentsRequest {
    const KEY: i16 = 45;
    type Response = AlterPartitionReassignmentsResponse;
}

impl Request for ListPartitionReassignmentsRequest {
    const KEY: i16 = 46;
    type Response = ListPartitionReassignmentsResponse;
}

impl Request for OffsetDeleteRequest {
    const KEY: i16 = 47;
    type Response = OffsetDeleteResponse;
}

impl Request for DescribeClientQuotasRequest {
    const KEY: i16 = 48;
    type Response = DescribeClientQuotasResponse;
}

impl Request for AlterClientQuotasRequest {
    const KEY: i16 = 49;
    type Response = AlterClientQuotasResponse;
}

impl Request for DescribeUserScramCredentialsRequest {
    const KEY: i16 = 50;
    type Response = DescribeUserScramCredentialsResponse;
}

impl Request for AlterUserScramCredentialsRequest {
    const KEY: i16 = 51;
    type Response = AlterUserScramCredentialsResponse;
}

impl Request for VoteRequest {
    const KEY: i16 = 52;
    type Response = VoteResponse;
}

impl Request for BeginQuorumEpochRequest {
    const KEY: i16 = 53;
    type Response = BeginQuorumEpochResponse;
}

impl Request for EndQuorumEpochRequest {
    const KEY: i16 = 54;
    type Response = EndQuorumEpochResponse;
}

impl Request for DescribeQuorumRequest {
    const KEY: i16 = 55;
    type Response = DescribeQuorumResponse;
}

impl Request for AlterPartitionRequest {
    const KEY: i16 = 56;
    type Response = AlterPartitionResponse;
}

impl Request for UpdateFeaturesRequest {
    const KEY: i16 = 57;
    type Response = UpdateFeaturesResponse;
}

impl Request for EnvelopeRequest {
    const KEY: i16 = 58;
    type Response = EnvelopeResponse;
}

impl Request for FetchSnapshotRequest {
    const KEY: i16 = 59;
    type Response = FetchSnapshotResponse;
}

impl Request for DescribeClusterRequest {
    const KEY: i16 = 60;
    type Response = DescribeClusterResponse;
}

impl Request for DescribeProducersRequest {
    const KEY: i16 = 61;
    type Response = DescribeProducersResponse;
}

impl Request for BrokerRegistrationRequest {
    const KEY: i16 = 62;
    type Response = BrokerRegistrationResponse;
}

impl Request for BrokerHeartbeatRequest {
    const KEY: i16 = 63;
    type Response = BrokerHeartbeatResponse;
}

impl Request for UnregisterBrokerRequest {
    const KEY: i16 = 64;
    type Response = UnregisterBrokerResponse;
}

impl Request for DescribeTransactionsRequest {
    const KEY: i16 = 65;
    type Response = DescribeTransactionsResponse;
}

impl Request for ListTransactionsRequest {
    const KEY: i16 = 66;
    type Response = ListTransactionsResponse;
}

impl Request for AllocateProducerIdsRequest {
    const KEY: i16 = 67;
    type Response = AllocateProducerIdsResponse;
}

impl Request for ConsumerGroupHeartbeatRequest {
    const KEY: i16 = 68;
    type Response = ConsumerGroupHeartbeatResponse;
}

impl Request for ControllerRegistrationRequest {
    const KEY: i16 = 70;
    type Response = ControllerRegistrationResponse;
}

impl Request for GetTelemetrySubscriptionsRequest {
    const KEY: i16 = 71;
    type Response = GetTelemetrySubscriptionsResponse;
}

impl Request for PushTelemetryRequest {
    const KEY: i16 = 72;
    type Response = PushTelemetryResponse;
}

impl Request for AssignReplicasToDirsRequest {
    const KEY: i16 = 73;
    type Response = AssignReplicasToDirsResponse;
}

impl Request for ListClientMetricsResourcesRequest {
    const KEY: i16 = 74;
    type Response = ListClientMetricsResourcesResponse;
}

/// Valid API keys in the Kafka protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKey {
    /// API key for request ProduceRequest
    ProduceKey = 0,
    /// API key for request FetchRequest
    FetchKey = 1,
    /// API key for request ListOffsetsRequest
    ListOffsetsKey = 2,
    /// API key for request MetadataRequest
    MetadataKey = 3,
    /// API key for request LeaderAndIsrRequest
    LeaderAndIsrKey = 4,
    /// API key for request StopReplicaRequest
    StopReplicaKey = 5,
    /// API key for request UpdateMetadataRequest
    UpdateMetadataKey = 6,
    /// API key for request ControlledShutdownRequest
    ControlledShutdownKey = 7,
    /// API key for request OffsetCommitRequest
    OffsetCommitKey = 8,
    /// API key for request OffsetFetchRequest
    OffsetFetchKey = 9,
    /// API key for request FindCoordinatorRequest
    FindCoordinatorKey = 10,
    /// API key for request JoinGroupRequest
    JoinGroupKey = 11,
    /// API key for request HeartbeatRequest
    HeartbeatKey = 12,
    /// API key for request LeaveGroupRequest
    LeaveGroupKey = 13,
    /// API key for request SyncGroupRequest
    SyncGroupKey = 14,
    /// API key for request DescribeGroupsRequest
    DescribeGroupsKey = 15,
    /// API key for request ListGroupsRequest
    ListGroupsKey = 16,
    /// API key for request SaslHandshakeRequest
    SaslHandshakeKey = 17,
    /// API key for request ApiVersionsRequest
    ApiVersionsKey = 18,
    /// API key for request CreateTopicsRequest
    CreateTopicsKey = 19,
    /// API key for request DeleteTopicsRequest
    DeleteTopicsKey = 20,
    /// API key for request DeleteRecordsRequest
    DeleteRecordsKey = 21,
    /// API key for request InitProducerIdRequest
    InitProducerIdKey = 22,
    /// API key for request OffsetForLeaderEpochRequest
    OffsetForLeaderEpochKey = 23,
    /// API key for request AddPartitionsToTxnRequest
    AddPartitionsToTxnKey = 24,
    /// API key for request AddOffsetsToTxnRequest
    AddOffsetsToTxnKey = 25,
    /// API key for request EndTxnRequest
    EndTxnKey = 26,
    /// API key for request WriteTxnMarkersRequest
    WriteTxnMarkersKey = 27,
    /// API key for request TxnOffsetCommitRequest
    TxnOffsetCommitKey = 28,
    /// API key for request DescribeAclsRequest
    DescribeAclsKey = 29,
    /// API key for request CreateAclsRequest
    CreateAclsKey = 30,
    /// API key for request DeleteAclsRequest
    DeleteAclsKey = 31,
    /// API key for request DescribeConfigsRequest
    DescribeConfigsKey = 32,
    /// API key for request AlterConfigsRequest
    AlterConfigsKey = 33,
    /// API key for request AlterReplicaLogDirsRequest
    AlterReplicaLogDirsKey = 34,
    /// API key for request DescribeLogDirsRequest
    DescribeLogDirsKey = 35,
    /// API key for request SaslAuthenticateRequest
    SaslAuthenticateKey = 36,
    /// API key for request CreatePartitionsRequest
    CreatePartitionsKey = 37,
    /// API key for request CreateDelegationTokenRequest
    CreateDelegationTokenKey = 38,
    /// API key for request RenewDelegationTokenRequest
    RenewDelegationTokenKey = 39,
    /// API key for request ExpireDelegationTokenRequest
    ExpireDelegationTokenKey = 40,
    /// API key for request DescribeDelegationTokenRequest
    DescribeDelegationTokenKey = 41,
    /// API key for request DeleteGroupsRequest
    DeleteGroupsKey = 42,
    /// API key for request ElectLeadersRequest
    ElectLeadersKey = 43,
    /// API key for request IncrementalAlterConfigsRequest
    IncrementalAlterConfigsKey = 44,
    /// API key for request AlterPartitionReassignmentsRequest
    AlterPartitionReassignmentsKey = 45,
    /// API key for request ListPartitionReassignmentsRequest
    ListPartitionReassignmentsKey = 46,
    /// API key for request OffsetDeleteRequest
    OffsetDeleteKey = 47,
    /// API key for request DescribeClientQuotasRequest
    DescribeClientQuotasKey = 48,
    /// API key for request AlterClientQuotasRequest
    AlterClientQuotasKey = 49,
    /// API key for request DescribeUserScramCredentialsRequest
    DescribeUserScramCredentialsKey = 50,
    /// API key for request AlterUserScramCredentialsRequest
    AlterUserScramCredentialsKey = 51,
    /// API key for request VoteRequest
    VoteKey = 52,
    /// API key for request BeginQuorumEpochRequest
    BeginQuorumEpochKey = 53,
    /// API key for request EndQuorumEpochRequest
    EndQuorumEpochKey = 54,
    /// API key for request DescribeQuorumRequest
    DescribeQuorumKey = 55,
    /// API key for request AlterPartitionRequest
    AlterPartitionKey = 56,
    /// API key for request UpdateFeaturesRequest
    UpdateFeaturesKey = 57,
    /// API key for request EnvelopeRequest
    EnvelopeKey = 58,
    /// API key for request FetchSnapshotRequest
    FetchSnapshotKey = 59,
    /// API key for request DescribeClusterRequest
    DescribeClusterKey = 60,
    /// API key for request DescribeProducersRequest
    DescribeProducersKey = 61,
    /// API key for request BrokerRegistrationRequest
    BrokerRegistrationKey = 62,
    /// API key for request BrokerHeartbeatRequest
    BrokerHeartbeatKey = 63,
    /// API key for request UnregisterBrokerRequest
    UnregisterBrokerKey = 64,
    /// API key for request DescribeTransactionsRequest
    DescribeTransactionsKey = 65,
    /// API key for request ListTransactionsRequest
    ListTransactionsKey = 66,
    /// API key for request AllocateProducerIdsRequest
    AllocateProducerIdsKey = 67,
    /// API key for request ConsumerGroupHeartbeatRequest
    ConsumerGroupHeartbeatKey = 68,
    /// API key for request ControllerRegistrationRequest
    ControllerRegistrationKey = 70,
    /// API key for request GetTelemetrySubscriptionsRequest
    GetTelemetrySubscriptionsKey = 71,
    /// API key for request PushTelemetryRequest
    PushTelemetryKey = 72,
    /// API key for request AssignReplicasToDirsRequest
    AssignReplicasToDirsKey = 73,
    /// API key for request ListClientMetricsResourcesRequest
    ListClientMetricsResourcesKey = 74,
}

impl ApiKey {
    /// Get the version of request header that needs to be prepended to this message
    pub fn request_header_version(&self, version: i16) -> i16 {
        match self {
            ApiKey::ProduceKey => ProduceRequest::header_version(version),
            ApiKey::FetchKey => FetchRequest::header_version(version),
            ApiKey::ListOffsetsKey => ListOffsetsRequest::header_version(version),
            ApiKey::MetadataKey => MetadataRequest::header_version(version),
            ApiKey::LeaderAndIsrKey => LeaderAndIsrRequest::header_version(version),
            ApiKey::StopReplicaKey => StopReplicaRequest::header_version(version),
            ApiKey::UpdateMetadataKey => UpdateMetadataRequest::header_version(version),
            ApiKey::ControlledShutdownKey => ControlledShutdownRequest::header_version(version),
            ApiKey::OffsetCommitKey => OffsetCommitRequest::header_version(version),
            ApiKey::OffsetFetchKey => OffsetFetchRequest::header_version(version),
            ApiKey::FindCoordinatorKey => FindCoordinatorRequest::header_version(version),
            ApiKey::JoinGroupKey => JoinGroupRequest::header_version(version),
            ApiKey::HeartbeatKey => HeartbeatRequest::header_version(version),
            ApiKey::LeaveGroupKey => LeaveGroupRequest::header_version(version),
            ApiKey::SyncGroupKey => SyncGroupRequest::header_version(version),
            ApiKey::DescribeGroupsKey => DescribeGroupsRequest::header_version(version),
            ApiKey::ListGroupsKey => ListGroupsRequest::header_version(version),
            ApiKey::SaslHandshakeKey => SaslHandshakeRequest::header_version(version),
            ApiKey::ApiVersionsKey => ApiVersionsRequest::header_version(version),
            ApiKey::CreateTopicsKey => CreateTopicsRequest::header_version(version),
            ApiKey::DeleteTopicsKey => DeleteTopicsRequest::header_version(version),
            ApiKey::DeleteRecordsKey => DeleteRecordsRequest::header_version(version),
            ApiKey::InitProducerIdKey => InitProducerIdRequest::header_version(version),
            ApiKey::OffsetForLeaderEpochKey => OffsetForLeaderEpochRequest::header_version(version),
            ApiKey::AddPartitionsToTxnKey => AddPartitionsToTxnRequest::header_version(version),
            ApiKey::AddOffsetsToTxnKey => AddOffsetsToTxnRequest::header_version(version),
            ApiKey::EndTxnKey => EndTxnRequest::header_version(version),
            ApiKey::WriteTxnMarkersKey => WriteTxnMarkersRequest::header_version(version),
            ApiKey::TxnOffsetCommitKey => TxnOffsetCommitRequest::header_version(version),
            ApiKey::DescribeAclsKey => DescribeAclsRequest::header_version(version),
            ApiKey::CreateAclsKey => CreateAclsRequest::header_version(version),
            ApiKey::DeleteAclsKey => DeleteAclsRequest::header_version(version),
            ApiKey::DescribeConfigsKey => DescribeConfigsRequest::header_version(version),
            ApiKey::AlterConfigsKey => AlterConfigsRequest::header_version(version),
            ApiKey::AlterReplicaLogDirsKey => AlterReplicaLogDirsRequest::header_version(version),
            ApiKey::DescribeLogDirsKey => DescribeLogDirsRequest::header_version(version),
            ApiKey::SaslAuthenticateKey => SaslAuthenticateRequest::header_version(version),
            ApiKey::CreatePartitionsKey => CreatePartitionsRequest::header_version(version),
            ApiKey::CreateDelegationTokenKey => {
                CreateDelegationTokenRequest::header_version(version)
            }
            ApiKey::RenewDelegationTokenKey => RenewDelegationTokenRequest::header_version(version),
            ApiKey::ExpireDelegationTokenKey => {
                ExpireDelegationTokenRequest::header_version(version)
            }
            ApiKey::DescribeDelegationTokenKey => {
                DescribeDelegationTokenRequest::header_version(version)
            }
            ApiKey::DeleteGroupsKey => DeleteGroupsRequest::header_version(version),
            ApiKey::ElectLeadersKey => ElectLeadersRequest::header_version(version),
            ApiKey::IncrementalAlterConfigsKey => {
                IncrementalAlterConfigsRequest::header_version(version)
            }
            ApiKey::AlterPartitionReassignmentsKey => {
                AlterPartitionReassignmentsRequest::header_version(version)
            }
            ApiKey::ListPartitionReassignmentsKey => {
                ListPartitionReassignmentsRequest::header_version(version)
            }
            ApiKey::OffsetDeleteKey => OffsetDeleteRequest::header_version(version),
            ApiKey::DescribeClientQuotasKey => DescribeClientQuotasRequest::header_version(version),
            ApiKey::AlterClientQuotasKey => AlterClientQuotasRequest::header_version(version),
            ApiKey::DescribeUserScramCredentialsKey => {
                DescribeUserScramCredentialsRequest::header_version(version)
            }
            ApiKey::AlterUserScramCredentialsKey => {
                AlterUserScramCredentialsRequest::header_version(version)
            }
            ApiKey::VoteKey => VoteRequest::header_version(version),
            ApiKey::BeginQuorumEpochKey => BeginQuorumEpochRequest::header_version(version),
            ApiKey::EndQuorumEpochKey => EndQuorumEpochRequest::header_version(version),
            ApiKey::DescribeQuorumKey => DescribeQuorumRequest::header_version(version),
            ApiKey::AlterPartitionKey => AlterPartitionRequest::header_version(version),
            ApiKey::UpdateFeaturesKey => UpdateFeaturesRequest::header_version(version),
            ApiKey::EnvelopeKey => EnvelopeRequest::header_version(version),
            ApiKey::FetchSnapshotKey => FetchSnapshotRequest::header_version(version),
            ApiKey::DescribeClusterKey => DescribeClusterRequest::header_version(version),
            ApiKey::DescribeProducersKey => DescribeProducersRequest::header_version(version),
            ApiKey::BrokerRegistrationKey => BrokerRegistrationRequest::header_version(version),
            ApiKey::BrokerHeartbeatKey => BrokerHeartbeatRequest::header_version(version),
            ApiKey::UnregisterBrokerKey => UnregisterBrokerRequest::header_version(version),
            ApiKey::DescribeTransactionsKey => DescribeTransactionsRequest::header_version(version),
            ApiKey::ListTransactionsKey => ListTransactionsRequest::header_version(version),
            ApiKey::AllocateProducerIdsKey => AllocateProducerIdsRequest::header_version(version),
            ApiKey::ConsumerGroupHeartbeatKey => {
                ConsumerGroupHeartbeatRequest::header_version(version)
            }
            ApiKey::ControllerRegistrationKey => {
                ControllerRegistrationRequest::header_version(version)
            }
            ApiKey::GetTelemetrySubscriptionsKey => {
                GetTelemetrySubscriptionsRequest::header_version(version)
            }
            ApiKey::PushTelemetryKey => PushTelemetryRequest::header_version(version),
            ApiKey::AssignReplicasToDirsKey => AssignReplicasToDirsRequest::header_version(version),
            ApiKey::ListClientMetricsResourcesKey => {
                ListClientMetricsResourcesRequest::header_version(version)
            }
        }
    }
    /// Get the version of response header that needs to be prepended to this message
    pub fn response_header_version(&self, version: i16) -> i16 {
        match self {
            ApiKey::ProduceKey => ProduceResponse::header_version(version),
            ApiKey::FetchKey => FetchResponse::header_version(version),
            ApiKey::ListOffsetsKey => ListOffsetsResponse::header_version(version),
            ApiKey::MetadataKey => MetadataResponse::header_version(version),
            ApiKey::LeaderAndIsrKey => LeaderAndIsrResponse::header_version(version),
            ApiKey::StopReplicaKey => StopReplicaResponse::header_version(version),
            ApiKey::UpdateMetadataKey => UpdateMetadataResponse::header_version(version),
            ApiKey::ControlledShutdownKey => ControlledShutdownResponse::header_version(version),
            ApiKey::OffsetCommitKey => OffsetCommitResponse::header_version(version),
            ApiKey::OffsetFetchKey => OffsetFetchResponse::header_version(version),
            ApiKey::FindCoordinatorKey => FindCoordinatorResponse::header_version(version),
            ApiKey::JoinGroupKey => JoinGroupResponse::header_version(version),
            ApiKey::HeartbeatKey => HeartbeatResponse::header_version(version),
            ApiKey::LeaveGroupKey => LeaveGroupResponse::header_version(version),
            ApiKey::SyncGroupKey => SyncGroupResponse::header_version(version),
            ApiKey::DescribeGroupsKey => DescribeGroupsResponse::header_version(version),
            ApiKey::ListGroupsKey => ListGroupsResponse::header_version(version),
            ApiKey::SaslHandshakeKey => SaslHandshakeResponse::header_version(version),
            ApiKey::ApiVersionsKey => ApiVersionsResponse::header_version(version),
            ApiKey::CreateTopicsKey => CreateTopicsResponse::header_version(version),
            ApiKey::DeleteTopicsKey => DeleteTopicsResponse::header_version(version),
            ApiKey::DeleteRecordsKey => DeleteRecordsResponse::header_version(version),
            ApiKey::InitProducerIdKey => InitProducerIdResponse::header_version(version),
            ApiKey::OffsetForLeaderEpochKey => {
                OffsetForLeaderEpochResponse::header_version(version)
            }
            ApiKey::AddPartitionsToTxnKey => AddPartitionsToTxnResponse::header_version(version),
            ApiKey::AddOffsetsToTxnKey => AddOffsetsToTxnResponse::header_version(version),
            ApiKey::EndTxnKey => EndTxnResponse::header_version(version),
            ApiKey::WriteTxnMarkersKey => WriteTxnMarkersResponse::header_version(version),
            ApiKey::TxnOffsetCommitKey => TxnOffsetCommitResponse::header_version(version),
            ApiKey::DescribeAclsKey => DescribeAclsResponse::header_version(version),
            ApiKey::CreateAclsKey => CreateAclsResponse::header_version(version),
            ApiKey::DeleteAclsKey => DeleteAclsResponse::header_version(version),
            ApiKey::DescribeConfigsKey => DescribeConfigsResponse::header_version(version),
            ApiKey::AlterConfigsKey => AlterConfigsResponse::header_version(version),
            ApiKey::AlterReplicaLogDirsKey => AlterReplicaLogDirsResponse::header_version(version),
            ApiKey::DescribeLogDirsKey => DescribeLogDirsResponse::header_version(version),
            ApiKey::SaslAuthenticateKey => SaslAuthenticateResponse::header_version(version),
            ApiKey::CreatePartitionsKey => CreatePartitionsResponse::header_version(version),
            ApiKey::CreateDelegationTokenKey => {
                CreateDelegationTokenResponse::header_version(version)
            }
            ApiKey::RenewDelegationTokenKey => {
                RenewDelegationTokenResponse::header_version(version)
            }
            ApiKey::ExpireDelegationTokenKey => {
                ExpireDelegationTokenResponse::header_version(version)
            }
            ApiKey::DescribeDelegationTokenKey => {
                DescribeDelegationTokenResponse::header_version(version)
            }
            ApiKey::DeleteGroupsKey => DeleteGroupsResponse::header_version(version),
            ApiKey::ElectLeadersKey => ElectLeadersResponse::header_version(version),
            ApiKey::IncrementalAlterConfigsKey => {
                IncrementalAlterConfigsResponse::header_version(version)
            }
            ApiKey::AlterPartitionReassignmentsKey => {
                AlterPartitionReassignmentsResponse::header_version(version)
            }
            ApiKey::ListPartitionReassignmentsKey => {
                ListPartitionReassignmentsResponse::header_version(version)
            }
            ApiKey::OffsetDeleteKey => OffsetDeleteResponse::header_version(version),
            ApiKey::DescribeClientQuotasKey => {
                DescribeClientQuotasResponse::header_version(version)
            }
            ApiKey::AlterClientQuotasKey => AlterClientQuotasResponse::header_version(version),
            ApiKey::DescribeUserScramCredentialsKey => {
                DescribeUserScramCredentialsResponse::header_version(version)
            }
            ApiKey::AlterUserScramCredentialsKey => {
                AlterUserScramCredentialsResponse::header_version(version)
            }
            ApiKey::VoteKey => VoteResponse::header_version(version),
            ApiKey::BeginQuorumEpochKey => BeginQuorumEpochResponse::header_version(version),
            ApiKey::EndQuorumEpochKey => EndQuorumEpochResponse::header_version(version),
            ApiKey::DescribeQuorumKey => DescribeQuorumResponse::header_version(version),
            ApiKey::AlterPartitionKey => AlterPartitionResponse::header_version(version),
            ApiKey::UpdateFeaturesKey => UpdateFeaturesResponse::header_version(version),
            ApiKey::EnvelopeKey => EnvelopeResponse::header_version(version),
            ApiKey::FetchSnapshotKey => FetchSnapshotResponse::header_version(version),
            ApiKey::DescribeClusterKey => DescribeClusterResponse::header_version(version),
            ApiKey::DescribeProducersKey => DescribeProducersResponse::header_version(version),
            ApiKey::BrokerRegistrationKey => BrokerRegistrationResponse::header_version(version),
            ApiKey::BrokerHeartbeatKey => BrokerHeartbeatResponse::header_version(version),
            ApiKey::UnregisterBrokerKey => UnregisterBrokerResponse::header_version(version),
            ApiKey::DescribeTransactionsKey => {
                DescribeTransactionsResponse::header_version(version)
            }
            ApiKey::ListTransactionsKey => ListTransactionsResponse::header_version(version),
            ApiKey::AllocateProducerIdsKey => AllocateProducerIdsResponse::header_version(version),
            ApiKey::ConsumerGroupHeartbeatKey => {
                ConsumerGroupHeartbeatResponse::header_version(version)
            }
            ApiKey::ControllerRegistrationKey => {
                ControllerRegistrationResponse::header_version(version)
            }
            ApiKey::GetTelemetrySubscriptionsKey => {
                GetTelemetrySubscriptionsResponse::header_version(version)
            }
            ApiKey::PushTelemetryKey => PushTelemetryResponse::header_version(version),
            ApiKey::AssignReplicasToDirsKey => {
                AssignReplicasToDirsResponse::header_version(version)
            }
            ApiKey::ListClientMetricsResourcesKey => {
                ListClientMetricsResourcesResponse::header_version(version)
            }
        }
    }
}
impl TryFrom<i16> for ApiKey {
    type Error = ();

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == ApiKey::ProduceKey as i16 => Ok(ApiKey::ProduceKey),
            x if x == ApiKey::FetchKey as i16 => Ok(ApiKey::FetchKey),
            x if x == ApiKey::ListOffsetsKey as i16 => Ok(ApiKey::ListOffsetsKey),
            x if x == ApiKey::MetadataKey as i16 => Ok(ApiKey::MetadataKey),
            x if x == ApiKey::LeaderAndIsrKey as i16 => Ok(ApiKey::LeaderAndIsrKey),
            x if x == ApiKey::StopReplicaKey as i16 => Ok(ApiKey::StopReplicaKey),
            x if x == ApiKey::UpdateMetadataKey as i16 => Ok(ApiKey::UpdateMetadataKey),
            x if x == ApiKey::ControlledShutdownKey as i16 => Ok(ApiKey::ControlledShutdownKey),
            x if x == ApiKey::OffsetCommitKey as i16 => Ok(ApiKey::OffsetCommitKey),
            x if x == ApiKey::OffsetFetchKey as i16 => Ok(ApiKey::OffsetFetchKey),
            x if x == ApiKey::FindCoordinatorKey as i16 => Ok(ApiKey::FindCoordinatorKey),
            x if x == ApiKey::JoinGroupKey as i16 => Ok(ApiKey::JoinGroupKey),
            x if x == ApiKey::HeartbeatKey as i16 => Ok(ApiKey::HeartbeatKey),
            x if x == ApiKey::LeaveGroupKey as i16 => Ok(ApiKey::LeaveGroupKey),
            x if x == ApiKey::SyncGroupKey as i16 => Ok(ApiKey::SyncGroupKey),
            x if x == ApiKey::DescribeGroupsKey as i16 => Ok(ApiKey::DescribeGroupsKey),
            x if x == ApiKey::ListGroupsKey as i16 => Ok(ApiKey::ListGroupsKey),
            x if x == ApiKey::SaslHandshakeKey as i16 => Ok(ApiKey::SaslHandshakeKey),
            x if x == ApiKey::ApiVersionsKey as i16 => Ok(ApiKey::ApiVersionsKey),
            x if x == ApiKey::CreateTopicsKey as i16 => Ok(ApiKey::CreateTopicsKey),
            x if x == ApiKey::DeleteTopicsKey as i16 => Ok(ApiKey::DeleteTopicsKey),
            x if x == ApiKey::DeleteRecordsKey as i16 => Ok(ApiKey::DeleteRecordsKey),
            x if x == ApiKey::InitProducerIdKey as i16 => Ok(ApiKey::InitProducerIdKey),
            x if x == ApiKey::OffsetForLeaderEpochKey as i16 => Ok(ApiKey::OffsetForLeaderEpochKey),
            x if x == ApiKey::AddPartitionsToTxnKey as i16 => Ok(ApiKey::AddPartitionsToTxnKey),
            x if x == ApiKey::AddOffsetsToTxnKey as i16 => Ok(ApiKey::AddOffsetsToTxnKey),
            x if x == ApiKey::EndTxnKey as i16 => Ok(ApiKey::EndTxnKey),
            x if x == ApiKey::WriteTxnMarkersKey as i16 => Ok(ApiKey::WriteTxnMarkersKey),
            x if x == ApiKey::TxnOffsetCommitKey as i16 => Ok(ApiKey::TxnOffsetCommitKey),
            x if x == ApiKey::DescribeAclsKey as i16 => Ok(ApiKey::DescribeAclsKey),
            x if x == ApiKey::CreateAclsKey as i16 => Ok(ApiKey::CreateAclsKey),
            x if x == ApiKey::DeleteAclsKey as i16 => Ok(ApiKey::DeleteAclsKey),
            x if x == ApiKey::DescribeConfigsKey as i16 => Ok(ApiKey::DescribeConfigsKey),
            x if x == ApiKey::AlterConfigsKey as i16 => Ok(ApiKey::AlterConfigsKey),
            x if x == ApiKey::AlterReplicaLogDirsKey as i16 => Ok(ApiKey::AlterReplicaLogDirsKey),
            x if x == ApiKey::DescribeLogDirsKey as i16 => Ok(ApiKey::DescribeLogDirsKey),
            x if x == ApiKey::SaslAuthenticateKey as i16 => Ok(ApiKey::SaslAuthenticateKey),
            x if x == ApiKey::CreatePartitionsKey as i16 => Ok(ApiKey::CreatePartitionsKey),
            x if x == ApiKey::CreateDelegationTokenKey as i16 => {
                Ok(ApiKey::CreateDelegationTokenKey)
            }
            x if x == ApiKey::RenewDelegationTokenKey as i16 => Ok(ApiKey::RenewDelegationTokenKey),
            x if x == ApiKey::ExpireDelegationTokenKey as i16 => {
                Ok(ApiKey::ExpireDelegationTokenKey)
            }
            x if x == ApiKey::DescribeDelegationTokenKey as i16 => {
                Ok(ApiKey::DescribeDelegationTokenKey)
            }
            x if x == ApiKey::DeleteGroupsKey as i16 => Ok(ApiKey::DeleteGroupsKey),
            x if x == ApiKey::ElectLeadersKey as i16 => Ok(ApiKey::ElectLeadersKey),
            x if x == ApiKey::IncrementalAlterConfigsKey as i16 => {
                Ok(ApiKey::IncrementalAlterConfigsKey)
            }
            x if x == ApiKey::AlterPartitionReassignmentsKey as i16 => {
                Ok(ApiKey::AlterPartitionReassignmentsKey)
            }
            x if x == ApiKey::ListPartitionReassignmentsKey as i16 => {
                Ok(ApiKey::ListPartitionReassignmentsKey)
            }
            x if x == ApiKey::OffsetDeleteKey as i16 => Ok(ApiKey::OffsetDeleteKey),
            x if x == ApiKey::DescribeClientQuotasKey as i16 => Ok(ApiKey::DescribeClientQuotasKey),
            x if x == ApiKey::AlterClientQuotasKey as i16 => Ok(ApiKey::AlterClientQuotasKey),
            x if x == ApiKey::DescribeUserScramCredentialsKey as i16 => {
                Ok(ApiKey::DescribeUserScramCredentialsKey)
            }
            x if x == ApiKey::AlterUserScramCredentialsKey as i16 => {
                Ok(ApiKey::AlterUserScramCredentialsKey)
            }
            x if x == ApiKey::VoteKey as i16 => Ok(ApiKey::VoteKey),
            x if x == ApiKey::BeginQuorumEpochKey as i16 => Ok(ApiKey::BeginQuorumEpochKey),
            x if x == ApiKey::EndQuorumEpochKey as i16 => Ok(ApiKey::EndQuorumEpochKey),
            x if x == ApiKey::DescribeQuorumKey as i16 => Ok(ApiKey::DescribeQuorumKey),
            x if x == ApiKey::AlterPartitionKey as i16 => Ok(ApiKey::AlterPartitionKey),
            x if x == ApiKey::UpdateFeaturesKey as i16 => Ok(ApiKey::UpdateFeaturesKey),
            x if x == ApiKey::EnvelopeKey as i16 => Ok(ApiKey::EnvelopeKey),
            x if x == ApiKey::FetchSnapshotKey as i16 => Ok(ApiKey::FetchSnapshotKey),
            x if x == ApiKey::DescribeClusterKey as i16 => Ok(ApiKey::DescribeClusterKey),
            x if x == ApiKey::DescribeProducersKey as i16 => Ok(ApiKey::DescribeProducersKey),
            x if x == ApiKey::BrokerRegistrationKey as i16 => Ok(ApiKey::BrokerRegistrationKey),
            x if x == ApiKey::BrokerHeartbeatKey as i16 => Ok(ApiKey::BrokerHeartbeatKey),
            x if x == ApiKey::UnregisterBrokerKey as i16 => Ok(ApiKey::UnregisterBrokerKey),
            x if x == ApiKey::DescribeTransactionsKey as i16 => Ok(ApiKey::DescribeTransactionsKey),
            x if x == ApiKey::ListTransactionsKey as i16 => Ok(ApiKey::ListTransactionsKey),
            x if x == ApiKey::AllocateProducerIdsKey as i16 => Ok(ApiKey::AllocateProducerIdsKey),
            x if x == ApiKey::ConsumerGroupHeartbeatKey as i16 => {
                Ok(ApiKey::ConsumerGroupHeartbeatKey)
            }
            x if x == ApiKey::ControllerRegistrationKey as i16 => {
                Ok(ApiKey::ControllerRegistrationKey)
            }
            x if x == ApiKey::GetTelemetrySubscriptionsKey as i16 => {
                Ok(ApiKey::GetTelemetrySubscriptionsKey)
            }
            x if x == ApiKey::PushTelemetryKey as i16 => Ok(ApiKey::PushTelemetryKey),
            x if x == ApiKey::AssignReplicasToDirsKey as i16 => Ok(ApiKey::AssignReplicasToDirsKey),
            x if x == ApiKey::ListClientMetricsResourcesKey as i16 => {
                Ok(ApiKey::ListClientMetricsResourcesKey)
            }
            _ => Err(()),
        }
    }
}

/// Wrapping enum for all requests in the Kafka protocol.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum RequestKind {
    /// ProduceRequest,
    Produce(ProduceRequest),
    /// FetchRequest,
    Fetch(FetchRequest),
    /// ListOffsetsRequest,
    ListOffsets(ListOffsetsRequest),
    /// MetadataRequest,
    Metadata(MetadataRequest),
    /// LeaderAndIsrRequest,
    LeaderAndIsr(LeaderAndIsrRequest),
    /// StopReplicaRequest,
    StopReplica(StopReplicaRequest),
    /// UpdateMetadataRequest,
    UpdateMetadata(UpdateMetadataRequest),
    /// ControlledShutdownRequest,
    ControlledShutdown(ControlledShutdownRequest),
    /// OffsetCommitRequest,
    OffsetCommit(OffsetCommitRequest),
    /// OffsetFetchRequest,
    OffsetFetch(OffsetFetchRequest),
    /// FindCoordinatorRequest,
    FindCoordinator(FindCoordinatorRequest),
    /// JoinGroupRequest,
    JoinGroup(JoinGroupRequest),
    /// HeartbeatRequest,
    Heartbeat(HeartbeatRequest),
    /// LeaveGroupRequest,
    LeaveGroup(LeaveGroupRequest),
    /// SyncGroupRequest,
    SyncGroup(SyncGroupRequest),
    /// DescribeGroupsRequest,
    DescribeGroups(DescribeGroupsRequest),
    /// ListGroupsRequest,
    ListGroups(ListGroupsRequest),
    /// SaslHandshakeRequest,
    SaslHandshake(SaslHandshakeRequest),
    /// ApiVersionsRequest,
    ApiVersions(ApiVersionsRequest),
    /// CreateTopicsRequest,
    CreateTopics(CreateTopicsRequest),
    /// DeleteTopicsRequest,
    DeleteTopics(DeleteTopicsRequest),
    /// DeleteRecordsRequest,
    DeleteRecords(DeleteRecordsRequest),
    /// InitProducerIdRequest,
    InitProducerId(InitProducerIdRequest),
    /// OffsetForLeaderEpochRequest,
    OffsetForLeaderEpoch(OffsetForLeaderEpochRequest),
    /// AddPartitionsToTxnRequest,
    AddPartitionsToTxn(AddPartitionsToTxnRequest),
    /// AddOffsetsToTxnRequest,
    AddOffsetsToTxn(AddOffsetsToTxnRequest),
    /// EndTxnRequest,
    EndTxn(EndTxnRequest),
    /// WriteTxnMarkersRequest,
    WriteTxnMarkers(WriteTxnMarkersRequest),
    /// TxnOffsetCommitRequest,
    TxnOffsetCommit(TxnOffsetCommitRequest),
    /// DescribeAclsRequest,
    DescribeAcls(DescribeAclsRequest),
    /// CreateAclsRequest,
    CreateAcls(CreateAclsRequest),
    /// DeleteAclsRequest,
    DeleteAcls(DeleteAclsRequest),
    /// DescribeConfigsRequest,
    DescribeConfigs(DescribeConfigsRequest),
    /// AlterConfigsRequest,
    AlterConfigs(AlterConfigsRequest),
    /// AlterReplicaLogDirsRequest,
    AlterReplicaLogDirs(AlterReplicaLogDirsRequest),
    /// DescribeLogDirsRequest,
    DescribeLogDirs(DescribeLogDirsRequest),
    /// SaslAuthenticateRequest,
    SaslAuthenticate(SaslAuthenticateRequest),
    /// CreatePartitionsRequest,
    CreatePartitions(CreatePartitionsRequest),
    /// CreateDelegationTokenRequest,
    CreateDelegationToken(CreateDelegationTokenRequest),
    /// RenewDelegationTokenRequest,
    RenewDelegationToken(RenewDelegationTokenRequest),
    /// ExpireDelegationTokenRequest,
    ExpireDelegationToken(ExpireDelegationTokenRequest),
    /// DescribeDelegationTokenRequest,
    DescribeDelegationToken(DescribeDelegationTokenRequest),
    /// DeleteGroupsRequest,
    DeleteGroups(DeleteGroupsRequest),
    /// ElectLeadersRequest,
    ElectLeaders(ElectLeadersRequest),
    /// IncrementalAlterConfigsRequest,
    IncrementalAlterConfigs(IncrementalAlterConfigsRequest),
    /// AlterPartitionReassignmentsRequest,
    AlterPartitionReassignments(AlterPartitionReassignmentsRequest),
    /// ListPartitionReassignmentsRequest,
    ListPartitionReassignments(ListPartitionReassignmentsRequest),
    /// OffsetDeleteRequest,
    OffsetDelete(OffsetDeleteRequest),
    /// DescribeClientQuotasRequest,
    DescribeClientQuotas(DescribeClientQuotasRequest),
    /// AlterClientQuotasRequest,
    AlterClientQuotas(AlterClientQuotasRequest),
    /// DescribeUserScramCredentialsRequest,
    DescribeUserScramCredentials(DescribeUserScramCredentialsRequest),
    /// AlterUserScramCredentialsRequest,
    AlterUserScramCredentials(AlterUserScramCredentialsRequest),
    /// VoteRequest,
    Vote(VoteRequest),
    /// BeginQuorumEpochRequest,
    BeginQuorumEpoch(BeginQuorumEpochRequest),
    /// EndQuorumEpochRequest,
    EndQuorumEpoch(EndQuorumEpochRequest),
    /// DescribeQuorumRequest,
    DescribeQuorum(DescribeQuorumRequest),
    /// AlterPartitionRequest,
    AlterPartition(AlterPartitionRequest),
    /// UpdateFeaturesRequest,
    UpdateFeatures(UpdateFeaturesRequest),
    /// EnvelopeRequest,
    Envelope(EnvelopeRequest),
    /// FetchSnapshotRequest,
    FetchSnapshot(FetchSnapshotRequest),
    /// DescribeClusterRequest,
    DescribeCluster(DescribeClusterRequest),
    /// DescribeProducersRequest,
    DescribeProducers(DescribeProducersRequest),
    /// BrokerRegistrationRequest,
    BrokerRegistration(BrokerRegistrationRequest),
    /// BrokerHeartbeatRequest,
    BrokerHeartbeat(BrokerHeartbeatRequest),
    /// UnregisterBrokerRequest,
    UnregisterBroker(UnregisterBrokerRequest),
    /// DescribeTransactionsRequest,
    DescribeTransactions(DescribeTransactionsRequest),
    /// ListTransactionsRequest,
    ListTransactions(ListTransactionsRequest),
    /// AllocateProducerIdsRequest,
    AllocateProducerIds(AllocateProducerIdsRequest),
    /// ConsumerGroupHeartbeatRequest,
    ConsumerGroupHeartbeat(ConsumerGroupHeartbeatRequest),
    /// ControllerRegistrationRequest,
    ControllerRegistration(ControllerRegistrationRequest),
    /// GetTelemetrySubscriptionsRequest,
    GetTelemetrySubscriptions(GetTelemetrySubscriptionsRequest),
    /// PushTelemetryRequest,
    PushTelemetry(PushTelemetryRequest),
    /// AssignReplicasToDirsRequest,
    AssignReplicasToDirs(AssignReplicasToDirsRequest),
    /// ListClientMetricsResourcesRequest,
    ListClientMetricsResources(ListClientMetricsResourcesRequest),
}

impl From<ProduceRequest> for RequestKind {
    fn from(value: ProduceRequest) -> RequestKind {
        RequestKind::Produce(value)
    }
}

impl From<FetchRequest> for RequestKind {
    fn from(value: FetchRequest) -> RequestKind {
        RequestKind::Fetch(value)
    }
}

impl From<ListOffsetsRequest> for RequestKind {
    fn from(value: ListOffsetsRequest) -> RequestKind {
        RequestKind::ListOffsets(value)
    }
}

impl From<MetadataRequest> for RequestKind {
    fn from(value: MetadataRequest) -> RequestKind {
        RequestKind::Metadata(value)
    }
}

impl From<LeaderAndIsrRequest> for RequestKind {
    fn from(value: LeaderAndIsrRequest) -> RequestKind {
        RequestKind::LeaderAndIsr(value)
    }
}

impl From<StopReplicaRequest> for RequestKind {
    fn from(value: StopReplicaRequest) -> RequestKind {
        RequestKind::StopReplica(value)
    }
}

impl From<UpdateMetadataRequest> for RequestKind {
    fn from(value: UpdateMetadataRequest) -> RequestKind {
        RequestKind::UpdateMetadata(value)
    }
}

impl From<ControlledShutdownRequest> for RequestKind {
    fn from(value: ControlledShutdownRequest) -> RequestKind {
        RequestKind::ControlledShutdown(value)
    }
}

impl From<OffsetCommitRequest> for RequestKind {
    fn from(value: OffsetCommitRequest) -> RequestKind {
        RequestKind::OffsetCommit(value)
    }
}

impl From<OffsetFetchRequest> for RequestKind {
    fn from(value: OffsetFetchRequest) -> RequestKind {
        RequestKind::OffsetFetch(value)
    }
}

impl From<FindCoordinatorRequest> for RequestKind {
    fn from(value: FindCoordinatorRequest) -> RequestKind {
        RequestKind::FindCoordinator(value)
    }
}

impl From<JoinGroupRequest> for RequestKind {
    fn from(value: JoinGroupRequest) -> RequestKind {
        RequestKind::JoinGroup(value)
    }
}

impl From<HeartbeatRequest> for RequestKind {
    fn from(value: HeartbeatRequest) -> RequestKind {
        RequestKind::Heartbeat(value)
    }
}

impl From<LeaveGroupRequest> for RequestKind {
    fn from(value: LeaveGroupRequest) -> RequestKind {
        RequestKind::LeaveGroup(value)
    }
}

impl From<SyncGroupRequest> for RequestKind {
    fn from(value: SyncGroupRequest) -> RequestKind {
        RequestKind::SyncGroup(value)
    }
}

impl From<DescribeGroupsRequest> for RequestKind {
    fn from(value: DescribeGroupsRequest) -> RequestKind {
        RequestKind::DescribeGroups(value)
    }
}

impl From<ListGroupsRequest> for RequestKind {
    fn from(value: ListGroupsRequest) -> RequestKind {
        RequestKind::ListGroups(value)
    }
}

impl From<SaslHandshakeRequest> for RequestKind {
    fn from(value: SaslHandshakeRequest) -> RequestKind {
        RequestKind::SaslHandshake(value)
    }
}

impl From<ApiVersionsRequest> for RequestKind {
    fn from(value: ApiVersionsRequest) -> RequestKind {
        RequestKind::ApiVersions(value)
    }
}

impl From<CreateTopicsRequest> for RequestKind {
    fn from(value: CreateTopicsRequest) -> RequestKind {
        RequestKind::CreateTopics(value)
    }
}

impl From<DeleteTopicsRequest> for RequestKind {
    fn from(value: DeleteTopicsRequest) -> RequestKind {
        RequestKind::DeleteTopics(value)
    }
}

impl From<DeleteRecordsRequest> for RequestKind {
    fn from(value: DeleteRecordsRequest) -> RequestKind {
        RequestKind::DeleteRecords(value)
    }
}

impl From<InitProducerIdRequest> for RequestKind {
    fn from(value: InitProducerIdRequest) -> RequestKind {
        RequestKind::InitProducerId(value)
    }
}

impl From<OffsetForLeaderEpochRequest> for RequestKind {
    fn from(value: OffsetForLeaderEpochRequest) -> RequestKind {
        RequestKind::OffsetForLeaderEpoch(value)
    }
}

impl From<AddPartitionsToTxnRequest> for RequestKind {
    fn from(value: AddPartitionsToTxnRequest) -> RequestKind {
        RequestKind::AddPartitionsToTxn(value)
    }
}

impl From<AddOffsetsToTxnRequest> for RequestKind {
    fn from(value: AddOffsetsToTxnRequest) -> RequestKind {
        RequestKind::AddOffsetsToTxn(value)
    }
}

impl From<EndTxnRequest> for RequestKind {
    fn from(value: EndTxnRequest) -> RequestKind {
        RequestKind::EndTxn(value)
    }
}

impl From<WriteTxnMarkersRequest> for RequestKind {
    fn from(value: WriteTxnMarkersRequest) -> RequestKind {
        RequestKind::WriteTxnMarkers(value)
    }
}

impl From<TxnOffsetCommitRequest> for RequestKind {
    fn from(value: TxnOffsetCommitRequest) -> RequestKind {
        RequestKind::TxnOffsetCommit(value)
    }
}

impl From<DescribeAclsRequest> for RequestKind {
    fn from(value: DescribeAclsRequest) -> RequestKind {
        RequestKind::DescribeAcls(value)
    }
}

impl From<CreateAclsRequest> for RequestKind {
    fn from(value: CreateAclsRequest) -> RequestKind {
        RequestKind::CreateAcls(value)
    }
}

impl From<DeleteAclsRequest> for RequestKind {
    fn from(value: DeleteAclsRequest) -> RequestKind {
        RequestKind::DeleteAcls(value)
    }
}

impl From<DescribeConfigsRequest> for RequestKind {
    fn from(value: DescribeConfigsRequest) -> RequestKind {
        RequestKind::DescribeConfigs(value)
    }
}

impl From<AlterConfigsRequest> for RequestKind {
    fn from(value: AlterConfigsRequest) -> RequestKind {
        RequestKind::AlterConfigs(value)
    }
}

impl From<AlterReplicaLogDirsRequest> for RequestKind {
    fn from(value: AlterReplicaLogDirsRequest) -> RequestKind {
        RequestKind::AlterReplicaLogDirs(value)
    }
}

impl From<DescribeLogDirsRequest> for RequestKind {
    fn from(value: DescribeLogDirsRequest) -> RequestKind {
        RequestKind::DescribeLogDirs(value)
    }
}

impl From<SaslAuthenticateRequest> for RequestKind {
    fn from(value: SaslAuthenticateRequest) -> RequestKind {
        RequestKind::SaslAuthenticate(value)
    }
}

impl From<CreatePartitionsRequest> for RequestKind {
    fn from(value: CreatePartitionsRequest) -> RequestKind {
        RequestKind::CreatePartitions(value)
    }
}

impl From<CreateDelegationTokenRequest> for RequestKind {
    fn from(value: CreateDelegationTokenRequest) -> RequestKind {
        RequestKind::CreateDelegationToken(value)
    }
}

impl From<RenewDelegationTokenRequest> for RequestKind {
    fn from(value: RenewDelegationTokenRequest) -> RequestKind {
        RequestKind::RenewDelegationToken(value)
    }
}

impl From<ExpireDelegationTokenRequest> for RequestKind {
    fn from(value: ExpireDelegationTokenRequest) -> RequestKind {
        RequestKind::ExpireDelegationToken(value)
    }
}

impl From<DescribeDelegationTokenRequest> for RequestKind {
    fn from(value: DescribeDelegationTokenRequest) -> RequestKind {
        RequestKind::DescribeDelegationToken(value)
    }
}

impl From<DeleteGroupsRequest> for RequestKind {
    fn from(value: DeleteGroupsRequest) -> RequestKind {
        RequestKind::DeleteGroups(value)
    }
}

impl From<ElectLeadersRequest> for RequestKind {
    fn from(value: ElectLeadersRequest) -> RequestKind {
        RequestKind::ElectLeaders(value)
    }
}

impl From<IncrementalAlterConfigsRequest> for RequestKind {
    fn from(value: IncrementalAlterConfigsRequest) -> RequestKind {
        RequestKind::IncrementalAlterConfigs(value)
    }
}

impl From<AlterPartitionReassignmentsRequest> for RequestKind {
    fn from(value: AlterPartitionReassignmentsRequest) -> RequestKind {
        RequestKind::AlterPartitionReassignments(value)
    }
}

impl From<ListPartitionReassignmentsRequest> for RequestKind {
    fn from(value: ListPartitionReassignmentsRequest) -> RequestKind {
        RequestKind::ListPartitionReassignments(value)
    }
}

impl From<OffsetDeleteRequest> for RequestKind {
    fn from(value: OffsetDeleteRequest) -> RequestKind {
        RequestKind::OffsetDelete(value)
    }
}

impl From<DescribeClientQuotasRequest> for RequestKind {
    fn from(value: DescribeClientQuotasRequest) -> RequestKind {
        RequestKind::DescribeClientQuotas(value)
    }
}

impl From<AlterClientQuotasRequest> for RequestKind {
    fn from(value: AlterClientQuotasRequest) -> RequestKind {
        RequestKind::AlterClientQuotas(value)
    }
}

impl From<DescribeUserScramCredentialsRequest> for RequestKind {
    fn from(value: DescribeUserScramCredentialsRequest) -> RequestKind {
        RequestKind::DescribeUserScramCredentials(value)
    }
}

impl From<AlterUserScramCredentialsRequest> for RequestKind {
    fn from(value: AlterUserScramCredentialsRequest) -> RequestKind {
        RequestKind::AlterUserScramCredentials(value)
    }
}

impl From<VoteRequest> for RequestKind {
    fn from(value: VoteRequest) -> RequestKind {
        RequestKind::Vote(value)
    }
}

impl From<BeginQuorumEpochRequest> for RequestKind {
    fn from(value: BeginQuorumEpochRequest) -> RequestKind {
        RequestKind::BeginQuorumEpoch(value)
    }
}

impl From<EndQuorumEpochRequest> for RequestKind {
    fn from(value: EndQuorumEpochRequest) -> RequestKind {
        RequestKind::EndQuorumEpoch(value)
    }
}

impl From<DescribeQuorumRequest> for RequestKind {
    fn from(value: DescribeQuorumRequest) -> RequestKind {
        RequestKind::DescribeQuorum(value)
    }
}

impl From<AlterPartitionRequest> for RequestKind {
    fn from(value: AlterPartitionRequest) -> RequestKind {
        RequestKind::AlterPartition(value)
    }
}

impl From<UpdateFeaturesRequest> for RequestKind {
    fn from(value: UpdateFeaturesRequest) -> RequestKind {
        RequestKind::UpdateFeatures(value)
    }
}

impl From<EnvelopeRequest> for RequestKind {
    fn from(value: EnvelopeRequest) -> RequestKind {
        RequestKind::Envelope(value)
    }
}

impl From<FetchSnapshotRequest> for RequestKind {
    fn from(value: FetchSnapshotRequest) -> RequestKind {
        RequestKind::FetchSnapshot(value)
    }
}

impl From<DescribeClusterRequest> for RequestKind {
    fn from(value: DescribeClusterRequest) -> RequestKind {
        RequestKind::DescribeCluster(value)
    }
}

impl From<DescribeProducersRequest> for RequestKind {
    fn from(value: DescribeProducersRequest) -> RequestKind {
        RequestKind::DescribeProducers(value)
    }
}

impl From<BrokerRegistrationRequest> for RequestKind {
    fn from(value: BrokerRegistrationRequest) -> RequestKind {
        RequestKind::BrokerRegistration(value)
    }
}

impl From<BrokerHeartbeatRequest> for RequestKind {
    fn from(value: BrokerHeartbeatRequest) -> RequestKind {
        RequestKind::BrokerHeartbeat(value)
    }
}

impl From<UnregisterBrokerRequest> for RequestKind {
    fn from(value: UnregisterBrokerRequest) -> RequestKind {
        RequestKind::UnregisterBroker(value)
    }
}

impl From<DescribeTransactionsRequest> for RequestKind {
    fn from(value: DescribeTransactionsRequest) -> RequestKind {
        RequestKind::DescribeTransactions(value)
    }
}

impl From<ListTransactionsRequest> for RequestKind {
    fn from(value: ListTransactionsRequest) -> RequestKind {
        RequestKind::ListTransactions(value)
    }
}

impl From<AllocateProducerIdsRequest> for RequestKind {
    fn from(value: AllocateProducerIdsRequest) -> RequestKind {
        RequestKind::AllocateProducerIds(value)
    }
}

impl From<ConsumerGroupHeartbeatRequest> for RequestKind {
    fn from(value: ConsumerGroupHeartbeatRequest) -> RequestKind {
        RequestKind::ConsumerGroupHeartbeat(value)
    }
}

impl From<ControllerRegistrationRequest> for RequestKind {
    fn from(value: ControllerRegistrationRequest) -> RequestKind {
        RequestKind::ControllerRegistration(value)
    }
}

impl From<GetTelemetrySubscriptionsRequest> for RequestKind {
    fn from(value: GetTelemetrySubscriptionsRequest) -> RequestKind {
        RequestKind::GetTelemetrySubscriptions(value)
    }
}

impl From<PushTelemetryRequest> for RequestKind {
    fn from(value: PushTelemetryRequest) -> RequestKind {
        RequestKind::PushTelemetry(value)
    }
}

impl From<AssignReplicasToDirsRequest> for RequestKind {
    fn from(value: AssignReplicasToDirsRequest) -> RequestKind {
        RequestKind::AssignReplicasToDirs(value)
    }
}

impl From<ListClientMetricsResourcesRequest> for RequestKind {
    fn from(value: ListClientMetricsResourcesRequest) -> RequestKind {
        RequestKind::ListClientMetricsResources(value)
    }
}

/// Wrapping enum for all responses in the Kafka protocol.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseKind {
    /// ProduceResponse,
    Produce(ProduceResponse),
    /// FetchResponse,
    Fetch(FetchResponse),
    /// ListOffsetsResponse,
    ListOffsets(ListOffsetsResponse),
    /// MetadataResponse,
    Metadata(MetadataResponse),
    /// LeaderAndIsrResponse,
    LeaderAndIsr(LeaderAndIsrResponse),
    /// StopReplicaResponse,
    StopReplica(StopReplicaResponse),
    /// UpdateMetadataResponse,
    UpdateMetadata(UpdateMetadataResponse),
    /// ControlledShutdownResponse,
    ControlledShutdown(ControlledShutdownResponse),
    /// OffsetCommitResponse,
    OffsetCommit(OffsetCommitResponse),
    /// OffsetFetchResponse,
    OffsetFetch(OffsetFetchResponse),
    /// FindCoordinatorResponse,
    FindCoordinator(FindCoordinatorResponse),
    /// JoinGroupResponse,
    JoinGroup(JoinGroupResponse),
    /// HeartbeatResponse,
    Heartbeat(HeartbeatResponse),
    /// LeaveGroupResponse,
    LeaveGroup(LeaveGroupResponse),
    /// SyncGroupResponse,
    SyncGroup(SyncGroupResponse),
    /// DescribeGroupsResponse,
    DescribeGroups(DescribeGroupsResponse),
    /// ListGroupsResponse,
    ListGroups(ListGroupsResponse),
    /// SaslHandshakeResponse,
    SaslHandshake(SaslHandshakeResponse),
    /// ApiVersionsResponse,
    ApiVersions(ApiVersionsResponse),
    /// CreateTopicsResponse,
    CreateTopics(CreateTopicsResponse),
    /// DeleteTopicsResponse,
    DeleteTopics(DeleteTopicsResponse),
    /// DeleteRecordsResponse,
    DeleteRecords(DeleteRecordsResponse),
    /// InitProducerIdResponse,
    InitProducerId(InitProducerIdResponse),
    /// OffsetForLeaderEpochResponse,
    OffsetForLeaderEpoch(OffsetForLeaderEpochResponse),
    /// AddPartitionsToTxnResponse,
    AddPartitionsToTxn(AddPartitionsToTxnResponse),
    /// AddOffsetsToTxnResponse,
    AddOffsetsToTxn(AddOffsetsToTxnResponse),
    /// EndTxnResponse,
    EndTxn(EndTxnResponse),
    /// WriteTxnMarkersResponse,
    WriteTxnMarkers(WriteTxnMarkersResponse),
    /// TxnOffsetCommitResponse,
    TxnOffsetCommit(TxnOffsetCommitResponse),
    /// DescribeAclsResponse,
    DescribeAcls(DescribeAclsResponse),
    /// CreateAclsResponse,
    CreateAcls(CreateAclsResponse),
    /// DeleteAclsResponse,
    DeleteAcls(DeleteAclsResponse),
    /// DescribeConfigsResponse,
    DescribeConfigs(DescribeConfigsResponse),
    /// AlterConfigsResponse,
    AlterConfigs(AlterConfigsResponse),
    /// AlterReplicaLogDirsResponse,
    AlterReplicaLogDirs(AlterReplicaLogDirsResponse),
    /// DescribeLogDirsResponse,
    DescribeLogDirs(DescribeLogDirsResponse),
    /// SaslAuthenticateResponse,
    SaslAuthenticate(SaslAuthenticateResponse),
    /// CreatePartitionsResponse,
    CreatePartitions(CreatePartitionsResponse),
    /// CreateDelegationTokenResponse,
    CreateDelegationToken(CreateDelegationTokenResponse),
    /// RenewDelegationTokenResponse,
    RenewDelegationToken(RenewDelegationTokenResponse),
    /// ExpireDelegationTokenResponse,
    ExpireDelegationToken(ExpireDelegationTokenResponse),
    /// DescribeDelegationTokenResponse,
    DescribeDelegationToken(DescribeDelegationTokenResponse),
    /// DeleteGroupsResponse,
    DeleteGroups(DeleteGroupsResponse),
    /// ElectLeadersResponse,
    ElectLeaders(ElectLeadersResponse),
    /// IncrementalAlterConfigsResponse,
    IncrementalAlterConfigs(IncrementalAlterConfigsResponse),
    /// AlterPartitionReassignmentsResponse,
    AlterPartitionReassignments(AlterPartitionReassignmentsResponse),
    /// ListPartitionReassignmentsResponse,
    ListPartitionReassignments(ListPartitionReassignmentsResponse),
    /// OffsetDeleteResponse,
    OffsetDelete(OffsetDeleteResponse),
    /// DescribeClientQuotasResponse,
    DescribeClientQuotas(DescribeClientQuotasResponse),
    /// AlterClientQuotasResponse,
    AlterClientQuotas(AlterClientQuotasResponse),
    /// DescribeUserScramCredentialsResponse,
    DescribeUserScramCredentials(DescribeUserScramCredentialsResponse),
    /// AlterUserScramCredentialsResponse,
    AlterUserScramCredentials(AlterUserScramCredentialsResponse),
    /// VoteResponse,
    Vote(VoteResponse),
    /// BeginQuorumEpochResponse,
    BeginQuorumEpoch(BeginQuorumEpochResponse),
    /// EndQuorumEpochResponse,
    EndQuorumEpoch(EndQuorumEpochResponse),
    /// DescribeQuorumResponse,
    DescribeQuorum(DescribeQuorumResponse),
    /// AlterPartitionResponse,
    AlterPartition(AlterPartitionResponse),
    /// UpdateFeaturesResponse,
    UpdateFeatures(UpdateFeaturesResponse),
    /// EnvelopeResponse,
    Envelope(EnvelopeResponse),
    /// FetchSnapshotResponse,
    FetchSnapshot(FetchSnapshotResponse),
    /// DescribeClusterResponse,
    DescribeCluster(DescribeClusterResponse),
    /// DescribeProducersResponse,
    DescribeProducers(DescribeProducersResponse),
    /// BrokerRegistrationResponse,
    BrokerRegistration(BrokerRegistrationResponse),
    /// BrokerHeartbeatResponse,
    BrokerHeartbeat(BrokerHeartbeatResponse),
    /// UnregisterBrokerResponse,
    UnregisterBroker(UnregisterBrokerResponse),
    /// DescribeTransactionsResponse,
    DescribeTransactions(DescribeTransactionsResponse),
    /// ListTransactionsResponse,
    ListTransactions(ListTransactionsResponse),
    /// AllocateProducerIdsResponse,
    AllocateProducerIds(AllocateProducerIdsResponse),
    /// ConsumerGroupHeartbeatResponse,
    ConsumerGroupHeartbeat(ConsumerGroupHeartbeatResponse),
    /// ControllerRegistrationResponse,
    ControllerRegistration(ControllerRegistrationResponse),
    /// GetTelemetrySubscriptionsResponse,
    GetTelemetrySubscriptions(GetTelemetrySubscriptionsResponse),
    /// PushTelemetryResponse,
    PushTelemetry(PushTelemetryResponse),
    /// AssignReplicasToDirsResponse,
    AssignReplicasToDirs(AssignReplicasToDirsResponse),
    /// ListClientMetricsResourcesResponse,
    ListClientMetricsResources(ListClientMetricsResourcesResponse),
}

impl From<ProduceResponse> for ResponseKind {
    fn from(value: ProduceResponse) -> ResponseKind {
        ResponseKind::Produce(value)
    }
}

impl From<FetchResponse> for ResponseKind {
    fn from(value: FetchResponse) -> ResponseKind {
        ResponseKind::Fetch(value)
    }
}

impl From<ListOffsetsResponse> for ResponseKind {
    fn from(value: ListOffsetsResponse) -> ResponseKind {
        ResponseKind::ListOffsets(value)
    }
}

impl From<MetadataResponse> for ResponseKind {
    fn from(value: MetadataResponse) -> ResponseKind {
        ResponseKind::Metadata(value)
    }
}

impl From<LeaderAndIsrResponse> for ResponseKind {
    fn from(value: LeaderAndIsrResponse) -> ResponseKind {
        ResponseKind::LeaderAndIsr(value)
    }
}

impl From<StopReplicaResponse> for ResponseKind {
    fn from(value: StopReplicaResponse) -> ResponseKind {
        ResponseKind::StopReplica(value)
    }
}

impl From<UpdateMetadataResponse> for ResponseKind {
    fn from(value: UpdateMetadataResponse) -> ResponseKind {
        ResponseKind::UpdateMetadata(value)
    }
}

impl From<ControlledShutdownResponse> for ResponseKind {
    fn from(value: ControlledShutdownResponse) -> ResponseKind {
        ResponseKind::ControlledShutdown(value)
    }
}

impl From<OffsetCommitResponse> for ResponseKind {
    fn from(value: OffsetCommitResponse) -> ResponseKind {
        ResponseKind::OffsetCommit(value)
    }
}

impl From<OffsetFetchResponse> for ResponseKind {
    fn from(value: OffsetFetchResponse) -> ResponseKind {
        ResponseKind::OffsetFetch(value)
    }
}

impl From<FindCoordinatorResponse> for ResponseKind {
    fn from(value: FindCoordinatorResponse) -> ResponseKind {
        ResponseKind::FindCoordinator(value)
    }
}

impl From<JoinGroupResponse> for ResponseKind {
    fn from(value: JoinGroupResponse) -> ResponseKind {
        ResponseKind::JoinGroup(value)
    }
}

impl From<HeartbeatResponse> for ResponseKind {
    fn from(value: HeartbeatResponse) -> ResponseKind {
        ResponseKind::Heartbeat(value)
    }
}

impl From<LeaveGroupResponse> for ResponseKind {
    fn from(value: LeaveGroupResponse) -> ResponseKind {
        ResponseKind::LeaveGroup(value)
    }
}

impl From<SyncGroupResponse> for ResponseKind {
    fn from(value: SyncGroupResponse) -> ResponseKind {
        ResponseKind::SyncGroup(value)
    }
}

impl From<DescribeGroupsResponse> for ResponseKind {
    fn from(value: DescribeGroupsResponse) -> ResponseKind {
        ResponseKind::DescribeGroups(value)
    }
}

impl From<ListGroupsResponse> for ResponseKind {
    fn from(value: ListGroupsResponse) -> ResponseKind {
        ResponseKind::ListGroups(value)
    }
}

impl From<SaslHandshakeResponse> for ResponseKind {
    fn from(value: SaslHandshakeResponse) -> ResponseKind {
        ResponseKind::SaslHandshake(value)
    }
}

impl From<ApiVersionsResponse> for ResponseKind {
    fn from(value: ApiVersionsResponse) -> ResponseKind {
        ResponseKind::ApiVersions(value)
    }
}

impl From<CreateTopicsResponse> for ResponseKind {
    fn from(value: CreateTopicsResponse) -> ResponseKind {
        ResponseKind::CreateTopics(value)
    }
}

impl From<DeleteTopicsResponse> for ResponseKind {
    fn from(value: DeleteTopicsResponse) -> ResponseKind {
        ResponseKind::DeleteTopics(value)
    }
}

impl From<DeleteRecordsResponse> for ResponseKind {
    fn from(value: DeleteRecordsResponse) -> ResponseKind {
        ResponseKind::DeleteRecords(value)
    }
}

impl From<InitProducerIdResponse> for ResponseKind {
    fn from(value: InitProducerIdResponse) -> ResponseKind {
        ResponseKind::InitProducerId(value)
    }
}

impl From<OffsetForLeaderEpochResponse> for ResponseKind {
    fn from(value: OffsetForLeaderEpochResponse) -> ResponseKind {
        ResponseKind::OffsetForLeaderEpoch(value)
    }
}

impl From<AddPartitionsToTxnResponse> for ResponseKind {
    fn from(value: AddPartitionsToTxnResponse) -> ResponseKind {
        ResponseKind::AddPartitionsToTxn(value)
    }
}

impl From<AddOffsetsToTxnResponse> for ResponseKind {
    fn from(value: AddOffsetsToTxnResponse) -> ResponseKind {
        ResponseKind::AddOffsetsToTxn(value)
    }
}

impl From<EndTxnResponse> for ResponseKind {
    fn from(value: EndTxnResponse) -> ResponseKind {
        ResponseKind::EndTxn(value)
    }
}

impl From<WriteTxnMarkersResponse> for ResponseKind {
    fn from(value: WriteTxnMarkersResponse) -> ResponseKind {
        ResponseKind::WriteTxnMarkers(value)
    }
}

impl From<TxnOffsetCommitResponse> for ResponseKind {
    fn from(value: TxnOffsetCommitResponse) -> ResponseKind {
        ResponseKind::TxnOffsetCommit(value)
    }
}

impl From<DescribeAclsResponse> for ResponseKind {
    fn from(value: DescribeAclsResponse) -> ResponseKind {
        ResponseKind::DescribeAcls(value)
    }
}

impl From<CreateAclsResponse> for ResponseKind {
    fn from(value: CreateAclsResponse) -> ResponseKind {
        ResponseKind::CreateAcls(value)
    }
}

impl From<DeleteAclsResponse> for ResponseKind {
    fn from(value: DeleteAclsResponse) -> ResponseKind {
        ResponseKind::DeleteAcls(value)
    }
}

impl From<DescribeConfigsResponse> for ResponseKind {
    fn from(value: DescribeConfigsResponse) -> ResponseKind {
        ResponseKind::DescribeConfigs(value)
    }
}

impl From<AlterConfigsResponse> for ResponseKind {
    fn from(value: AlterConfigsResponse) -> ResponseKind {
        ResponseKind::AlterConfigs(value)
    }
}

impl From<AlterReplicaLogDirsResponse> for ResponseKind {
    fn from(value: AlterReplicaLogDirsResponse) -> ResponseKind {
        ResponseKind::AlterReplicaLogDirs(value)
    }
}

impl From<DescribeLogDirsResponse> for ResponseKind {
    fn from(value: DescribeLogDirsResponse) -> ResponseKind {
        ResponseKind::DescribeLogDirs(value)
    }
}

impl From<SaslAuthenticateResponse> for ResponseKind {
    fn from(value: SaslAuthenticateResponse) -> ResponseKind {
        ResponseKind::SaslAuthenticate(value)
    }
}

impl From<CreatePartitionsResponse> for ResponseKind {
    fn from(value: CreatePartitionsResponse) -> ResponseKind {
        ResponseKind::CreatePartitions(value)
    }
}

impl From<CreateDelegationTokenResponse> for ResponseKind {
    fn from(value: CreateDelegationTokenResponse) -> ResponseKind {
        ResponseKind::CreateDelegationToken(value)
    }
}

impl From<RenewDelegationTokenResponse> for ResponseKind {
    fn from(value: RenewDelegationTokenResponse) -> ResponseKind {
        ResponseKind::RenewDelegationToken(value)
    }
}

impl From<ExpireDelegationTokenResponse> for ResponseKind {
    fn from(value: ExpireDelegationTokenResponse) -> ResponseKind {
        ResponseKind::ExpireDelegationToken(value)
    }
}

impl From<DescribeDelegationTokenResponse> for ResponseKind {
    fn from(value: DescribeDelegationTokenResponse) -> ResponseKind {
        ResponseKind::DescribeDelegationToken(value)
    }
}

impl From<DeleteGroupsResponse> for ResponseKind {
    fn from(value: DeleteGroupsResponse) -> ResponseKind {
        ResponseKind::DeleteGroups(value)
    }
}

impl From<ElectLeadersResponse> for ResponseKind {
    fn from(value: ElectLeadersResponse) -> ResponseKind {
        ResponseKind::ElectLeaders(value)
    }
}

impl From<IncrementalAlterConfigsResponse> for ResponseKind {
    fn from(value: IncrementalAlterConfigsResponse) -> ResponseKind {
        ResponseKind::IncrementalAlterConfigs(value)
    }
}

impl From<AlterPartitionReassignmentsResponse> for ResponseKind {
    fn from(value: AlterPartitionReassignmentsResponse) -> ResponseKind {
        ResponseKind::AlterPartitionReassignments(value)
    }
}

impl From<ListPartitionReassignmentsResponse> for ResponseKind {
    fn from(value: ListPartitionReassignmentsResponse) -> ResponseKind {
        ResponseKind::ListPartitionReassignments(value)
    }
}

impl From<OffsetDeleteResponse> for ResponseKind {
    fn from(value: OffsetDeleteResponse) -> ResponseKind {
        ResponseKind::OffsetDelete(value)
    }
}

impl From<DescribeClientQuotasResponse> for ResponseKind {
    fn from(value: DescribeClientQuotasResponse) -> ResponseKind {
        ResponseKind::DescribeClientQuotas(value)
    }
}

impl From<AlterClientQuotasResponse> for ResponseKind {
    fn from(value: AlterClientQuotasResponse) -> ResponseKind {
        ResponseKind::AlterClientQuotas(value)
    }
}

impl From<DescribeUserScramCredentialsResponse> for ResponseKind {
    fn from(value: DescribeUserScramCredentialsResponse) -> ResponseKind {
        ResponseKind::DescribeUserScramCredentials(value)
    }
}

impl From<AlterUserScramCredentialsResponse> for ResponseKind {
    fn from(value: AlterUserScramCredentialsResponse) -> ResponseKind {
        ResponseKind::AlterUserScramCredentials(value)
    }
}

impl From<VoteResponse> for ResponseKind {
    fn from(value: VoteResponse) -> ResponseKind {
        ResponseKind::Vote(value)
    }
}

impl From<BeginQuorumEpochResponse> for ResponseKind {
    fn from(value: BeginQuorumEpochResponse) -> ResponseKind {
        ResponseKind::BeginQuorumEpoch(value)
    }
}

impl From<EndQuorumEpochResponse> for ResponseKind {
    fn from(value: EndQuorumEpochResponse) -> ResponseKind {
        ResponseKind::EndQuorumEpoch(value)
    }
}

impl From<DescribeQuorumResponse> for ResponseKind {
    fn from(value: DescribeQuorumResponse) -> ResponseKind {
        ResponseKind::DescribeQuorum(value)
    }
}

impl From<AlterPartitionResponse> for ResponseKind {
    fn from(value: AlterPartitionResponse) -> ResponseKind {
        ResponseKind::AlterPartition(value)
    }
}

impl From<UpdateFeaturesResponse> for ResponseKind {
    fn from(value: UpdateFeaturesResponse) -> ResponseKind {
        ResponseKind::UpdateFeatures(value)
    }
}

impl From<EnvelopeResponse> for ResponseKind {
    fn from(value: EnvelopeResponse) -> ResponseKind {
        ResponseKind::Envelope(value)
    }
}

impl From<FetchSnapshotResponse> for ResponseKind {
    fn from(value: FetchSnapshotResponse) -> ResponseKind {
        ResponseKind::FetchSnapshot(value)
    }
}

impl From<DescribeClusterResponse> for ResponseKind {
    fn from(value: DescribeClusterResponse) -> ResponseKind {
        ResponseKind::DescribeCluster(value)
    }
}

impl From<DescribeProducersResponse> for ResponseKind {
    fn from(value: DescribeProducersResponse) -> ResponseKind {
        ResponseKind::DescribeProducers(value)
    }
}

impl From<BrokerRegistrationResponse> for ResponseKind {
    fn from(value: BrokerRegistrationResponse) -> ResponseKind {
        ResponseKind::BrokerRegistration(value)
    }
}

impl From<BrokerHeartbeatResponse> for ResponseKind {
    fn from(value: BrokerHeartbeatResponse) -> ResponseKind {
        ResponseKind::BrokerHeartbeat(value)
    }
}

impl From<UnregisterBrokerResponse> for ResponseKind {
    fn from(value: UnregisterBrokerResponse) -> ResponseKind {
        ResponseKind::UnregisterBroker(value)
    }
}

impl From<DescribeTransactionsResponse> for ResponseKind {
    fn from(value: DescribeTransactionsResponse) -> ResponseKind {
        ResponseKind::DescribeTransactions(value)
    }
}

impl From<ListTransactionsResponse> for ResponseKind {
    fn from(value: ListTransactionsResponse) -> ResponseKind {
        ResponseKind::ListTransactions(value)
    }
}

impl From<AllocateProducerIdsResponse> for ResponseKind {
    fn from(value: AllocateProducerIdsResponse) -> ResponseKind {
        ResponseKind::AllocateProducerIds(value)
    }
}

impl From<ConsumerGroupHeartbeatResponse> for ResponseKind {
    fn from(value: ConsumerGroupHeartbeatResponse) -> ResponseKind {
        ResponseKind::ConsumerGroupHeartbeat(value)
    }
}

impl From<ControllerRegistrationResponse> for ResponseKind {
    fn from(value: ControllerRegistrationResponse) -> ResponseKind {
        ResponseKind::ControllerRegistration(value)
    }
}

impl From<GetTelemetrySubscriptionsResponse> for ResponseKind {
    fn from(value: GetTelemetrySubscriptionsResponse) -> ResponseKind {
        ResponseKind::GetTelemetrySubscriptions(value)
    }
}

impl From<PushTelemetryResponse> for ResponseKind {
    fn from(value: PushTelemetryResponse) -> ResponseKind {
        ResponseKind::PushTelemetry(value)
    }
}

impl From<AssignReplicasToDirsResponse> for ResponseKind {
    fn from(value: AssignReplicasToDirsResponse) -> ResponseKind {
        ResponseKind::AssignReplicasToDirs(value)
    }
}

impl From<ListClientMetricsResourcesResponse> for ResponseKind {
    fn from(value: ListClientMetricsResourcesResponse) -> ResponseKind {
        ResponseKind::ListClientMetricsResources(value)
    }
}

/// The ID of the controller broker.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy)]
pub struct BrokerId(pub i32);

impl From<i32> for BrokerId {
    fn from(other: i32) -> Self {
        Self(other)
    }
}
impl From<BrokerId> for i32 {
    fn from(other: BrokerId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<i32> for BrokerId {
    fn borrow(&self) -> &i32 {
        &self.0
    }
}
impl std::ops::Deref for BrokerId {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<i32> for BrokerId {
    fn eq(&self, other: &i32) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<BrokerId> for i32 {
    fn eq(&self, other: &BrokerId) -> bool {
        self == &other.0
    }
}
impl NewType<i32> for BrokerId {}

/// The group id
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct GroupId(pub StrBytes);

impl From<StrBytes> for GroupId {
    fn from(other: StrBytes) -> Self {
        Self(other)
    }
}
impl From<GroupId> for StrBytes {
    fn from(other: GroupId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<StrBytes> for GroupId {
    fn borrow(&self) -> &StrBytes {
        &self.0
    }
}
impl std::ops::Deref for GroupId {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<StrBytes> for GroupId {
    fn eq(&self, other: &StrBytes) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<GroupId> for StrBytes {
    fn eq(&self, other: &GroupId) -> bool {
        self == &other.0
    }
}
impl NewType<StrBytes> for GroupId {}

/// The first producer ID in this range, inclusive
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy)]
pub struct ProducerId(pub i64);

impl From<i64> for ProducerId {
    fn from(other: i64) -> Self {
        Self(other)
    }
}
impl From<ProducerId> for i64 {
    fn from(other: ProducerId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<i64> for ProducerId {
    fn borrow(&self) -> &i64 {
        &self.0
    }
}
impl std::ops::Deref for ProducerId {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<i64> for ProducerId {
    fn eq(&self, other: &i64) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<ProducerId> for i64 {
    fn eq(&self, other: &ProducerId) -> bool {
        self == &other.0
    }
}
impl NewType<i64> for ProducerId {}

///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TopicName(pub StrBytes);

impl From<StrBytes> for TopicName {
    fn from(other: StrBytes) -> Self {
        Self(other)
    }
}
impl From<TopicName> for StrBytes {
    fn from(other: TopicName) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<StrBytes> for TopicName {
    fn borrow(&self) -> &StrBytes {
        &self.0
    }
}
impl std::ops::Deref for TopicName {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<StrBytes> for TopicName {
    fn eq(&self, other: &StrBytes) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<TopicName> for StrBytes {
    fn eq(&self, other: &TopicName) -> bool {
        self == &other.0
    }
}
impl NewType<StrBytes> for TopicName {}

///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TransactionalId(pub StrBytes);

impl From<StrBytes> for TransactionalId {
    fn from(other: StrBytes) -> Self {
        Self(other)
    }
}
impl From<TransactionalId> for StrBytes {
    fn from(other: TransactionalId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<StrBytes> for TransactionalId {
    fn borrow(&self) -> &StrBytes {
        &self.0
    }
}
impl std::ops::Deref for TransactionalId {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<StrBytes> for TransactionalId {
    fn eq(&self, other: &StrBytes) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<TransactionalId> for StrBytes {
    fn eq(&self, other: &TransactionalId) -> bool {
        self == &other.0
    }
}
impl NewType<StrBytes> for TransactionalId {}
