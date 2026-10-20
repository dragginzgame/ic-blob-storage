use super::*;
use model::{Binding, QueryTarget, Target};

#[test]
fn help_and_invalid_arguments_never_enter_the_query() {
    for (args, expected) in [(vec!["--help".into()], 0), (vec![], 2)] {
        let result = query_command::<Target>(
            &args,
            "usage",
            |_| Err(Failure::Arguments),
            |target| target,
            |_| panic!("help and invalid input must not query"),
            |_, _| panic!("invalid input has no report"),
        );
        assert_eq!(result.exit_code, expected);
        if expected == 0 {
            assert_eq!(result.output, "usage");
        } else {
            let error: serde_json::Value = serde_json::from_str(&result.output).unwrap();
            assert_eq!(
                error,
                json!({"scope":"pocketic_fixture", "error":"invalid_arguments", "target":null})
            );
        }
    }
}

#[test]
fn query_results_preserve_blockers_and_the_original_target_on_failure() {
    for (outcome, expected) in [
        (
            Ok(ops::Report {
                blocked: false,
                value: json!({"amount":"340282366920938463463374607431768211455"}),
            }),
            0,
        ),
        (
            Ok(ops::Report {
                blocked: true,
                value: json!({"blockers":["uncertain"]}),
            }),
            4,
        ),
        (Err(Failure::Denied), 3),
    ] {
        let target = Target {
            transport: QueryTarget {
                server: "127.0.0.1:1".parse().unwrap(),
                instance: 7,
                canister: ic_testkit::Fake::principal(1).into(),
                caller: ic_testkit::Fake::principal(2).into(),
            },
            binding: Binding::Funding(ic_testkit::Fake::principal(3).into()),
        };
        let original_target = ops::target_json(&target);
        let expected_report = outcome.as_ref().ok().map(|report| report.value.clone());
        let result = query_command(
            &[],
            "usage",
            |_| Ok(target),
            |target| target,
            |_| outcome,
            |target, value| json!({"target":ops::target_json(target), "report":value}),
        );
        assert_eq!(result.exit_code, expected);
        let value: serde_json::Value = serde_json::from_str(&result.output).unwrap();
        assert_eq!(value["target"], original_target);
        if let Some(report) = expected_report {
            assert_eq!(value["report"], report);
        } else {
            assert_eq!(value["error"], "denied");
        }
    }
}
