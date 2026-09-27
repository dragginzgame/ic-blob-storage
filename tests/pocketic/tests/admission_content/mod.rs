//! Actual caller isolation and passive discovery through the shared owner.
use super::*;
use blob_test_protocol::admission::{ContentLookup, ContentObservation, ContentState};

mod descriptor;
mod planning;

fn query(
    f: &Fixture,
    actor: Principal,
    input: ContentLookup,
) -> Result<Option<ContentObservation>, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "lookup_content", (input,))
        .expect("typed content query")
}

#[test]
fn content_discovery_keeps_exact_identity_isolation_and_uncertain_history() {
    let f = Fixture::new();
    let enrollment = f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    let input = ContentLookup {
        service: f.service,
        tenant: f.project,
        namespace: 1,
        root: p.request.root,
    };
    assert_eq!(query(&f, f.project, input), Ok(None));
    f.admit(p);
    let reserved = f.observe(p);
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(query(&f, actor, input), Err(Failure::NotProject));
    }
    assert_eq!(
        query(
            &f,
            f.other,
            ContentLookup {
                tenant: f.other,
                ..input
            }
        ),
        Ok(None)
    );
    assert_eq!(
        query(
            &f,
            f.project,
            ContentLookup {
                service: f.other,
                ..input
            }
        ),
        Err(Failure::WrongService)
    );
    assert_eq!(
        query(
            &f,
            f.project,
            ContentLookup {
                namespace: 2,
                ..input
            }
        ),
        Err(Failure::WrongNamespace)
    );
    assert_eq!(
        query(&f, f.project, input),
        Ok(Some(ContentObservation {
            request: p.request,
            state: ContentState::Reserved
        }))
    );
    assert_eq!(f.observe(p), reserved);
    f.prepare(p, &v);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Ok(Outcome::Exposed)
    );
    let exposed = f.observe(p);
    f.call(f.operator, f.enrollment(Some(enrollment), false))
        .unwrap();
    f.restart();
    assert_eq!(
        query(&f, f.project, input),
        Ok(Some(ContentObservation {
            request: p.request,
            state: ContentState::ExposurePossible
        }))
    );
    assert_eq!(f.observe(p), exposed);
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)),
        Ok(Outcome::Changed(true))
    );
    assert_eq!(
        query(&f, f.project, input),
        Ok(Some(ContentObservation {
            request: p.request,
            state: ContentState::ExposurePossible
        }))
    );
    assert_eq!(f.observe(p).usage, exposed.usage);
}

#[test]
fn content_discovery_keeps_cancelled_history_after_stop_start() {
    let f = Fixture::new();
    f.enroll();
    let p = f.permission(&vectors::vector("abc-text", 1));
    f.admit(p);
    f.call(f.project, Command::Revoke(p.request)).unwrap();
    let original = f.observe(p);
    f.restart();
    let input = ContentLookup {
        service: f.service,
        tenant: f.project,
        namespace: 1,
        root: p.request.root,
    };
    assert_eq!(
        query(&f, f.project, input),
        Ok(Some(ContentObservation {
            request: p.request,
            state: ContentState::Cancelled
        }))
    );
    assert_eq!(f.observe(p), original);
}
