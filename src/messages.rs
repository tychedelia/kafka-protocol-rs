//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.

use protocol_base::{NewType, Request, StrBytes};
use std::convert::TryFrom;

pub mod add_offsets_to_txn_request;
pub use add_offsets_to_txn_request::AddOffsetsToTxnRequest;

pub mod add_offsets_to_txn_response;
pub use add_offsets_to_txn_response::AddOffsetsToTxnResponse;

pub mod add_partitions_to_txn_request;
pub use add_partitions_to_txn_request::AddPartitionsToTxnRequest;

pub mod add_partitions_to_txn_response;
pub use add_partitions_to_txn_response::AddPartitionsToTxnResponse;

pub mod allocate_producer_ids_request;
pub use allocate_producer_ids_request::AllocateProducerIdsRequest;

pub mod allocate_producer_ids_response;
pub use allocate_producer_ids_response::AllocateProducerIdsResponse;

pub mod alter_client_quotas_request;
pub use alter_client_quotas_request::AlterClientQuotasRequest;

pub mod alter_client_quotas_response;
pub use alter_client_quotas_response::AlterClientQuotasResponse;

pub mod alter_configs_request;
pub use alter_configs_request::AlterConfigsRequest;

pub mod alter_configs_response;
pub use alter_configs_response::AlterConfigsResponse;

pub mod alter_isr_request;
pub use alter_isr_request::AlterIsrRequest;

pub mod alter_isr_response;
pub use alter_isr_response::AlterIsrResponse;

pub mod alter_partition_reassignments_request;
pub use alter_partition_reassignments_request::AlterPartitionReassignmentsRequest;

pub mod alter_partition_reassignments_response;
pub use alter_partition_reassignments_response::AlterPartitionReassignmentsResponse;

pub mod alter_replica_log_dirs_request;
pub use alter_replica_log_dirs_request::AlterReplicaLogDirsRequest;

pub mod alter_replica_log_dirs_response;
pub use alter_replica_log_dirs_response::AlterReplicaLogDirsResponse;

pub mod alter_user_scram_credentials_request;
pub use alter_user_scram_credentials_request::AlterUserScramCredentialsRequest;

pub mod alter_user_scram_credentials_response;
pub use alter_user_scram_credentials_response::AlterUserScramCredentialsResponse;

pub mod api_versions_request;
pub use api_versions_request::ApiVersionsRequest;

pub mod api_versions_response;
pub use api_versions_response::ApiVersionsResponse;

pub mod begin_quorum_epoch_request;
pub use begin_quorum_epoch_request::BeginQuorumEpochRequest;

pub mod begin_quorum_epoch_response;
pub use begin_quorum_epoch_response::BeginQuorumEpochResponse;

pub mod broker_heartbeat_request;
pub use broker_heartbeat_request::BrokerHeartbeatRequest;

pub mod broker_heartbeat_response;
pub use broker_heartbeat_response::BrokerHeartbeatResponse;

pub mod broker_registration_request;
pub use broker_registration_request::BrokerRegistrationRequest;

pub mod broker_registration_response;
pub use broker_registration_response::BrokerRegistrationResponse;

pub mod consumer_protocol_assignment;
pub use consumer_protocol_assignment::ConsumerProtocolAssignment;

pub mod consumer_protocol_subscription;
pub use consumer_protocol_subscription::ConsumerProtocolSubscription;

pub mod controlled_shutdown_request;
pub use controlled_shutdown_request::ControlledShutdownRequest;

pub mod controlled_shutdown_response;
pub use controlled_shutdown_response::ControlledShutdownResponse;

pub mod create_acls_request;
pub use create_acls_request::CreateAclsRequest;

pub mod create_acls_response;
pub use create_acls_response::CreateAclsResponse;

pub mod create_delegation_token_request;
pub use create_delegation_token_request::CreateDelegationTokenRequest;

pub mod create_delegation_token_response;
pub use create_delegation_token_response::CreateDelegationTokenResponse;

pub mod create_partitions_request;
pub use create_partitions_request::CreatePartitionsRequest;

pub mod create_partitions_response;
pub use create_partitions_response::CreatePartitionsResponse;

pub mod create_topics_request;
pub use create_topics_request::CreateTopicsRequest;

pub mod create_topics_response;
pub use create_topics_response::CreateTopicsResponse;

pub mod default_principal_data;
pub use default_principal_data::DefaultPrincipalData;

pub mod delete_acls_request;
pub use delete_acls_request::DeleteAclsRequest;

pub mod delete_acls_response;
pub use delete_acls_response::DeleteAclsResponse;

pub mod delete_groups_request;
pub use delete_groups_request::DeleteGroupsRequest;

pub mod delete_groups_response;
pub use delete_groups_response::DeleteGroupsResponse;

pub mod delete_records_request;
pub use delete_records_request::DeleteRecordsRequest;

pub mod delete_records_response;
pub use delete_records_response::DeleteRecordsResponse;

pub mod delete_topics_request;
pub use delete_topics_request::DeleteTopicsRequest;

pub mod delete_topics_response;
pub use delete_topics_response::DeleteTopicsResponse;

pub mod describe_acls_request;
pub use describe_acls_request::DescribeAclsRequest;

pub mod describe_acls_response;
pub use describe_acls_response::DescribeAclsResponse;

pub mod describe_client_quotas_request;
pub use describe_client_quotas_request::DescribeClientQuotasRequest;

pub mod describe_client_quotas_response;
pub use describe_client_quotas_response::DescribeClientQuotasResponse;

pub mod describe_cluster_request;
pub use describe_cluster_request::DescribeClusterRequest;

pub mod describe_cluster_response;
pub use describe_cluster_response::DescribeClusterResponse;

pub mod describe_configs_request;
pub use describe_configs_request::DescribeConfigsRequest;

pub mod describe_configs_response;
pub use describe_configs_response::DescribeConfigsResponse;

pub mod describe_delegation_token_request;
pub use describe_delegation_token_request::DescribeDelegationTokenRequest;

pub mod describe_delegation_token_response;
pub use describe_delegation_token_response::DescribeDelegationTokenResponse;

pub mod describe_groups_request;
pub use describe_groups_request::DescribeGroupsRequest;

pub mod describe_groups_response;
pub use describe_groups_response::DescribeGroupsResponse;

pub mod describe_log_dirs_request;
pub use describe_log_dirs_request::DescribeLogDirsRequest;

pub mod describe_log_dirs_response;
pub use describe_log_dirs_response::DescribeLogDirsResponse;

pub mod describe_producers_request;
pub use describe_producers_request::DescribeProducersRequest;

pub mod describe_producers_response;
pub use describe_producers_response::DescribeProducersResponse;

pub mod describe_quorum_request;
pub use describe_quorum_request::DescribeQuorumRequest;

pub mod describe_quorum_response;
pub use describe_quorum_response::DescribeQuorumResponse;

pub mod describe_transactions_request;
pub use describe_transactions_request::DescribeTransactionsRequest;

pub mod describe_transactions_response;
pub use describe_transactions_response::DescribeTransactionsResponse;

pub mod describe_user_scram_credentials_request;
pub use describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest;

pub mod describe_user_scram_credentials_response;
pub use describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse;

pub mod elect_leaders_request;
pub use elect_leaders_request::ElectLeadersRequest;

pub mod elect_leaders_response;
pub use elect_leaders_response::ElectLeadersResponse;

pub mod end_quorum_epoch_request;
pub use end_quorum_epoch_request::EndQuorumEpochRequest;

pub mod end_quorum_epoch_response;
pub use end_quorum_epoch_response::EndQuorumEpochResponse;

pub mod end_txn_request;
pub use end_txn_request::EndTxnRequest;

pub mod end_txn_response;
pub use end_txn_response::EndTxnResponse;

pub mod envelope_request;
pub use envelope_request::EnvelopeRequest;

pub mod envelope_response;
pub use envelope_response::EnvelopeResponse;

pub mod expire_delegation_token_request;
pub use expire_delegation_token_request::ExpireDelegationTokenRequest;

pub mod expire_delegation_token_response;
pub use expire_delegation_token_response::ExpireDelegationTokenResponse;

pub mod fetch_request;
pub use fetch_request::FetchRequest;

pub mod fetch_response;
pub use fetch_response::FetchResponse;

pub mod fetch_snapshot_request;
pub use fetch_snapshot_request::FetchSnapshotRequest;

pub mod fetch_snapshot_response;
pub use fetch_snapshot_response::FetchSnapshotResponse;

pub mod find_coordinator_request;
pub use find_coordinator_request::FindCoordinatorRequest;

pub mod find_coordinator_response;
pub use find_coordinator_response::FindCoordinatorResponse;

pub mod heartbeat_request;
pub use heartbeat_request::HeartbeatRequest;

pub mod heartbeat_response;
pub use heartbeat_response::HeartbeatResponse;

pub mod incremental_alter_configs_request;
pub use incremental_alter_configs_request::IncrementalAlterConfigsRequest;

pub mod incremental_alter_configs_response;
pub use incremental_alter_configs_response::IncrementalAlterConfigsResponse;

pub mod init_producer_id_request;
pub use init_producer_id_request::InitProducerIdRequest;

pub mod init_producer_id_response;
pub use init_producer_id_response::InitProducerIdResponse;

pub mod join_group_request;
pub use join_group_request::JoinGroupRequest;

pub mod join_group_response;
pub use join_group_response::JoinGroupResponse;

pub mod leader_and_isr_request;
pub use leader_and_isr_request::LeaderAndIsrRequest;

pub mod leader_and_isr_response;
pub use leader_and_isr_response::LeaderAndIsrResponse;

pub mod leader_change_message;
pub use leader_change_message::LeaderChangeMessage;

pub mod leave_group_request;
pub use leave_group_request::LeaveGroupRequest;

pub mod leave_group_response;
pub use leave_group_response::LeaveGroupResponse;

pub mod list_groups_request;
pub use list_groups_request::ListGroupsRequest;

pub mod list_groups_response;
pub use list_groups_response::ListGroupsResponse;

pub mod list_offsets_request;
pub use list_offsets_request::ListOffsetsRequest;

pub mod list_offsets_response;
pub use list_offsets_response::ListOffsetsResponse;

pub mod list_partition_reassignments_request;
pub use list_partition_reassignments_request::ListPartitionReassignmentsRequest;

pub mod list_partition_reassignments_response;
pub use list_partition_reassignments_response::ListPartitionReassignmentsResponse;

pub mod list_transactions_request;
pub use list_transactions_request::ListTransactionsRequest;

pub mod list_transactions_response;
pub use list_transactions_response::ListTransactionsResponse;

pub mod metadata_request;
pub use metadata_request::MetadataRequest;

pub mod metadata_response;
pub use metadata_response::MetadataResponse;

pub mod offset_commit_request;
pub use offset_commit_request::OffsetCommitRequest;

pub mod offset_commit_response;
pub use offset_commit_response::OffsetCommitResponse;

pub mod offset_delete_request;
pub use offset_delete_request::OffsetDeleteRequest;

pub mod offset_delete_response;
pub use offset_delete_response::OffsetDeleteResponse;

pub mod offset_fetch_request;
pub use offset_fetch_request::OffsetFetchRequest;

pub mod offset_fetch_response;
pub use offset_fetch_response::OffsetFetchResponse;

pub mod offset_for_leader_epoch_request;
pub use offset_for_leader_epoch_request::OffsetForLeaderEpochRequest;

pub mod offset_for_leader_epoch_response;
pub use offset_for_leader_epoch_response::OffsetForLeaderEpochResponse;

pub mod produce_request;
pub use produce_request::ProduceRequest;

pub mod produce_response;
pub use produce_response::ProduceResponse;

pub mod renew_delegation_token_request;
pub use renew_delegation_token_request::RenewDelegationTokenRequest;

pub mod renew_delegation_token_response;
pub use renew_delegation_token_response::RenewDelegationTokenResponse;

pub mod request_header;
pub use request_header::RequestHeader;

pub mod response_header;
pub use response_header::ResponseHeader;

pub mod sasl_authenticate_request;
pub use sasl_authenticate_request::SaslAuthenticateRequest;

pub mod sasl_authenticate_response;
pub use sasl_authenticate_response::SaslAuthenticateResponse;

pub mod sasl_handshake_request;
pub use sasl_handshake_request::SaslHandshakeRequest;

pub mod sasl_handshake_response;
pub use sasl_handshake_response::SaslHandshakeResponse;

pub mod snapshot_footer_record;
pub use snapshot_footer_record::SnapshotFooterRecord;

pub mod snapshot_header_record;
pub use snapshot_header_record::SnapshotHeaderRecord;

pub mod stop_replica_request;
pub use stop_replica_request::StopReplicaRequest;

pub mod stop_replica_response;
pub use stop_replica_response::StopReplicaResponse;

pub mod sync_group_request;
pub use sync_group_request::SyncGroupRequest;

pub mod sync_group_response;
pub use sync_group_response::SyncGroupResponse;

pub mod txn_offset_commit_request;
pub use txn_offset_commit_request::TxnOffsetCommitRequest;

pub mod txn_offset_commit_response;
pub use txn_offset_commit_response::TxnOffsetCommitResponse;

pub mod unregister_broker_request;
pub use unregister_broker_request::UnregisterBrokerRequest;

pub mod unregister_broker_response;
pub use unregister_broker_response::UnregisterBrokerResponse;

pub mod update_features_request;
pub use update_features_request::UpdateFeaturesRequest;

pub mod update_features_response;
pub use update_features_response::UpdateFeaturesResponse;

pub mod update_metadata_request;
pub use update_metadata_request::UpdateMetadataRequest;

pub mod update_metadata_response;
pub use update_metadata_response::UpdateMetadataResponse;

pub mod vote_request;
pub use vote_request::VoteRequest;

pub mod vote_response;
pub use vote_response::VoteResponse;

pub mod write_txn_markers_request;
pub use write_txn_markers_request::WriteTxnMarkersRequest;

pub mod write_txn_markers_response;
pub use write_txn_markers_response::WriteTxnMarkersResponse;

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

impl Request for AlterIsrRequest {
    const KEY: i16 = 56;
    type Response = AlterIsrResponse;
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

pub enum ApiKey {
    ProduceRequest = 0,
    FetchRequest = 1,
    ListOffsetsRequest = 2,
    MetadataRequest = 3,
    LeaderAndIsrRequest = 4,
    StopReplicaRequest = 5,
    UpdateMetadataRequest = 6,
    ControlledShutdownRequest = 7,
    OffsetCommitRequest = 8,
    OffsetFetchRequest = 9,
    FindCoordinatorRequest = 10,
    JoinGroupRequest = 11,
    HeartbeatRequest = 12,
    LeaveGroupRequest = 13,
    SyncGroupRequest = 14,
    DescribeGroupsRequest = 15,
    ListGroupsRequest = 16,
    SaslHandshakeRequest = 17,
    ApiVersionsRequest = 18,
    CreateTopicsRequest = 19,
    DeleteTopicsRequest = 20,
    DeleteRecordsRequest = 21,
    InitProducerIdRequest = 22,
    OffsetForLeaderEpochRequest = 23,
    AddPartitionsToTxnRequest = 24,
    AddOffsetsToTxnRequest = 25,
    EndTxnRequest = 26,
    WriteTxnMarkersRequest = 27,
    TxnOffsetCommitRequest = 28,
    DescribeAclsRequest = 29,
    CreateAclsRequest = 30,
    DeleteAclsRequest = 31,
    DescribeConfigsRequest = 32,
    AlterConfigsRequest = 33,
    AlterReplicaLogDirsRequest = 34,
    DescribeLogDirsRequest = 35,
    SaslAuthenticateRequest = 36,
    CreatePartitionsRequest = 37,
    CreateDelegationTokenRequest = 38,
    RenewDelegationTokenRequest = 39,
    ExpireDelegationTokenRequest = 40,
    DescribeDelegationTokenRequest = 41,
    DeleteGroupsRequest = 42,
    ElectLeadersRequest = 43,
    IncrementalAlterConfigsRequest = 44,
    AlterPartitionReassignmentsRequest = 45,
    ListPartitionReassignmentsRequest = 46,
    OffsetDeleteRequest = 47,
    DescribeClientQuotasRequest = 48,
    AlterClientQuotasRequest = 49,
    DescribeUserScramCredentialsRequest = 50,
    AlterUserScramCredentialsRequest = 51,
    VoteRequest = 52,
    BeginQuorumEpochRequest = 53,
    EndQuorumEpochRequest = 54,
    DescribeQuorumRequest = 55,
    AlterIsrRequest = 56,
    UpdateFeaturesRequest = 57,
    EnvelopeRequest = 58,
    FetchSnapshotRequest = 59,
    DescribeClusterRequest = 60,
    DescribeProducersRequest = 61,
    BrokerRegistrationRequest = 62,
    BrokerHeartbeatRequest = 63,
    UnregisterBrokerRequest = 64,
    DescribeTransactionsRequest = 65,
    ListTransactionsRequest = 66,
    AllocateProducerIdsRequest = 67,
}

impl TryFrom<i16> for ApiKey {
    type Error = ();

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == ApiKey::ProduceRequest as i16 => Ok(ApiKey::ProduceRequest),
            x if x == ApiKey::FetchRequest as i16 => Ok(ApiKey::FetchRequest),
            x if x == ApiKey::ListOffsetsRequest as i16 => Ok(ApiKey::ListOffsetsRequest),
            x if x == ApiKey::MetadataRequest as i16 => Ok(ApiKey::MetadataRequest),
            x if x == ApiKey::LeaderAndIsrRequest as i16 => Ok(ApiKey::LeaderAndIsrRequest),
            x if x == ApiKey::StopReplicaRequest as i16 => Ok(ApiKey::StopReplicaRequest),
            x if x == ApiKey::UpdateMetadataRequest as i16 => Ok(ApiKey::UpdateMetadataRequest),
            x if x == ApiKey::ControlledShutdownRequest as i16 => Ok(ApiKey::ControlledShutdownRequest),
            x if x == ApiKey::OffsetCommitRequest as i16 => Ok(ApiKey::OffsetCommitRequest),
            x if x == ApiKey::OffsetFetchRequest as i16 => Ok(ApiKey::OffsetFetchRequest),
            x if x == ApiKey::FindCoordinatorRequest as i16 => Ok(ApiKey::FindCoordinatorRequest),
            x if x == ApiKey::JoinGroupRequest as i16 => Ok(ApiKey::JoinGroupRequest),
            x if x == ApiKey::HeartbeatRequest as i16 => Ok(ApiKey::HeartbeatRequest),
            x if x == ApiKey::LeaveGroupRequest as i16 => Ok(ApiKey::LeaveGroupRequest),
            x if x == ApiKey::SyncGroupRequest as i16 => Ok(ApiKey::SyncGroupRequest),
            x if x == ApiKey::DescribeGroupsRequest as i16 => Ok(ApiKey::DescribeGroupsRequest),
            x if x == ApiKey::ListGroupsRequest as i16 => Ok(ApiKey::ListGroupsRequest),
            x if x == ApiKey::SaslHandshakeRequest as i16 => Ok(ApiKey::SaslHandshakeRequest),
            x if x == ApiKey::ApiVersionsRequest as i16 => Ok(ApiKey::ApiVersionsRequest),
            x if x == ApiKey::CreateTopicsRequest as i16 => Ok(ApiKey::CreateTopicsRequest),
            x if x == ApiKey::DeleteTopicsRequest as i16 => Ok(ApiKey::DeleteTopicsRequest),
            x if x == ApiKey::DeleteRecordsRequest as i16 => Ok(ApiKey::DeleteRecordsRequest),
            x if x == ApiKey::InitProducerIdRequest as i16 => Ok(ApiKey::InitProducerIdRequest),
            x if x == ApiKey::OffsetForLeaderEpochRequest as i16 => Ok(ApiKey::OffsetForLeaderEpochRequest),
            x if x == ApiKey::AddPartitionsToTxnRequest as i16 => Ok(ApiKey::AddPartitionsToTxnRequest),
            x if x == ApiKey::AddOffsetsToTxnRequest as i16 => Ok(ApiKey::AddOffsetsToTxnRequest),
            x if x == ApiKey::EndTxnRequest as i16 => Ok(ApiKey::EndTxnRequest),
            x if x == ApiKey::WriteTxnMarkersRequest as i16 => Ok(ApiKey::WriteTxnMarkersRequest),
            x if x == ApiKey::TxnOffsetCommitRequest as i16 => Ok(ApiKey::TxnOffsetCommitRequest),
            x if x == ApiKey::DescribeAclsRequest as i16 => Ok(ApiKey::DescribeAclsRequest),
            x if x == ApiKey::CreateAclsRequest as i16 => Ok(ApiKey::CreateAclsRequest),
            x if x == ApiKey::DeleteAclsRequest as i16 => Ok(ApiKey::DeleteAclsRequest),
            x if x == ApiKey::DescribeConfigsRequest as i16 => Ok(ApiKey::DescribeConfigsRequest),
            x if x == ApiKey::AlterConfigsRequest as i16 => Ok(ApiKey::AlterConfigsRequest),
            x if x == ApiKey::AlterReplicaLogDirsRequest as i16 => Ok(ApiKey::AlterReplicaLogDirsRequest),
            x if x == ApiKey::DescribeLogDirsRequest as i16 => Ok(ApiKey::DescribeLogDirsRequest),
            x if x == ApiKey::SaslAuthenticateRequest as i16 => Ok(ApiKey::SaslAuthenticateRequest),
            x if x == ApiKey::CreatePartitionsRequest as i16 => Ok(ApiKey::CreatePartitionsRequest),
            x if x == ApiKey::CreateDelegationTokenRequest as i16 => Ok(ApiKey::CreateDelegationTokenRequest),
            x if x == ApiKey::RenewDelegationTokenRequest as i16 => Ok(ApiKey::RenewDelegationTokenRequest),
            x if x == ApiKey::ExpireDelegationTokenRequest as i16 => Ok(ApiKey::ExpireDelegationTokenRequest),
            x if x == ApiKey::DescribeDelegationTokenRequest as i16 => Ok(ApiKey::DescribeDelegationTokenRequest),
            x if x == ApiKey::DeleteGroupsRequest as i16 => Ok(ApiKey::DeleteGroupsRequest),
            x if x == ApiKey::ElectLeadersRequest as i16 => Ok(ApiKey::ElectLeadersRequest),
            x if x == ApiKey::IncrementalAlterConfigsRequest as i16 => Ok(ApiKey::IncrementalAlterConfigsRequest),
            x if x == ApiKey::AlterPartitionReassignmentsRequest as i16 => Ok(ApiKey::AlterPartitionReassignmentsRequest),
            x if x == ApiKey::ListPartitionReassignmentsRequest as i16 => Ok(ApiKey::ListPartitionReassignmentsRequest),
            x if x == ApiKey::OffsetDeleteRequest as i16 => Ok(ApiKey::OffsetDeleteRequest),
            x if x == ApiKey::DescribeClientQuotasRequest as i16 => Ok(ApiKey::DescribeClientQuotasRequest),
            x if x == ApiKey::AlterClientQuotasRequest as i16 => Ok(ApiKey::AlterClientQuotasRequest),
            x if x == ApiKey::DescribeUserScramCredentialsRequest as i16 => Ok(ApiKey::DescribeUserScramCredentialsRequest),
            x if x == ApiKey::AlterUserScramCredentialsRequest as i16 => Ok(ApiKey::AlterUserScramCredentialsRequest),
            x if x == ApiKey::VoteRequest as i16 => Ok(ApiKey::VoteRequest),
            x if x == ApiKey::BeginQuorumEpochRequest as i16 => Ok(ApiKey::BeginQuorumEpochRequest),
            x if x == ApiKey::EndQuorumEpochRequest as i16 => Ok(ApiKey::EndQuorumEpochRequest),
            x if x == ApiKey::DescribeQuorumRequest as i16 => Ok(ApiKey::DescribeQuorumRequest),
            x if x == ApiKey::AlterIsrRequest as i16 => Ok(ApiKey::AlterIsrRequest),
            x if x == ApiKey::UpdateFeaturesRequest as i16 => Ok(ApiKey::UpdateFeaturesRequest),
            x if x == ApiKey::EnvelopeRequest as i16 => Ok(ApiKey::EnvelopeRequest),
            x if x == ApiKey::FetchSnapshotRequest as i16 => Ok(ApiKey::FetchSnapshotRequest),
            x if x == ApiKey::DescribeClusterRequest as i16 => Ok(ApiKey::DescribeClusterRequest),
            x if x == ApiKey::DescribeProducersRequest as i16 => Ok(ApiKey::DescribeProducersRequest),
            x if x == ApiKey::BrokerRegistrationRequest as i16 => Ok(ApiKey::BrokerRegistrationRequest),
            x if x == ApiKey::BrokerHeartbeatRequest as i16 => Ok(ApiKey::BrokerHeartbeatRequest),
            x if x == ApiKey::UnregisterBrokerRequest as i16 => Ok(ApiKey::UnregisterBrokerRequest),
            x if x == ApiKey::DescribeTransactionsRequest as i16 => Ok(ApiKey::DescribeTransactionsRequest),
            x if x == ApiKey::ListTransactionsRequest as i16 => Ok(ApiKey::ListTransactionsRequest),
            x if x == ApiKey::AllocateProducerIdsRequest as i16 => Ok(ApiKey::AllocateProducerIdsRequest),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum RequestKind {
    ProduceRequest(ProduceRequest),
    FetchRequest(FetchRequest),
    ListOffsetsRequest(ListOffsetsRequest),
    MetadataRequest(MetadataRequest),
    LeaderAndIsrRequest(LeaderAndIsrRequest),
    StopReplicaRequest(StopReplicaRequest),
    UpdateMetadataRequest(UpdateMetadataRequest),
    ControlledShutdownRequest(ControlledShutdownRequest),
    OffsetCommitRequest(OffsetCommitRequest),
    OffsetFetchRequest(OffsetFetchRequest),
    FindCoordinatorRequest(FindCoordinatorRequest),
    JoinGroupRequest(JoinGroupRequest),
    HeartbeatRequest(HeartbeatRequest),
    LeaveGroupRequest(LeaveGroupRequest),
    SyncGroupRequest(SyncGroupRequest),
    DescribeGroupsRequest(DescribeGroupsRequest),
    ListGroupsRequest(ListGroupsRequest),
    SaslHandshakeRequest(SaslHandshakeRequest),
    ApiVersionsRequest(ApiVersionsRequest),
    CreateTopicsRequest(CreateTopicsRequest),
    DeleteTopicsRequest(DeleteTopicsRequest),
    DeleteRecordsRequest(DeleteRecordsRequest),
    InitProducerIdRequest(InitProducerIdRequest),
    OffsetForLeaderEpochRequest(OffsetForLeaderEpochRequest),
    AddPartitionsToTxnRequest(AddPartitionsToTxnRequest),
    AddOffsetsToTxnRequest(AddOffsetsToTxnRequest),
    EndTxnRequest(EndTxnRequest),
    WriteTxnMarkersRequest(WriteTxnMarkersRequest),
    TxnOffsetCommitRequest(TxnOffsetCommitRequest),
    DescribeAclsRequest(DescribeAclsRequest),
    CreateAclsRequest(CreateAclsRequest),
    DeleteAclsRequest(DeleteAclsRequest),
    DescribeConfigsRequest(DescribeConfigsRequest),
    AlterConfigsRequest(AlterConfigsRequest),
    AlterReplicaLogDirsRequest(AlterReplicaLogDirsRequest),
    DescribeLogDirsRequest(DescribeLogDirsRequest),
    SaslAuthenticateRequest(SaslAuthenticateRequest),
    CreatePartitionsRequest(CreatePartitionsRequest),
    CreateDelegationTokenRequest(CreateDelegationTokenRequest),
    RenewDelegationTokenRequest(RenewDelegationTokenRequest),
    ExpireDelegationTokenRequest(ExpireDelegationTokenRequest),
    DescribeDelegationTokenRequest(DescribeDelegationTokenRequest),
    DeleteGroupsRequest(DeleteGroupsRequest),
    ElectLeadersRequest(ElectLeadersRequest),
    IncrementalAlterConfigsRequest(IncrementalAlterConfigsRequest),
    AlterPartitionReassignmentsRequest(AlterPartitionReassignmentsRequest),
    ListPartitionReassignmentsRequest(ListPartitionReassignmentsRequest),
    OffsetDeleteRequest(OffsetDeleteRequest),
    DescribeClientQuotasRequest(DescribeClientQuotasRequest),
    AlterClientQuotasRequest(AlterClientQuotasRequest),
    DescribeUserScramCredentialsRequest(DescribeUserScramCredentialsRequest),
    AlterUserScramCredentialsRequest(AlterUserScramCredentialsRequest),
    VoteRequest(VoteRequest),
    BeginQuorumEpochRequest(BeginQuorumEpochRequest),
    EndQuorumEpochRequest(EndQuorumEpochRequest),
    DescribeQuorumRequest(DescribeQuorumRequest),
    AlterIsrRequest(AlterIsrRequest),
    UpdateFeaturesRequest(UpdateFeaturesRequest),
    EnvelopeRequest(EnvelopeRequest),
    FetchSnapshotRequest(FetchSnapshotRequest),
    DescribeClusterRequest(DescribeClusterRequest),
    DescribeProducersRequest(DescribeProducersRequest),
    BrokerRegistrationRequest(BrokerRegistrationRequest),
    BrokerHeartbeatRequest(BrokerHeartbeatRequest),
    UnregisterBrokerRequest(UnregisterBrokerRequest),
    DescribeTransactionsRequest(DescribeTransactionsRequest),
    ListTransactionsRequest(ListTransactionsRequest),
    AllocateProducerIdsRequest(AllocateProducerIdsRequest),
}

#[derive(Debug)]
pub enum ResponseKind {
    ProduceResponse(ProduceResponse),
    FetchResponse(FetchResponse),
    ListOffsetsResponse(ListOffsetsResponse),
    MetadataResponse(MetadataResponse),
    LeaderAndIsrResponse(LeaderAndIsrResponse),
    StopReplicaResponse(StopReplicaResponse),
    UpdateMetadataResponse(UpdateMetadataResponse),
    ControlledShutdownResponse(ControlledShutdownResponse),
    OffsetCommitResponse(OffsetCommitResponse),
    OffsetFetchResponse(OffsetFetchResponse),
    FindCoordinatorResponse(FindCoordinatorResponse),
    JoinGroupResponse(JoinGroupResponse),
    HeartbeatResponse(HeartbeatResponse),
    LeaveGroupResponse(LeaveGroupResponse),
    SyncGroupResponse(SyncGroupResponse),
    DescribeGroupsResponse(DescribeGroupsResponse),
    ListGroupsResponse(ListGroupsResponse),
    SaslHandshakeResponse(SaslHandshakeResponse),
    ApiVersionsResponse(ApiVersionsResponse),
    CreateTopicsResponse(CreateTopicsResponse),
    DeleteTopicsResponse(DeleteTopicsResponse),
    DeleteRecordsResponse(DeleteRecordsResponse),
    InitProducerIdResponse(InitProducerIdResponse),
    OffsetForLeaderEpochResponse(OffsetForLeaderEpochResponse),
    AddPartitionsToTxnResponse(AddPartitionsToTxnResponse),
    AddOffsetsToTxnResponse(AddOffsetsToTxnResponse),
    EndTxnResponse(EndTxnResponse),
    WriteTxnMarkersResponse(WriteTxnMarkersResponse),
    TxnOffsetCommitResponse(TxnOffsetCommitResponse),
    DescribeAclsResponse(DescribeAclsResponse),
    CreateAclsResponse(CreateAclsResponse),
    DeleteAclsResponse(DeleteAclsResponse),
    DescribeConfigsResponse(DescribeConfigsResponse),
    AlterConfigsResponse(AlterConfigsResponse),
    AlterReplicaLogDirsResponse(AlterReplicaLogDirsResponse),
    DescribeLogDirsResponse(DescribeLogDirsResponse),
    SaslAuthenticateResponse(SaslAuthenticateResponse),
    CreatePartitionsResponse(CreatePartitionsResponse),
    CreateDelegationTokenResponse(CreateDelegationTokenResponse),
    RenewDelegationTokenResponse(RenewDelegationTokenResponse),
    ExpireDelegationTokenResponse(ExpireDelegationTokenResponse),
    DescribeDelegationTokenResponse(DescribeDelegationTokenResponse),
    DeleteGroupsResponse(DeleteGroupsResponse),
    ElectLeadersResponse(ElectLeadersResponse),
    IncrementalAlterConfigsResponse(IncrementalAlterConfigsResponse),
    AlterPartitionReassignmentsResponse(AlterPartitionReassignmentsResponse),
    ListPartitionReassignmentsResponse(ListPartitionReassignmentsResponse),
    OffsetDeleteResponse(OffsetDeleteResponse),
    DescribeClientQuotasResponse(DescribeClientQuotasResponse),
    AlterClientQuotasResponse(AlterClientQuotasResponse),
    DescribeUserScramCredentialsResponse(DescribeUserScramCredentialsResponse),
    AlterUserScramCredentialsResponse(AlterUserScramCredentialsResponse),
    VoteResponse(VoteResponse),
    BeginQuorumEpochResponse(BeginQuorumEpochResponse),
    EndQuorumEpochResponse(EndQuorumEpochResponse),
    DescribeQuorumResponse(DescribeQuorumResponse),
    AlterIsrResponse(AlterIsrResponse),
    UpdateFeaturesResponse(UpdateFeaturesResponse),
    EnvelopeResponse(EnvelopeResponse),
    FetchSnapshotResponse(FetchSnapshotResponse),
    DescribeClusterResponse(DescribeClusterResponse),
    DescribeProducersResponse(DescribeProducersResponse),
    BrokerRegistrationResponse(BrokerRegistrationResponse),
    BrokerHeartbeatResponse(BrokerHeartbeatResponse),
    UnregisterBrokerResponse(UnregisterBrokerResponse),
    DescribeTransactionsResponse(DescribeTransactionsResponse),
    ListTransactionsResponse(ListTransactionsResponse),
    AllocateProducerIdsResponse(AllocateProducerIdsResponse),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy)]
pub struct BrokerId(pub i32);

impl From<i32> for BrokerId {
    fn from(other: i32) -> Self { Self(other) }
}
impl From<BrokerId> for i32 {
    fn from(other: BrokerId) -> Self { other.0 }
}
impl std::borrow::Borrow<i32> for BrokerId {
    fn borrow(&self) -> &i32 { &self.0 }
}
impl std::ops::Deref for BrokerId {
    type Target = i32;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::cmp::PartialEq<i32> for BrokerId {
    fn eq(&self, other: &i32) -> bool { &self.0 == other }
}
impl std::cmp::PartialEq<BrokerId> for i32 {
    fn eq(&self, other: &BrokerId) -> bool { self == &other.0 }
}
impl NewType<i32> for BrokerId {}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct GroupId(pub StrBytes);

impl From<StrBytes> for GroupId {
    fn from(other: StrBytes) -> Self { Self(other) }
}
impl From<GroupId> for StrBytes {
    fn from(other: GroupId) -> Self { other.0 }
}
impl std::borrow::Borrow<StrBytes> for GroupId {
    fn borrow(&self) -> &StrBytes { &self.0 }
}
impl std::ops::Deref for GroupId {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::cmp::PartialEq<StrBytes> for GroupId {
    fn eq(&self, other: &StrBytes) -> bool { &self.0 == other }
}
impl std::cmp::PartialEq<GroupId> for StrBytes {
    fn eq(&self, other: &GroupId) -> bool { self == &other.0 }
}
impl NewType<StrBytes> for GroupId {}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy)]
pub struct ProducerId(pub i64);

impl From<i64> for ProducerId {
    fn from(other: i64) -> Self { Self(other) }
}
impl From<ProducerId> for i64 {
    fn from(other: ProducerId) -> Self { other.0 }
}
impl std::borrow::Borrow<i64> for ProducerId {
    fn borrow(&self) -> &i64 { &self.0 }
}
impl std::ops::Deref for ProducerId {
    type Target = i64;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::cmp::PartialEq<i64> for ProducerId {
    fn eq(&self, other: &i64) -> bool { &self.0 == other }
}
impl std::cmp::PartialEq<ProducerId> for i64 {
    fn eq(&self, other: &ProducerId) -> bool { self == &other.0 }
}
impl NewType<i64> for ProducerId {}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TopicName(pub StrBytes);

impl From<StrBytes> for TopicName {
    fn from(other: StrBytes) -> Self { Self(other) }
}
impl From<TopicName> for StrBytes {
    fn from(other: TopicName) -> Self { other.0 }
}
impl std::borrow::Borrow<StrBytes> for TopicName {
    fn borrow(&self) -> &StrBytes { &self.0 }
}
impl std::ops::Deref for TopicName {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::cmp::PartialEq<StrBytes> for TopicName {
    fn eq(&self, other: &StrBytes) -> bool { &self.0 == other }
}
impl std::cmp::PartialEq<TopicName> for StrBytes {
    fn eq(&self, other: &TopicName) -> bool { self == &other.0 }
}
impl NewType<StrBytes> for TopicName {}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TransactionalId(pub StrBytes);

impl From<StrBytes> for TransactionalId {
    fn from(other: StrBytes) -> Self { Self(other) }
}
impl From<TransactionalId> for StrBytes {
    fn from(other: TransactionalId) -> Self { other.0 }
}
impl std::borrow::Borrow<StrBytes> for TransactionalId {
    fn borrow(&self) -> &StrBytes { &self.0 }
}
impl std::ops::Deref for TransactionalId {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::cmp::PartialEq<StrBytes> for TransactionalId {
    fn eq(&self, other: &StrBytes) -> bool { &self.0 == other }
}
impl std::cmp::PartialEq<TransactionalId> for StrBytes {
    fn eq(&self, other: &TransactionalId) -> bool { self == &other.0 }
}
impl NewType<StrBytes> for TransactionalId {}

