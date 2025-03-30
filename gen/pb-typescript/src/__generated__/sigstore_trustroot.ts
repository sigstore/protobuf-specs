// Code generated by protoc-gen-ts_proto. DO NOT EDIT.
// versions:
//   protoc-gen-ts_proto  v2.6.1
//   protoc               v5.29.4
// source: sigstore_trustroot.proto

/* eslint-disable */
import {
  DistinguishedName,
  HashAlgorithm,
  hashAlgorithmFromJSON,
  hashAlgorithmToJSON,
  LogId,
  PublicKey,
  TimeRange,
  X509CertificateChain,
} from "./sigstore_common";

/**
 * ServiceSelector specifies how a client SHOULD select a set of
 * Services to connect to. A client SHOULD throw an error if
 * the value is SERVICE_SELECTOR_UNDEFINED.
 */
export enum ServiceSelector {
  SERVICE_SELECTOR_UNDEFINED = 0,
  /**
   * ALL - Clients SHOULD select all Services based on supported API version
   * and validity window.
   */
  ALL = 1,
  /**
   * ANY - Clients SHOULD select one Service based on supported API version
   * and validity window. It is up to the client implementation to
   * decide how to select the Service, e.g. random or round-robin.
   */
  ANY = 2,
  /**
   * EXACT - Clients SHOULD select a specific number of Services based on
   * supported API version and validity window, using the provided
   * `count`. It is up to the client implementation to decide how to
   * select the Service, e.g. random or round-robin.
   */
  EXACT = 3,
}

export function serviceSelectorFromJSON(object: any): ServiceSelector {
  switch (object) {
    case 0:
    case "SERVICE_SELECTOR_UNDEFINED":
      return ServiceSelector.SERVICE_SELECTOR_UNDEFINED;
    case 1:
    case "ALL":
      return ServiceSelector.ALL;
    case 2:
    case "ANY":
      return ServiceSelector.ANY;
    case 3:
    case "EXACT":
      return ServiceSelector.EXACT;
    default:
      throw new globalThis.Error("Unrecognized enum value " + object + " for enum ServiceSelector");
  }
}

export function serviceSelectorToJSON(object: ServiceSelector): string {
  switch (object) {
    case ServiceSelector.SERVICE_SELECTOR_UNDEFINED:
      return "SERVICE_SELECTOR_UNDEFINED";
    case ServiceSelector.ALL:
      return "ALL";
    case ServiceSelector.ANY:
      return "ANY";
    case ServiceSelector.EXACT:
      return "EXACT";
    default:
      throw new globalThis.Error("Unrecognized enum value " + object + " for enum ServiceSelector");
  }
}

/**
 * TransparencyLogInstance describes the immutable parameters from a
 * transparency log.
 * See https://www.rfc-editor.org/rfc/rfc9162.html#name-log-parameters
 * for more details.
 * The included parameters are the minimal set required to identify a log,
 * and verify an inclusion proof/promise.
 */
export interface TransparencyLogInstance {
  /** The base URL at which can be used to URLs for the client. */
  baseUrl: string;
  /** The hash algorithm used for the Merkle Tree. */
  hashAlgorithm: HashAlgorithm;
  /**
   * The public key used to verify signatures generated by the log.
   * This attribute contains the signature algorithm used by the log.
   */
  publicKey:
    | PublicKey
    | undefined;
  /**
   * The unique identifier for this transparency log.
   * Represented as the SHA-256 hash of the log's public key,
   * calculated over the DER encoding of the key represented as
   * SubjectPublicKeyInfo.
   * See https://www.rfc-editor.org/rfc/rfc6962#section-3.2
   */
  logId:
    | LogId
    | undefined;
  /**
   * The checkpoint key identifier for the log used in a checkpoint.
   * Optional, not provided for logs that do not generate checkpoints.
   * For logs that do generate checkpoints, if not set, assume
   * log_id equals checkpoint_key_id.
   * Follows the specification described here
   * for ECDSA and Ed25519 signatures:
   * https://github.com/C2SP/C2SP/blob/main/signed-note.md#signatures
   * For RSA signatures, the key ID will match the ECDSA format, the
   * hashed DER-encoded SPKI public key. Publicly witnessed logs MUST NOT
   * use RSA-signed checkpoints, since witnesses do not support
   * RSA signatures.
   * This is provided for convenience. Clients can also calculate the
   * checkpoint key ID given the log's public key.
   * SHOULD be set for logs generating Ed25519 signatures.
   * SHOULD be 4 bytes long, as a truncated hash.
   */
  checkpointKeyId: LogId | undefined;
}

/**
 * CertificateAuthority enlists the information required to identify which
 * CA to use and perform signature verification.
 */
export interface CertificateAuthority {
  /**
   * The root certificate MUST be self-signed, and so the subject and
   * issuer are the same.
   */
  subject:
    | DistinguishedName
    | undefined;
  /**
   * The URI identifies the certificate authority.
   *
   * It is RECOMMENDED that the URI is the base URL for the certificate
   * authority, that can be provided to any SDK/client provided
   * by the certificate authority to interact with the certificate
   * authority.
   */
  uri: string;
  /**
   * The certificate chain for this CA. The last certificate in the chain
   * MUST be the trust anchor. The trust anchor MAY be a self-signed root
   * CA certificate or MAY be an intermediate CA certificate.
   */
  certChain:
    | X509CertificateChain
    | undefined;
  /**
   * The time the *entire* chain was valid. This is at max the
   * longest interval when *all* certificates in the chain were valid,
   * but it MAY be shorter. Clients MUST check timestamps against *both*
   * the `valid_for` time range *and* the entire certificate chain.
   *
   * The TimeRange should be considered valid *inclusive* of the
   * endpoints.
   */
  validFor: TimeRange | undefined;
}

/**
 * TrustedRoot describes the client's complete set of trusted entities.
 * How the TrustedRoot is populated is not specified, but can be a
 * combination of many sources such as TUF repositories, files on disk etc.
 *
 * The TrustedRoot is not meant to be used for any artifact verification, only
 * to capture the complete/global set of trusted verification materials.
 * When verifying an artifact, based on the artifact and policies, a selection
 * of keys/authorities are expected to be extracted and provided to the
 * verification function. This way the set of keys/authorities can be kept to
 * a minimal set by the policy to gain better control over what signatures
 * that are allowed.
 *
 * The embedded transparency logs, CT logs, CAs and TSAs MUST include any
 * previously used instance -- otherwise signatures made in the past cannot
 * be verified.
 *
 * All the listed instances SHOULD be sorted by the 'valid_for' in ascending
 * order, that is, the oldest instance first. Only the last instance is
 * allowed to have their 'end' timestamp unset. All previous instances MUST
 * have a closed interval of validity. The last instance MAY have a closed
 * interval. Clients MUST accept instances that overlaps in time, if not
 * clients may experience problems during rotations of verification
 * materials.
 *
 * To be able to manage planned rotations of either transparency logs or
 * certificate authorities, clienst MUST accept lists of instances where
 * the last instance have a 'valid_for' that belongs to the future.
 * This should not be a problem as clients SHOULD first seek the trust root
 * for a suitable instance before creating a per artifact trust root (that
 * is, a sub-set of the complete trust root) that is used for verification.
 */
export interface TrustedRoot {
  /**
   * MUST be application/vnd.dev.sigstore.trustedroot.v0.1+json
   * when encoded as JSON.
   * Clients MUST be able to process and parse content with the media
   * type defined in the old format:
   * application/vnd.dev.sigstore.trustedroot+json;version=0.1
   */
  mediaType: string;
  /** A set of trusted Rekor servers. */
  tlogs: TransparencyLogInstance[];
  /**
   * A set of trusted certificate authorities (e.g Fulcio), and any
   * intermediate certificates they provide.
   * If a CA is issuing multiple intermediate certificate, each
   * combination shall be represented as separate chain. I.e, a single
   * root cert may appear in multiple chains but with different
   * intermediate and/or leaf certificates.
   * The certificates are intended to be used for verifying artifact
   * signatures.
   */
  certificateAuthorities: CertificateAuthority[];
  /** A set of trusted certificate transparency logs. */
  ctlogs: TransparencyLogInstance[];
  /** A set of trusted timestamping authorities. */
  timestampAuthorities: CertificateAuthority[];
}

/**
 * SigningConfig represents the trusted entities/state needed by Sigstore
 * signing. In particular, it primarily contains service URLs that a Sigstore
 * signer may need to connect to for the online aspects of signing.
 */
export interface SigningConfig {
  /**
   * MUST be application/vnd.dev.sigstore.signingconfig.v0.2+json
   * Clients MAY choose to also support
   * application/vnd.dev.sigstore.signingconfig.v0.1+json
   */
  mediaType: string;
  /**
   * URLs to Fulcio-compatible CAs, capable of receiving
   * Certificate Signing Requests (CSRs) and responding with
   * issued certificates.
   *
   * These URLs MUST be the "base" URL for the CAs, which clients
   * should construct an appropriate CSR endpoint on top of.
   * For example, if a CA URL is `https://example.com/ca`, then
   * the client MAY construct the CSR endpoint as
   * `https://example.com/ca/api/v2/signingCert`.
   *
   * Clients MUST select only one Service with the highest API version
   * that the client is compatible with, that is within its
   * validity period, and has the newest validity start date.
   * Client SHOULD select the first Service that meets this requirement.
   * All listed Services SHOULD be sorted by the `valid_for` window in
   * descending order, with the newest instance first.
   */
  caUrls: Service[];
  /**
   * URLs to OpenID Connect identity providers.
   *
   * These URLs MUST be the "base" URLs for the OIDC IdPs, which clients
   * should perform well-known OpenID Connect discovery against.
   *
   * Clients MUST select only one Service with the highest API version
   * that the client is compatible with, that is within its
   * validity period, and has the newest validity start date.
   * Client SHOULD select the first Service that meets this requirement.
   * All listed Services SHOULD be sorted by the `valid_for` window in
   * descending order, with the newest instance first.
   */
  oidcUrls: Service[];
  /**
   * URLs to Rekor transparency logs.
   *
   * These URL MUST be the "base" URLs for the transparency logs,
   * which clients should construct appropriate API endpoints on top of.
   *
   * Clients MUST select Services with the highest API version
   * that the client is compatible with, that are within its
   * validity period, and have the newest validity start dates.
   * All listed Services SHOULD be sorted by the `valid_for` window in
   * descending order, with the newest instance first.
   *
   * Clients MUST select Services based on the selector value of
   * `rekor_tlog_config`.
   */
  rekorTlogUrls: Service[];
  /**
   * Specifies how a client should select the set of Rekor transparency
   * logs to write to.
   */
  rekorTlogConfig:
    | ServiceConfiguration
    | undefined;
  /**
   * URLs to RFC 3161 Time Stamping Authorities (TSA).
   *
   * These URLs MUST be the *full* URL for the TSA, meaning that it
   * should be suitable for submitting Time Stamp Requests (TSRs) to
   * via HTTP, per RFC 3161.
   *
   * Clients MUST select Services with the highest API version
   * that the client is compatible with, that are within its
   * validity period, and have the newest validity start dates.
   * All listed Services SHOULD be sorted by the `valid_for` window in
   * descending order, with the newest instance first.
   *
   * Clients MUST select Services based on the selector value of
   * `tsa_config`.
   */
  tsaUrls: Service[];
  /**
   * Specifies how a client should select the set of TSAs to request
   * signed timestamps from.
   */
  tsaConfig: ServiceConfiguration | undefined;
}

/**
 * Service represents an instance of a service that is a part of Sigstore infrastructure.
 * Clients MUST use the API version hint to determine the service with the
 * highest API version that the client is compatible with. Clients MUST also
 * only connect to services within the specified validity period and that has the
 * newest validity start date.
 */
export interface Service {
  /** URL of the service. MUST include scheme and authority. MAY include path. */
  url: string;
  /**
   * Specifies the major API version. A value of 0 represents a service that
   * has not yet been released.
   */
  majorApiVersion: number;
  /**
   * Validity period of a service. A service that has only a start date
   * SHOULD be considered the most recent instance of that service, but
   * the client MUST NOT assume there is only one valid instance.
   * The TimeRange MUST be considered valid *inclusive* of the
   * endpoints.
   */
  validFor: TimeRange | undefined;
}

/**
 * ServiceConfiguration specifies how a client should select a set of
 * Services to connect to, along with a count when a specific number
 * of Services is requested.
 */
export interface ServiceConfiguration {
  /** How a client should select a set of Services to connect to. */
  selector: ServiceSelector;
  /**
   * count specifies the number of Services the client should use.
   * Only used when selector is set to EXACT, and count MUST be greater
   * than 0. count MUST be less than or equal to the number of Services.
   */
  count: number;
}

/**
 * ClientTrustConfig describes the complete state needed by a client
 * to perform both signing and verification operations against a particular
 * instance of Sigstore.
 */
export interface ClientTrustConfig {
  /** MUST be application/vnd.dev.sigstore.clienttrustconfig.v0.1+json */
  mediaType: string;
  /** The root of trust, which MUST be present. */
  trustedRoot:
    | TrustedRoot
    | undefined;
  /** Configuration for signing clients, which MUST be present. */
  signingConfig: SigningConfig | undefined;
}

export const TransparencyLogInstance: MessageFns<TransparencyLogInstance> = {
  fromJSON(object: any): TransparencyLogInstance {
    return {
      baseUrl: isSet(object.baseUrl) ? globalThis.String(object.baseUrl) : "",
      hashAlgorithm: isSet(object.hashAlgorithm) ? hashAlgorithmFromJSON(object.hashAlgorithm) : 0,
      publicKey: isSet(object.publicKey) ? PublicKey.fromJSON(object.publicKey) : undefined,
      logId: isSet(object.logId) ? LogId.fromJSON(object.logId) : undefined,
      checkpointKeyId: isSet(object.checkpointKeyId) ? LogId.fromJSON(object.checkpointKeyId) : undefined,
    };
  },

  toJSON(message: TransparencyLogInstance): unknown {
    const obj: any = {};
    if (message.baseUrl !== "") {
      obj.baseUrl = message.baseUrl;
    }
    if (message.hashAlgorithm !== 0) {
      obj.hashAlgorithm = hashAlgorithmToJSON(message.hashAlgorithm);
    }
    if (message.publicKey !== undefined) {
      obj.publicKey = PublicKey.toJSON(message.publicKey);
    }
    if (message.logId !== undefined) {
      obj.logId = LogId.toJSON(message.logId);
    }
    if (message.checkpointKeyId !== undefined) {
      obj.checkpointKeyId = LogId.toJSON(message.checkpointKeyId);
    }
    return obj;
  },
};

export const CertificateAuthority: MessageFns<CertificateAuthority> = {
  fromJSON(object: any): CertificateAuthority {
    return {
      subject: isSet(object.subject) ? DistinguishedName.fromJSON(object.subject) : undefined,
      uri: isSet(object.uri) ? globalThis.String(object.uri) : "",
      certChain: isSet(object.certChain) ? X509CertificateChain.fromJSON(object.certChain) : undefined,
      validFor: isSet(object.validFor) ? TimeRange.fromJSON(object.validFor) : undefined,
    };
  },

  toJSON(message: CertificateAuthority): unknown {
    const obj: any = {};
    if (message.subject !== undefined) {
      obj.subject = DistinguishedName.toJSON(message.subject);
    }
    if (message.uri !== "") {
      obj.uri = message.uri;
    }
    if (message.certChain !== undefined) {
      obj.certChain = X509CertificateChain.toJSON(message.certChain);
    }
    if (message.validFor !== undefined) {
      obj.validFor = TimeRange.toJSON(message.validFor);
    }
    return obj;
  },
};

export const TrustedRoot: MessageFns<TrustedRoot> = {
  fromJSON(object: any): TrustedRoot {
    return {
      mediaType: isSet(object.mediaType) ? globalThis.String(object.mediaType) : "",
      tlogs: globalThis.Array.isArray(object?.tlogs)
        ? object.tlogs.map((e: any) => TransparencyLogInstance.fromJSON(e))
        : [],
      certificateAuthorities: globalThis.Array.isArray(object?.certificateAuthorities)
        ? object.certificateAuthorities.map((e: any) => CertificateAuthority.fromJSON(e))
        : [],
      ctlogs: globalThis.Array.isArray(object?.ctlogs)
        ? object.ctlogs.map((e: any) => TransparencyLogInstance.fromJSON(e))
        : [],
      timestampAuthorities: globalThis.Array.isArray(object?.timestampAuthorities)
        ? object.timestampAuthorities.map((e: any) => CertificateAuthority.fromJSON(e))
        : [],
    };
  },

  toJSON(message: TrustedRoot): unknown {
    const obj: any = {};
    if (message.mediaType !== "") {
      obj.mediaType = message.mediaType;
    }
    if (message.tlogs?.length) {
      obj.tlogs = message.tlogs.map((e) => TransparencyLogInstance.toJSON(e));
    }
    if (message.certificateAuthorities?.length) {
      obj.certificateAuthorities = message.certificateAuthorities.map((e) => CertificateAuthority.toJSON(e));
    }
    if (message.ctlogs?.length) {
      obj.ctlogs = message.ctlogs.map((e) => TransparencyLogInstance.toJSON(e));
    }
    if (message.timestampAuthorities?.length) {
      obj.timestampAuthorities = message.timestampAuthorities.map((e) => CertificateAuthority.toJSON(e));
    }
    return obj;
  },
};

export const SigningConfig: MessageFns<SigningConfig> = {
  fromJSON(object: any): SigningConfig {
    return {
      mediaType: isSet(object.mediaType) ? globalThis.String(object.mediaType) : "",
      caUrls: globalThis.Array.isArray(object?.caUrls) ? object.caUrls.map((e: any) => Service.fromJSON(e)) : [],
      oidcUrls: globalThis.Array.isArray(object?.oidcUrls) ? object.oidcUrls.map((e: any) => Service.fromJSON(e)) : [],
      rekorTlogUrls: globalThis.Array.isArray(object?.rekorTlogUrls)
        ? object.rekorTlogUrls.map((e: any) => Service.fromJSON(e))
        : [],
      rekorTlogConfig: isSet(object.rekorTlogConfig)
        ? ServiceConfiguration.fromJSON(object.rekorTlogConfig)
        : undefined,
      tsaUrls: globalThis.Array.isArray(object?.tsaUrls) ? object.tsaUrls.map((e: any) => Service.fromJSON(e)) : [],
      tsaConfig: isSet(object.tsaConfig) ? ServiceConfiguration.fromJSON(object.tsaConfig) : undefined,
    };
  },

  toJSON(message: SigningConfig): unknown {
    const obj: any = {};
    if (message.mediaType !== "") {
      obj.mediaType = message.mediaType;
    }
    if (message.caUrls?.length) {
      obj.caUrls = message.caUrls.map((e) => Service.toJSON(e));
    }
    if (message.oidcUrls?.length) {
      obj.oidcUrls = message.oidcUrls.map((e) => Service.toJSON(e));
    }
    if (message.rekorTlogUrls?.length) {
      obj.rekorTlogUrls = message.rekorTlogUrls.map((e) => Service.toJSON(e));
    }
    if (message.rekorTlogConfig !== undefined) {
      obj.rekorTlogConfig = ServiceConfiguration.toJSON(message.rekorTlogConfig);
    }
    if (message.tsaUrls?.length) {
      obj.tsaUrls = message.tsaUrls.map((e) => Service.toJSON(e));
    }
    if (message.tsaConfig !== undefined) {
      obj.tsaConfig = ServiceConfiguration.toJSON(message.tsaConfig);
    }
    return obj;
  },
};

export const Service: MessageFns<Service> = {
  fromJSON(object: any): Service {
    return {
      url: isSet(object.url) ? globalThis.String(object.url) : "",
      majorApiVersion: isSet(object.majorApiVersion) ? globalThis.Number(object.majorApiVersion) : 0,
      validFor: isSet(object.validFor) ? TimeRange.fromJSON(object.validFor) : undefined,
    };
  },

  toJSON(message: Service): unknown {
    const obj: any = {};
    if (message.url !== "") {
      obj.url = message.url;
    }
    if (message.majorApiVersion !== 0) {
      obj.majorApiVersion = Math.round(message.majorApiVersion);
    }
    if (message.validFor !== undefined) {
      obj.validFor = TimeRange.toJSON(message.validFor);
    }
    return obj;
  },
};

export const ServiceConfiguration: MessageFns<ServiceConfiguration> = {
  fromJSON(object: any): ServiceConfiguration {
    return {
      selector: isSet(object.selector) ? serviceSelectorFromJSON(object.selector) : 0,
      count: isSet(object.count) ? globalThis.Number(object.count) : 0,
    };
  },

  toJSON(message: ServiceConfiguration): unknown {
    const obj: any = {};
    if (message.selector !== 0) {
      obj.selector = serviceSelectorToJSON(message.selector);
    }
    if (message.count !== 0) {
      obj.count = Math.round(message.count);
    }
    return obj;
  },
};

export const ClientTrustConfig: MessageFns<ClientTrustConfig> = {
  fromJSON(object: any): ClientTrustConfig {
    return {
      mediaType: isSet(object.mediaType) ? globalThis.String(object.mediaType) : "",
      trustedRoot: isSet(object.trustedRoot) ? TrustedRoot.fromJSON(object.trustedRoot) : undefined,
      signingConfig: isSet(object.signingConfig) ? SigningConfig.fromJSON(object.signingConfig) : undefined,
    };
  },

  toJSON(message: ClientTrustConfig): unknown {
    const obj: any = {};
    if (message.mediaType !== "") {
      obj.mediaType = message.mediaType;
    }
    if (message.trustedRoot !== undefined) {
      obj.trustedRoot = TrustedRoot.toJSON(message.trustedRoot);
    }
    if (message.signingConfig !== undefined) {
      obj.signingConfig = SigningConfig.toJSON(message.signingConfig);
    }
    return obj;
  },
};

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

interface MessageFns<T> {
  fromJSON(object: any): T;
  toJSON(message: T): unknown;
}
