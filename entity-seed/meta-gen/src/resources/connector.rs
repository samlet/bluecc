use rust_fsm::*;

state_machine! {
    Connector(Closed)

    Closed(Open) => Openning,
    Failed(TimerTriggered) => Openning,
    Openning => {
        Successful => Opened,
        Unsuccessful => Failed [SetupTimer]
    }
}
