/* eslint-disable */
import { Timestamp } from "./google/protobuf/timestamp";

/**
 * NewEntry defines the CloudEvents (https://cloudevents.io/) message that
 * is published when a new entry is added to Rekor. Adheres to the v1.0.2 schema
 * defined in https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/formats/cloudevents.json.
 */
export interface NewEntry {
  /** The CloudEvents specification version. */
  specVersion: string;
  /** The unique id of the entry in the log. */
  id: string;
  /** The JSON-encoded dev.sigstore.rekor.v1.TransparencyLogEntry message. */
  data: string;
  /** The type of the encoded data. Must be "application/json". */
  dataType: string;
  /** The type of the message. Must be "dev.sigstore.rekor.events.v1.NewEntry". */
  type: string;
  /** The source of the event. */
  source: string;
  /**
   * The kind (aka type) of the entry. Must match the value in the
   * dev.sigstore.rekor.v1.TransparencyLogEntry.KindVersion.kind field.
   */
  entryKind: string;
  /**
   * The subject identifiers for the entities that signed the artifact
   * uploaded to Rekor. This field may not be set if the entry_kind is not a
   * format that supports this type of metadata.
   */
  subjects: string[];
  /**
   * The timestamp that the entry was added to the log. Should match the
   * value in dev.sigstore.rekor.v1.TransparencyLogEntry.integrated_time.
   */
  time: Date | undefined;
}

function createBaseNewEntry(): NewEntry {
  return {
    specVersion: "",
    id: "",
    data: "",
    dataType: "",
    type: "",
    source: "",
    entryKind: "",
    subjects: [],
    time: undefined,
  };
}

export const NewEntry = {
  fromJSON(object: any): NewEntry {
    return {
      specVersion: isSet(object.specversion) ? String(object.specversion) : "",
      id: isSet(object.id) ? String(object.id) : "",
      data: isSet(object.data) ? String(object.data) : "",
      dataType: isSet(object.datacontenttype) ? String(object.datacontenttype) : "",
      type: isSet(object.type) ? String(object.type) : "",
      source: isSet(object.source) ? String(object.source) : "",
      entryKind: isSet(object.rekor_entry_kind) ? String(object.rekor_entry_kind) : "",
      subjects: Array.isArray(object?.rekor_signing_subjects)
        ? object.rekor_signing_subjects.map((e: any) => String(e))
        : [],
      time: isSet(object.time) ? fromJsonTimestamp(object.time) : undefined,
    };
  },

  toJSON(message: NewEntry): unknown {
    const obj: any = {};
    message.specVersion !== undefined && (obj.specversion = message.specVersion);
    message.id !== undefined && (obj.id = message.id);
    message.data !== undefined && (obj.data = message.data);
    message.dataType !== undefined && (obj.datacontenttype = message.dataType);
    message.type !== undefined && (obj.type = message.type);
    message.source !== undefined && (obj.source = message.source);
    message.entryKind !== undefined && (obj.rekor_entry_kind = message.entryKind);
    if (message.subjects) {
      obj.rekor_signing_subjects = message.subjects.map((e) => e);
    } else {
      obj.rekor_signing_subjects = [];
    }
    message.time !== undefined && (obj.time = message.time.toISOString());
    return obj;
  },
};

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

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
