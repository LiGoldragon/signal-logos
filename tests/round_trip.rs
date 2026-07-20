use signal_logos::{Error, Rejection, Reply, Request, encode_reply, encode_request};
use signal_sema_storage::{
    ContentHash, DocumentKey, DocumentKind, FixtureScope, SlotIdentifier, SlotSummary,
    SubscriptionIdentifier, Version,
};

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

#[test]
fn subscription_reply_round_trips_against_encoded_storage_contract() {
    let source = SlotSummary {
        key: DocumentKey {
            scope: FixtureScope(3),
            kind: DocumentKind::Logos,
            slot: SlotIdentifier(5),
        },
        version: Version(7),
        hash: ContentHash([11; 32]),
    };
    let value = Reply::Subscribed {
        identifier: SubscriptionIdentifier(13),
        initial: vec![source],
    };

    let bytes = match encode_reply(&value) {
        Ok(bytes) => bytes,
        Err(Error::Encoding(source)) => panic!("typed archive error: {source}"),
    };

    assert_eq!(
        rkyv::from_bytes::<Reply, rkyv::rancor::Error>(&bytes).unwrap(),
        value
    );
}
