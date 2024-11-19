/* eslint-disable */
import { Bundle } from "./sigstore_bundle";
import {
  HashAlgorithm,
  hashAlgorithmFromJSON,
  hashAlgorithmToJSON,
  PublicKeyDetails,
  publicKeyDetailsFromJSON,
  publicKeyDetailsToJSON,
} from "./sigstore_common";
import { TrustedRoot } from "./sigstore_trustroot";
import { Artifact } from "./sigstore_verification";

export interface FulcioSigningMaterial {
  /** The OIDC identity token to use for retrieving a signing certificate from fulcio. */
  identityToken: string;
  /** The type of key to use for signing. */
  keyDetails?: PublicKeyDetails | undefined;
}

export interface SigningOptions {
  signingMaterials?: { $case: "fulcioSigningMaterial"; fulcioSigningMaterial: FulcioSigningMaterial };
}

export interface BundleContentOptions {
  content?: { $case: "messageSignature"; messageSignature: BundleContentOptions_MessageSignature } | {
    $case: "dsseEnvelope";
    dsseEnvelope: BundleContentOptions_DSSE;
  };
}

export interface BundleContentOptions_MessageSignature {
  hashAlgorithm: HashAlgorithm;
}

export interface BundleContentOptions_DSSE {
  payload: Buffer;
  payloadType: string;
}

export interface TSAOptions {
}

/** Input captures all that is needed to call the signing method. */
export interface SigningInput {
  trustedRoot: TrustedRoot | undefined;
  signingOptions: SigningOptions | undefined;
  bundleContentOptions: BundleContentOptions | undefined;
  tsaOptions: TSAOptions | undefined;
  artifact?: Artifact | undefined;
}

export interface SigningResult {
  output?: { $case: "bundle"; bundle: Bundle } | { $case: "error"; error: SigningResult_Error };
}

export interface SigningResult_Error {
  message: string;
}

function createBaseFulcioSigningMaterial(): FulcioSigningMaterial {
  return { identityToken: "", keyDetails: undefined };
}

export const FulcioSigningMaterial = {
  fromJSON(object: any): FulcioSigningMaterial {
    return {
      identityToken: isSet(object.identityToken) ? String(object.identityToken) : "",
      keyDetails: isSet(object.keyDetails) ? publicKeyDetailsFromJSON(object.keyDetails) : undefined,
    };
  },

  toJSON(message: FulcioSigningMaterial): unknown {
    const obj: any = {};
    message.identityToken !== undefined && (obj.identityToken = message.identityToken);
    message.keyDetails !== undefined &&
      (obj.keyDetails = message.keyDetails !== undefined ? publicKeyDetailsToJSON(message.keyDetails) : undefined);
    return obj;
  },
};

function createBaseSigningOptions(): SigningOptions {
  return { signingMaterials: undefined };
}

export const SigningOptions = {
  fromJSON(object: any): SigningOptions {
    return {
      signingMaterials: isSet(object.fulcioSigningMaterial)
        ? {
          $case: "fulcioSigningMaterial",
          fulcioSigningMaterial: FulcioSigningMaterial.fromJSON(object.fulcioSigningMaterial),
        }
        : undefined,
    };
  },

  toJSON(message: SigningOptions): unknown {
    const obj: any = {};
    message.signingMaterials?.$case === "fulcioSigningMaterial" &&
      (obj.fulcioSigningMaterial = message.signingMaterials?.fulcioSigningMaterial
        ? FulcioSigningMaterial.toJSON(message.signingMaterials?.fulcioSigningMaterial)
        : undefined);
    return obj;
  },
};

function createBaseBundleContentOptions(): BundleContentOptions {
  return { content: undefined };
}

export const BundleContentOptions = {
  fromJSON(object: any): BundleContentOptions {
    return {
      content: isSet(object.messageSignature)
        ? {
          $case: "messageSignature",
          messageSignature: BundleContentOptions_MessageSignature.fromJSON(object.messageSignature),
        }
        : isSet(object.dsseEnvelope)
        ? { $case: "dsseEnvelope", dsseEnvelope: BundleContentOptions_DSSE.fromJSON(object.dsseEnvelope) }
        : undefined,
    };
  },

  toJSON(message: BundleContentOptions): unknown {
    const obj: any = {};
    message.content?.$case === "messageSignature" && (obj.messageSignature = message.content?.messageSignature
      ? BundleContentOptions_MessageSignature.toJSON(message.content?.messageSignature)
      : undefined);
    message.content?.$case === "dsseEnvelope" && (obj.dsseEnvelope = message.content?.dsseEnvelope
      ? BundleContentOptions_DSSE.toJSON(message.content?.dsseEnvelope)
      : undefined);
    return obj;
  },
};

function createBaseBundleContentOptions_MessageSignature(): BundleContentOptions_MessageSignature {
  return { hashAlgorithm: 0 };
}

export const BundleContentOptions_MessageSignature = {
  fromJSON(object: any): BundleContentOptions_MessageSignature {
    return { hashAlgorithm: isSet(object.hashAlgorithm) ? hashAlgorithmFromJSON(object.hashAlgorithm) : 0 };
  },

  toJSON(message: BundleContentOptions_MessageSignature): unknown {
    const obj: any = {};
    message.hashAlgorithm !== undefined && (obj.hashAlgorithm = hashAlgorithmToJSON(message.hashAlgorithm));
    return obj;
  },
};

function createBaseBundleContentOptions_DSSE(): BundleContentOptions_DSSE {
  return { payload: Buffer.alloc(0), payloadType: "" };
}

export const BundleContentOptions_DSSE = {
  fromJSON(object: any): BundleContentOptions_DSSE {
    return {
      payload: isSet(object.payload) ? Buffer.from(bytesFromBase64(object.payload)) : Buffer.alloc(0),
      payloadType: isSet(object.payloadType) ? String(object.payloadType) : "",
    };
  },

  toJSON(message: BundleContentOptions_DSSE): unknown {
    const obj: any = {};
    message.payload !== undefined &&
      (obj.payload = base64FromBytes(message.payload !== undefined ? message.payload : Buffer.alloc(0)));
    message.payloadType !== undefined && (obj.payloadType = message.payloadType);
    return obj;
  },
};

function createBaseTSAOptions(): TSAOptions {
  return {};
}

export const TSAOptions = {
  fromJSON(_: any): TSAOptions {
    return {};
  },

  toJSON(_: TSAOptions): unknown {
    const obj: any = {};
    return obj;
  },
};

function createBaseSigningInput(): SigningInput {
  return {
    trustedRoot: undefined,
    signingOptions: undefined,
    bundleContentOptions: undefined,
    tsaOptions: undefined,
    artifact: undefined,
  };
}

export const SigningInput = {
  fromJSON(object: any): SigningInput {
    return {
      trustedRoot: isSet(object.trustedRoot) ? TrustedRoot.fromJSON(object.trustedRoot) : undefined,
      signingOptions: isSet(object.signingOptions) ? SigningOptions.fromJSON(object.signingOptions) : undefined,
      bundleContentOptions: isSet(object.bundleContentOptions)
        ? BundleContentOptions.fromJSON(object.bundleContentOptions)
        : undefined,
      tsaOptions: isSet(object.tsaOptions) ? TSAOptions.fromJSON(object.tsaOptions) : undefined,
      artifact: isSet(object.artifact) ? Artifact.fromJSON(object.artifact) : undefined,
    };
  },

  toJSON(message: SigningInput): unknown {
    const obj: any = {};
    message.trustedRoot !== undefined &&
      (obj.trustedRoot = message.trustedRoot ? TrustedRoot.toJSON(message.trustedRoot) : undefined);
    message.signingOptions !== undefined &&
      (obj.signingOptions = message.signingOptions ? SigningOptions.toJSON(message.signingOptions) : undefined);
    message.bundleContentOptions !== undefined && (obj.bundleContentOptions = message.bundleContentOptions
      ? BundleContentOptions.toJSON(message.bundleContentOptions)
      : undefined);
    message.tsaOptions !== undefined &&
      (obj.tsaOptions = message.tsaOptions ? TSAOptions.toJSON(message.tsaOptions) : undefined);
    message.artifact !== undefined && (obj.artifact = message.artifact ? Artifact.toJSON(message.artifact) : undefined);
    return obj;
  },
};

function createBaseSigningResult(): SigningResult {
  return { output: undefined };
}

export const SigningResult = {
  fromJSON(object: any): SigningResult {
    return {
      output: isSet(object.bundle)
        ? { $case: "bundle", bundle: Bundle.fromJSON(object.bundle) }
        : isSet(object.error)
        ? { $case: "error", error: SigningResult_Error.fromJSON(object.error) }
        : undefined,
    };
  },

  toJSON(message: SigningResult): unknown {
    const obj: any = {};
    message.output?.$case === "bundle" &&
      (obj.bundle = message.output?.bundle ? Bundle.toJSON(message.output?.bundle) : undefined);
    message.output?.$case === "error" &&
      (obj.error = message.output?.error ? SigningResult_Error.toJSON(message.output?.error) : undefined);
    return obj;
  },
};

function createBaseSigningResult_Error(): SigningResult_Error {
  return { message: "" };
}

export const SigningResult_Error = {
  fromJSON(object: any): SigningResult_Error {
    return { message: isSet(object.message) ? String(object.message) : "" };
  },

  toJSON(message: SigningResult_Error): unknown {
    const obj: any = {};
    message.message !== undefined && (obj.message = message.message);
    return obj;
  },
};

declare var self: any | undefined;
declare var window: any | undefined;
declare var global: any | undefined;
var tsProtoGlobalThis: any = (() => {
  if (typeof globalThis !== "undefined") {
    return globalThis;
  }
  if (typeof self !== "undefined") {
    return self;
  }
  if (typeof window !== "undefined") {
    return window;
  }
  if (typeof global !== "undefined") {
    return global;
  }
  throw "Unable to locate global object";
})();

function bytesFromBase64(b64: string): Uint8Array {
  if (tsProtoGlobalThis.Buffer) {
    return Uint8Array.from(tsProtoGlobalThis.Buffer.from(b64, "base64"));
  } else {
    const bin = tsProtoGlobalThis.atob(b64);
    const arr = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; ++i) {
      arr[i] = bin.charCodeAt(i);
    }
    return arr;
  }
}

function base64FromBytes(arr: Uint8Array): string {
  if (tsProtoGlobalThis.Buffer) {
    return tsProtoGlobalThis.Buffer.from(arr).toString("base64");
  } else {
    const bin: string[] = [];
    arr.forEach((byte) => {
      bin.push(String.fromCharCode(byte));
    });
    return tsProtoGlobalThis.btoa(bin.join(""));
  }
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
