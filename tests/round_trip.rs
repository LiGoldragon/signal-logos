use signal_logos::{Request, encode_request};
use signal_sema_storage::{ContentHash, FixtureScope};
#[test]
fn project_round_trips() {
    let value = Request::ProjectRust {
        scope: FixtureScope(1),
        logos: ContentHash([2; 32]),
    };
    let bytes = encode_request(&value).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Request, rkyv::rancor::Error>(&bytes).unwrap(),
        value
    )
}
