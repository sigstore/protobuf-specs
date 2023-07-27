/* eslint-disable */
import { Any } from "./google/protobuf/any";
import { Timestamp } from "./google/protobuf/timestamp";

export interface CloudEvent {
  /** Required Attributes */
  id: string;
  /** URI-reference */
  source: string;
  specVersion: string;
  type: string;
  /** Optional & Extension Attributes */
  attributes: { [key: string]: CloudEvent_CloudEventAttributeValue };
  data?: { $case: "binaryData"; binaryData: Buffer } | { $case: "textData"; textData: string } | {
    $case: "protoData";
    protoData: Any;
  };
}

export interface CloudEvent_AttributesEntry {
  key: string;
  value: CloudEvent_CloudEventAttributeValue | undefined;
}

export interface CloudEvent_CloudEventAttributeValue {
  attr?:
    | { $case: "ceBoolean"; ceBoolean: boolean }
    | { $case: "ceInteger"; ceInteger: number }
    | { $case: "ceString"; ceString: string }
    | { $case: "ceBytes"; ceBytes: Buffer }
    | { $case: "ceUri"; ceUri: string }
    | { $case: "ceUriRef"; ceUriRef: string }
    | { $case: "ceTimestamp"; ceTimestamp: Date };
}

export interface CloudEventBatch {
  events: CloudEvent[];
}

function createBaseCloudEvent(): CloudEvent {
  return { id: "", source: "", specVersion: "", type: "", attributes: {}, data: undefined };
}

export const CloudEvent = {
  fromJSON(object: any): CloudEvent {
    return {
      id: isSet(object.id) ? String(object.id) : "",
      source: isSet(object.source) ? String(object.source) : "",
      specVersion: isSet(object.specVersion) ? String(object.specVersion) : "",
      type: isSet(object.type) ? String(object.type) : "",
      attributes: isObject(object.attributes)
        ? Object.entries(object.attributes).reduce<{ [key: string]: CloudEvent_CloudEventAttributeValue }>(
          (acc, [key, value]) => {
            acc[key] = CloudEvent_CloudEventAttributeValue.fromJSON(value);
            return acc;
          },
          {},
        )
        : {},
      data: isSet(object.binaryData)
        ? { $case: "binaryData", binaryData: Buffer.from(bytesFromBase64(object.binaryData)) }
        : isSet(object.textData)
        ? { $case: "textData", textData: String(object.textData) }
        : isSet(object.protoData)
        ? { $case: "protoData", protoData: Any.fromJSON(object.protoData) }
        : undefined,
    };
  },

  toJSON(message: CloudEvent): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.source !== undefined && (obj.source = message.source);
    message.specVersion !== undefined && (obj.specVersion = message.specVersion);
    message.type !== undefined && (obj.type = message.type);
    obj.attributes = {};
    if (message.attributes) {
      Object.entries(message.attributes).forEach(([k, v]) => {
        obj.attributes[k] = CloudEvent_CloudEventAttributeValue.toJSON(v);
      });
    }
    message.data?.$case === "binaryData" &&
      (obj.binaryData = message.data?.binaryData !== undefined ? base64FromBytes(message.data?.binaryData) : undefined);
    message.data?.$case === "textData" && (obj.textData = message.data?.textData);
    message.data?.$case === "protoData" &&
      (obj.protoData = message.data?.protoData ? Any.toJSON(message.data?.protoData) : undefined);
    return obj;
  },
};

function createBaseCloudEvent_AttributesEntry(): CloudEvent_AttributesEntry {
  return { key: "", value: undefined };
}

export const CloudEvent_AttributesEntry = {
  fromJSON(object: any): CloudEvent_AttributesEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? CloudEvent_CloudEventAttributeValue.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: CloudEvent_AttributesEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value ? CloudEvent_CloudEventAttributeValue.toJSON(message.value) : undefined);
    return obj;
  },
};

function createBaseCloudEvent_CloudEventAttributeValue(): CloudEvent_CloudEventAttributeValue {
  return { attr: undefined };
}

export const CloudEvent_CloudEventAttributeValue = {
  fromJSON(object: any): CloudEvent_CloudEventAttributeValue {
    return {
      attr: isSet(object.ceBoolean)
        ? { $case: "ceBoolean", ceBoolean: Boolean(object.ceBoolean) }
        : isSet(object.ceInteger)
        ? { $case: "ceInteger", ceInteger: Number(object.ceInteger) }
        : isSet(object.ceString)
        ? { $case: "ceString", ceString: String(object.ceString) }
        : isSet(object.ceBytes)
        ? { $case: "ceBytes", ceBytes: Buffer.from(bytesFromBase64(object.ceBytes)) }
        : isSet(object.ceUri)
        ? { $case: "ceUri", ceUri: String(object.ceUri) }
        : isSet(object.ceUriRef)
        ? { $case: "ceUriRef", ceUriRef: String(object.ceUriRef) }
        : isSet(object.ceTimestamp)
        ? { $case: "ceTimestamp", ceTimestamp: fromJsonTimestamp(object.ceTimestamp) }
        : undefined,
    };
  },

  toJSON(message: CloudEvent_CloudEventAttributeValue): unknown {
    const obj: any = {};
    message.attr?.$case === "ceBoolean" && (obj.ceBoolean = message.attr?.ceBoolean);
    message.attr?.$case === "ceInteger" && (obj.ceInteger = Math.round(message.attr?.ceInteger));
    message.attr?.$case === "ceString" && (obj.ceString = message.attr?.ceString);
    message.attr?.$case === "ceBytes" &&
      (obj.ceBytes = message.attr?.ceBytes !== undefined ? base64FromBytes(message.attr?.ceBytes) : undefined);
    message.attr?.$case === "ceUri" && (obj.ceUri = message.attr?.ceUri);
    message.attr?.$case === "ceUriRef" && (obj.ceUriRef = message.attr?.ceUriRef);
    message.attr?.$case === "ceTimestamp" && (obj.ceTimestamp = message.attr?.ceTimestamp.toISOString());
    return obj;
  },
};

function createBaseCloudEventBatch(): CloudEventBatch {
  return { events: [] };
}

export const CloudEventBatch = {
  fromJSON(object: any): CloudEventBatch {
    return { events: Array.isArray(object?.events) ? object.events.map((e: any) => CloudEvent.fromJSON(e)) : [] };
  },

  toJSON(message: CloudEventBatch): unknown {
    const obj: any = {};
    if (message.events) {
      obj.events = message.events.map((e) => e ? CloudEvent.toJSON(e) : undefined);
    } else {
      obj.events = [];
    }
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

function fromTimestamp(t: Timestamp): Date {
  let millis = Number(t.seconds) * 1_000;
  millis += t.nanos / 1_000_000;
  return new Date(millis);
}

function fromJsonTimestamp(o: any): Date {
  if (o instanceof Date) {
    return o;
  } else if (typeof o === "string") {
    return new Date(o);
  } else {
    return fromTimestamp(Timestamp.fromJSON(o));
  }
}

function isObject(value: any): boolean {
  return typeof value === "object" && value !== null;
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
