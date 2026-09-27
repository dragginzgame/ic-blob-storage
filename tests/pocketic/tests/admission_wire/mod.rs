//! Encode exactly the operation-specific input sent by the admission fixture.
use blob_test_protocol::admission::{
    Command,
    input::{EnrollmentInput, PreparationInput, ReferenceInput},
    release::LifecycleCommand,
};

pub(super) fn encode(command: Command) -> (&'static str, Vec<u8>) {
    fn input<T: candid::CandidType>(method: &'static str, value: T) -> (&'static str, Vec<u8>) {
        (method, candid::encode_one(value).unwrap())
    }
    match command {
        Command::Enroll {
            tenant,
            expected,
            active,
        } => input(
            "enroll",
            EnrollmentInput {
                tenant,
                expected,
                active,
            },
        ),
        Command::Admit(value) => input("admit", value),
        Command::Prepare(request, manifest) => {
            input("prepare", PreparationInput { request, manifest })
        }
        Command::Expose(root) => input("expose", root),
        Command::Revoke(request) => input("revoke", request),
        Command::FixtureLifecycle(command) => match command {
            LifecycleCommand::SubstituteCompletion(request) => {
                input("substitute_completion", request)
            }
            LifecycleCommand::SubstituteDeletion(request) => input("substitute_deletion", request),
            LifecycleCommand::SubstituteSettlement(request) => {
                input("substitute_settlement", request)
            }
            LifecycleCommand::Reference {
                object,
                reference,
                operation,
                retain,
            } => input(
                "reference",
                ReferenceInput {
                    object,
                    reference,
                    operation,
                    retain,
                },
            ),
        },
    }
}
