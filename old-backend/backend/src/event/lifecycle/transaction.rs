use crate::lifecycle_event;
use crate::wrapper::entity::transaction::Transaction;

lifecycle_event! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct TransactionCreation {
        pub(crate) transaction: Transaction,
    }
}

lifecycle_event! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct TransactionDeletion {
        pub(crate) transaction: Transaction,
    }
}

lifecycle_event! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct TransactionUpdate {
        pub(crate) old_transaction: Transaction,
        pub(crate) new_transaction: Transaction,
    }
}
