# Architecture
This pure contract crate owns the typed binary public operations for the logos daemon. Runtime, storage, actors, and policy remain in the daemon repository. Git dependencies are exact pushed revisions.

The storage types are pinned to `signal-sema-storage` 0.3.0, whose live Logos document arm is a validated archive of one complete current `WholeLogos` encoded form paired with the validated production nested vocabulary-name archive. This contract transports the resulting `ContentHash` and slot summaries; it does not reinterpret the archive, expose the superseded per-item carrier, or recreate a flat name table.

## Revisable leans
- **Signal-frame bypass.** This contract exposes raw rkyv `encode_request`/`encode_reply` payloads, and its daemon frames them with a hand-rolled u32-length + rkyv envelope rather than the workspace's shared `signal-frame` kernel. That trades away `signal-frame`'s short-header tap-anywhere observability — the uniform exchange framing readable at any hop. The lean holds while the prototype's point-to-point socket suffices. Revise it when cross-hop observability, shared handshake or version negotiation, or a common frame taxonomy is needed, adopting `signal-frame`'s `ExchangeFrame` as the transport.
