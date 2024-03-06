# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: sigstore_rekor.proto
# plugin: python-betterproto
# This file has been @generated

from dataclasses import dataclass
from typing import List

import betterproto

from ...common import v1 as __common_v1__


@dataclass(eq=False, repr=False)
class KindVersion(betterproto.Message):
    """KindVersion contains the entry's kind and api version."""

    kind: str = betterproto.string_field(1)
    """
    Kind is the type of entry being stored in the log. See here for a list:
    https://github.com/sigstore/rekor/tree/main/pkg/types
    """

    version: str = betterproto.string_field(2)
    """The specific api version of the type."""


@dataclass(eq=False, repr=False)
class Checkpoint(betterproto.Message):
    """
    The checkpoint MUST contain a signature of the tree head (root hash), size
    of the tree and the transparency log's unique identifier (log ID). It MAY
    also be followed by any optional data. The result is a string,  the format
    is described here https://github.com/transparency-
    dev/formats/blob/main/log/README.md The details are here https://github.com
    /sigstore/rekor/blob/a6e58f72b6b18cc06cefe61808efd562b9726330/pkg/util/sign
    ed_note.go#L114 The signature has the same format as
    InclusionPromise.signed_entry_timestamp. See below for more details.
    """

    envelope: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class InclusionProof(betterproto.Message):
    """
    InclusionProof is the proof returned from the transparency log. Can be used
    for offline or online verification against the log.
    """

    log_index: int = betterproto.int64_field(1)
    """The index of the entry in the tree it was written to."""

    root_hash: bytes = betterproto.bytes_field(2)
    """
    The hash digest stored at the root of the merkle tree at the time the proof
    was generated.
    """

    tree_size: int = betterproto.int64_field(3)
    """The size of the merkle tree at the time the proof was generated."""

    hashes: List[bytes] = betterproto.bytes_field(4)
    """
    A list of hashes required to compute the inclusion proof, sorted in order
    from leaf to root. Note that leaf and root hashes are not included. The
    root hash is available separately in this message, and the leaf hash should
    be calculated by the client.
    """

    checkpoint: "Checkpoint" = betterproto.message_field(5)
    """
    Signature of the tree head, as of the time of this proof was generated. See
    above info on 'Checkpoint' for more details.
    """


@dataclass(eq=False, repr=False)
class InclusionPromise(betterproto.Message):
    """
    The inclusion promise is calculated by Rekor. It's calculated as a
    signature over a canonical JSON serialization of the persisted entry, the
    log ID, log index and the integration timestamp. See https://github.com/sig
    store/rekor/blob/a6e58f72b6b18cc06cefe61808efd562b9726330/pkg/api/entries.g
    o#L54 The format of the signature depends on the transparency log's public
    key. If the signature algorithm requires a hash function and/or a signature
    scheme (e.g. RSA) those has to be retrieved out-of-band from the log's
    operators, together with the public key. This is used to verify the
    integration timestamp's value and that the log has promised to include the
    entry.
    """

    signed_entry_timestamp: bytes = betterproto.bytes_field(1)


@dataclass(eq=False, repr=False)
class TransparencyLogEntry(betterproto.Message):
    """
    TransparencyLogEntry captures all the details required from Rekor to
    reconstruct an entry, given that the payload is provided via other means.
    This type can easily be created from the existing response from Rekor.
    Future iterations could rely on Rekor returning the minimal set of
    attributes (excluding the payload) that are required for verifying the
    inclusion promise. The inclusion promise (called SignedEntryTimestamp in
    the response from Rekor) is similar to a Signed Certificate Timestamp as
    described here https://www.rfc-editor.org/rfc/rfc6962.html#section-3.2.
    """

    log_index: int = betterproto.int64_field(1)
    """The global index of the entry, used when querying the log by index."""

    log_id: "__common_v1__.LogId" = betterproto.message_field(2)
    """The unique identifier of the log."""

    kind_version: "KindVersion" = betterproto.message_field(3)
    """
    The kind (type) and version of the object associated with this entry. These
    values are required to construct the entry during verification.
    """

    integrated_time: int = betterproto.int64_field(4)
    """The UNIX timestamp from the log when the entry was persisted."""

    inclusion_promise: "InclusionPromise" = betterproto.message_field(5)
    """
    The inclusion promise/signed entry timestamp from the log. Required for
    v0.1 bundles, and MUST be verified. Optional for >= v0.2 bundles, and
    SHOULD be verified when present. Also may be used as a signed timestamp.
    """

    inclusion_proof: "InclusionProof" = betterproto.message_field(6)
    """
    The inclusion proof can be used for offline or online verification that the
    entry was appended to the log, and that the log has not been altered.
    """

    canonicalized_body: bytes = betterproto.bytes_field(7)
    """
    Optional. The canonicalized transparency log entry, used to reconstruct the
    Signed Entry Timestamp (SET) during verification. The contents of this
    field are the same as the `body` field in a Rekor response, meaning that it
    does **not** include the "full" canonicalized form (of log index, ID, etc.)
    which are exposed as separate fields. The verifier is responsible for
    combining the `canonicalized_body`, `log_index`, `log_id`, and
    `integrated_time` into the payload that the SET's signature is generated
    over. This field is intended to be used in cases where the SET cannot be
    produced determinisitically (e.g. inconsistent JSON field ordering,
    differing whitespace, etc). If set, clients MUST verify that the signature
    referenced in the `canonicalized_body` matches the signature provided in
    the `Bundle.content`. If not set, clients are responsible for constructing
    an equivalent payload from other sources to verify the signature.
    """
