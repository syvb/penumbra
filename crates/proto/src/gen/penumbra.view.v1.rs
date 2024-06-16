/// Filters in an `AuctionsRequest` will be combined using `AND` logic -- that
/// is, the more filters you add, the fewer responses you're likely to get.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionsRequest {
    /// If present, filter balances to only include the account specified by the `AddressIndex`.
    #[prost(message, optional, tag = "1")]
    pub account_filter: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
    /// If present, include inactive auctions as well as active ones.
    #[prost(bool, tag = "2")]
    pub include_inactive: bool,
    /// If set, query a fullnode for the current state of the auctions.
    #[prost(bool, tag = "3")]
    pub query_latest_state: bool,
    /// If present, filter to only include auctions whose IDs are in this array.
    #[prost(message, repeated, tag = "4")]
    pub auction_ids_filter: ::prost::alloc::vec::Vec<
        super::super::core::component::auction::v1::AuctionId,
    >,
}
impl ::prost::Name for AuctionsRequest {
    const NAME: &'static str = "AuctionsRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionsResponse {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<
        super::super::core::component::auction::v1::AuctionId,
    >,
    /// The state of the returned auction.
    ///
    /// Only present when `query_latest_state` was provided.
    #[prost(message, optional, tag = "2")]
    pub auction: ::core::option::Option<::pbjson_types::Any>,
    /// The state of any DEX positions relevant to the returned auction.
    ///
    /// Only present when `query_latest_state` was provided.
    /// Could be empty, depending on the auction state.
    #[prost(message, repeated, tag = "3")]
    pub positions: ::prost::alloc::vec::Vec<
        super::super::core::component::dex::v1::Position,
    >,
    /// The note recording the auction NFT.
    #[prost(message, optional, tag = "4")]
    pub note_record: ::core::option::Option<SpendableNoteRecord>,
    /// The sequence number of the auction state *as known to the local view
    /// service*. Note that the local view service may lag behind the fullnode. For
    /// example, if the chain hits an auction's `end_height`, but the user hasn't
    /// yet exchanged their sequence-0 (opened) auction NFT for a sequence-1
    /// (closed) auction NFT, the local view service will have a sequnce number of
    /// 0.
    ///
    /// Dutch auctions move from:
    /// 0 (opened) => 1 (closed) => n (withdrawn)
    #[prost(uint64, tag = "5")]
    pub local_seq: u64,
}
impl ::prost::Name for AuctionsResponse {
    const NAME: &'static str = "AuctionsResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeAndBuildRequest {
    /// The transaction plan to authorize and build.
    #[prost(message, optional, tag = "1")]
    pub transaction_plan: ::core::option::Option<
        super::super::core::transaction::v1::TransactionPlan,
    >,
}
impl ::prost::Name for AuthorizeAndBuildRequest {
    const NAME: &'static str = "AuthorizeAndBuildRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeAndBuildResponse {
    #[prost(oneof = "authorize_and_build_response::Status", tags = "1, 2")]
    pub status: ::core::option::Option<authorize_and_build_response::Status>,
}
/// Nested message and enum types in `AuthorizeAndBuildResponse`.
pub mod authorize_and_build_response {
    /// Signals that building is in progress.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuildProgress {
        /// An approximate progress of the build, from 0 to 1.
        #[prost(float, tag = "1")]
        pub progress: f32,
    }
    impl ::prost::Name for BuildProgress {
        const NAME: &'static str = "BuildProgress";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.AuthorizeAndBuildResponse.{}", Self::NAME
            )
        }
    }
    /// Signals that the transaction is complete.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Complete {
        /// The finished transaction.
        #[prost(message, optional, tag = "1")]
        pub transaction: ::core::option::Option<
            super::super::super::core::transaction::v1::Transaction,
        >,
    }
    impl ::prost::Name for Complete {
        const NAME: &'static str = "Complete";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.AuthorizeAndBuildResponse.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "1")]
        BuildProgress(BuildProgress),
        #[prost(message, tag = "2")]
        Complete(Complete),
    }
}
impl ::prost::Name for AuthorizeAndBuildResponse {
    const NAME: &'static str = "AuthorizeAndBuildResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTransactionRequest {
    /// The transaction to broadcast.
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<
        super::super::core::transaction::v1::Transaction,
    >,
    /// If true, wait for the view service to detect the transaction during sync.
    #[prost(bool, tag = "2")]
    pub await_detection: bool,
}
impl ::prost::Name for BroadcastTransactionRequest {
    const NAME: &'static str = "BroadcastTransactionRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTransactionResponse {
    #[prost(oneof = "broadcast_transaction_response::Status", tags = "1, 2")]
    pub status: ::core::option::Option<broadcast_transaction_response::Status>,
}
/// Nested message and enum types in `BroadcastTransactionResponse`.
pub mod broadcast_transaction_response {
    /// Signals that the transaction was broadcast successfully (but has not been confirmed).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BroadcastSuccess {
        /// The hash of the transaction that was broadcast.
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<
            super::super::super::core::txhash::v1::TransactionId,
        >,
    }
    impl ::prost::Name for BroadcastSuccess {
        const NAME: &'static str = "BroadcastSuccess";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.BroadcastTransactionResponse.{}", Self::NAME
            )
        }
    }
    /// Signals that the transaction has been confirmed on-chain and detected by the view server.
    ///
    /// Will not be sent unless await_detection was true.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Confirmed {
        /// The hash of the transaction that was broadcast.
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<
            super::super::super::core::txhash::v1::TransactionId,
        >,
        /// The height in which the transaction was detected as included in the chain, if any.
        #[prost(uint64, tag = "2")]
        pub detection_height: u64,
    }
    impl ::prost::Name for Confirmed {
        const NAME: &'static str = "Confirmed";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.BroadcastTransactionResponse.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "1")]
        BroadcastSuccess(BroadcastSuccess),
        #[prost(message, tag = "2")]
        Confirmed(Confirmed),
    }
}
impl ::prost::Name for BroadcastTransactionResponse {
    const NAME: &'static str = "BroadcastTransactionResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionPlannerRequest {
    /// The expiry height for the requested TransactionPlan
    #[prost(uint64, tag = "1")]
    pub expiry_height: u64,
    /// The memo for the requested TransactionPlan.
    /// The memo must be unspecified unless `outputs` is nonempty.
    #[prost(message, optional, tag = "3")]
    pub memo: ::core::option::Option<super::super::core::transaction::v1::MemoPlaintext>,
    /// If present, only spends funds from the given account.
    #[prost(message, optional, tag = "4")]
    pub source: ::core::option::Option<super::super::core::keys::v1::AddressIndex>,
    /// Request contents
    #[prost(message, repeated, tag = "20")]
    pub outputs: ::prost::alloc::vec::Vec<transaction_planner_request::Output>,
    #[prost(message, repeated, tag = "21")]
    pub spends: ::prost::alloc::vec::Vec<transaction_planner_request::Spend>,
    #[prost(message, repeated, tag = "30")]
    pub swaps: ::prost::alloc::vec::Vec<transaction_planner_request::Swap>,
    #[prost(message, repeated, tag = "31")]
    pub swap_claims: ::prost::alloc::vec::Vec<transaction_planner_request::SwapClaim>,
    #[prost(message, repeated, tag = "40")]
    pub delegations: ::prost::alloc::vec::Vec<transaction_planner_request::Delegate>,
    #[prost(message, repeated, tag = "50")]
    pub undelegations: ::prost::alloc::vec::Vec<transaction_planner_request::Undelegate>,
    #[prost(message, repeated, tag = "51")]
    pub undelegation_claims: ::prost::alloc::vec::Vec<
        transaction_planner_request::UndelegateClaim,
    >,
    #[prost(message, repeated, tag = "60")]
    pub ibc_relay_actions: ::prost::alloc::vec::Vec<
        super::super::core::component::ibc::v1::IbcRelay,
    >,
    #[prost(message, repeated, tag = "61")]
    pub ics20_withdrawals: ::prost::alloc::vec::Vec<
        super::super::core::component::ibc::v1::Ics20Withdrawal,
    >,
    #[prost(message, repeated, tag = "70")]
    pub position_opens: ::prost::alloc::vec::Vec<
        transaction_planner_request::PositionOpen,
    >,
    #[prost(message, repeated, tag = "71")]
    pub position_closes: ::prost::alloc::vec::Vec<
        transaction_planner_request::PositionClose,
    >,
    #[prost(message, repeated, tag = "72")]
    pub position_withdraws: ::prost::alloc::vec::Vec<
        transaction_planner_request::PositionWithdraw,
    >,
    #[prost(message, repeated, tag = "73")]
    pub dutch_auction_schedule_actions: ::prost::alloc::vec::Vec<
        transaction_planner_request::ActionDutchAuctionSchedule,
    >,
    #[prost(message, repeated, tag = "74")]
    pub dutch_auction_end_actions: ::prost::alloc::vec::Vec<
        transaction_planner_request::ActionDutchAuctionEnd,
    >,
    #[prost(message, repeated, tag = "75")]
    pub dutch_auction_withdraw_actions: ::prost::alloc::vec::Vec<
        transaction_planner_request::ActionDutchAuctionWithdraw,
    >,
    /// The epoch index of the transaction being planned.
    #[deprecated]
    #[prost(uint64, tag = "200")]
    pub epoch_index: u64,
    /// The epoch of the transaction being planned.
    #[prost(message, optional, tag = "201")]
    pub epoch: ::core::option::Option<super::super::core::component::sct::v1::Epoch>,
    /// Specifies either that the planner should compute fees automatically or that it should use a fixed fee amount.
    #[prost(oneof = "transaction_planner_request::FeeMode", tags = "100, 101")]
    pub fee_mode: ::core::option::Option<transaction_planner_request::FeeMode>,
}
/// Nested message and enum types in `TransactionPlannerRequest`.
pub mod transaction_planner_request {
    /// Request message subtypes
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Output {
        /// The amount and denomination in which the Output is issued.
        #[prost(message, optional, tag = "1")]
        pub value: ::core::option::Option<super::super::super::core::asset::v1::Value>,
        /// The address to which Output will be sent.
        #[prost(message, optional, tag = "2")]
        pub address: ::core::option::Option<
            super::super::super::core::keys::v1::Address,
        >,
    }
    impl ::prost::Name for Output {
        const NAME: &'static str = "Output";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Spend {
        /// The input amount and denomination in which the Spend is issued.
        #[prost(message, optional, tag = "1")]
        pub value: ::core::option::Option<super::super::super::core::asset::v1::Value>,
        /// The source address from which the Spend will be sent.
        #[prost(message, optional, tag = "2")]
        pub address: ::core::option::Option<
            super::super::super::core::keys::v1::Address,
        >,
    }
    impl ::prost::Name for Spend {
        const NAME: &'static str = "Spend";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Swap {
        /// The input amount and denomination to be traded in the Swap.
        #[prost(message, optional, tag = "1")]
        pub value: ::core::option::Option<super::super::super::core::asset::v1::Value>,
        /// The denomination to be received as a Output of the Swap.
        #[prost(message, optional, tag = "2")]
        pub target_asset: ::core::option::Option<
            super::super::super::core::asset::v1::AssetId,
        >,
        /// The pre-paid fee to be paid for claiming the Swap outputs.
        #[prost(message, optional, tag = "3")]
        pub fee: ::core::option::Option<
            super::super::super::core::component::fee::v1::Fee,
        >,
        /// The address to which swap claim output will be sent.
        #[prost(message, optional, tag = "4")]
        pub claim_address: ::core::option::Option<
            super::super::super::core::keys::v1::Address,
        >,
    }
    impl ::prost::Name for Swap {
        const NAME: &'static str = "Swap";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SwapClaim {
        /// SwapCommitment to identify the Swap to be claimed.
        /// Use the commitment from the Swap message:
        /// penumbra.core.component.dex.v1.Swap.body.payload.commitment.
        #[prost(message, optional, tag = "1")]
        pub swap_commitment: ::core::option::Option<
            super::super::super::crypto::tct::v1::StateCommitment,
        >,
    }
    impl ::prost::Name for SwapClaim {
        const NAME: &'static str = "SwapClaim";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delegate {
        #[prost(message, optional, tag = "1")]
        pub amount: ::core::option::Option<super::super::super::core::num::v1::Amount>,
        #[prost(message, optional, tag = "3")]
        pub rate_data: ::core::option::Option<
            super::super::super::core::component::stake::v1::RateData,
        >,
    }
    impl ::prost::Name for Delegate {
        const NAME: &'static str = "Delegate";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Undelegate {
        #[prost(message, optional, tag = "1")]
        pub value: ::core::option::Option<super::super::super::core::asset::v1::Value>,
        #[prost(message, optional, tag = "2")]
        pub rate_data: ::core::option::Option<
            super::super::super::core::component::stake::v1::RateData,
        >,
    }
    impl ::prost::Name for Undelegate {
        const NAME: &'static str = "Undelegate";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UndelegateClaim {
        /// The identity key of the validator to finish undelegating from.
        #[prost(message, optional, tag = "1")]
        pub validator_identity: ::core::option::Option<
            super::super::super::core::keys::v1::IdentityKey,
        >,
        /// The epoch in which unbonding began, used to verify the penalty.
        #[deprecated]
        #[prost(uint64, tag = "2")]
        pub start_epoch_index: u64,
        /// The penalty applied to undelegation, in bps^2 (10e-8).
        /// In the happy path (no slashing), this is 0.
        #[prost(message, optional, tag = "3")]
        pub penalty: ::core::option::Option<
            super::super::super::core::component::stake::v1::Penalty,
        >,
        /// The amount of unbonding tokens to claim.
        /// This is a bare number because its denom is determined by the preceding data.
        #[prost(message, optional, tag = "4")]
        pub unbonding_amount: ::core::option::Option<
            super::super::super::core::num::v1::Amount,
        >,
        /// The height at which unbonding began.
        #[prost(uint64, tag = "5")]
        pub unbonding_start_height: u64,
    }
    impl ::prost::Name for UndelegateClaim {
        const NAME: &'static str = "UndelegateClaim";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionOpen {
        /// Contains the data defining the position, sufficient to compute its `PositionId`.
        ///
        /// Positions are immutable, so the `PositionData` (and hence the `PositionId`)
        /// are unchanged over the entire lifetime of the position.
        #[prost(message, optional, tag = "1")]
        pub position: ::core::option::Option<
            super::super::super::core::component::dex::v1::Position,
        >,
    }
    impl ::prost::Name for PositionOpen {
        const NAME: &'static str = "PositionOpen";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionClose {
        /// The position to close.
        #[prost(message, optional, tag = "1")]
        pub position_id: ::core::option::Option<
            super::super::super::core::component::dex::v1::PositionId,
        >,
    }
    impl ::prost::Name for PositionClose {
        const NAME: &'static str = "PositionClose";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionWithdraw {
        /// The position to withdraw.
        #[prost(message, optional, tag = "1")]
        pub position_id: ::core::option::Option<
            super::super::super::core::component::dex::v1::PositionId,
        >,
        /// The position's final reserves.
        #[prost(message, optional, tag = "2")]
        pub reserves: ::core::option::Option<
            super::super::super::core::component::dex::v1::Reserves,
        >,
        /// The trading pair of the position.
        #[prost(message, optional, tag = "3")]
        pub trading_pair: ::core::option::Option<
            super::super::super::core::component::dex::v1::TradingPair,
        >,
    }
    impl ::prost::Name for PositionWithdraw {
        const NAME: &'static str = "PositionWithdraw";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionDutchAuctionSchedule {
        /// The description of the auction to schedule.
        #[prost(message, optional, tag = "1")]
        pub description: ::core::option::Option<
            super::super::super::core::component::auction::v1::DutchAuctionDescription,
        >,
    }
    impl ::prost::Name for ActionDutchAuctionSchedule {
        const NAME: &'static str = "ActionDutchAuctionSchedule";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionDutchAuctionEnd {
        /// The unique id of the auction to close.
        #[prost(message, optional, tag = "1")]
        pub auction_id: ::core::option::Option<
            super::super::super::core::component::auction::v1::AuctionId,
        >,
    }
    impl ::prost::Name for ActionDutchAuctionEnd {
        const NAME: &'static str = "ActionDutchAuctionEnd";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionDutchAuctionWithdraw {
        /// The auction to withdraw funds from.
        #[prost(message, optional, tag = "1")]
        pub auction_id: ::core::option::Option<
            super::super::super::core::component::auction::v1::AuctionId,
        >,
        /// The sequence number of the withdrawal.
        #[prost(uint64, tag = "2")]
        pub seq: u64,
    }
    impl ::prost::Name for ActionDutchAuctionWithdraw {
        const NAME: &'static str = "ActionDutchAuctionWithdraw";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.TransactionPlannerRequest.{}", Self::NAME
            )
        }
    }
    /// Specifies either that the planner should compute fees automatically or that it should use a fixed fee amount.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeeMode {
        /// Automatically compute a fee based on gas use.
        #[prost(message, tag = "100")]
        AutoFee(super::super::super::core::component::fee::v1::FeeTier),
        /// A manually set fee, rather than automatically computing a fee based on gas use.
        #[prost(message, tag = "101")]
        ManualFee(super::super::super::core::component::fee::v1::Fee),
    }
}
impl ::prost::Name for TransactionPlannerRequest {
    const NAME: &'static str = "TransactionPlannerRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionPlannerResponse {
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<
        super::super::core::transaction::v1::TransactionPlan,
    >,
}
impl ::prost::Name for TransactionPlannerResponse {
    const NAME: &'static str = "TransactionPlannerResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressByIndexRequest {
    #[prost(message, optional, tag = "1")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
}
impl ::prost::Name for AddressByIndexRequest {
    const NAME: &'static str = "AddressByIndexRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressByIndexResponse {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::core::keys::v1::Address>,
}
impl ::prost::Name for AddressByIndexResponse {
    const NAME: &'static str = "AddressByIndexResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletIdRequest {}
impl ::prost::Name for WalletIdRequest {
    const NAME: &'static str = "WalletIdRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletIdResponse {
    #[prost(message, optional, tag = "1")]
    pub wallet_id: ::core::option::Option<super::super::core::keys::v1::WalletId>,
}
impl ::prost::Name for WalletIdResponse {
    const NAME: &'static str = "WalletIdResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexByAddressRequest {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::core::keys::v1::Address>,
}
impl ::prost::Name for IndexByAddressRequest {
    const NAME: &'static str = "IndexByAddressRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexByAddressResponse {
    /// Will be absent if given an address not viewable by this viewing service
    #[prost(message, optional, tag = "1")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
}
impl ::prost::Name for IndexByAddressResponse {
    const NAME: &'static str = "IndexByAddressResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralAddressRequest {
    #[prost(message, optional, tag = "1")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
}
impl ::prost::Name for EphemeralAddressRequest {
    const NAME: &'static str = "EphemeralAddressRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::core::keys::v1::Address>,
}
impl ::prost::Name for EphemeralAddressResponse {
    const NAME: &'static str = "EphemeralAddressResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancesRequest {
    /// If present, filter balances to only include the account specified by the `AddressIndex`.
    #[prost(message, optional, tag = "1")]
    pub account_filter: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
    /// If present, filter balances to only include the specified asset ID.
    #[prost(message, optional, tag = "2")]
    pub asset_id_filter: ::core::option::Option<super::super::core::asset::v1::AssetId>,
}
impl ::prost::Name for BalancesRequest {
    const NAME: &'static str = "BalancesRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancesResponse {
    /// Deprecated: use `account_address` instead.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<super::super::core::keys::v1::AddressIndex>,
    /// Deprecated: use `balance_view` instead.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::core::asset::v1::Value>,
    /// The default address for the account.
    ///
    /// Note that the returned balance is for all funds sent to the account,
    /// not just funds sent to its default address.
    #[prost(message, optional, tag = "3")]
    pub account_address: ::core::option::Option<
        super::super::core::keys::v1::AddressView,
    >,
    /// The account's balance, with metadata.
    #[prost(message, optional, tag = "4")]
    pub balance_view: ::core::option::Option<super::super::core::asset::v1::ValueView>,
}
impl ::prost::Name for BalancesResponse {
    const NAME: &'static str = "BalancesResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests sync status of the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
impl ::prost::Name for StatusRequest {
    const NAME: &'static str = "StatusRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Returns the status of the view service and whether it is synchronized with the chain state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    /// The height the view service has synchronized to so far when doing a full linear sync
    #[prost(uint64, tag = "1")]
    pub full_sync_height: u64,
    /// The height the view service has synchronized to so far when doing a partial sync
    #[prost(uint64, tag = "2")]
    pub partial_sync_height: u64,
    /// Whether the view service is catching up with the chain state
    #[prost(bool, tag = "3")]
    pub catching_up: bool,
}
impl ::prost::Name for StatusResponse {
    const NAME: &'static str = "StatusResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests streaming updates on the sync height until the view service is synchronized.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusStreamRequest {}
impl ::prost::Name for StatusStreamRequest {
    const NAME: &'static str = "StatusStreamRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// A streaming sync status update
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusStreamResponse {
    /// The latest known block height
    #[prost(uint64, tag = "1")]
    pub latest_known_block_height: u64,
    /// The height the view service has synchronized to so far when doing a full linear sync
    #[prost(uint64, tag = "2")]
    pub full_sync_height: u64,
    /// The height the view service has synchronized to so far when doing a partial sync
    #[prost(uint64, tag = "3")]
    pub partial_sync_height: u64,
}
impl ::prost::Name for StatusStreamResponse {
    const NAME: &'static str = "StatusStreamResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// A query for notes known by the view service.
///
/// This message uses the fact that all proto fields are optional
/// to allow various filtering on the returned notes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotesRequest {
    /// If set, return spent notes as well as unspent notes.
    #[prost(bool, tag = "2")]
    pub include_spent: bool,
    /// If set, only return notes with the specified asset id.
    #[prost(message, optional, tag = "3")]
    pub asset_id: ::core::option::Option<super::super::core::asset::v1::AssetId>,
    /// If set, only return notes with the specified address incore.component.dex.v1.
    #[prost(message, optional, tag = "4")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
    /// If set, stop returning notes once the total exceeds this amount.
    ///
    /// Ignored if `asset_id` is unset or if `include_spent` is set.
    #[prost(message, optional, tag = "6")]
    pub amount_to_spend: ::core::option::Option<super::super::core::num::v1::Amount>,
}
impl ::prost::Name for NotesRequest {
    const NAME: &'static str = "NotesRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// A query for notes to be used for voting on a proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotesForVotingRequest {
    /// The starting height of the proposal.
    #[prost(uint64, tag = "1")]
    pub votable_at_height: u64,
    /// If set, only return notes with the specified asset id.
    #[prost(message, optional, tag = "3")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
}
impl ::prost::Name for NotesForVotingRequest {
    const NAME: &'static str = "NotesForVotingRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessRequest {
    /// The transaction plan to witness
    #[prost(message, optional, tag = "3")]
    pub transaction_plan: ::core::option::Option<
        super::super::core::transaction::v1::TransactionPlan,
    >,
}
impl ::prost::Name for WitnessRequest {
    const NAME: &'static str = "WitnessRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessResponse {
    #[prost(message, optional, tag = "1")]
    pub witness_data: ::core::option::Option<
        super::super::core::transaction::v1::WitnessData,
    >,
}
impl ::prost::Name for WitnessResponse {
    const NAME: &'static str = "WitnessResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessAndBuildRequest {
    #[prost(message, optional, tag = "1")]
    pub transaction_plan: ::core::option::Option<
        super::super::core::transaction::v1::TransactionPlan,
    >,
    #[prost(message, optional, tag = "2")]
    pub authorization_data: ::core::option::Option<
        super::super::core::transaction::v1::AuthorizationData,
    >,
}
impl ::prost::Name for WitnessAndBuildRequest {
    const NAME: &'static str = "WitnessAndBuildRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessAndBuildResponse {
    #[prost(oneof = "witness_and_build_response::Status", tags = "1, 2")]
    pub status: ::core::option::Option<witness_and_build_response::Status>,
}
/// Nested message and enum types in `WitnessAndBuildResponse`.
pub mod witness_and_build_response {
    /// Signals that building is in progress.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuildProgress {
        /// An approximate progress of the build, from 0 to 1.
        #[prost(float, tag = "1")]
        pub progress: f32,
    }
    impl ::prost::Name for BuildProgress {
        const NAME: &'static str = "BuildProgress";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.WitnessAndBuildResponse.{}", Self::NAME
            )
        }
    }
    /// Signals that the transaction is complete.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Complete {
        /// The finished transaction.
        #[prost(message, optional, tag = "1")]
        pub transaction: ::core::option::Option<
            super::super::super::core::transaction::v1::Transaction,
        >,
    }
    impl ::prost::Name for Complete {
        const NAME: &'static str = "Complete";
        const PACKAGE: &'static str = "penumbra.view.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.view.v1.WitnessAndBuildResponse.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "1")]
        BuildProgress(BuildProgress),
        #[prost(message, tag = "2")]
        Complete(Complete),
    }
}
impl ::prost::Name for WitnessAndBuildResponse {
    const NAME: &'static str = "WitnessAndBuildResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests all assets known to the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsRequest {
    /// If set to false (default), returns all assets, regardless of whether the rest of the fields of
    /// the request indicate a filter.
    #[prost(bool, tag = "1")]
    pub filtered: bool,
    /// Include these specific denominations in the response.
    #[prost(message, repeated, tag = "2")]
    pub include_specific_denominations: ::prost::alloc::vec::Vec<
        super::super::core::asset::v1::Denom,
    >,
    /// Include all delegation tokens, to any validator, in the response.
    #[prost(bool, tag = "3")]
    pub include_delegation_tokens: bool,
    /// Include all unbonding tokens, from any validator, in the response.
    #[prost(bool, tag = "4")]
    pub include_unbonding_tokens: bool,
    /// Include all LP NFTs in the response.
    #[prost(bool, tag = "5")]
    pub include_lp_nfts: bool,
    /// Include all proposal NFTs in the response.
    #[prost(bool, tag = "6")]
    pub include_proposal_nfts: bool,
    /// Include all voting receipt tokens in the response.
    #[prost(bool, tag = "7")]
    pub include_voting_receipt_tokens: bool,
}
impl ::prost::Name for AssetsRequest {
    const NAME: &'static str = "AssetsRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests all assets known to the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsResponse {
    #[prost(message, optional, tag = "2")]
    pub denom_metadata: ::core::option::Option<super::super::core::asset::v1::Metadata>,
}
impl ::prost::Name for AssetsResponse {
    const NAME: &'static str = "AssetsResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests the current app parameters from the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppParametersRequest {}
impl ::prost::Name for AppParametersRequest {
    const NAME: &'static str = "AppParametersRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppParametersResponse {
    #[prost(message, optional, tag = "1")]
    pub parameters: ::core::option::Option<super::super::core::app::v1::AppParameters>,
}
impl ::prost::Name for AppParametersResponse {
    const NAME: &'static str = "AppParametersResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests the current gas prices from the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasPricesRequest {}
impl ::prost::Name for GasPricesRequest {
    const NAME: &'static str = "GasPricesRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasPricesResponse {
    /// The current gas prices, in the preferred (native) token.
    #[prost(message, optional, tag = "1")]
    pub gas_prices: ::core::option::Option<
        super::super::core::component::fee::v1::GasPrices,
    >,
    /// Other gas prices for other accepted tokens.
    #[prost(message, repeated, tag = "2")]
    pub alt_gas_prices: ::prost::alloc::vec::Vec<
        super::super::core::component::fee::v1::GasPrices,
    >,
}
impl ::prost::Name for GasPricesResponse {
    const NAME: &'static str = "GasPricesResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests the current FMD parameters from the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FmdParametersRequest {}
impl ::prost::Name for FmdParametersRequest {
    const NAME: &'static str = "FMDParametersRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FmdParametersResponse {
    #[prost(message, optional, tag = "1")]
    pub parameters: ::core::option::Option<
        super::super::core::component::shielded_pool::v1::FmdParameters,
    >,
}
impl ::prost::Name for FmdParametersResponse {
    const NAME: &'static str = "FMDParametersResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteByCommitmentRequest {
    #[prost(message, optional, tag = "2")]
    pub note_commitment: ::core::option::Option<
        super::super::crypto::tct::v1::StateCommitment,
    >,
    /// If set to true, waits to return until the requested note is detected.
    #[prost(bool, tag = "3")]
    pub await_detection: bool,
}
impl ::prost::Name for NoteByCommitmentRequest {
    const NAME: &'static str = "NoteByCommitmentRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteByCommitmentResponse {
    #[prost(message, optional, tag = "1")]
    pub spendable_note: ::core::option::Option<SpendableNoteRecord>,
}
impl ::prost::Name for NoteByCommitmentResponse {
    const NAME: &'static str = "NoteByCommitmentResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapByCommitmentRequest {
    #[prost(message, optional, tag = "2")]
    pub swap_commitment: ::core::option::Option<
        super::super::crypto::tct::v1::StateCommitment,
    >,
    /// If set to true, waits to return until the requested swap is detected.
    #[prost(bool, tag = "3")]
    pub await_detection: bool,
}
impl ::prost::Name for SwapByCommitmentRequest {
    const NAME: &'static str = "SwapByCommitmentRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapByCommitmentResponse {
    #[prost(message, optional, tag = "1")]
    pub swap: ::core::option::Option<SwapRecord>,
}
impl ::prost::Name for SwapByCommitmentResponse {
    const NAME: &'static str = "SwapByCommitmentResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnclaimedSwapsRequest {}
impl ::prost::Name for UnclaimedSwapsRequest {
    const NAME: &'static str = "UnclaimedSwapsRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnclaimedSwapsResponse {
    #[prost(message, optional, tag = "1")]
    pub swap: ::core::option::Option<SwapRecord>,
}
impl ::prost::Name for UnclaimedSwapsResponse {
    const NAME: &'static str = "UnclaimedSwapsResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullifierStatusRequest {
    #[prost(message, optional, tag = "2")]
    pub nullifier: ::core::option::Option<
        super::super::core::component::sct::v1::Nullifier,
    >,
    #[prost(bool, tag = "3")]
    pub await_detection: bool,
}
impl ::prost::Name for NullifierStatusRequest {
    const NAME: &'static str = "NullifierStatusRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullifierStatusResponse {
    #[prost(bool, tag = "1")]
    pub spent: bool,
}
impl ::prost::Name for NullifierStatusResponse {
    const NAME: &'static str = "NullifierStatusResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoByHashRequest {
    /// The transaction hash to query for.
    #[prost(message, optional, tag = "2")]
    pub id: ::core::option::Option<super::super::core::txhash::v1::TransactionId>,
}
impl ::prost::Name for TransactionInfoByHashRequest {
    const NAME: &'static str = "TransactionInfoByHashRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoRequest {
    /// If present, return only transactions after this height.
    #[prost(uint64, tag = "1")]
    pub start_height: u64,
    /// If present, return only transactions before this height.
    #[prost(uint64, tag = "2")]
    pub end_height: u64,
}
impl ::prost::Name for TransactionInfoRequest {
    const NAME: &'static str = "TransactionInfoRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// The height the transaction was included in a block, if known.
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// The hash of the transaction.
    #[prost(message, optional, tag = "2")]
    pub id: ::core::option::Option<super::super::core::txhash::v1::TransactionId>,
    /// The transaction data itself.
    #[prost(message, optional, tag = "3")]
    pub transaction: ::core::option::Option<
        super::super::core::transaction::v1::Transaction,
    >,
    /// The transaction perspective, as seen by this view server.
    #[prost(message, optional, tag = "4")]
    pub perspective: ::core::option::Option<
        super::super::core::transaction::v1::TransactionPerspective,
    >,
    /// A precomputed transaction view of `transaction` from `perspective`, included for convenience of clients that don't have support for viewing transactions on their own.
    #[prost(message, optional, tag = "5")]
    pub view: ::core::option::Option<
        super::super::core::transaction::v1::TransactionView,
    >,
}
impl ::prost::Name for TransactionInfo {
    const NAME: &'static str = "TransactionInfo";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub tx_info: ::core::option::Option<TransactionInfo>,
}
impl ::prost::Name for TransactionInfoResponse {
    const NAME: &'static str = "TransactionInfoResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoByHashResponse {
    #[prost(message, optional, tag = "1")]
    pub tx_info: ::core::option::Option<TransactionInfo>,
}
impl ::prost::Name for TransactionInfoByHashResponse {
    const NAME: &'static str = "TransactionInfoByHashResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotesResponse {
    #[prost(message, optional, tag = "1")]
    pub note_record: ::core::option::Option<SpendableNoteRecord>,
}
impl ::prost::Name for NotesResponse {
    const NAME: &'static str = "NotesResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotesForVotingResponse {
    #[prost(message, optional, tag = "1")]
    pub note_record: ::core::option::Option<SpendableNoteRecord>,
    #[prost(message, optional, tag = "2")]
    pub identity_key: ::core::option::Option<super::super::core::keys::v1::IdentityKey>,
}
impl ::prost::Name for NotesForVotingResponse {
    const NAME: &'static str = "NotesForVotingResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// A note plaintext with associated metadata about its status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendableNoteRecord {
    /// The note commitment, identifying the note.
    #[prost(message, optional, tag = "1")]
    pub note_commitment: ::core::option::Option<
        super::super::crypto::tct::v1::StateCommitment,
    >,
    /// The note plaintext itself.
    #[prost(message, optional, tag = "2")]
    pub note: ::core::option::Option<
        super::super::core::component::shielded_pool::v1::Note,
    >,
    /// A precomputed decryption of the note's address incore.component.dex.v1.
    #[prost(message, optional, tag = "3")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
    /// The note's nullifier.
    #[prost(message, optional, tag = "4")]
    pub nullifier: ::core::option::Option<
        super::super::core::component::sct::v1::Nullifier,
    >,
    /// The height at which the note was created.
    #[prost(uint64, tag = "5")]
    pub height_created: u64,
    /// Records whether the note was spent (and if so, at what height).
    #[prost(uint64, tag = "6")]
    pub height_spent: u64,
    /// The note position.
    #[prost(uint64, tag = "7")]
    pub position: u64,
    /// The source of the note
    #[prost(message, optional, tag = "8")]
    pub source: ::core::option::Option<
        super::super::core::component::sct::v1::CommitmentSource,
    >,
    /// The sender's return address, if known.
    #[prost(message, optional, tag = "9")]
    pub return_address: ::core::option::Option<
        super::super::core::keys::v1::AddressView,
    >,
}
impl ::prost::Name for SpendableNoteRecord {
    const NAME: &'static str = "SpendableNoteRecord";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapRecord {
    #[prost(message, optional, tag = "1")]
    pub swap_commitment: ::core::option::Option<
        super::super::crypto::tct::v1::StateCommitment,
    >,
    #[prost(message, optional, tag = "2")]
    pub swap: ::core::option::Option<
        super::super::core::component::dex::v1::SwapPlaintext,
    >,
    #[prost(uint64, tag = "3")]
    pub position: u64,
    #[prost(message, optional, tag = "4")]
    pub nullifier: ::core::option::Option<
        super::super::core::component::sct::v1::Nullifier,
    >,
    #[prost(message, optional, tag = "5")]
    pub output_data: ::core::option::Option<
        super::super::core::component::dex::v1::BatchSwapOutputData,
    >,
    #[prost(uint64, tag = "6")]
    pub height_claimed: u64,
    #[prost(message, optional, tag = "7")]
    pub source: ::core::option::Option<
        super::super::core::component::sct::v1::CommitmentSource,
    >,
}
impl ::prost::Name for SwapRecord {
    const NAME: &'static str = "SwapRecord";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedPositionIdsRequest {
    /// If present, return only positions with this position state.
    #[prost(message, optional, tag = "1")]
    pub position_state: ::core::option::Option<
        super::super::core::component::dex::v1::PositionState,
    >,
    /// If present, return only positions for this trading pair.
    #[prost(message, optional, tag = "2")]
    pub trading_pair: ::core::option::Option<
        super::super::core::component::dex::v1::TradingPair,
    >,
}
impl ::prost::Name for OwnedPositionIdsRequest {
    const NAME: &'static str = "OwnedPositionIdsRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedPositionIdsResponse {
    #[prost(message, optional, tag = "1")]
    pub position_id: ::core::option::Option<
        super::super::core::component::dex::v1::PositionId,
    >,
}
impl ::prost::Name for OwnedPositionIdsResponse {
    const NAME: &'static str = "OwnedPositionIdsResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests information on an asset by asset id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMetadataByIdRequest {
    /// The asset id to request information on.
    #[prost(message, optional, tag = "2")]
    pub asset_id: ::core::option::Option<super::super::core::asset::v1::AssetId>,
}
impl ::prost::Name for AssetMetadataByIdRequest {
    const NAME: &'static str = "AssetMetadataByIdRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMetadataByIdResponse {
    /// If present, information on the requested asset.
    ///
    /// If the requested asset was unknown, this field will not be present.
    #[prost(message, optional, tag = "1")]
    pub denom_metadata: ::core::option::Option<super::super::core::asset::v1::Metadata>,
}
impl ::prost::Name for AssetMetadataByIdResponse {
    const NAME: &'static str = "AssetMetadataByIdResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests `ValueView`s of delegation tokens for the given address index. The
/// returned `ValueView`s will include the `ValidatorInfo` for the delegated
/// validator in their `extended_metadata` fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationsByAddressIndexRequest {
    /// The address index to fetch delegation balances for.
    #[prost(message, optional, tag = "1")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
    #[prost(enumeration = "delegations_by_address_index_request::Filter", tag = "2")]
    pub filter: i32,
}
/// Nested message and enum types in `DelegationsByAddressIndexRequest`.
pub mod delegations_by_address_index_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Filter {
        /// By default, returns delegations for all active validators. For validators
        /// that the given address index has no delegation tokens for, a `ValueView`
        /// with a balance of `0` will be returned.
        Unspecified = 0,
        /// Returns only delegations to active validators that the given address
        /// index holds delegation tokens for.
        AllActiveWithNonzeroBalances = 1,
        /// Return delegations for all validators, whether active or not. For
        /// validators that the given address index has no delegation tokens for, a
        /// `ValueView` with a balance of `0` will be returned.
        All = 2,
    }
    impl Filter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Filter::Unspecified => "FILTER_UNSPECIFIED",
                Filter::AllActiveWithNonzeroBalances => {
                    "FILTER_ALL_ACTIVE_WITH_NONZERO_BALANCES"
                }
                Filter::All => "FILTER_ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILTER_UNSPECIFIED" => Some(Self::Unspecified),
                "FILTER_ALL_ACTIVE_WITH_NONZERO_BALANCES" => {
                    Some(Self::AllActiveWithNonzeroBalances)
                }
                "FILTER_ALL" => Some(Self::All),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for DelegationsByAddressIndexRequest {
    const NAME: &'static str = "DelegationsByAddressIndexRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Contains a `ValueView` of delegation tokens for the requested address index.
/// The `ValueView` includes the `ValidatorInfo` for the delegated validator in
/// cits `extended_metadata` field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationsByAddressIndexResponse {
    #[prost(message, optional, tag = "1")]
    pub value_view: ::core::option::Option<super::super::core::asset::v1::ValueView>,
}
impl ::prost::Name for DelegationsByAddressIndexResponse {
    const NAME: &'static str = "DelegationsByAddressIndexResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Requests unbonding tokens for a given address index, with optional filtering
/// for whether the tokens are currently claimable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingTokensByAddressIndexRequest {
    #[prost(
        enumeration = "unbonding_tokens_by_address_index_request::Filter",
        tag = "1"
    )]
    pub filter: i32,
    /// The address index to fetch unbonding tokens for.
    #[prost(message, optional, tag = "2")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
}
/// Nested message and enum types in `UnbondingTokensByAddressIndexRequest`.
pub mod unbonding_tokens_by_address_index_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Filter {
        /// Return all unbonding tokens, regardless of whether they're claimable
        /// right now.
        Unspecified = 0,
        /// Return all unbonding tokens that are currently claimable. This includes:
        ///
        /// * tokens that have passed the `unbonding_delay` (from `StakeParameters`)
        /// * tokens for unbonded validators
        Claimable = 1,
        /// Return all unbonding tokens that are not yet claimable, because they are
        /// still in the `unbonding_delay` (from `StakeParameters`) period.
        NotYetClaimable = 2,
    }
    impl Filter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Filter::Unspecified => "FILTER_UNSPECIFIED",
                Filter::Claimable => "FILTER_CLAIMABLE",
                Filter::NotYetClaimable => "FILTER_NOT_YET_CLAIMABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILTER_UNSPECIFIED" => Some(Self::Unspecified),
                "FILTER_CLAIMABLE" => Some(Self::Claimable),
                "FILTER_NOT_YET_CLAIMABLE" => Some(Self::NotYetClaimable),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for UnbondingTokensByAddressIndexRequest {
    const NAME: &'static str = "UnbondingTokensByAddressIndexRequest";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Returns unbonding tokens for the given address index, optionally filtered by
/// whether the tokens are currently claimable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingTokensByAddressIndexResponse {
    /// A `ValueView` representing the amount of the given unbonding token.
    #[prost(message, optional, tag = "1")]
    pub value_view: ::core::option::Option<super::super::core::asset::v1::ValueView>,
    /// Whether the unbonding token is currently claimable. This will only be
    /// `true` if the `unbonding_delay` (from `StakeParameters`) has passed or the
    /// validator has unbonded.
    #[prost(bool, tag = "2")]
    pub claimable: bool,
}
impl ::prost::Name for UnbondingTokensByAddressIndexResponse {
    const NAME: &'static str = "UnbondingTokensByAddressIndexResponse";
    const PACKAGE: &'static str = "penumbra.view.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.view.v1.{}", Self::NAME)
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod view_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The view RPC is used by a view client, who wants to do some
    /// transaction-related actions, to request data from a view service, which is
    /// responsible for synchronizing and scanning the public chain state with one or
    /// more full viewing keys.
    #[derive(Debug, Clone)]
    pub struct ViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ViewServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ViewServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ViewServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ViewServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Get current status of chain sync
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/Status",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "Status"));
            self.inner.unary(req, path, codec).await
        }
        /// Stream sync status updates until the view service has caught up with the chain.
        /// Returns a stream of `StatusStreamResponse`s.
        pub async fn status_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StatusStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/StatusStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "StatusStream"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Queries for notes that have been accepted by the chain.
        /// Returns a stream of `NotesResponse`s.
        pub async fn notes(
            &mut self,
            request: impl tonic::IntoRequest<super::NotesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NotesResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/Notes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "Notes"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Returns a stream of `NotesForVotingResponse`s.
        pub async fn notes_for_voting(
            &mut self,
            request: impl tonic::IntoRequest<super::NotesForVotingRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NotesForVotingResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/NotesForVoting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "NotesForVoting"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Queries for metadata about known assets.
        /// Returns a stream of `AssetsResponse`s.
        pub async fn assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::AssetsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/Assets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "Assets"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Query for metadata about a specific asset, by asset ID.
        ///
        /// This is the same as the method on the shielded pool's `QueryService`, but exposing it
        /// here allows a view server to provide more specific or opinionated asset metadata -- like
        /// using an asset registry to provide tickers, symbols, etc.
        pub async fn asset_metadata_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetMetadataByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetMetadataByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/AssetMetadataById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "AssetMetadataById"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for the current app parameters.
        pub async fn app_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::AppParametersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AppParametersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/AppParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "AppParameters"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for the current gas prices.
        pub async fn gas_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GasPricesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GasPricesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/GasPrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "GasPrices"));
            self.inner.unary(req, path, codec).await
        }
        /// Query for the current FMD parameters.
        pub async fn fmd_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::FmdParametersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FmdParametersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/FMDParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "FMDParameters"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for an address given an address index
        pub async fn address_by_index(
            &mut self,
            request: impl tonic::IntoRequest<super::AddressByIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddressByIndexResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/AddressByIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "AddressByIndex"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for wallet id
        pub async fn wallet_id(
            &mut self,
            request: impl tonic::IntoRequest<super::WalletIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WalletIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/WalletId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "WalletId"));
            self.inner.unary(req, path, codec).await
        }
        /// Query for an address given an address index
        pub async fn index_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::IndexByAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IndexByAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/IndexByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "IndexByAddress"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for an ephemeral address
        pub async fn ephemeral_address(
            &mut self,
            request: impl tonic::IntoRequest<super::EphemeralAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EphemeralAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/EphemeralAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "EphemeralAddress"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for balance of a given address.
        /// Returns a stream of `BalancesResponses`.
        pub async fn balances(
            &mut self,
            request: impl tonic::IntoRequest<super::BalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BalancesResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/Balances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "Balances"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Query for a note by its note commitment, optionally waiting until the note is detected.
        pub async fn note_by_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::NoteByCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NoteByCommitmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/NoteByCommitment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "NoteByCommitment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for a swap by its swap commitment, optionally waiting until the swap is detected.
        pub async fn swap_by_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::SwapByCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SwapByCommitmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/SwapByCommitment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "SwapByCommitment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for all unclaimed swaps.
        pub async fn unclaimed_swaps(
            &mut self,
            request: impl tonic::IntoRequest<super::UnclaimedSwapsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::UnclaimedSwapsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/UnclaimedSwaps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "UnclaimedSwaps"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Query for whether a nullifier has been spent, optionally waiting until it is spent.
        pub async fn nullifier_status(
            &mut self,
            request: impl tonic::IntoRequest<super::NullifierStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NullifierStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/NullifierStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "NullifierStatus"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for a given transaction by its hash.
        pub async fn transaction_info_by_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionInfoByHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfoByHashResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/TransactionInfoByHash",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.view.v1.ViewService",
                        "TransactionInfoByHash",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query for the full transactions in the given range of blocks.
        /// Returns a stream of `TransactionInfoResponse`s.
        pub async fn transaction_info(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TransactionInfoResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/TransactionInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "TransactionInfo"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Query for owned position IDs for the given trading pair and in the given position state.
        pub async fn owned_position_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::OwnedPositionIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::OwnedPositionIdsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/OwnedPositionIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "OwnedPositionIds"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Translates a high-level intent ("send X funds to Y address") into a complete transaction plan.
        pub async fn transaction_planner(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionPlannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionPlannerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/TransactionPlanner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "TransactionPlanner"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns authentication data for the given transaction plan.
        ///
        /// This method takes a complete transaction plan, so that the client can get a
        /// consistent set of authentication paths to a common root for the entire
        /// transaction.  (Otherwise, if a client made multiple requests, the wallet
        /// service could have advanced the state commitment tree  between queries).
        pub async fn witness(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WitnessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/Witness",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "Witness"));
            self.inner.unary(req, path, codec).await
        }
        /// Like `Witness`, but immediately uses the witness data to build (prove) the transaction.
        ///
        /// This method is useful for clients that can't easily do proving themselves, either because
        /// they're not written in Rust and can't easily import the proving code, or because they don't
        /// have access to proving keys, or some other reason.
        ///
        /// This method streams status updates to the caller before finally returning the transaction.
        pub async fn witness_and_build(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessAndBuildRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WitnessAndBuildResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/WitnessAndBuild",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "WitnessAndBuild"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Authorize a transaction plan and build the transaction.
        ///
        /// This method is only supported on view servers that have access to a custody
        /// service.  Otherwise, it will fail.
        ///
        /// Penumbra's transaction authorization mechanism is designed so transactions
        /// can be signed and built (proved) concurrently. This allows implementations
        /// to, e.g., start proving optimistically while presenting the user with an
        /// approval dialog.
        ///
        /// This method streams status updates to the caller before finally returning the transaction.
        pub async fn authorize_and_build(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthorizeAndBuildRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::AuthorizeAndBuildResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/AuthorizeAndBuild",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.view.v1.ViewService", "AuthorizeAndBuild"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Broadcast a transaction to the network, optionally waiting for full confirmation.
        ///
        /// This method streams status updates to the caller before finally returning confirmation.
        pub async fn broadcast_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::BroadcastTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::BroadcastTransactionResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/BroadcastTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.view.v1.ViewService",
                        "BroadcastTransaction",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Get delegation tokens for a given address index. Each delegation token will
        /// be represented by a `ValueView` with the given address index's balance of
        /// that token. Each `ValueView`'s `extended_metadata` field will contain the
        /// `ValidatorInfo` of the delegated validator.
        pub async fn delegations_by_address_index(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegationsByAddressIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::DelegationsByAddressIndexResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/DelegationsByAddressIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.view.v1.ViewService",
                        "DelegationsByAddressIndex",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Get unbonding tokens for the given address index, optionally filtered by
        /// whether the tokens are currently claimable.
        pub async fn unbonding_tokens_by_address_index(
            &mut self,
            request: impl tonic::IntoRequest<super::UnbondingTokensByAddressIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::UnbondingTokensByAddressIndexResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/UnbondingTokensByAddressIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.view.v1.ViewService",
                        "UnbondingTokensByAddressIndex",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Gets the auctions controlled by the user's wallet.
        pub async fn auctions(
            &mut self,
            request: impl tonic::IntoRequest<super::AuctionsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::AuctionsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.view.v1.ViewService/Auctions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("penumbra.view.v1.ViewService", "Auctions"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod view_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ViewServiceServer.
    #[async_trait]
    pub trait ViewService: Send + Sync + 'static {
        /// Get current status of chain sync
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status>;
        /// Server streaming response type for the StatusStream method.
        type StatusStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::StatusStreamResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Stream sync status updates until the view service has caught up with the chain.
        /// Returns a stream of `StatusStreamResponse`s.
        async fn status_stream(
            &self,
            request: tonic::Request<super::StatusStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StatusStreamStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the Notes method.
        type NotesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::NotesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Queries for notes that have been accepted by the chain.
        /// Returns a stream of `NotesResponse`s.
        async fn notes(
            &self,
            request: tonic::Request<super::NotesRequest>,
        ) -> std::result::Result<tonic::Response<Self::NotesStream>, tonic::Status>;
        /// Server streaming response type for the NotesForVoting method.
        type NotesForVotingStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::NotesForVotingResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Returns a stream of `NotesForVotingResponse`s.
        async fn notes_for_voting(
            &self,
            request: tonic::Request<super::NotesForVotingRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::NotesForVotingStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the Assets method.
        type AssetsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::AssetsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Queries for metadata about known assets.
        /// Returns a stream of `AssetsResponse`s.
        async fn assets(
            &self,
            request: tonic::Request<super::AssetsRequest>,
        ) -> std::result::Result<tonic::Response<Self::AssetsStream>, tonic::Status>;
        /// Query for metadata about a specific asset, by asset ID.
        ///
        /// This is the same as the method on the shielded pool's `QueryService`, but exposing it
        /// here allows a view server to provide more specific or opinionated asset metadata -- like
        /// using an asset registry to provide tickers, symbols, etc.
        async fn asset_metadata_by_id(
            &self,
            request: tonic::Request<super::AssetMetadataByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetMetadataByIdResponse>,
            tonic::Status,
        >;
        /// Query for the current app parameters.
        async fn app_parameters(
            &self,
            request: tonic::Request<super::AppParametersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AppParametersResponse>,
            tonic::Status,
        >;
        /// Query for the current gas prices.
        async fn gas_prices(
            &self,
            request: tonic::Request<super::GasPricesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GasPricesResponse>,
            tonic::Status,
        >;
        /// Query for the current FMD parameters.
        async fn fmd_parameters(
            &self,
            request: tonic::Request<super::FmdParametersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FmdParametersResponse>,
            tonic::Status,
        >;
        /// Query for an address given an address index
        async fn address_by_index(
            &self,
            request: tonic::Request<super::AddressByIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddressByIndexResponse>,
            tonic::Status,
        >;
        /// Query for wallet id
        async fn wallet_id(
            &self,
            request: tonic::Request<super::WalletIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WalletIdResponse>,
            tonic::Status,
        >;
        /// Query for an address given an address index
        async fn index_by_address(
            &self,
            request: tonic::Request<super::IndexByAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IndexByAddressResponse>,
            tonic::Status,
        >;
        /// Query for an ephemeral address
        async fn ephemeral_address(
            &self,
            request: tonic::Request<super::EphemeralAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EphemeralAddressResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the Balances method.
        type BalancesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BalancesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Query for balance of a given address.
        /// Returns a stream of `BalancesResponses`.
        async fn balances(
            &self,
            request: tonic::Request<super::BalancesRequest>,
        ) -> std::result::Result<tonic::Response<Self::BalancesStream>, tonic::Status>;
        /// Query for a note by its note commitment, optionally waiting until the note is detected.
        async fn note_by_commitment(
            &self,
            request: tonic::Request<super::NoteByCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NoteByCommitmentResponse>,
            tonic::Status,
        >;
        /// Query for a swap by its swap commitment, optionally waiting until the swap is detected.
        async fn swap_by_commitment(
            &self,
            request: tonic::Request<super::SwapByCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SwapByCommitmentResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the UnclaimedSwaps method.
        type UnclaimedSwapsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::UnclaimedSwapsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Query for all unclaimed swaps.
        async fn unclaimed_swaps(
            &self,
            request: tonic::Request<super::UnclaimedSwapsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::UnclaimedSwapsStream>,
            tonic::Status,
        >;
        /// Query for whether a nullifier has been spent, optionally waiting until it is spent.
        async fn nullifier_status(
            &self,
            request: tonic::Request<super::NullifierStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NullifierStatusResponse>,
            tonic::Status,
        >;
        /// Query for a given transaction by its hash.
        async fn transaction_info_by_hash(
            &self,
            request: tonic::Request<super::TransactionInfoByHashRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfoByHashResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the TransactionInfo method.
        type TransactionInfoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::TransactionInfoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Query for the full transactions in the given range of blocks.
        /// Returns a stream of `TransactionInfoResponse`s.
        async fn transaction_info(
            &self,
            request: tonic::Request<super::TransactionInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::TransactionInfoStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the OwnedPositionIds method.
        type OwnedPositionIdsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::OwnedPositionIdsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Query for owned position IDs for the given trading pair and in the given position state.
        async fn owned_position_ids(
            &self,
            request: tonic::Request<super::OwnedPositionIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::OwnedPositionIdsStream>,
            tonic::Status,
        >;
        /// Translates a high-level intent ("send X funds to Y address") into a complete transaction plan.
        async fn transaction_planner(
            &self,
            request: tonic::Request<super::TransactionPlannerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionPlannerResponse>,
            tonic::Status,
        >;
        /// Returns authentication data for the given transaction plan.
        ///
        /// This method takes a complete transaction plan, so that the client can get a
        /// consistent set of authentication paths to a common root for the entire
        /// transaction.  (Otherwise, if a client made multiple requests, the wallet
        /// service could have advanced the state commitment tree  between queries).
        async fn witness(
            &self,
            request: tonic::Request<super::WitnessRequest>,
        ) -> std::result::Result<tonic::Response<super::WitnessResponse>, tonic::Status>;
        /// Server streaming response type for the WitnessAndBuild method.
        type WitnessAndBuildStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WitnessAndBuildResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Like `Witness`, but immediately uses the witness data to build (prove) the transaction.
        ///
        /// This method is useful for clients that can't easily do proving themselves, either because
        /// they're not written in Rust and can't easily import the proving code, or because they don't
        /// have access to proving keys, or some other reason.
        ///
        /// This method streams status updates to the caller before finally returning the transaction.
        async fn witness_and_build(
            &self,
            request: tonic::Request<super::WitnessAndBuildRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WitnessAndBuildStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the AuthorizeAndBuild method.
        type AuthorizeAndBuildStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::AuthorizeAndBuildResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Authorize a transaction plan and build the transaction.
        ///
        /// This method is only supported on view servers that have access to a custody
        /// service.  Otherwise, it will fail.
        ///
        /// Penumbra's transaction authorization mechanism is designed so transactions
        /// can be signed and built (proved) concurrently. This allows implementations
        /// to, e.g., start proving optimistically while presenting the user with an
        /// approval dialog.
        ///
        /// This method streams status updates to the caller before finally returning the transaction.
        async fn authorize_and_build(
            &self,
            request: tonic::Request<super::AuthorizeAndBuildRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::AuthorizeAndBuildStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the BroadcastTransaction method.
        type BroadcastTransactionStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::BroadcastTransactionResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Broadcast a transaction to the network, optionally waiting for full confirmation.
        ///
        /// This method streams status updates to the caller before finally returning confirmation.
        async fn broadcast_transaction(
            &self,
            request: tonic::Request<super::BroadcastTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::BroadcastTransactionStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DelegationsByAddressIndex method.
        type DelegationsByAddressIndexStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::DelegationsByAddressIndexResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Get delegation tokens for a given address index. Each delegation token will
        /// be represented by a `ValueView` with the given address index's balance of
        /// that token. Each `ValueView`'s `extended_metadata` field will contain the
        /// `ValidatorInfo` of the delegated validator.
        async fn delegations_by_address_index(
            &self,
            request: tonic::Request<super::DelegationsByAddressIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::DelegationsByAddressIndexStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the UnbondingTokensByAddressIndex method.
        type UnbondingTokensByAddressIndexStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::UnbondingTokensByAddressIndexResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Get unbonding tokens for the given address index, optionally filtered by
        /// whether the tokens are currently claimable.
        async fn unbonding_tokens_by_address_index(
            &self,
            request: tonic::Request<super::UnbondingTokensByAddressIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::UnbondingTokensByAddressIndexStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the Auctions method.
        type AuctionsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::AuctionsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Gets the auctions controlled by the user's wallet.
        async fn auctions(
            &self,
            request: tonic::Request<super::AuctionsRequest>,
        ) -> std::result::Result<tonic::Response<Self::AuctionsStream>, tonic::Status>;
    }
    /// The view RPC is used by a view client, who wants to do some
    /// transaction-related actions, to request data from a view service, which is
    /// responsible for synchronizing and scanning the public chain state with one or
    /// more full viewing keys.
    #[derive(Debug)]
    pub struct ViewServiceServer<T: ViewService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ViewService> ViewServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ViewServiceServer<T>
    where
        T: ViewService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/penumbra.view.v1.ViewService/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::StatusRequest>
                    for StatusSvc<T> {
                        type Response = super::StatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/StatusStream" => {
                    #[allow(non_camel_case_types)]
                    struct StatusStreamSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::StatusStreamRequest>
                    for StatusStreamSvc<T> {
                        type Response = super::StatusStreamResponse;
                        type ResponseStream = T::StatusStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::status_stream(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/Notes" => {
                    #[allow(non_camel_case_types)]
                    struct NotesSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::NotesRequest>
                    for NotesSvc<T> {
                        type Response = super::NotesResponse;
                        type ResponseStream = T::NotesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NotesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::notes(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NotesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/NotesForVoting" => {
                    #[allow(non_camel_case_types)]
                    struct NotesForVotingSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::NotesForVotingRequest>
                    for NotesForVotingSvc<T> {
                        type Response = super::NotesForVotingResponse;
                        type ResponseStream = T::NotesForVotingStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NotesForVotingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::notes_for_voting(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NotesForVotingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/Assets" => {
                    #[allow(non_camel_case_types)]
                    struct AssetsSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::AssetsRequest>
                    for AssetsSvc<T> {
                        type Response = super::AssetsResponse;
                        type ResponseStream = T::AssetsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::assets(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/AssetMetadataById" => {
                    #[allow(non_camel_case_types)]
                    struct AssetMetadataByIdSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::AssetMetadataByIdRequest>
                    for AssetMetadataByIdSvc<T> {
                        type Response = super::AssetMetadataByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetMetadataByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::asset_metadata_by_id(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetMetadataByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/AppParameters" => {
                    #[allow(non_camel_case_types)]
                    struct AppParametersSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::AppParametersRequest>
                    for AppParametersSvc<T> {
                        type Response = super::AppParametersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AppParametersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::app_parameters(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AppParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/GasPrices" => {
                    #[allow(non_camel_case_types)]
                    struct GasPricesSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::GasPricesRequest>
                    for GasPricesSvc<T> {
                        type Response = super::GasPricesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GasPricesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::gas_prices(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GasPricesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/FMDParameters" => {
                    #[allow(non_camel_case_types)]
                    struct FMDParametersSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::FmdParametersRequest>
                    for FMDParametersSvc<T> {
                        type Response = super::FmdParametersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FmdParametersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::fmd_parameters(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FMDParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/AddressByIndex" => {
                    #[allow(non_camel_case_types)]
                    struct AddressByIndexSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::AddressByIndexRequest>
                    for AddressByIndexSvc<T> {
                        type Response = super::AddressByIndexResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddressByIndexRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::address_by_index(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddressByIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/WalletId" => {
                    #[allow(non_camel_case_types)]
                    struct WalletIdSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::WalletIdRequest>
                    for WalletIdSvc<T> {
                        type Response = super::WalletIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WalletIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::wallet_id(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WalletIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/IndexByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct IndexByAddressSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::IndexByAddressRequest>
                    for IndexByAddressSvc<T> {
                        type Response = super::IndexByAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IndexByAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::index_by_address(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IndexByAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/EphemeralAddress" => {
                    #[allow(non_camel_case_types)]
                    struct EphemeralAddressSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::EphemeralAddressRequest>
                    for EphemeralAddressSvc<T> {
                        type Response = super::EphemeralAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EphemeralAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::ephemeral_address(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EphemeralAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/Balances" => {
                    #[allow(non_camel_case_types)]
                    struct BalancesSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::BalancesRequest>
                    for BalancesSvc<T> {
                        type Response = super::BalancesResponse;
                        type ResponseStream = T::BalancesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalancesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::balances(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalancesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/NoteByCommitment" => {
                    #[allow(non_camel_case_types)]
                    struct NoteByCommitmentSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::NoteByCommitmentRequest>
                    for NoteByCommitmentSvc<T> {
                        type Response = super::NoteByCommitmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NoteByCommitmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::note_by_commitment(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NoteByCommitmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/SwapByCommitment" => {
                    #[allow(non_camel_case_types)]
                    struct SwapByCommitmentSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::SwapByCommitmentRequest>
                    for SwapByCommitmentSvc<T> {
                        type Response = super::SwapByCommitmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SwapByCommitmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::swap_by_commitment(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SwapByCommitmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/UnclaimedSwaps" => {
                    #[allow(non_camel_case_types)]
                    struct UnclaimedSwapsSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::UnclaimedSwapsRequest>
                    for UnclaimedSwapsSvc<T> {
                        type Response = super::UnclaimedSwapsResponse;
                        type ResponseStream = T::UnclaimedSwapsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnclaimedSwapsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::unclaimed_swaps(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnclaimedSwapsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/NullifierStatus" => {
                    #[allow(non_camel_case_types)]
                    struct NullifierStatusSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::NullifierStatusRequest>
                    for NullifierStatusSvc<T> {
                        type Response = super::NullifierStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NullifierStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::nullifier_status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NullifierStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/TransactionInfoByHash" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionInfoByHashSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::TransactionInfoByHashRequest>
                    for TransactionInfoByHashSvc<T> {
                        type Response = super::TransactionInfoByHashResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionInfoByHashRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::transaction_info_by_hash(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransactionInfoByHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/TransactionInfo" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionInfoSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::TransactionInfoRequest,
                    > for TransactionInfoSvc<T> {
                        type Response = super::TransactionInfoResponse;
                        type ResponseStream = T::TransactionInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::transaction_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransactionInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/OwnedPositionIds" => {
                    #[allow(non_camel_case_types)]
                    struct OwnedPositionIdsSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::OwnedPositionIdsRequest,
                    > for OwnedPositionIdsSvc<T> {
                        type Response = super::OwnedPositionIdsResponse;
                        type ResponseStream = T::OwnedPositionIdsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OwnedPositionIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::owned_position_ids(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OwnedPositionIdsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/TransactionPlanner" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionPlannerSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::TransactionPlannerRequest>
                    for TransactionPlannerSvc<T> {
                        type Response = super::TransactionPlannerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionPlannerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::transaction_planner(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransactionPlannerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/Witness" => {
                    #[allow(non_camel_case_types)]
                    struct WitnessSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::UnaryService<super::WitnessRequest>
                    for WitnessSvc<T> {
                        type Response = super::WitnessResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WitnessRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::witness(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WitnessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/WitnessAndBuild" => {
                    #[allow(non_camel_case_types)]
                    struct WitnessAndBuildSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::WitnessAndBuildRequest,
                    > for WitnessAndBuildSvc<T> {
                        type Response = super::WitnessAndBuildResponse;
                        type ResponseStream = T::WitnessAndBuildStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WitnessAndBuildRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::witness_and_build(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WitnessAndBuildSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/AuthorizeAndBuild" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeAndBuildSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::AuthorizeAndBuildRequest,
                    > for AuthorizeAndBuildSvc<T> {
                        type Response = super::AuthorizeAndBuildResponse;
                        type ResponseStream = T::AuthorizeAndBuildStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthorizeAndBuildRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::authorize_and_build(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthorizeAndBuildSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/BroadcastTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastTransactionSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::BroadcastTransactionRequest,
                    > for BroadcastTransactionSvc<T> {
                        type Response = super::BroadcastTransactionResponse;
                        type ResponseStream = T::BroadcastTransactionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BroadcastTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::broadcast_transaction(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/DelegationsByAddressIndex" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationsByAddressIndexSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::DelegationsByAddressIndexRequest,
                    > for DelegationsByAddressIndexSvc<T> {
                        type Response = super::DelegationsByAddressIndexResponse;
                        type ResponseStream = T::DelegationsByAddressIndexStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DelegationsByAddressIndexRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::delegations_by_address_index(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegationsByAddressIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/UnbondingTokensByAddressIndex" => {
                    #[allow(non_camel_case_types)]
                    struct UnbondingTokensByAddressIndexSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<
                        super::UnbondingTokensByAddressIndexRequest,
                    > for UnbondingTokensByAddressIndexSvc<T> {
                        type Response = super::UnbondingTokensByAddressIndexResponse;
                        type ResponseStream = T::UnbondingTokensByAddressIndexStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnbondingTokensByAddressIndexRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::unbonding_tokens_by_address_index(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnbondingTokensByAddressIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.view.v1.ViewService/Auctions" => {
                    #[allow(non_camel_case_types)]
                    struct AuctionsSvc<T: ViewService>(pub Arc<T>);
                    impl<
                        T: ViewService,
                    > tonic::server::ServerStreamingService<super::AuctionsRequest>
                    for AuctionsSvc<T> {
                        type Response = super::AuctionsResponse;
                        type ResponseStream = T::AuctionsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuctionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ViewService>::auctions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuctionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ViewService> Clone for ViewServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ViewService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ViewService> tonic::server::NamedService for ViewServiceServer<T> {
        const NAME: &'static str = "penumbra.view.v1.ViewService";
    }
}
