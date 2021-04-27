use rust_fsm::*;

state_machine! {
    OrderItemStatusChart(ItemCreated)
        ItemCreated =>  {
                ApproveItem => ItemApproved,
                RejectItem => ItemRejected,
                CancelItem => ItemCancelled,
        },
        ItemApproved =>  {
                CompleteItem => ItemCompleted,
                CancelItem => ItemCancelled,
        },
        ItemCompleted(ApproveItem) => ItemApproved,
}
