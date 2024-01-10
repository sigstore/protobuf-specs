# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: sigstore_common.proto
# plugin: python-betterproto
from dataclasses import dataclass
from datetime import datetime
from typing import (
    List,
    Optional,
)

import betterproto


class HashAlgorithm(betterproto.Enum):
    """
    Only a subset of the secure hash standard algorithms are supported. See
    <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf> for more
    details. UNSPECIFIED SHOULD not be used, primary reason for inclusion is to
    force any proto JSON serialization to emit the used hash algorithm, as
    default option is to *omit* the default value of an enum (which is the
    first value, represented by '0'.
    """

    HASH_ALGORITHM_UNSPECIFIED = 0
    SHA2_256 = 1


class PublicKeyDetails(betterproto.Enum):
    """
    Details of a specific public key, capturing the the key encoding method,
    and signature algorithm. To avoid the possibility of contradicting formats
    such as PKCS1 with ED25519 the valid permutations are listed as a linear
    set instead of a cartesian set (i.e one combined variable instead of two,
    one for encoding and one for the signature algorithm).
    """

    PUBLIC_KEY_DETAILS_UNSPECIFIED = 0
    PKCS1_RSA_PKCS1V5 = 1
    """RSA"""

    PKCS1_RSA_PSS = 2
    PKIX_RSA_PKCS1V5 = 3
    PKIX_RSA_PSS = 4
    PKIX_ECDSA_P256_SHA_256 = 5
    """ECDSA"""

    PKIX_ECDSA_P256_HMAC_SHA_256 = 6
    PKIX_ED25519 = 7
    """Ed 25519"""


class SubjectAlternativeNameType(betterproto.Enum):
    SUBJECT_ALTERNATIVE_NAME_TYPE_UNSPECIFIED = 0
    EMAIL = 1
    URI = 2
    OTHER_NAME = 3
    """
    OID 1.3.6.1.4.1.57264.1.7 See
    https://github.com/sigstore/fulcio/blob/main/docs/oid-info.md#1361415726417
    --othername-san for more details.
    """


@dataclass(eq=False, repr=False)
class HashOutput(betterproto.Message):
    """
    HashOutput captures a digest of a 'message' (generic octet sequence) and
    the corresponding hash algorithm used.
    """

    algorithm: "HashAlgorithm" = betterproto.enum_field(1)
    digest: bytes = betterproto.bytes_field(2)
    """
    This is the raw octets of the message digest as computed by the hash
    algorithm.
    """


@dataclass(eq=False, repr=False)
class MessageSignature(betterproto.Message):
    """MessageSignature stores the computed signature over a message."""

    message_digest: "HashOutput" = betterproto.message_field(1)
    """
    Message digest can be used to identify the artifact. Clients MUST NOT
    attempt to use this digest to verify the associated signature; it is
    intended solely for identification.
    """

    signature: bytes = betterproto.bytes_field(2)
    """
    The raw bytes as returned from the signature algorithm. The signature
    algorithm (and so the format of the signature bytes) are determined by the
    contents of the 'verification_material', either a key-pair or a
    certificate. If using a certificate, the certificate contains the required
    information on the signature algorithm. When using a key pair, the
    algorithm MUST be part of the public key, which MUST be communicated out-
    of-band.
    """


@dataclass(eq=False, repr=False)
class LogId(betterproto.Message):
    """LogId captures the identity of a transparency log."""

    key_id: bytes = betterproto.bytes_field(1)
    """
    The unique id of the log, represented as the SHA-256 hash of the log's
    public key, calculated over the DER encoding of the key represented as
    SubjectPublicKeyInfo. See https://www.rfc-
    editor.org/rfc/rfc6962#section-3.2
    """


@dataclass(eq=False, repr=False)
class Rfc3161SignedTimestamp(betterproto.Message):
    """This message holds a RFC 3161 timestamp."""

    signed_timestamp: bytes = betterproto.bytes_field(1)
    """
    Signed timestamp is the DER encoded TimeStampResponse. See https://www.rfc-
    editor.org/rfc/rfc3161.html#section-2.4.2
    """


@dataclass(eq=False, repr=False)
class PublicKey(betterproto.Message):
    raw_bytes: Optional[bytes] = betterproto.bytes_field(
        1, optional=True, group="_raw_bytes"
    )
    """
    DER-encoded public key, encoding method is specified by the key_details
    attribute.
    """

    key_details: "PublicKeyDetails" = betterproto.enum_field(2)
    """Key encoding and signature algorithm to use for this key."""

    valid_for: Optional["TimeRange"] = betterproto.message_field(
        3, optional=True, group="_valid_for"
    )
    """Optional validity period for this key, *inclusive* of the endpoints."""


@dataclass(eq=False, repr=False)
class PublicKeyIdentifier(betterproto.Message):
    """
    PublicKeyIdentifier can be used to identify an (out of band) delivered key,
    to verify a signature.
    """

    hint: str = betterproto.string_field(1)
    """
    Optional unauthenticated hint on which key to use. The format of the hint
    must be agreed upon out of band by the signer and the verifiers, and so is
    not subject to this specification. Example use-case is to specify the
    public key to use, from a trusted key-ring. Implementors are RECOMMENDED to
    derive the value from the public key as described in RFC 6962. See:
    <https://www.rfc-editor.org/rfc/rfc6962#section-3.2>
    """


@dataclass(eq=False, repr=False)
class ObjectIdentifier(betterproto.Message):
    """An ASN.1 OBJECT IDENTIFIER"""

    id: List[int] = betterproto.int32_field(1)


@dataclass(eq=False, repr=False)
class ObjectIdentifierValuePair(betterproto.Message):
    """An OID and the corresponding (byte) value."""

    oid: "ObjectIdentifier" = betterproto.message_field(1)
    value: bytes = betterproto.bytes_field(2)


@dataclass(eq=False, repr=False)
class DistinguishedName(betterproto.Message):
    organization: str = betterproto.string_field(1)
    common_name: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class X509Certificate(betterproto.Message):
    raw_bytes: bytes = betterproto.bytes_field(1)
    """DER-encoded X.509 certificate."""


@dataclass(eq=False, repr=False)
class SubjectAlternativeName(betterproto.Message):
    type: "SubjectAlternativeNameType" = betterproto.enum_field(1)
    regexp: str = betterproto.string_field(2, group="identity")
    """A regular expression describing the expected value for the SAN."""

    value: str = betterproto.string_field(3, group="identity")
    """The exact value to match against."""


@dataclass(eq=False, repr=False)
class X509CertificateChain(betterproto.Message):
    """
    A collection of X.509 certificates. This "chain" can be used in multiple
    contexts, such as providing a root CA certificate within a TUF root of
    trust or multiple untrusted certificates for the purpose of chain building.
    """

    certificates: List["X509Certificate"] = betterproto.message_field(1)
    """
    One or more DER-encoded certificates. In some contexts (such as
    `VerificationMaterial.certificate`), this sequence has an imposed order.
    Unless explicitly specified, there is otherwise no guaranteed order.
    """


@dataclass(eq=False, repr=False)
class TimeRange(betterproto.Message):
    """
    The time range is closed and includes both the start and end times, (i.e.,
    [start, end]). End is optional to be able to capture a period that has
    started but has no known end.
    """

    start: datetime = betterproto.message_field(1)
    end: Optional[datetime] = betterproto.message_field(2, optional=True, group="_end")
