pub trait IndexerExt: Sized {
    fn with_default_penumbra_app_views(self) -> Self;
}

impl IndexerExt for cometindex::Indexer {
    fn with_default_penumbra_app_views(self) -> Self {
        self.with_index(crate::shielded_pool::fmd::ClueSet {})
            .with_index(crate::stake::ValidatorSet {})
            .with_index(crate::stake::Slashings {})
            .with_index(crate::stake::MissedBlocks {})
            .with_index(crate::stake::DelegationTxs {})
            .with_index(crate::stake::UndelegationTxs {})
            .with_index(crate::governance::GovernanceProposals {})
    }
}
