use signal_logos::{Error, Rejection, Reply, Request, encode_reply, encode_request};
use signal_sema_storage::{ContentHash, FixtureScope};

#[test]
fn project_round_trips_through_typed_codec_boundary() {
    let value = Request::ProjectRust {
        scope: FixtureScope(1),
        logos: ContentHash([2; 32]),
    };
    let bytes = match encode_request(&value) {
        Ok(bytes) => bytes,
        Err(Error::Encoding(source)) => panic!("typed archive error: {source}"),
    };
    assert_eq!(
        rkyv::from_bytes::<Request, rkyv::rancor::Error>(&bytes).unwrap(),
        value
    );

    let reply: Result<Vec<u8>, Error> = encode_reply(&Reply::Rejected(Rejection::ProjectionFailed));
    assert!(reply.is_ok());
}
