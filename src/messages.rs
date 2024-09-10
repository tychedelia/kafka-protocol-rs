//! Messages used by the Kafka protocol.
//!
//! These messages are generated programmatically. See the [Kafka's protocol documentation](https://kafka.apache.org/protocol.html) for more information about a given message type.
// WARNING: the items of this module are generated and should not be edited directly.

#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
use crate::protocol::Decodable;
#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
use crate::protocol::Encodable;
#[cfg(all(feature = "client", feature = "broker"))]
use crate::protocol::Request;
use crate::protocol::{HeaderVersion, NewType, StrBytes};
#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
use anyhow::Context;
use anyhow::Result;
use std::convert::TryFrom;

pub mod consumer_protocol_assignment;
pub use consumer_protocol_assignment::ConsumerProtocolAssignment;

pub mod consumer_protocol_subscription;
pub use consumer_protocol_subscription::ConsumerProtocolSubscription;

pub mod default_principal_data;
pub use default_principal_data::DefaultPrincipalData;

pub mod k_raft_version_record;
pub use k_raft_version_record::KRaftVersionRecord;

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

pub mod voters_record;
pub use voters_record::VotersRecord;

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

pub mod consumer_group_describe_request;
pub use consumer_group_describe_request::ConsumerGroupDescribeRequest;

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

pub mod describe_topic_partitions_request;
pub use describe_topic_partitions_request::DescribeTopicPartitionsRequest;

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

pub mod consumer_group_describe_response;
pub use consumer_group_describe_response::ConsumerGroupDescribeResponse;

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

pub mod describe_topic_partitions_response;
pub use describe_topic_partitions_response::DescribeTopicPartitionsResponse;

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ProduceRequest {
    const KEY: i16 = 0;
    type Response = ProduceResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for FetchRequest {
    const KEY: i16 = 1;
    type Response = FetchResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ListOffsetsRequest {
    const KEY: i16 = 2;
    type Response = ListOffsetsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for MetadataRequest {
    const KEY: i16 = 3;
    type Response = MetadataResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for LeaderAndIsrRequest {
    const KEY: i16 = 4;
    type Response = LeaderAndIsrResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for StopReplicaRequest {
    const KEY: i16 = 5;
    type Response = StopReplicaResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for UpdateMetadataRequest {
    const KEY: i16 = 6;
    type Response = UpdateMetadataResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ControlledShutdownRequest {
    const KEY: i16 = 7;
    type Response = ControlledShutdownResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for OffsetCommitRequest {
    const KEY: i16 = 8;
    type Response = OffsetCommitResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for OffsetFetchRequest {
    const KEY: i16 = 9;
    type Response = OffsetFetchResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for FindCoordinatorRequest {
    const KEY: i16 = 10;
    type Response = FindCoordinatorResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for JoinGroupRequest {
    const KEY: i16 = 11;
    type Response = JoinGroupResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for HeartbeatRequest {
    const KEY: i16 = 12;
    type Response = HeartbeatResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for LeaveGroupRequest {
    const KEY: i16 = 13;
    type Response = LeaveGroupResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for SyncGroupRequest {
    const KEY: i16 = 14;
    type Response = SyncGroupResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeGroupsRequest {
    const KEY: i16 = 15;
    type Response = DescribeGroupsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ListGroupsRequest {
    const KEY: i16 = 16;
    type Response = ListGroupsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for SaslHandshakeRequest {
    const KEY: i16 = 17;
    type Response = SaslHandshakeResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ApiVersionsRequest {
    const KEY: i16 = 18;
    type Response = ApiVersionsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for CreateTopicsRequest {
    const KEY: i16 = 19;
    type Response = CreateTopicsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DeleteTopicsRequest {
    const KEY: i16 = 20;
    type Response = DeleteTopicsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DeleteRecordsRequest {
    const KEY: i16 = 21;
    type Response = DeleteRecordsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for InitProducerIdRequest {
    const KEY: i16 = 22;
    type Response = InitProducerIdResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for OffsetForLeaderEpochRequest {
    const KEY: i16 = 23;
    type Response = OffsetForLeaderEpochResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AddPartitionsToTxnRequest {
    const KEY: i16 = 24;
    type Response = AddPartitionsToTxnResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AddOffsetsToTxnRequest {
    const KEY: i16 = 25;
    type Response = AddOffsetsToTxnResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for EndTxnRequest {
    const KEY: i16 = 26;
    type Response = EndTxnResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for WriteTxnMarkersRequest {
    const KEY: i16 = 27;
    type Response = WriteTxnMarkersResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for TxnOffsetCommitRequest {
    const KEY: i16 = 28;
    type Response = TxnOffsetCommitResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeAclsRequest {
    const KEY: i16 = 29;
    type Response = DescribeAclsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for CreateAclsRequest {
    const KEY: i16 = 30;
    type Response = CreateAclsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DeleteAclsRequest {
    const KEY: i16 = 31;
    type Response = DeleteAclsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeConfigsRequest {
    const KEY: i16 = 32;
    type Response = DescribeConfigsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AlterConfigsRequest {
    const KEY: i16 = 33;
    type Response = AlterConfigsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AlterReplicaLogDirsRequest {
    const KEY: i16 = 34;
    type Response = AlterReplicaLogDirsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeLogDirsRequest {
    const KEY: i16 = 35;
    type Response = DescribeLogDirsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for SaslAuthenticateRequest {
    const KEY: i16 = 36;
    type Response = SaslAuthenticateResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for CreatePartitionsRequest {
    const KEY: i16 = 37;
    type Response = CreatePartitionsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for CreateDelegationTokenRequest {
    const KEY: i16 = 38;
    type Response = CreateDelegationTokenResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for RenewDelegationTokenRequest {
    const KEY: i16 = 39;
    type Response = RenewDelegationTokenResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ExpireDelegationTokenRequest {
    const KEY: i16 = 40;
    type Response = ExpireDelegationTokenResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeDelegationTokenRequest {
    const KEY: i16 = 41;
    type Response = DescribeDelegationTokenResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DeleteGroupsRequest {
    const KEY: i16 = 42;
    type Response = DeleteGroupsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ElectLeadersRequest {
    const KEY: i16 = 43;
    type Response = ElectLeadersResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for IncrementalAlterConfigsRequest {
    const KEY: i16 = 44;
    type Response = IncrementalAlterConfigsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AlterPartitionReassignmentsRequest {
    const KEY: i16 = 45;
    type Response = AlterPartitionReassignmentsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ListPartitionReassignmentsRequest {
    const KEY: i16 = 46;
    type Response = ListPartitionReassignmentsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for OffsetDeleteRequest {
    const KEY: i16 = 47;
    type Response = OffsetDeleteResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeClientQuotasRequest {
    const KEY: i16 = 48;
    type Response = DescribeClientQuotasResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AlterClientQuotasRequest {
    const KEY: i16 = 49;
    type Response = AlterClientQuotasResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeUserScramCredentialsRequest {
    const KEY: i16 = 50;
    type Response = DescribeUserScramCredentialsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AlterUserScramCredentialsRequest {
    const KEY: i16 = 51;
    type Response = AlterUserScramCredentialsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for VoteRequest {
    const KEY: i16 = 52;
    type Response = VoteResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for BeginQuorumEpochRequest {
    const KEY: i16 = 53;
    type Response = BeginQuorumEpochResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for EndQuorumEpochRequest {
    const KEY: i16 = 54;
    type Response = EndQuorumEpochResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeQuorumRequest {
    const KEY: i16 = 55;
    type Response = DescribeQuorumResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AlterPartitionRequest {
    const KEY: i16 = 56;
    type Response = AlterPartitionResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for UpdateFeaturesRequest {
    const KEY: i16 = 57;
    type Response = UpdateFeaturesResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for EnvelopeRequest {
    const KEY: i16 = 58;
    type Response = EnvelopeResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for FetchSnapshotRequest {
    const KEY: i16 = 59;
    type Response = FetchSnapshotResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeClusterRequest {
    const KEY: i16 = 60;
    type Response = DescribeClusterResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeProducersRequest {
    const KEY: i16 = 61;
    type Response = DescribeProducersResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for BrokerRegistrationRequest {
    const KEY: i16 = 62;
    type Response = BrokerRegistrationResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for BrokerHeartbeatRequest {
    const KEY: i16 = 63;
    type Response = BrokerHeartbeatResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for UnregisterBrokerRequest {
    const KEY: i16 = 64;
    type Response = UnregisterBrokerResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeTransactionsRequest {
    const KEY: i16 = 65;
    type Response = DescribeTransactionsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ListTransactionsRequest {
    const KEY: i16 = 66;
    type Response = ListTransactionsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AllocateProducerIdsRequest {
    const KEY: i16 = 67;
    type Response = AllocateProducerIdsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ConsumerGroupHeartbeatRequest {
    const KEY: i16 = 68;
    type Response = ConsumerGroupHeartbeatResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ConsumerGroupDescribeRequest {
    const KEY: i16 = 69;
    type Response = ConsumerGroupDescribeResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ControllerRegistrationRequest {
    const KEY: i16 = 70;
    type Response = ControllerRegistrationResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for GetTelemetrySubscriptionsRequest {
    const KEY: i16 = 71;
    type Response = GetTelemetrySubscriptionsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for PushTelemetryRequest {
    const KEY: i16 = 72;
    type Response = PushTelemetryResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for AssignReplicasToDirsRequest {
    const KEY: i16 = 73;
    type Response = AssignReplicasToDirsResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for ListClientMetricsResourcesRequest {
    const KEY: i16 = 74;
    type Response = ListClientMetricsResourcesResponse;
}

#[cfg(all(feature = "client", feature = "broker"))]
impl Request for DescribeTopicPartitionsRequest {
    const KEY: i16 = 75;
    type Response = DescribeTopicPartitionsResponse;
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
    /// API key for request ConsumerGroupDescribeRequest
    ConsumerGroupDescribeKey = 69,
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
    /// API key for request DescribeTopicPartitionsRequest
    DescribeTopicPartitionsKey = 75,
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
            ApiKey::ConsumerGroupDescribeKey => {
                ConsumerGroupDescribeRequest::header_version(version)
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
            ApiKey::DescribeTopicPartitionsKey => {
                DescribeTopicPartitionsRequest::header_version(version)
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
            ApiKey::ConsumerGroupDescribeKey => {
                ConsumerGroupDescribeResponse::header_version(version)
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
            ApiKey::DescribeTopicPartitionsKey => {
                DescribeTopicPartitionsResponse::header_version(version)
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
            x if x == ApiKey::ConsumerGroupDescribeKey as i16 => {
                Ok(ApiKey::ConsumerGroupDescribeKey)
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
            x if x == ApiKey::DescribeTopicPartitionsKey as i16 => {
                Ok(ApiKey::DescribeTopicPartitionsKey)
            }
            _ => Err(()),
        }
    }
}

/// Wrapping enum for all requests in the Kafka protocol.
#[cfg(feature = "messages_enums")]
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
    /// ConsumerGroupDescribeRequest,
    ConsumerGroupDescribe(ConsumerGroupDescribeRequest),
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
    /// DescribeTopicPartitionsRequest,
    DescribeTopicPartitions(DescribeTopicPartitionsRequest),
}

#[cfg(feature = "messages_enums")]
impl RequestKind {
    /// Encode the message into the target buffer
    #[cfg(feature = "client")]
    pub fn encode(&self, bytes: &mut bytes::BytesMut, version: i16) -> anyhow::Result<()> {
        match self {
            RequestKind::Produce(x) => encode(x, bytes, version),
            RequestKind::Fetch(x) => encode(x, bytes, version),
            RequestKind::ListOffsets(x) => encode(x, bytes, version),
            RequestKind::Metadata(x) => encode(x, bytes, version),
            RequestKind::LeaderAndIsr(x) => encode(x, bytes, version),
            RequestKind::StopReplica(x) => encode(x, bytes, version),
            RequestKind::UpdateMetadata(x) => encode(x, bytes, version),
            RequestKind::ControlledShutdown(x) => encode(x, bytes, version),
            RequestKind::OffsetCommit(x) => encode(x, bytes, version),
            RequestKind::OffsetFetch(x) => encode(x, bytes, version),
            RequestKind::FindCoordinator(x) => encode(x, bytes, version),
            RequestKind::JoinGroup(x) => encode(x, bytes, version),
            RequestKind::Heartbeat(x) => encode(x, bytes, version),
            RequestKind::LeaveGroup(x) => encode(x, bytes, version),
            RequestKind::SyncGroup(x) => encode(x, bytes, version),
            RequestKind::DescribeGroups(x) => encode(x, bytes, version),
            RequestKind::ListGroups(x) => encode(x, bytes, version),
            RequestKind::SaslHandshake(x) => encode(x, bytes, version),
            RequestKind::ApiVersions(x) => encode(x, bytes, version),
            RequestKind::CreateTopics(x) => encode(x, bytes, version),
            RequestKind::DeleteTopics(x) => encode(x, bytes, version),
            RequestKind::DeleteRecords(x) => encode(x, bytes, version),
            RequestKind::InitProducerId(x) => encode(x, bytes, version),
            RequestKind::OffsetForLeaderEpoch(x) => encode(x, bytes, version),
            RequestKind::AddPartitionsToTxn(x) => encode(x, bytes, version),
            RequestKind::AddOffsetsToTxn(x) => encode(x, bytes, version),
            RequestKind::EndTxn(x) => encode(x, bytes, version),
            RequestKind::WriteTxnMarkers(x) => encode(x, bytes, version),
            RequestKind::TxnOffsetCommit(x) => encode(x, bytes, version),
            RequestKind::DescribeAcls(x) => encode(x, bytes, version),
            RequestKind::CreateAcls(x) => encode(x, bytes, version),
            RequestKind::DeleteAcls(x) => encode(x, bytes, version),
            RequestKind::DescribeConfigs(x) => encode(x, bytes, version),
            RequestKind::AlterConfigs(x) => encode(x, bytes, version),
            RequestKind::AlterReplicaLogDirs(x) => encode(x, bytes, version),
            RequestKind::DescribeLogDirs(x) => encode(x, bytes, version),
            RequestKind::SaslAuthenticate(x) => encode(x, bytes, version),
            RequestKind::CreatePartitions(x) => encode(x, bytes, version),
            RequestKind::CreateDelegationToken(x) => encode(x, bytes, version),
            RequestKind::RenewDelegationToken(x) => encode(x, bytes, version),
            RequestKind::ExpireDelegationToken(x) => encode(x, bytes, version),
            RequestKind::DescribeDelegationToken(x) => encode(x, bytes, version),
            RequestKind::DeleteGroups(x) => encode(x, bytes, version),
            RequestKind::ElectLeaders(x) => encode(x, bytes, version),
            RequestKind::IncrementalAlterConfigs(x) => encode(x, bytes, version),
            RequestKind::AlterPartitionReassignments(x) => encode(x, bytes, version),
            RequestKind::ListPartitionReassignments(x) => encode(x, bytes, version),
            RequestKind::OffsetDelete(x) => encode(x, bytes, version),
            RequestKind::DescribeClientQuotas(x) => encode(x, bytes, version),
            RequestKind::AlterClientQuotas(x) => encode(x, bytes, version),
            RequestKind::DescribeUserScramCredentials(x) => encode(x, bytes, version),
            RequestKind::AlterUserScramCredentials(x) => encode(x, bytes, version),
            RequestKind::Vote(x) => encode(x, bytes, version),
            RequestKind::BeginQuorumEpoch(x) => encode(x, bytes, version),
            RequestKind::EndQuorumEpoch(x) => encode(x, bytes, version),
            RequestKind::DescribeQuorum(x) => encode(x, bytes, version),
            RequestKind::AlterPartition(x) => encode(x, bytes, version),
            RequestKind::UpdateFeatures(x) => encode(x, bytes, version),
            RequestKind::Envelope(x) => encode(x, bytes, version),
            RequestKind::FetchSnapshot(x) => encode(x, bytes, version),
            RequestKind::DescribeCluster(x) => encode(x, bytes, version),
            RequestKind::DescribeProducers(x) => encode(x, bytes, version),
            RequestKind::BrokerRegistration(x) => encode(x, bytes, version),
            RequestKind::BrokerHeartbeat(x) => encode(x, bytes, version),
            RequestKind::UnregisterBroker(x) => encode(x, bytes, version),
            RequestKind::DescribeTransactions(x) => encode(x, bytes, version),
            RequestKind::ListTransactions(x) => encode(x, bytes, version),
            RequestKind::AllocateProducerIds(x) => encode(x, bytes, version),
            RequestKind::ConsumerGroupHeartbeat(x) => encode(x, bytes, version),
            RequestKind::ConsumerGroupDescribe(x) => encode(x, bytes, version),
            RequestKind::ControllerRegistration(x) => encode(x, bytes, version),
            RequestKind::GetTelemetrySubscriptions(x) => encode(x, bytes, version),
            RequestKind::PushTelemetry(x) => encode(x, bytes, version),
            RequestKind::AssignReplicasToDirs(x) => encode(x, bytes, version),
            RequestKind::ListClientMetricsResources(x) => encode(x, bytes, version),
            RequestKind::DescribeTopicPartitions(x) => encode(x, bytes, version),
        }
    }
    /// Decode the message from the provided buffer and version
    #[cfg(feature = "broker")]
    pub fn decode(
        api_key: ApiKey,
        bytes: &mut bytes::Bytes,
        version: i16,
    ) -> anyhow::Result<RequestKind> {
        match api_key {
            ApiKey::ProduceKey => Ok(RequestKind::Produce(decode(bytes, version)?)),
            ApiKey::FetchKey => Ok(RequestKind::Fetch(decode(bytes, version)?)),
            ApiKey::ListOffsetsKey => Ok(RequestKind::ListOffsets(decode(bytes, version)?)),
            ApiKey::MetadataKey => Ok(RequestKind::Metadata(decode(bytes, version)?)),
            ApiKey::LeaderAndIsrKey => Ok(RequestKind::LeaderAndIsr(decode(bytes, version)?)),
            ApiKey::StopReplicaKey => Ok(RequestKind::StopReplica(decode(bytes, version)?)),
            ApiKey::UpdateMetadataKey => Ok(RequestKind::UpdateMetadata(decode(bytes, version)?)),
            ApiKey::ControlledShutdownKey => {
                Ok(RequestKind::ControlledShutdown(decode(bytes, version)?))
            }
            ApiKey::OffsetCommitKey => Ok(RequestKind::OffsetCommit(decode(bytes, version)?)),
            ApiKey::OffsetFetchKey => Ok(RequestKind::OffsetFetch(decode(bytes, version)?)),
            ApiKey::FindCoordinatorKey => Ok(RequestKind::FindCoordinator(decode(bytes, version)?)),
            ApiKey::JoinGroupKey => Ok(RequestKind::JoinGroup(decode(bytes, version)?)),
            ApiKey::HeartbeatKey => Ok(RequestKind::Heartbeat(decode(bytes, version)?)),
            ApiKey::LeaveGroupKey => Ok(RequestKind::LeaveGroup(decode(bytes, version)?)),
            ApiKey::SyncGroupKey => Ok(RequestKind::SyncGroup(decode(bytes, version)?)),
            ApiKey::DescribeGroupsKey => Ok(RequestKind::DescribeGroups(decode(bytes, version)?)),
            ApiKey::ListGroupsKey => Ok(RequestKind::ListGroups(decode(bytes, version)?)),
            ApiKey::SaslHandshakeKey => Ok(RequestKind::SaslHandshake(decode(bytes, version)?)),
            ApiKey::ApiVersionsKey => Ok(RequestKind::ApiVersions(decode(bytes, version)?)),
            ApiKey::CreateTopicsKey => Ok(RequestKind::CreateTopics(decode(bytes, version)?)),
            ApiKey::DeleteTopicsKey => Ok(RequestKind::DeleteTopics(decode(bytes, version)?)),
            ApiKey::DeleteRecordsKey => Ok(RequestKind::DeleteRecords(decode(bytes, version)?)),
            ApiKey::InitProducerIdKey => Ok(RequestKind::InitProducerId(decode(bytes, version)?)),
            ApiKey::OffsetForLeaderEpochKey => {
                Ok(RequestKind::OffsetForLeaderEpoch(decode(bytes, version)?))
            }
            ApiKey::AddPartitionsToTxnKey => {
                Ok(RequestKind::AddPartitionsToTxn(decode(bytes, version)?))
            }
            ApiKey::AddOffsetsToTxnKey => Ok(RequestKind::AddOffsetsToTxn(decode(bytes, version)?)),
            ApiKey::EndTxnKey => Ok(RequestKind::EndTxn(decode(bytes, version)?)),
            ApiKey::WriteTxnMarkersKey => Ok(RequestKind::WriteTxnMarkers(decode(bytes, version)?)),
            ApiKey::TxnOffsetCommitKey => Ok(RequestKind::TxnOffsetCommit(decode(bytes, version)?)),
            ApiKey::DescribeAclsKey => Ok(RequestKind::DescribeAcls(decode(bytes, version)?)),
            ApiKey::CreateAclsKey => Ok(RequestKind::CreateAcls(decode(bytes, version)?)),
            ApiKey::DeleteAclsKey => Ok(RequestKind::DeleteAcls(decode(bytes, version)?)),
            ApiKey::DescribeConfigsKey => Ok(RequestKind::DescribeConfigs(decode(bytes, version)?)),
            ApiKey::AlterConfigsKey => Ok(RequestKind::AlterConfigs(decode(bytes, version)?)),
            ApiKey::AlterReplicaLogDirsKey => {
                Ok(RequestKind::AlterReplicaLogDirs(decode(bytes, version)?))
            }
            ApiKey::DescribeLogDirsKey => Ok(RequestKind::DescribeLogDirs(decode(bytes, version)?)),
            ApiKey::SaslAuthenticateKey => {
                Ok(RequestKind::SaslAuthenticate(decode(bytes, version)?))
            }
            ApiKey::CreatePartitionsKey => {
                Ok(RequestKind::CreatePartitions(decode(bytes, version)?))
            }
            ApiKey::CreateDelegationTokenKey => {
                Ok(RequestKind::CreateDelegationToken(decode(bytes, version)?))
            }
            ApiKey::RenewDelegationTokenKey => {
                Ok(RequestKind::RenewDelegationToken(decode(bytes, version)?))
            }
            ApiKey::ExpireDelegationTokenKey => {
                Ok(RequestKind::ExpireDelegationToken(decode(bytes, version)?))
            }
            ApiKey::DescribeDelegationTokenKey => Ok(RequestKind::DescribeDelegationToken(decode(
                bytes, version,
            )?)),
            ApiKey::DeleteGroupsKey => Ok(RequestKind::DeleteGroups(decode(bytes, version)?)),
            ApiKey::ElectLeadersKey => Ok(RequestKind::ElectLeaders(decode(bytes, version)?)),
            ApiKey::IncrementalAlterConfigsKey => Ok(RequestKind::IncrementalAlterConfigs(decode(
                bytes, version,
            )?)),
            ApiKey::AlterPartitionReassignmentsKey => Ok(RequestKind::AlterPartitionReassignments(
                decode(bytes, version)?,
            )),
            ApiKey::ListPartitionReassignmentsKey => Ok(RequestKind::ListPartitionReassignments(
                decode(bytes, version)?,
            )),
            ApiKey::OffsetDeleteKey => Ok(RequestKind::OffsetDelete(decode(bytes, version)?)),
            ApiKey::DescribeClientQuotasKey => {
                Ok(RequestKind::DescribeClientQuotas(decode(bytes, version)?))
            }
            ApiKey::AlterClientQuotasKey => {
                Ok(RequestKind::AlterClientQuotas(decode(bytes, version)?))
            }
            ApiKey::DescribeUserScramCredentialsKey => Ok(
                RequestKind::DescribeUserScramCredentials(decode(bytes, version)?),
            ),
            ApiKey::AlterUserScramCredentialsKey => Ok(RequestKind::AlterUserScramCredentials(
                decode(bytes, version)?,
            )),
            ApiKey::VoteKey => Ok(RequestKind::Vote(decode(bytes, version)?)),
            ApiKey::BeginQuorumEpochKey => {
                Ok(RequestKind::BeginQuorumEpoch(decode(bytes, version)?))
            }
            ApiKey::EndQuorumEpochKey => Ok(RequestKind::EndQuorumEpoch(decode(bytes, version)?)),
            ApiKey::DescribeQuorumKey => Ok(RequestKind::DescribeQuorum(decode(bytes, version)?)),
            ApiKey::AlterPartitionKey => Ok(RequestKind::AlterPartition(decode(bytes, version)?)),
            ApiKey::UpdateFeaturesKey => Ok(RequestKind::UpdateFeatures(decode(bytes, version)?)),
            ApiKey::EnvelopeKey => Ok(RequestKind::Envelope(decode(bytes, version)?)),
            ApiKey::FetchSnapshotKey => Ok(RequestKind::FetchSnapshot(decode(bytes, version)?)),
            ApiKey::DescribeClusterKey => Ok(RequestKind::DescribeCluster(decode(bytes, version)?)),
            ApiKey::DescribeProducersKey => {
                Ok(RequestKind::DescribeProducers(decode(bytes, version)?))
            }
            ApiKey::BrokerRegistrationKey => {
                Ok(RequestKind::BrokerRegistration(decode(bytes, version)?))
            }
            ApiKey::BrokerHeartbeatKey => Ok(RequestKind::BrokerHeartbeat(decode(bytes, version)?)),
            ApiKey::UnregisterBrokerKey => {
                Ok(RequestKind::UnregisterBroker(decode(bytes, version)?))
            }
            ApiKey::DescribeTransactionsKey => {
                Ok(RequestKind::DescribeTransactions(decode(bytes, version)?))
            }
            ApiKey::ListTransactionsKey => {
                Ok(RequestKind::ListTransactions(decode(bytes, version)?))
            }
            ApiKey::AllocateProducerIdsKey => {
                Ok(RequestKind::AllocateProducerIds(decode(bytes, version)?))
            }
            ApiKey::ConsumerGroupHeartbeatKey => {
                Ok(RequestKind::ConsumerGroupHeartbeat(decode(bytes, version)?))
            }
            ApiKey::ConsumerGroupDescribeKey => {
                Ok(RequestKind::ConsumerGroupDescribe(decode(bytes, version)?))
            }
            ApiKey::ControllerRegistrationKey => {
                Ok(RequestKind::ControllerRegistration(decode(bytes, version)?))
            }
            ApiKey::GetTelemetrySubscriptionsKey => Ok(RequestKind::GetTelemetrySubscriptions(
                decode(bytes, version)?,
            )),
            ApiKey::PushTelemetryKey => Ok(RequestKind::PushTelemetry(decode(bytes, version)?)),
            ApiKey::AssignReplicasToDirsKey => {
                Ok(RequestKind::AssignReplicasToDirs(decode(bytes, version)?))
            }
            ApiKey::ListClientMetricsResourcesKey => Ok(RequestKind::ListClientMetricsResources(
                decode(bytes, version)?,
            )),
            ApiKey::DescribeTopicPartitionsKey => Ok(RequestKind::DescribeTopicPartitions(decode(
                bytes, version,
            )?)),
        }
    }
}
#[cfg(feature = "messages_enums")]
impl From<ProduceRequest> for RequestKind {
    fn from(value: ProduceRequest) -> RequestKind {
        RequestKind::Produce(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<FetchRequest> for RequestKind {
    fn from(value: FetchRequest) -> RequestKind {
        RequestKind::Fetch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListOffsetsRequest> for RequestKind {
    fn from(value: ListOffsetsRequest) -> RequestKind {
        RequestKind::ListOffsets(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<MetadataRequest> for RequestKind {
    fn from(value: MetadataRequest) -> RequestKind {
        RequestKind::Metadata(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<LeaderAndIsrRequest> for RequestKind {
    fn from(value: LeaderAndIsrRequest) -> RequestKind {
        RequestKind::LeaderAndIsr(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<StopReplicaRequest> for RequestKind {
    fn from(value: StopReplicaRequest) -> RequestKind {
        RequestKind::StopReplica(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<UpdateMetadataRequest> for RequestKind {
    fn from(value: UpdateMetadataRequest) -> RequestKind {
        RequestKind::UpdateMetadata(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ControlledShutdownRequest> for RequestKind {
    fn from(value: ControlledShutdownRequest) -> RequestKind {
        RequestKind::ControlledShutdown(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetCommitRequest> for RequestKind {
    fn from(value: OffsetCommitRequest) -> RequestKind {
        RequestKind::OffsetCommit(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetFetchRequest> for RequestKind {
    fn from(value: OffsetFetchRequest) -> RequestKind {
        RequestKind::OffsetFetch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<FindCoordinatorRequest> for RequestKind {
    fn from(value: FindCoordinatorRequest) -> RequestKind {
        RequestKind::FindCoordinator(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<JoinGroupRequest> for RequestKind {
    fn from(value: JoinGroupRequest) -> RequestKind {
        RequestKind::JoinGroup(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<HeartbeatRequest> for RequestKind {
    fn from(value: HeartbeatRequest) -> RequestKind {
        RequestKind::Heartbeat(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<LeaveGroupRequest> for RequestKind {
    fn from(value: LeaveGroupRequest) -> RequestKind {
        RequestKind::LeaveGroup(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<SyncGroupRequest> for RequestKind {
    fn from(value: SyncGroupRequest) -> RequestKind {
        RequestKind::SyncGroup(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeGroupsRequest> for RequestKind {
    fn from(value: DescribeGroupsRequest) -> RequestKind {
        RequestKind::DescribeGroups(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListGroupsRequest> for RequestKind {
    fn from(value: ListGroupsRequest) -> RequestKind {
        RequestKind::ListGroups(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<SaslHandshakeRequest> for RequestKind {
    fn from(value: SaslHandshakeRequest) -> RequestKind {
        RequestKind::SaslHandshake(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ApiVersionsRequest> for RequestKind {
    fn from(value: ApiVersionsRequest) -> RequestKind {
        RequestKind::ApiVersions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreateTopicsRequest> for RequestKind {
    fn from(value: CreateTopicsRequest) -> RequestKind {
        RequestKind::CreateTopics(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteTopicsRequest> for RequestKind {
    fn from(value: DeleteTopicsRequest) -> RequestKind {
        RequestKind::DeleteTopics(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteRecordsRequest> for RequestKind {
    fn from(value: DeleteRecordsRequest) -> RequestKind {
        RequestKind::DeleteRecords(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<InitProducerIdRequest> for RequestKind {
    fn from(value: InitProducerIdRequest) -> RequestKind {
        RequestKind::InitProducerId(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetForLeaderEpochRequest> for RequestKind {
    fn from(value: OffsetForLeaderEpochRequest) -> RequestKind {
        RequestKind::OffsetForLeaderEpoch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AddPartitionsToTxnRequest> for RequestKind {
    fn from(value: AddPartitionsToTxnRequest) -> RequestKind {
        RequestKind::AddPartitionsToTxn(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AddOffsetsToTxnRequest> for RequestKind {
    fn from(value: AddOffsetsToTxnRequest) -> RequestKind {
        RequestKind::AddOffsetsToTxn(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<EndTxnRequest> for RequestKind {
    fn from(value: EndTxnRequest) -> RequestKind {
        RequestKind::EndTxn(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<WriteTxnMarkersRequest> for RequestKind {
    fn from(value: WriteTxnMarkersRequest) -> RequestKind {
        RequestKind::WriteTxnMarkers(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<TxnOffsetCommitRequest> for RequestKind {
    fn from(value: TxnOffsetCommitRequest) -> RequestKind {
        RequestKind::TxnOffsetCommit(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeAclsRequest> for RequestKind {
    fn from(value: DescribeAclsRequest) -> RequestKind {
        RequestKind::DescribeAcls(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreateAclsRequest> for RequestKind {
    fn from(value: CreateAclsRequest) -> RequestKind {
        RequestKind::CreateAcls(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteAclsRequest> for RequestKind {
    fn from(value: DeleteAclsRequest) -> RequestKind {
        RequestKind::DeleteAcls(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeConfigsRequest> for RequestKind {
    fn from(value: DescribeConfigsRequest) -> RequestKind {
        RequestKind::DescribeConfigs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterConfigsRequest> for RequestKind {
    fn from(value: AlterConfigsRequest) -> RequestKind {
        RequestKind::AlterConfigs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterReplicaLogDirsRequest> for RequestKind {
    fn from(value: AlterReplicaLogDirsRequest) -> RequestKind {
        RequestKind::AlterReplicaLogDirs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeLogDirsRequest> for RequestKind {
    fn from(value: DescribeLogDirsRequest) -> RequestKind {
        RequestKind::DescribeLogDirs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<SaslAuthenticateRequest> for RequestKind {
    fn from(value: SaslAuthenticateRequest) -> RequestKind {
        RequestKind::SaslAuthenticate(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreatePartitionsRequest> for RequestKind {
    fn from(value: CreatePartitionsRequest) -> RequestKind {
        RequestKind::CreatePartitions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreateDelegationTokenRequest> for RequestKind {
    fn from(value: CreateDelegationTokenRequest) -> RequestKind {
        RequestKind::CreateDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<RenewDelegationTokenRequest> for RequestKind {
    fn from(value: RenewDelegationTokenRequest) -> RequestKind {
        RequestKind::RenewDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ExpireDelegationTokenRequest> for RequestKind {
    fn from(value: ExpireDelegationTokenRequest) -> RequestKind {
        RequestKind::ExpireDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeDelegationTokenRequest> for RequestKind {
    fn from(value: DescribeDelegationTokenRequest) -> RequestKind {
        RequestKind::DescribeDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteGroupsRequest> for RequestKind {
    fn from(value: DeleteGroupsRequest) -> RequestKind {
        RequestKind::DeleteGroups(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ElectLeadersRequest> for RequestKind {
    fn from(value: ElectLeadersRequest) -> RequestKind {
        RequestKind::ElectLeaders(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<IncrementalAlterConfigsRequest> for RequestKind {
    fn from(value: IncrementalAlterConfigsRequest) -> RequestKind {
        RequestKind::IncrementalAlterConfigs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterPartitionReassignmentsRequest> for RequestKind {
    fn from(value: AlterPartitionReassignmentsRequest) -> RequestKind {
        RequestKind::AlterPartitionReassignments(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListPartitionReassignmentsRequest> for RequestKind {
    fn from(value: ListPartitionReassignmentsRequest) -> RequestKind {
        RequestKind::ListPartitionReassignments(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetDeleteRequest> for RequestKind {
    fn from(value: OffsetDeleteRequest) -> RequestKind {
        RequestKind::OffsetDelete(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeClientQuotasRequest> for RequestKind {
    fn from(value: DescribeClientQuotasRequest) -> RequestKind {
        RequestKind::DescribeClientQuotas(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterClientQuotasRequest> for RequestKind {
    fn from(value: AlterClientQuotasRequest) -> RequestKind {
        RequestKind::AlterClientQuotas(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeUserScramCredentialsRequest> for RequestKind {
    fn from(value: DescribeUserScramCredentialsRequest) -> RequestKind {
        RequestKind::DescribeUserScramCredentials(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterUserScramCredentialsRequest> for RequestKind {
    fn from(value: AlterUserScramCredentialsRequest) -> RequestKind {
        RequestKind::AlterUserScramCredentials(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<VoteRequest> for RequestKind {
    fn from(value: VoteRequest) -> RequestKind {
        RequestKind::Vote(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<BeginQuorumEpochRequest> for RequestKind {
    fn from(value: BeginQuorumEpochRequest) -> RequestKind {
        RequestKind::BeginQuorumEpoch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<EndQuorumEpochRequest> for RequestKind {
    fn from(value: EndQuorumEpochRequest) -> RequestKind {
        RequestKind::EndQuorumEpoch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeQuorumRequest> for RequestKind {
    fn from(value: DescribeQuorumRequest) -> RequestKind {
        RequestKind::DescribeQuorum(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterPartitionRequest> for RequestKind {
    fn from(value: AlterPartitionRequest) -> RequestKind {
        RequestKind::AlterPartition(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<UpdateFeaturesRequest> for RequestKind {
    fn from(value: UpdateFeaturesRequest) -> RequestKind {
        RequestKind::UpdateFeatures(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<EnvelopeRequest> for RequestKind {
    fn from(value: EnvelopeRequest) -> RequestKind {
        RequestKind::Envelope(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<FetchSnapshotRequest> for RequestKind {
    fn from(value: FetchSnapshotRequest) -> RequestKind {
        RequestKind::FetchSnapshot(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeClusterRequest> for RequestKind {
    fn from(value: DescribeClusterRequest) -> RequestKind {
        RequestKind::DescribeCluster(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeProducersRequest> for RequestKind {
    fn from(value: DescribeProducersRequest) -> RequestKind {
        RequestKind::DescribeProducers(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<BrokerRegistrationRequest> for RequestKind {
    fn from(value: BrokerRegistrationRequest) -> RequestKind {
        RequestKind::BrokerRegistration(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<BrokerHeartbeatRequest> for RequestKind {
    fn from(value: BrokerHeartbeatRequest) -> RequestKind {
        RequestKind::BrokerHeartbeat(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<UnregisterBrokerRequest> for RequestKind {
    fn from(value: UnregisterBrokerRequest) -> RequestKind {
        RequestKind::UnregisterBroker(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeTransactionsRequest> for RequestKind {
    fn from(value: DescribeTransactionsRequest) -> RequestKind {
        RequestKind::DescribeTransactions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListTransactionsRequest> for RequestKind {
    fn from(value: ListTransactionsRequest) -> RequestKind {
        RequestKind::ListTransactions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AllocateProducerIdsRequest> for RequestKind {
    fn from(value: AllocateProducerIdsRequest) -> RequestKind {
        RequestKind::AllocateProducerIds(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ConsumerGroupHeartbeatRequest> for RequestKind {
    fn from(value: ConsumerGroupHeartbeatRequest) -> RequestKind {
        RequestKind::ConsumerGroupHeartbeat(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ConsumerGroupDescribeRequest> for RequestKind {
    fn from(value: ConsumerGroupDescribeRequest) -> RequestKind {
        RequestKind::ConsumerGroupDescribe(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ControllerRegistrationRequest> for RequestKind {
    fn from(value: ControllerRegistrationRequest) -> RequestKind {
        RequestKind::ControllerRegistration(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<GetTelemetrySubscriptionsRequest> for RequestKind {
    fn from(value: GetTelemetrySubscriptionsRequest) -> RequestKind {
        RequestKind::GetTelemetrySubscriptions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<PushTelemetryRequest> for RequestKind {
    fn from(value: PushTelemetryRequest) -> RequestKind {
        RequestKind::PushTelemetry(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AssignReplicasToDirsRequest> for RequestKind {
    fn from(value: AssignReplicasToDirsRequest) -> RequestKind {
        RequestKind::AssignReplicasToDirs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListClientMetricsResourcesRequest> for RequestKind {
    fn from(value: ListClientMetricsResourcesRequest) -> RequestKind {
        RequestKind::ListClientMetricsResources(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeTopicPartitionsRequest> for RequestKind {
    fn from(value: DescribeTopicPartitionsRequest) -> RequestKind {
        RequestKind::DescribeTopicPartitions(value)
    }
}

#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
fn decode<T: Decodable>(bytes: &mut bytes::Bytes, version: i16) -> Result<T> {
    T::decode(bytes, version).with_context(|| {
        format!(
            "Failed to decode {} v{} body",
            std::any::type_name::<T>(),
            version
        )
    })
}

#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
fn encode<T: Encodable>(encodable: &T, bytes: &mut bytes::BytesMut, version: i16) -> Result<()> {
    encodable.encode(bytes, version).with_context(|| {
        format!(
            "Failed to encode {} v{} body",
            std::any::type_name::<T>(),
            version
        )
    })
}

/// Wrapping enum for all responses in the Kafka protocol.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "messages_enums")]
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
    /// ConsumerGroupDescribeResponse,
    ConsumerGroupDescribe(ConsumerGroupDescribeResponse),
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
    /// DescribeTopicPartitionsResponse,
    DescribeTopicPartitions(DescribeTopicPartitionsResponse),
}

#[cfg(feature = "messages_enums")]
impl ResponseKind {
    /// Encode the message into the target buffer
    #[cfg(feature = "broker")]
    pub fn encode(&self, bytes: &mut bytes::BytesMut, version: i16) -> anyhow::Result<()> {
        match self {
            ResponseKind::Produce(x) => encode(x, bytes, version),
            ResponseKind::Fetch(x) => encode(x, bytes, version),
            ResponseKind::ListOffsets(x) => encode(x, bytes, version),
            ResponseKind::Metadata(x) => encode(x, bytes, version),
            ResponseKind::LeaderAndIsr(x) => encode(x, bytes, version),
            ResponseKind::StopReplica(x) => encode(x, bytes, version),
            ResponseKind::UpdateMetadata(x) => encode(x, bytes, version),
            ResponseKind::ControlledShutdown(x) => encode(x, bytes, version),
            ResponseKind::OffsetCommit(x) => encode(x, bytes, version),
            ResponseKind::OffsetFetch(x) => encode(x, bytes, version),
            ResponseKind::FindCoordinator(x) => encode(x, bytes, version),
            ResponseKind::JoinGroup(x) => encode(x, bytes, version),
            ResponseKind::Heartbeat(x) => encode(x, bytes, version),
            ResponseKind::LeaveGroup(x) => encode(x, bytes, version),
            ResponseKind::SyncGroup(x) => encode(x, bytes, version),
            ResponseKind::DescribeGroups(x) => encode(x, bytes, version),
            ResponseKind::ListGroups(x) => encode(x, bytes, version),
            ResponseKind::SaslHandshake(x) => encode(x, bytes, version),
            ResponseKind::ApiVersions(x) => encode(x, bytes, version),
            ResponseKind::CreateTopics(x) => encode(x, bytes, version),
            ResponseKind::DeleteTopics(x) => encode(x, bytes, version),
            ResponseKind::DeleteRecords(x) => encode(x, bytes, version),
            ResponseKind::InitProducerId(x) => encode(x, bytes, version),
            ResponseKind::OffsetForLeaderEpoch(x) => encode(x, bytes, version),
            ResponseKind::AddPartitionsToTxn(x) => encode(x, bytes, version),
            ResponseKind::AddOffsetsToTxn(x) => encode(x, bytes, version),
            ResponseKind::EndTxn(x) => encode(x, bytes, version),
            ResponseKind::WriteTxnMarkers(x) => encode(x, bytes, version),
            ResponseKind::TxnOffsetCommit(x) => encode(x, bytes, version),
            ResponseKind::DescribeAcls(x) => encode(x, bytes, version),
            ResponseKind::CreateAcls(x) => encode(x, bytes, version),
            ResponseKind::DeleteAcls(x) => encode(x, bytes, version),
            ResponseKind::DescribeConfigs(x) => encode(x, bytes, version),
            ResponseKind::AlterConfigs(x) => encode(x, bytes, version),
            ResponseKind::AlterReplicaLogDirs(x) => encode(x, bytes, version),
            ResponseKind::DescribeLogDirs(x) => encode(x, bytes, version),
            ResponseKind::SaslAuthenticate(x) => encode(x, bytes, version),
            ResponseKind::CreatePartitions(x) => encode(x, bytes, version),
            ResponseKind::CreateDelegationToken(x) => encode(x, bytes, version),
            ResponseKind::RenewDelegationToken(x) => encode(x, bytes, version),
            ResponseKind::ExpireDelegationToken(x) => encode(x, bytes, version),
            ResponseKind::DescribeDelegationToken(x) => encode(x, bytes, version),
            ResponseKind::DeleteGroups(x) => encode(x, bytes, version),
            ResponseKind::ElectLeaders(x) => encode(x, bytes, version),
            ResponseKind::IncrementalAlterConfigs(x) => encode(x, bytes, version),
            ResponseKind::AlterPartitionReassignments(x) => encode(x, bytes, version),
            ResponseKind::ListPartitionReassignments(x) => encode(x, bytes, version),
            ResponseKind::OffsetDelete(x) => encode(x, bytes, version),
            ResponseKind::DescribeClientQuotas(x) => encode(x, bytes, version),
            ResponseKind::AlterClientQuotas(x) => encode(x, bytes, version),
            ResponseKind::DescribeUserScramCredentials(x) => encode(x, bytes, version),
            ResponseKind::AlterUserScramCredentials(x) => encode(x, bytes, version),
            ResponseKind::Vote(x) => encode(x, bytes, version),
            ResponseKind::BeginQuorumEpoch(x) => encode(x, bytes, version),
            ResponseKind::EndQuorumEpoch(x) => encode(x, bytes, version),
            ResponseKind::DescribeQuorum(x) => encode(x, bytes, version),
            ResponseKind::AlterPartition(x) => encode(x, bytes, version),
            ResponseKind::UpdateFeatures(x) => encode(x, bytes, version),
            ResponseKind::Envelope(x) => encode(x, bytes, version),
            ResponseKind::FetchSnapshot(x) => encode(x, bytes, version),
            ResponseKind::DescribeCluster(x) => encode(x, bytes, version),
            ResponseKind::DescribeProducers(x) => encode(x, bytes, version),
            ResponseKind::BrokerRegistration(x) => encode(x, bytes, version),
            ResponseKind::BrokerHeartbeat(x) => encode(x, bytes, version),
            ResponseKind::UnregisterBroker(x) => encode(x, bytes, version),
            ResponseKind::DescribeTransactions(x) => encode(x, bytes, version),
            ResponseKind::ListTransactions(x) => encode(x, bytes, version),
            ResponseKind::AllocateProducerIds(x) => encode(x, bytes, version),
            ResponseKind::ConsumerGroupHeartbeat(x) => encode(x, bytes, version),
            ResponseKind::ConsumerGroupDescribe(x) => encode(x, bytes, version),
            ResponseKind::ControllerRegistration(x) => encode(x, bytes, version),
            ResponseKind::GetTelemetrySubscriptions(x) => encode(x, bytes, version),
            ResponseKind::PushTelemetry(x) => encode(x, bytes, version),
            ResponseKind::AssignReplicasToDirs(x) => encode(x, bytes, version),
            ResponseKind::ListClientMetricsResources(x) => encode(x, bytes, version),
            ResponseKind::DescribeTopicPartitions(x) => encode(x, bytes, version),
        }
    }
    /// Decode the message from the provided buffer and version
    #[cfg(feature = "client")]
    pub fn decode(
        api_key: ApiKey,
        bytes: &mut bytes::Bytes,
        version: i16,
    ) -> anyhow::Result<ResponseKind> {
        match api_key {
            ApiKey::ProduceKey => Ok(ResponseKind::Produce(decode(bytes, version)?)),
            ApiKey::FetchKey => Ok(ResponseKind::Fetch(decode(bytes, version)?)),
            ApiKey::ListOffsetsKey => Ok(ResponseKind::ListOffsets(decode(bytes, version)?)),
            ApiKey::MetadataKey => Ok(ResponseKind::Metadata(decode(bytes, version)?)),
            ApiKey::LeaderAndIsrKey => Ok(ResponseKind::LeaderAndIsr(decode(bytes, version)?)),
            ApiKey::StopReplicaKey => Ok(ResponseKind::StopReplica(decode(bytes, version)?)),
            ApiKey::UpdateMetadataKey => Ok(ResponseKind::UpdateMetadata(decode(bytes, version)?)),
            ApiKey::ControlledShutdownKey => {
                Ok(ResponseKind::ControlledShutdown(decode(bytes, version)?))
            }
            ApiKey::OffsetCommitKey => Ok(ResponseKind::OffsetCommit(decode(bytes, version)?)),
            ApiKey::OffsetFetchKey => Ok(ResponseKind::OffsetFetch(decode(bytes, version)?)),
            ApiKey::FindCoordinatorKey => {
                Ok(ResponseKind::FindCoordinator(decode(bytes, version)?))
            }
            ApiKey::JoinGroupKey => Ok(ResponseKind::JoinGroup(decode(bytes, version)?)),
            ApiKey::HeartbeatKey => Ok(ResponseKind::Heartbeat(decode(bytes, version)?)),
            ApiKey::LeaveGroupKey => Ok(ResponseKind::LeaveGroup(decode(bytes, version)?)),
            ApiKey::SyncGroupKey => Ok(ResponseKind::SyncGroup(decode(bytes, version)?)),
            ApiKey::DescribeGroupsKey => Ok(ResponseKind::DescribeGroups(decode(bytes, version)?)),
            ApiKey::ListGroupsKey => Ok(ResponseKind::ListGroups(decode(bytes, version)?)),
            ApiKey::SaslHandshakeKey => Ok(ResponseKind::SaslHandshake(decode(bytes, version)?)),
            ApiKey::ApiVersionsKey => Ok(ResponseKind::ApiVersions(decode(bytes, version)?)),
            ApiKey::CreateTopicsKey => Ok(ResponseKind::CreateTopics(decode(bytes, version)?)),
            ApiKey::DeleteTopicsKey => Ok(ResponseKind::DeleteTopics(decode(bytes, version)?)),
            ApiKey::DeleteRecordsKey => Ok(ResponseKind::DeleteRecords(decode(bytes, version)?)),
            ApiKey::InitProducerIdKey => Ok(ResponseKind::InitProducerId(decode(bytes, version)?)),
            ApiKey::OffsetForLeaderEpochKey => {
                Ok(ResponseKind::OffsetForLeaderEpoch(decode(bytes, version)?))
            }
            ApiKey::AddPartitionsToTxnKey => {
                Ok(ResponseKind::AddPartitionsToTxn(decode(bytes, version)?))
            }
            ApiKey::AddOffsetsToTxnKey => {
                Ok(ResponseKind::AddOffsetsToTxn(decode(bytes, version)?))
            }
            ApiKey::EndTxnKey => Ok(ResponseKind::EndTxn(decode(bytes, version)?)),
            ApiKey::WriteTxnMarkersKey => {
                Ok(ResponseKind::WriteTxnMarkers(decode(bytes, version)?))
            }
            ApiKey::TxnOffsetCommitKey => {
                Ok(ResponseKind::TxnOffsetCommit(decode(bytes, version)?))
            }
            ApiKey::DescribeAclsKey => Ok(ResponseKind::DescribeAcls(decode(bytes, version)?)),
            ApiKey::CreateAclsKey => Ok(ResponseKind::CreateAcls(decode(bytes, version)?)),
            ApiKey::DeleteAclsKey => Ok(ResponseKind::DeleteAcls(decode(bytes, version)?)),
            ApiKey::DescribeConfigsKey => {
                Ok(ResponseKind::DescribeConfigs(decode(bytes, version)?))
            }
            ApiKey::AlterConfigsKey => Ok(ResponseKind::AlterConfigs(decode(bytes, version)?)),
            ApiKey::AlterReplicaLogDirsKey => {
                Ok(ResponseKind::AlterReplicaLogDirs(decode(bytes, version)?))
            }
            ApiKey::DescribeLogDirsKey => {
                Ok(ResponseKind::DescribeLogDirs(decode(bytes, version)?))
            }
            ApiKey::SaslAuthenticateKey => {
                Ok(ResponseKind::SaslAuthenticate(decode(bytes, version)?))
            }
            ApiKey::CreatePartitionsKey => {
                Ok(ResponseKind::CreatePartitions(decode(bytes, version)?))
            }
            ApiKey::CreateDelegationTokenKey => {
                Ok(ResponseKind::CreateDelegationToken(decode(bytes, version)?))
            }
            ApiKey::RenewDelegationTokenKey => {
                Ok(ResponseKind::RenewDelegationToken(decode(bytes, version)?))
            }
            ApiKey::ExpireDelegationTokenKey => {
                Ok(ResponseKind::ExpireDelegationToken(decode(bytes, version)?))
            }
            ApiKey::DescribeDelegationTokenKey => Ok(ResponseKind::DescribeDelegationToken(
                decode(bytes, version)?,
            )),
            ApiKey::DeleteGroupsKey => Ok(ResponseKind::DeleteGroups(decode(bytes, version)?)),
            ApiKey::ElectLeadersKey => Ok(ResponseKind::ElectLeaders(decode(bytes, version)?)),
            ApiKey::IncrementalAlterConfigsKey => Ok(ResponseKind::IncrementalAlterConfigs(
                decode(bytes, version)?,
            )),
            ApiKey::AlterPartitionReassignmentsKey => Ok(
                ResponseKind::AlterPartitionReassignments(decode(bytes, version)?),
            ),
            ApiKey::ListPartitionReassignmentsKey => Ok(ResponseKind::ListPartitionReassignments(
                decode(bytes, version)?,
            )),
            ApiKey::OffsetDeleteKey => Ok(ResponseKind::OffsetDelete(decode(bytes, version)?)),
            ApiKey::DescribeClientQuotasKey => {
                Ok(ResponseKind::DescribeClientQuotas(decode(bytes, version)?))
            }
            ApiKey::AlterClientQuotasKey => {
                Ok(ResponseKind::AlterClientQuotas(decode(bytes, version)?))
            }
            ApiKey::DescribeUserScramCredentialsKey => Ok(
                ResponseKind::DescribeUserScramCredentials(decode(bytes, version)?),
            ),
            ApiKey::AlterUserScramCredentialsKey => Ok(ResponseKind::AlterUserScramCredentials(
                decode(bytes, version)?,
            )),
            ApiKey::VoteKey => Ok(ResponseKind::Vote(decode(bytes, version)?)),
            ApiKey::BeginQuorumEpochKey => {
                Ok(ResponseKind::BeginQuorumEpoch(decode(bytes, version)?))
            }
            ApiKey::EndQuorumEpochKey => Ok(ResponseKind::EndQuorumEpoch(decode(bytes, version)?)),
            ApiKey::DescribeQuorumKey => Ok(ResponseKind::DescribeQuorum(decode(bytes, version)?)),
            ApiKey::AlterPartitionKey => Ok(ResponseKind::AlterPartition(decode(bytes, version)?)),
            ApiKey::UpdateFeaturesKey => Ok(ResponseKind::UpdateFeatures(decode(bytes, version)?)),
            ApiKey::EnvelopeKey => Ok(ResponseKind::Envelope(decode(bytes, version)?)),
            ApiKey::FetchSnapshotKey => Ok(ResponseKind::FetchSnapshot(decode(bytes, version)?)),
            ApiKey::DescribeClusterKey => {
                Ok(ResponseKind::DescribeCluster(decode(bytes, version)?))
            }
            ApiKey::DescribeProducersKey => {
                Ok(ResponseKind::DescribeProducers(decode(bytes, version)?))
            }
            ApiKey::BrokerRegistrationKey => {
                Ok(ResponseKind::BrokerRegistration(decode(bytes, version)?))
            }
            ApiKey::BrokerHeartbeatKey => {
                Ok(ResponseKind::BrokerHeartbeat(decode(bytes, version)?))
            }
            ApiKey::UnregisterBrokerKey => {
                Ok(ResponseKind::UnregisterBroker(decode(bytes, version)?))
            }
            ApiKey::DescribeTransactionsKey => {
                Ok(ResponseKind::DescribeTransactions(decode(bytes, version)?))
            }
            ApiKey::ListTransactionsKey => {
                Ok(ResponseKind::ListTransactions(decode(bytes, version)?))
            }
            ApiKey::AllocateProducerIdsKey => {
                Ok(ResponseKind::AllocateProducerIds(decode(bytes, version)?))
            }
            ApiKey::ConsumerGroupHeartbeatKey => Ok(ResponseKind::ConsumerGroupHeartbeat(decode(
                bytes, version,
            )?)),
            ApiKey::ConsumerGroupDescribeKey => {
                Ok(ResponseKind::ConsumerGroupDescribe(decode(bytes, version)?))
            }
            ApiKey::ControllerRegistrationKey => Ok(ResponseKind::ControllerRegistration(decode(
                bytes, version,
            )?)),
            ApiKey::GetTelemetrySubscriptionsKey => Ok(ResponseKind::GetTelemetrySubscriptions(
                decode(bytes, version)?,
            )),
            ApiKey::PushTelemetryKey => Ok(ResponseKind::PushTelemetry(decode(bytes, version)?)),
            ApiKey::AssignReplicasToDirsKey => {
                Ok(ResponseKind::AssignReplicasToDirs(decode(bytes, version)?))
            }
            ApiKey::ListClientMetricsResourcesKey => Ok(ResponseKind::ListClientMetricsResources(
                decode(bytes, version)?,
            )),
            ApiKey::DescribeTopicPartitionsKey => Ok(ResponseKind::DescribeTopicPartitions(
                decode(bytes, version)?,
            )),
        }
    }
    /// Get the version of request header that needs to be prepended to this message
    pub fn header_version(&self, version: i16) -> i16 {
        match self {
            ResponseKind::Produce(_) => ProduceResponse::header_version(version),
            ResponseKind::Fetch(_) => FetchResponse::header_version(version),
            ResponseKind::ListOffsets(_) => ListOffsetsResponse::header_version(version),
            ResponseKind::Metadata(_) => MetadataResponse::header_version(version),
            ResponseKind::LeaderAndIsr(_) => LeaderAndIsrResponse::header_version(version),
            ResponseKind::StopReplica(_) => StopReplicaResponse::header_version(version),
            ResponseKind::UpdateMetadata(_) => UpdateMetadataResponse::header_version(version),
            ResponseKind::ControlledShutdown(_) => {
                ControlledShutdownResponse::header_version(version)
            }
            ResponseKind::OffsetCommit(_) => OffsetCommitResponse::header_version(version),
            ResponseKind::OffsetFetch(_) => OffsetFetchResponse::header_version(version),
            ResponseKind::FindCoordinator(_) => FindCoordinatorResponse::header_version(version),
            ResponseKind::JoinGroup(_) => JoinGroupResponse::header_version(version),
            ResponseKind::Heartbeat(_) => HeartbeatResponse::header_version(version),
            ResponseKind::LeaveGroup(_) => LeaveGroupResponse::header_version(version),
            ResponseKind::SyncGroup(_) => SyncGroupResponse::header_version(version),
            ResponseKind::DescribeGroups(_) => DescribeGroupsResponse::header_version(version),
            ResponseKind::ListGroups(_) => ListGroupsResponse::header_version(version),
            ResponseKind::SaslHandshake(_) => SaslHandshakeResponse::header_version(version),
            ResponseKind::ApiVersions(_) => ApiVersionsResponse::header_version(version),
            ResponseKind::CreateTopics(_) => CreateTopicsResponse::header_version(version),
            ResponseKind::DeleteTopics(_) => DeleteTopicsResponse::header_version(version),
            ResponseKind::DeleteRecords(_) => DeleteRecordsResponse::header_version(version),
            ResponseKind::InitProducerId(_) => InitProducerIdResponse::header_version(version),
            ResponseKind::OffsetForLeaderEpoch(_) => {
                OffsetForLeaderEpochResponse::header_version(version)
            }
            ResponseKind::AddPartitionsToTxn(_) => {
                AddPartitionsToTxnResponse::header_version(version)
            }
            ResponseKind::AddOffsetsToTxn(_) => AddOffsetsToTxnResponse::header_version(version),
            ResponseKind::EndTxn(_) => EndTxnResponse::header_version(version),
            ResponseKind::WriteTxnMarkers(_) => WriteTxnMarkersResponse::header_version(version),
            ResponseKind::TxnOffsetCommit(_) => TxnOffsetCommitResponse::header_version(version),
            ResponseKind::DescribeAcls(_) => DescribeAclsResponse::header_version(version),
            ResponseKind::CreateAcls(_) => CreateAclsResponse::header_version(version),
            ResponseKind::DeleteAcls(_) => DeleteAclsResponse::header_version(version),
            ResponseKind::DescribeConfigs(_) => DescribeConfigsResponse::header_version(version),
            ResponseKind::AlterConfigs(_) => AlterConfigsResponse::header_version(version),
            ResponseKind::AlterReplicaLogDirs(_) => {
                AlterReplicaLogDirsResponse::header_version(version)
            }
            ResponseKind::DescribeLogDirs(_) => DescribeLogDirsResponse::header_version(version),
            ResponseKind::SaslAuthenticate(_) => SaslAuthenticateResponse::header_version(version),
            ResponseKind::CreatePartitions(_) => CreatePartitionsResponse::header_version(version),
            ResponseKind::CreateDelegationToken(_) => {
                CreateDelegationTokenResponse::header_version(version)
            }
            ResponseKind::RenewDelegationToken(_) => {
                RenewDelegationTokenResponse::header_version(version)
            }
            ResponseKind::ExpireDelegationToken(_) => {
                ExpireDelegationTokenResponse::header_version(version)
            }
            ResponseKind::DescribeDelegationToken(_) => {
                DescribeDelegationTokenResponse::header_version(version)
            }
            ResponseKind::DeleteGroups(_) => DeleteGroupsResponse::header_version(version),
            ResponseKind::ElectLeaders(_) => ElectLeadersResponse::header_version(version),
            ResponseKind::IncrementalAlterConfigs(_) => {
                IncrementalAlterConfigsResponse::header_version(version)
            }
            ResponseKind::AlterPartitionReassignments(_) => {
                AlterPartitionReassignmentsResponse::header_version(version)
            }
            ResponseKind::ListPartitionReassignments(_) => {
                ListPartitionReassignmentsResponse::header_version(version)
            }
            ResponseKind::OffsetDelete(_) => OffsetDeleteResponse::header_version(version),
            ResponseKind::DescribeClientQuotas(_) => {
                DescribeClientQuotasResponse::header_version(version)
            }
            ResponseKind::AlterClientQuotas(_) => {
                AlterClientQuotasResponse::header_version(version)
            }
            ResponseKind::DescribeUserScramCredentials(_) => {
                DescribeUserScramCredentialsResponse::header_version(version)
            }
            ResponseKind::AlterUserScramCredentials(_) => {
                AlterUserScramCredentialsResponse::header_version(version)
            }
            ResponseKind::Vote(_) => VoteResponse::header_version(version),
            ResponseKind::BeginQuorumEpoch(_) => BeginQuorumEpochResponse::header_version(version),
            ResponseKind::EndQuorumEpoch(_) => EndQuorumEpochResponse::header_version(version),
            ResponseKind::DescribeQuorum(_) => DescribeQuorumResponse::header_version(version),
            ResponseKind::AlterPartition(_) => AlterPartitionResponse::header_version(version),
            ResponseKind::UpdateFeatures(_) => UpdateFeaturesResponse::header_version(version),
            ResponseKind::Envelope(_) => EnvelopeResponse::header_version(version),
            ResponseKind::FetchSnapshot(_) => FetchSnapshotResponse::header_version(version),
            ResponseKind::DescribeCluster(_) => DescribeClusterResponse::header_version(version),
            ResponseKind::DescribeProducers(_) => {
                DescribeProducersResponse::header_version(version)
            }
            ResponseKind::BrokerRegistration(_) => {
                BrokerRegistrationResponse::header_version(version)
            }
            ResponseKind::BrokerHeartbeat(_) => BrokerHeartbeatResponse::header_version(version),
            ResponseKind::UnregisterBroker(_) => UnregisterBrokerResponse::header_version(version),
            ResponseKind::DescribeTransactions(_) => {
                DescribeTransactionsResponse::header_version(version)
            }
            ResponseKind::ListTransactions(_) => ListTransactionsResponse::header_version(version),
            ResponseKind::AllocateProducerIds(_) => {
                AllocateProducerIdsResponse::header_version(version)
            }
            ResponseKind::ConsumerGroupHeartbeat(_) => {
                ConsumerGroupHeartbeatResponse::header_version(version)
            }
            ResponseKind::ConsumerGroupDescribe(_) => {
                ConsumerGroupDescribeResponse::header_version(version)
            }
            ResponseKind::ControllerRegistration(_) => {
                ControllerRegistrationResponse::header_version(version)
            }
            ResponseKind::GetTelemetrySubscriptions(_) => {
                GetTelemetrySubscriptionsResponse::header_version(version)
            }
            ResponseKind::PushTelemetry(_) => PushTelemetryResponse::header_version(version),
            ResponseKind::AssignReplicasToDirs(_) => {
                AssignReplicasToDirsResponse::header_version(version)
            }
            ResponseKind::ListClientMetricsResources(_) => {
                ListClientMetricsResourcesResponse::header_version(version)
            }
            ResponseKind::DescribeTopicPartitions(_) => {
                DescribeTopicPartitionsResponse::header_version(version)
            }
        }
    }
}

#[cfg(feature = "messages_enums")]
impl From<ProduceResponse> for ResponseKind {
    fn from(value: ProduceResponse) -> ResponseKind {
        ResponseKind::Produce(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<FetchResponse> for ResponseKind {
    fn from(value: FetchResponse) -> ResponseKind {
        ResponseKind::Fetch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListOffsetsResponse> for ResponseKind {
    fn from(value: ListOffsetsResponse) -> ResponseKind {
        ResponseKind::ListOffsets(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<MetadataResponse> for ResponseKind {
    fn from(value: MetadataResponse) -> ResponseKind {
        ResponseKind::Metadata(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<LeaderAndIsrResponse> for ResponseKind {
    fn from(value: LeaderAndIsrResponse) -> ResponseKind {
        ResponseKind::LeaderAndIsr(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<StopReplicaResponse> for ResponseKind {
    fn from(value: StopReplicaResponse) -> ResponseKind {
        ResponseKind::StopReplica(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<UpdateMetadataResponse> for ResponseKind {
    fn from(value: UpdateMetadataResponse) -> ResponseKind {
        ResponseKind::UpdateMetadata(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ControlledShutdownResponse> for ResponseKind {
    fn from(value: ControlledShutdownResponse) -> ResponseKind {
        ResponseKind::ControlledShutdown(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetCommitResponse> for ResponseKind {
    fn from(value: OffsetCommitResponse) -> ResponseKind {
        ResponseKind::OffsetCommit(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetFetchResponse> for ResponseKind {
    fn from(value: OffsetFetchResponse) -> ResponseKind {
        ResponseKind::OffsetFetch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<FindCoordinatorResponse> for ResponseKind {
    fn from(value: FindCoordinatorResponse) -> ResponseKind {
        ResponseKind::FindCoordinator(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<JoinGroupResponse> for ResponseKind {
    fn from(value: JoinGroupResponse) -> ResponseKind {
        ResponseKind::JoinGroup(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<HeartbeatResponse> for ResponseKind {
    fn from(value: HeartbeatResponse) -> ResponseKind {
        ResponseKind::Heartbeat(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<LeaveGroupResponse> for ResponseKind {
    fn from(value: LeaveGroupResponse) -> ResponseKind {
        ResponseKind::LeaveGroup(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<SyncGroupResponse> for ResponseKind {
    fn from(value: SyncGroupResponse) -> ResponseKind {
        ResponseKind::SyncGroup(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeGroupsResponse> for ResponseKind {
    fn from(value: DescribeGroupsResponse) -> ResponseKind {
        ResponseKind::DescribeGroups(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListGroupsResponse> for ResponseKind {
    fn from(value: ListGroupsResponse) -> ResponseKind {
        ResponseKind::ListGroups(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<SaslHandshakeResponse> for ResponseKind {
    fn from(value: SaslHandshakeResponse) -> ResponseKind {
        ResponseKind::SaslHandshake(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ApiVersionsResponse> for ResponseKind {
    fn from(value: ApiVersionsResponse) -> ResponseKind {
        ResponseKind::ApiVersions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreateTopicsResponse> for ResponseKind {
    fn from(value: CreateTopicsResponse) -> ResponseKind {
        ResponseKind::CreateTopics(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteTopicsResponse> for ResponseKind {
    fn from(value: DeleteTopicsResponse) -> ResponseKind {
        ResponseKind::DeleteTopics(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteRecordsResponse> for ResponseKind {
    fn from(value: DeleteRecordsResponse) -> ResponseKind {
        ResponseKind::DeleteRecords(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<InitProducerIdResponse> for ResponseKind {
    fn from(value: InitProducerIdResponse) -> ResponseKind {
        ResponseKind::InitProducerId(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetForLeaderEpochResponse> for ResponseKind {
    fn from(value: OffsetForLeaderEpochResponse) -> ResponseKind {
        ResponseKind::OffsetForLeaderEpoch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AddPartitionsToTxnResponse> for ResponseKind {
    fn from(value: AddPartitionsToTxnResponse) -> ResponseKind {
        ResponseKind::AddPartitionsToTxn(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AddOffsetsToTxnResponse> for ResponseKind {
    fn from(value: AddOffsetsToTxnResponse) -> ResponseKind {
        ResponseKind::AddOffsetsToTxn(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<EndTxnResponse> for ResponseKind {
    fn from(value: EndTxnResponse) -> ResponseKind {
        ResponseKind::EndTxn(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<WriteTxnMarkersResponse> for ResponseKind {
    fn from(value: WriteTxnMarkersResponse) -> ResponseKind {
        ResponseKind::WriteTxnMarkers(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<TxnOffsetCommitResponse> for ResponseKind {
    fn from(value: TxnOffsetCommitResponse) -> ResponseKind {
        ResponseKind::TxnOffsetCommit(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeAclsResponse> for ResponseKind {
    fn from(value: DescribeAclsResponse) -> ResponseKind {
        ResponseKind::DescribeAcls(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreateAclsResponse> for ResponseKind {
    fn from(value: CreateAclsResponse) -> ResponseKind {
        ResponseKind::CreateAcls(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteAclsResponse> for ResponseKind {
    fn from(value: DeleteAclsResponse) -> ResponseKind {
        ResponseKind::DeleteAcls(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeConfigsResponse> for ResponseKind {
    fn from(value: DescribeConfigsResponse) -> ResponseKind {
        ResponseKind::DescribeConfigs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterConfigsResponse> for ResponseKind {
    fn from(value: AlterConfigsResponse) -> ResponseKind {
        ResponseKind::AlterConfigs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterReplicaLogDirsResponse> for ResponseKind {
    fn from(value: AlterReplicaLogDirsResponse) -> ResponseKind {
        ResponseKind::AlterReplicaLogDirs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeLogDirsResponse> for ResponseKind {
    fn from(value: DescribeLogDirsResponse) -> ResponseKind {
        ResponseKind::DescribeLogDirs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<SaslAuthenticateResponse> for ResponseKind {
    fn from(value: SaslAuthenticateResponse) -> ResponseKind {
        ResponseKind::SaslAuthenticate(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreatePartitionsResponse> for ResponseKind {
    fn from(value: CreatePartitionsResponse) -> ResponseKind {
        ResponseKind::CreatePartitions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<CreateDelegationTokenResponse> for ResponseKind {
    fn from(value: CreateDelegationTokenResponse) -> ResponseKind {
        ResponseKind::CreateDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<RenewDelegationTokenResponse> for ResponseKind {
    fn from(value: RenewDelegationTokenResponse) -> ResponseKind {
        ResponseKind::RenewDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ExpireDelegationTokenResponse> for ResponseKind {
    fn from(value: ExpireDelegationTokenResponse) -> ResponseKind {
        ResponseKind::ExpireDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeDelegationTokenResponse> for ResponseKind {
    fn from(value: DescribeDelegationTokenResponse) -> ResponseKind {
        ResponseKind::DescribeDelegationToken(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DeleteGroupsResponse> for ResponseKind {
    fn from(value: DeleteGroupsResponse) -> ResponseKind {
        ResponseKind::DeleteGroups(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ElectLeadersResponse> for ResponseKind {
    fn from(value: ElectLeadersResponse) -> ResponseKind {
        ResponseKind::ElectLeaders(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<IncrementalAlterConfigsResponse> for ResponseKind {
    fn from(value: IncrementalAlterConfigsResponse) -> ResponseKind {
        ResponseKind::IncrementalAlterConfigs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterPartitionReassignmentsResponse> for ResponseKind {
    fn from(value: AlterPartitionReassignmentsResponse) -> ResponseKind {
        ResponseKind::AlterPartitionReassignments(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListPartitionReassignmentsResponse> for ResponseKind {
    fn from(value: ListPartitionReassignmentsResponse) -> ResponseKind {
        ResponseKind::ListPartitionReassignments(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<OffsetDeleteResponse> for ResponseKind {
    fn from(value: OffsetDeleteResponse) -> ResponseKind {
        ResponseKind::OffsetDelete(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeClientQuotasResponse> for ResponseKind {
    fn from(value: DescribeClientQuotasResponse) -> ResponseKind {
        ResponseKind::DescribeClientQuotas(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterClientQuotasResponse> for ResponseKind {
    fn from(value: AlterClientQuotasResponse) -> ResponseKind {
        ResponseKind::AlterClientQuotas(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeUserScramCredentialsResponse> for ResponseKind {
    fn from(value: DescribeUserScramCredentialsResponse) -> ResponseKind {
        ResponseKind::DescribeUserScramCredentials(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterUserScramCredentialsResponse> for ResponseKind {
    fn from(value: AlterUserScramCredentialsResponse) -> ResponseKind {
        ResponseKind::AlterUserScramCredentials(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<VoteResponse> for ResponseKind {
    fn from(value: VoteResponse) -> ResponseKind {
        ResponseKind::Vote(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<BeginQuorumEpochResponse> for ResponseKind {
    fn from(value: BeginQuorumEpochResponse) -> ResponseKind {
        ResponseKind::BeginQuorumEpoch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<EndQuorumEpochResponse> for ResponseKind {
    fn from(value: EndQuorumEpochResponse) -> ResponseKind {
        ResponseKind::EndQuorumEpoch(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeQuorumResponse> for ResponseKind {
    fn from(value: DescribeQuorumResponse) -> ResponseKind {
        ResponseKind::DescribeQuorum(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AlterPartitionResponse> for ResponseKind {
    fn from(value: AlterPartitionResponse) -> ResponseKind {
        ResponseKind::AlterPartition(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<UpdateFeaturesResponse> for ResponseKind {
    fn from(value: UpdateFeaturesResponse) -> ResponseKind {
        ResponseKind::UpdateFeatures(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<EnvelopeResponse> for ResponseKind {
    fn from(value: EnvelopeResponse) -> ResponseKind {
        ResponseKind::Envelope(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<FetchSnapshotResponse> for ResponseKind {
    fn from(value: FetchSnapshotResponse) -> ResponseKind {
        ResponseKind::FetchSnapshot(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeClusterResponse> for ResponseKind {
    fn from(value: DescribeClusterResponse) -> ResponseKind {
        ResponseKind::DescribeCluster(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeProducersResponse> for ResponseKind {
    fn from(value: DescribeProducersResponse) -> ResponseKind {
        ResponseKind::DescribeProducers(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<BrokerRegistrationResponse> for ResponseKind {
    fn from(value: BrokerRegistrationResponse) -> ResponseKind {
        ResponseKind::BrokerRegistration(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<BrokerHeartbeatResponse> for ResponseKind {
    fn from(value: BrokerHeartbeatResponse) -> ResponseKind {
        ResponseKind::BrokerHeartbeat(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<UnregisterBrokerResponse> for ResponseKind {
    fn from(value: UnregisterBrokerResponse) -> ResponseKind {
        ResponseKind::UnregisterBroker(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeTransactionsResponse> for ResponseKind {
    fn from(value: DescribeTransactionsResponse) -> ResponseKind {
        ResponseKind::DescribeTransactions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListTransactionsResponse> for ResponseKind {
    fn from(value: ListTransactionsResponse) -> ResponseKind {
        ResponseKind::ListTransactions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AllocateProducerIdsResponse> for ResponseKind {
    fn from(value: AllocateProducerIdsResponse) -> ResponseKind {
        ResponseKind::AllocateProducerIds(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ConsumerGroupHeartbeatResponse> for ResponseKind {
    fn from(value: ConsumerGroupHeartbeatResponse) -> ResponseKind {
        ResponseKind::ConsumerGroupHeartbeat(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ConsumerGroupDescribeResponse> for ResponseKind {
    fn from(value: ConsumerGroupDescribeResponse) -> ResponseKind {
        ResponseKind::ConsumerGroupDescribe(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ControllerRegistrationResponse> for ResponseKind {
    fn from(value: ControllerRegistrationResponse) -> ResponseKind {
        ResponseKind::ControllerRegistration(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<GetTelemetrySubscriptionsResponse> for ResponseKind {
    fn from(value: GetTelemetrySubscriptionsResponse) -> ResponseKind {
        ResponseKind::GetTelemetrySubscriptions(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<PushTelemetryResponse> for ResponseKind {
    fn from(value: PushTelemetryResponse) -> ResponseKind {
        ResponseKind::PushTelemetry(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<AssignReplicasToDirsResponse> for ResponseKind {
    fn from(value: AssignReplicasToDirsResponse) -> ResponseKind {
        ResponseKind::AssignReplicasToDirs(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<ListClientMetricsResourcesResponse> for ResponseKind {
    fn from(value: ListClientMetricsResourcesResponse) -> ResponseKind {
        ResponseKind::ListClientMetricsResources(value)
    }
}

#[cfg(feature = "messages_enums")]
impl From<DescribeTopicPartitionsResponse> for ResponseKind {
    fn from(value: DescribeTopicPartitionsResponse) -> ResponseKind {
        ResponseKind::DescribeTopicPartitions(value)
    }
}

/// The ID of the leader broker.
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

/// The group ID string.
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

/// The topic name.
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
