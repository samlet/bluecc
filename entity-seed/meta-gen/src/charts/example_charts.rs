use rust_fsm::*;

state_machine! {
    ExampleStatusChart(ExstInDesign)
        ExstTested =>  {
                ExampleCompleted => ExstComplete,
                CancelExample => ExstCancelled,
        },
        ExstImplemented =>  {
                TestingComplete => ExstTested,
                CancelExample => ExstCancelled,
        },
        ExstDefined =>  {
                Approve => ExstApproved,
                CancelExample => ExstCancelled,
        },
        ExstInDesign =>  {
                DefinitionComplete => ExstDefined,
                CancelExample => ExstCancelled,
        },
        ExstApproved =>  {
                ImplementationComplete => ExstImplemented,
                CancelExample => ExstCancelled,
        },
}
