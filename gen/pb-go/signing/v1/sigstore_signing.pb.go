// Copyright 2024 The Sigstore Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        v3.21.6
// source: sigstore_signing.proto

package v1

import (
	v13 "github.com/sigstore/protobuf-specs/gen/pb-go/bundle/v1"
	v1 "github.com/sigstore/protobuf-specs/gen/pb-go/common/v1"
	v11 "github.com/sigstore/protobuf-specs/gen/pb-go/trustroot/v1"
	v12 "github.com/sigstore/protobuf-specs/gen/pb-go/verification/v1"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type FulcioSigningMaterial struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The OIDC identity token to use for retrieving a signing certificate from fulcio.
	IdentityToken string `protobuf:"bytes,1,opt,name=identity_token,json=identityToken,proto3" json:"identity_token,omitempty"`
	// The type of key to use for signing.
	KeyDetails *v1.PublicKeyDetails `protobuf:"varint,2,opt,name=key_details,json=keyDetails,proto3,enum=dev.sigstore.common.v1.PublicKeyDetails,oneof" json:"key_details,omitempty"`
}

func (x *FulcioSigningMaterial) Reset() {
	*x = FulcioSigningMaterial{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FulcioSigningMaterial) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FulcioSigningMaterial) ProtoMessage() {}

func (x *FulcioSigningMaterial) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FulcioSigningMaterial.ProtoReflect.Descriptor instead.
func (*FulcioSigningMaterial) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{0}
}

func (x *FulcioSigningMaterial) GetIdentityToken() string {
	if x != nil {
		return x.IdentityToken
	}
	return ""
}

func (x *FulcioSigningMaterial) GetKeyDetails() v1.PublicKeyDetails {
	if x != nil && x.KeyDetails != nil {
		return *x.KeyDetails
	}
	return v1.PublicKeyDetails(0)
}

type SigningOptions struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to SigningMaterials:
	//
	//	*SigningOptions_FulcioSigningMaterial
	SigningMaterials isSigningOptions_SigningMaterials `protobuf_oneof:"signing_materials"`
}

func (x *SigningOptions) Reset() {
	*x = SigningOptions{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SigningOptions) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SigningOptions) ProtoMessage() {}

func (x *SigningOptions) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SigningOptions.ProtoReflect.Descriptor instead.
func (*SigningOptions) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{1}
}

func (m *SigningOptions) GetSigningMaterials() isSigningOptions_SigningMaterials {
	if m != nil {
		return m.SigningMaterials
	}
	return nil
}

func (x *SigningOptions) GetFulcioSigningMaterial() *FulcioSigningMaterial {
	if x, ok := x.GetSigningMaterials().(*SigningOptions_FulcioSigningMaterial); ok {
		return x.FulcioSigningMaterial
	}
	return nil
}

type isSigningOptions_SigningMaterials interface {
	isSigningOptions_SigningMaterials()
}

type SigningOptions_FulcioSigningMaterial struct {
	// The OIDC identity token to use for retrieving a signing certificate from fulcio.
	FulcioSigningMaterial *FulcioSigningMaterial `protobuf:"bytes,1,opt,name=fulcio_signing_material,json=fulcioSigningMaterial,proto3,oneof"`
}

func (*SigningOptions_FulcioSigningMaterial) isSigningOptions_SigningMaterials() {}

type BundleContentOptions struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Content:
	//
	//	*BundleContentOptions_MessageSignature_
	//	*BundleContentOptions_DsseEnvelope
	Content isBundleContentOptions_Content `protobuf_oneof:"content"`
}

func (x *BundleContentOptions) Reset() {
	*x = BundleContentOptions{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BundleContentOptions) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BundleContentOptions) ProtoMessage() {}

func (x *BundleContentOptions) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BundleContentOptions.ProtoReflect.Descriptor instead.
func (*BundleContentOptions) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{2}
}

func (m *BundleContentOptions) GetContent() isBundleContentOptions_Content {
	if m != nil {
		return m.Content
	}
	return nil
}

func (x *BundleContentOptions) GetMessageSignature() *BundleContentOptions_MessageSignature {
	if x, ok := x.GetContent().(*BundleContentOptions_MessageSignature_); ok {
		return x.MessageSignature
	}
	return nil
}

func (x *BundleContentOptions) GetDsseEnvelope() *BundleContentOptions_DSSE {
	if x, ok := x.GetContent().(*BundleContentOptions_DsseEnvelope); ok {
		return x.DsseEnvelope
	}
	return nil
}

type isBundleContentOptions_Content interface {
	isBundleContentOptions_Content()
}

type BundleContentOptions_MessageSignature_ struct {
	MessageSignature *BundleContentOptions_MessageSignature `protobuf:"bytes,1,opt,name=message_signature,json=messageSignature,proto3,oneof"`
}

type BundleContentOptions_DsseEnvelope struct {
	DsseEnvelope *BundleContentOptions_DSSE `protobuf:"bytes,2,opt,name=dsse_envelope,json=dsseEnvelope,proto3,oneof"`
}

func (*BundleContentOptions_MessageSignature_) isBundleContentOptions_Content() {}

func (*BundleContentOptions_DsseEnvelope) isBundleContentOptions_Content() {}

type TSAOptions struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *TSAOptions) Reset() {
	*x = TSAOptions{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TSAOptions) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TSAOptions) ProtoMessage() {}

func (x *TSAOptions) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TSAOptions.ProtoReflect.Descriptor instead.
func (*TSAOptions) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{3}
}

// Input captures all that is needed to call the signing method.
type SigningInput struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	TrustedRoot          *v11.TrustedRoot      `protobuf:"bytes,1,opt,name=trusted_root,json=trustedRoot,proto3" json:"trusted_root,omitempty"`
	SigningOptions       *SigningOptions       `protobuf:"bytes,2,opt,name=signing_options,json=signingOptions,proto3" json:"signing_options,omitempty"`
	BundleContentOptions *BundleContentOptions `protobuf:"bytes,3,opt,name=bundle_content_options,json=bundleContentOptions,proto3" json:"bundle_content_options,omitempty"`
	TsaOptions           *TSAOptions           `protobuf:"bytes,4,opt,name=tsa_options,json=tsaOptions,proto3" json:"tsa_options,omitempty"`
	Artifact             *v12.Artifact         `protobuf:"bytes,5,opt,name=artifact,proto3,oneof" json:"artifact,omitempty"`
}

func (x *SigningInput) Reset() {
	*x = SigningInput{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SigningInput) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SigningInput) ProtoMessage() {}

func (x *SigningInput) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SigningInput.ProtoReflect.Descriptor instead.
func (*SigningInput) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{4}
}

func (x *SigningInput) GetTrustedRoot() *v11.TrustedRoot {
	if x != nil {
		return x.TrustedRoot
	}
	return nil
}

func (x *SigningInput) GetSigningOptions() *SigningOptions {
	if x != nil {
		return x.SigningOptions
	}
	return nil
}

func (x *SigningInput) GetBundleContentOptions() *BundleContentOptions {
	if x != nil {
		return x.BundleContentOptions
	}
	return nil
}

func (x *SigningInput) GetTsaOptions() *TSAOptions {
	if x != nil {
		return x.TsaOptions
	}
	return nil
}

func (x *SigningInput) GetArtifact() *v12.Artifact {
	if x != nil {
		return x.Artifact
	}
	return nil
}

type SigningResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Output:
	//
	//	*SigningResult_Bundle
	//	*SigningResult_Error_
	Output isSigningResult_Output `protobuf_oneof:"output"`
}

func (x *SigningResult) Reset() {
	*x = SigningResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SigningResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SigningResult) ProtoMessage() {}

func (x *SigningResult) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SigningResult.ProtoReflect.Descriptor instead.
func (*SigningResult) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{5}
}

func (m *SigningResult) GetOutput() isSigningResult_Output {
	if m != nil {
		return m.Output
	}
	return nil
}

func (x *SigningResult) GetBundle() *v13.Bundle {
	if x, ok := x.GetOutput().(*SigningResult_Bundle); ok {
		return x.Bundle
	}
	return nil
}

func (x *SigningResult) GetError() *SigningResult_Error {
	if x, ok := x.GetOutput().(*SigningResult_Error_); ok {
		return x.Error
	}
	return nil
}

type isSigningResult_Output interface {
	isSigningResult_Output()
}

type SigningResult_Bundle struct {
	Bundle *v13.Bundle `protobuf:"bytes,1,opt,name=bundle,proto3,oneof"`
}

type SigningResult_Error_ struct {
	Error *SigningResult_Error `protobuf:"bytes,2,opt,name=error,proto3,oneof"`
}

func (*SigningResult_Bundle) isSigningResult_Output() {}

func (*SigningResult_Error_) isSigningResult_Output() {}

type BundleContentOptions_MessageSignature struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	HashAlgorithm v1.HashAlgorithm `protobuf:"varint,1,opt,name=hash_algorithm,json=hashAlgorithm,proto3,enum=dev.sigstore.common.v1.HashAlgorithm" json:"hash_algorithm,omitempty"`
}

func (x *BundleContentOptions_MessageSignature) Reset() {
	*x = BundleContentOptions_MessageSignature{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BundleContentOptions_MessageSignature) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BundleContentOptions_MessageSignature) ProtoMessage() {}

func (x *BundleContentOptions_MessageSignature) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BundleContentOptions_MessageSignature.ProtoReflect.Descriptor instead.
func (*BundleContentOptions_MessageSignature) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{2, 0}
}

func (x *BundleContentOptions_MessageSignature) GetHashAlgorithm() v1.HashAlgorithm {
	if x != nil {
		return x.HashAlgorithm
	}
	return v1.HashAlgorithm(0)
}

type BundleContentOptions_DSSE struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Payload     []byte `protobuf:"bytes,1,opt,name=payload,proto3" json:"payload,omitempty"`
	PayloadType string `protobuf:"bytes,2,opt,name=payloadType,proto3" json:"payloadType,omitempty"`
}

func (x *BundleContentOptions_DSSE) Reset() {
	*x = BundleContentOptions_DSSE{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BundleContentOptions_DSSE) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BundleContentOptions_DSSE) ProtoMessage() {}

func (x *BundleContentOptions_DSSE) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BundleContentOptions_DSSE.ProtoReflect.Descriptor instead.
func (*BundleContentOptions_DSSE) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{2, 1}
}

func (x *BundleContentOptions_DSSE) GetPayload() []byte {
	if x != nil {
		return x.Payload
	}
	return nil
}

func (x *BundleContentOptions_DSSE) GetPayloadType() string {
	if x != nil {
		return x.PayloadType
	}
	return ""
}

type SigningResult_Error struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Message string `protobuf:"bytes,1,opt,name=message,proto3" json:"message,omitempty"`
}

func (x *SigningResult_Error) Reset() {
	*x = SigningResult_Error{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sigstore_signing_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SigningResult_Error) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SigningResult_Error) ProtoMessage() {}

func (x *SigningResult_Error) ProtoReflect() protoreflect.Message {
	mi := &file_sigstore_signing_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SigningResult_Error.ProtoReflect.Descriptor instead.
func (*SigningResult_Error) Descriptor() ([]byte, []int) {
	return file_sigstore_signing_proto_rawDescGZIP(), []int{5, 0}
}

func (x *SigningResult_Error) GetMessage() string {
	if x != nil {
		return x.Message
	}
	return ""
}

var File_sigstore_signing_proto protoreflect.FileDescriptor

var file_sigstore_signing_proto_rawDesc = []byte{
	0x0a, 0x16, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x69,
	0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x64, 0x65, 0x76, 0x2e, 0x73, 0x69,
	0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76,
	0x31, 0x1a, 0x18, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x74, 0x72, 0x75, 0x73,
	0x74, 0x72, 0x6f, 0x6f, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x73, 0x69, 0x67,
	0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x1a, 0x15, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x62, 0x75, 0x6e,
	0x64, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1b, 0x73, 0x69, 0x67, 0x73, 0x74,
	0x6f, 0x72, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x9e, 0x01, 0x0a, 0x15, 0x46, 0x75, 0x6c, 0x63, 0x69,
	0x6f, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c,
	0x12, 0x25, 0x0a, 0x0e, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x74, 0x6f, 0x6b,
	0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x79, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x4e, 0x0a, 0x0b, 0x6b, 0x65, 0x79, 0x5f, 0x64,
	0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x28, 0x2e, 0x64,
	0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x6d,
	0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x44,
	0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x48, 0x00, 0x52, 0x0a, 0x6b, 0x65, 0x79, 0x44, 0x65, 0x74,
	0x61, 0x69, 0x6c, 0x73, 0x88, 0x01, 0x01, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x6b, 0x65, 0x79, 0x5f,
	0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x8f, 0x01, 0x0a, 0x0e, 0x53, 0x69, 0x67, 0x6e,
	0x69, 0x6e, 0x67, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x68, 0x0a, 0x17, 0x66, 0x75,
	0x6c, 0x63, 0x69, 0x6f, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x6d, 0x61, 0x74,
	0x65, 0x72, 0x69, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x64, 0x65,
	0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69,
	0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x46, 0x75, 0x6c, 0x63, 0x69, 0x6f, 0x53, 0x69, 0x67, 0x6e,
	0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x15, 0x66,
	0x75, 0x6c, 0x63, 0x69, 0x6f, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x65,
	0x72, 0x69, 0x61, 0x6c, 0x42, 0x13, 0x0a, 0x11, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x5f,
	0x6d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x73, 0x22, 0x91, 0x03, 0x0a, 0x14, 0x42, 0x75,
	0x6e, 0x64, 0x6c, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x12, 0x6d, 0x0a, 0x11, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69,
	0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3e, 0x2e,
	0x64, 0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x69, 0x67,
	0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x43, 0x6f,
	0x6e, 0x74, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x4d, 0x65, 0x73,
	0x73, 0x61, 0x67, 0x65, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x48, 0x00, 0x52,
	0x10, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
	0x65, 0x12, 0x59, 0x0a, 0x0d, 0x64, 0x73, 0x73, 0x65, 0x5f, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f,
	0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73,
	0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x2e,
	0x76, 0x31, 0x2e, 0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
	0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x44, 0x53, 0x53, 0x45, 0x48, 0x00, 0x52, 0x0c,
	0x64, 0x73, 0x73, 0x65, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x1a, 0x60, 0x0a, 0x10,
	0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
	0x12, 0x4c, 0x0a, 0x0e, 0x68, 0x61, 0x73, 0x68, 0x5f, 0x61, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74,
	0x68, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x25, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73,
	0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76,
	0x31, 0x2e, 0x48, 0x61, 0x73, 0x68, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x52,
	0x0d, 0x68, 0x61, 0x73, 0x68, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x1a, 0x42,
	0x0a, 0x04, 0x44, 0x53, 0x53, 0x45, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61,
	0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
	0x12, 0x20, 0x0a, 0x0b, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x54, 0x79, 0x70, 0x65, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x54, 0x79,
	0x70, 0x65, 0x42, 0x09, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22, 0x0c, 0x0a,
	0x0a, 0x54, 0x53, 0x41, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0xac, 0x03, 0x0a, 0x0c,
	0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x12, 0x49, 0x0a, 0x0c,
	0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x6f, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x26, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72,
	0x65, 0x2e, 0x74, 0x72, 0x75, 0x73, 0x74, 0x72, 0x6f, 0x6f, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x54,
	0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x0b, 0x74, 0x72, 0x75, 0x73,
	0x74, 0x65, 0x64, 0x52, 0x6f, 0x6f, 0x74, 0x12, 0x50, 0x0a, 0x0f, 0x73, 0x69, 0x67, 0x6e, 0x69,
	0x6e, 0x67, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x27, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e,
	0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x69,
	0x6e, 0x67, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x0e, 0x73, 0x69, 0x67, 0x6e, 0x69,
	0x6e, 0x67, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x63, 0x0a, 0x16, 0x62, 0x75, 0x6e,
	0x64, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x6f, 0x70, 0x74, 0x69,
	0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x64, 0x65, 0x76, 0x2e,
	0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67,
	0x2e, 0x76, 0x31, 0x2e, 0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
	0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x14, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65,
	0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x44,
	0x0a, 0x0b, 0x74, 0x73, 0x61, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f,
	0x72, 0x65, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x53,
	0x41, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x0a, 0x74, 0x73, 0x61, 0x4f, 0x70, 0x74,
	0x69, 0x6f, 0x6e, 0x73, 0x12, 0x47, 0x0a, 0x08, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74,
	0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73, 0x69, 0x67,
	0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x48, 0x00,
	0x52, 0x08, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x88, 0x01, 0x01, 0x42, 0x0b, 0x0a,
	0x09, 0x5f, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x22, 0xbc, 0x01, 0x0a, 0x0d, 0x53,
	0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x38, 0x0a, 0x06,
	0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64,
	0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x62, 0x75, 0x6e, 0x64,
	0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x48, 0x00, 0x52, 0x06,
	0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x12, 0x44, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x64, 0x65, 0x76, 0x2e, 0x73, 0x69, 0x67, 0x73,
	0x74, 0x6f, 0x72, 0x65, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e,
	0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x2e, 0x45, 0x72,
	0x72, 0x6f, 0x72, 0x48, 0x00, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x1a, 0x21, 0x0a, 0x05,
	0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x42,
	0x08, 0x0a, 0x06, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x42, 0x80, 0x01, 0x0a, 0x1d, 0x64, 0x65,
	0x76, 0x2e, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x2e, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x42, 0x0c, 0x53, 0x69, 0x67,
	0x6e, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x37, 0x67, 0x69, 0x74,
	0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x73, 0x2f,
	0x67, 0x65, 0x6e, 0x2f, 0x70, 0x62, 0x2d, 0x67, 0x6f, 0x2f, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e,
	0x67, 0x2f, 0x76, 0x31, 0xea, 0x02, 0x15, 0x53, 0x69, 0x67, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x3a,
	0x3a, 0x53, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sigstore_signing_proto_rawDescOnce sync.Once
	file_sigstore_signing_proto_rawDescData = file_sigstore_signing_proto_rawDesc
)

func file_sigstore_signing_proto_rawDescGZIP() []byte {
	file_sigstore_signing_proto_rawDescOnce.Do(func() {
		file_sigstore_signing_proto_rawDescData = protoimpl.X.CompressGZIP(file_sigstore_signing_proto_rawDescData)
	})
	return file_sigstore_signing_proto_rawDescData
}

var file_sigstore_signing_proto_msgTypes = make([]protoimpl.MessageInfo, 9)
var file_sigstore_signing_proto_goTypes = []interface{}{
	(*FulcioSigningMaterial)(nil),                 // 0: dev.sigstore.signing.v1.FulcioSigningMaterial
	(*SigningOptions)(nil),                        // 1: dev.sigstore.signing.v1.SigningOptions
	(*BundleContentOptions)(nil),                  // 2: dev.sigstore.signing.v1.BundleContentOptions
	(*TSAOptions)(nil),                            // 3: dev.sigstore.signing.v1.TSAOptions
	(*SigningInput)(nil),                          // 4: dev.sigstore.signing.v1.SigningInput
	(*SigningResult)(nil),                         // 5: dev.sigstore.signing.v1.SigningResult
	(*BundleContentOptions_MessageSignature)(nil), // 6: dev.sigstore.signing.v1.BundleContentOptions.MessageSignature
	(*BundleContentOptions_DSSE)(nil),             // 7: dev.sigstore.signing.v1.BundleContentOptions.DSSE
	(*SigningResult_Error)(nil),                   // 8: dev.sigstore.signing.v1.SigningResult.Error
	(v1.PublicKeyDetails)(0),                      // 9: dev.sigstore.common.v1.PublicKeyDetails
	(*v11.TrustedRoot)(nil),                       // 10: dev.sigstore.trustroot.v1.TrustedRoot
	(*v12.Artifact)(nil),                          // 11: dev.sigstore.verification.v1.Artifact
	(*v13.Bundle)(nil),                            // 12: dev.sigstore.bundle.v1.Bundle
	(v1.HashAlgorithm)(0),                         // 13: dev.sigstore.common.v1.HashAlgorithm
}
var file_sigstore_signing_proto_depIdxs = []int32{
	9,  // 0: dev.sigstore.signing.v1.FulcioSigningMaterial.key_details:type_name -> dev.sigstore.common.v1.PublicKeyDetails
	0,  // 1: dev.sigstore.signing.v1.SigningOptions.fulcio_signing_material:type_name -> dev.sigstore.signing.v1.FulcioSigningMaterial
	6,  // 2: dev.sigstore.signing.v1.BundleContentOptions.message_signature:type_name -> dev.sigstore.signing.v1.BundleContentOptions.MessageSignature
	7,  // 3: dev.sigstore.signing.v1.BundleContentOptions.dsse_envelope:type_name -> dev.sigstore.signing.v1.BundleContentOptions.DSSE
	10, // 4: dev.sigstore.signing.v1.SigningInput.trusted_root:type_name -> dev.sigstore.trustroot.v1.TrustedRoot
	1,  // 5: dev.sigstore.signing.v1.SigningInput.signing_options:type_name -> dev.sigstore.signing.v1.SigningOptions
	2,  // 6: dev.sigstore.signing.v1.SigningInput.bundle_content_options:type_name -> dev.sigstore.signing.v1.BundleContentOptions
	3,  // 7: dev.sigstore.signing.v1.SigningInput.tsa_options:type_name -> dev.sigstore.signing.v1.TSAOptions
	11, // 8: dev.sigstore.signing.v1.SigningInput.artifact:type_name -> dev.sigstore.verification.v1.Artifact
	12, // 9: dev.sigstore.signing.v1.SigningResult.bundle:type_name -> dev.sigstore.bundle.v1.Bundle
	8,  // 10: dev.sigstore.signing.v1.SigningResult.error:type_name -> dev.sigstore.signing.v1.SigningResult.Error
	13, // 11: dev.sigstore.signing.v1.BundleContentOptions.MessageSignature.hash_algorithm:type_name -> dev.sigstore.common.v1.HashAlgorithm
	12, // [12:12] is the sub-list for method output_type
	12, // [12:12] is the sub-list for method input_type
	12, // [12:12] is the sub-list for extension type_name
	12, // [12:12] is the sub-list for extension extendee
	0,  // [0:12] is the sub-list for field type_name
}

func init() { file_sigstore_signing_proto_init() }
func file_sigstore_signing_proto_init() {
	if File_sigstore_signing_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sigstore_signing_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FulcioSigningMaterial); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SigningOptions); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BundleContentOptions); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TSAOptions); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SigningInput); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SigningResult); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BundleContentOptions_MessageSignature); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BundleContentOptions_DSSE); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sigstore_signing_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SigningResult_Error); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_sigstore_signing_proto_msgTypes[0].OneofWrappers = []interface{}{}
	file_sigstore_signing_proto_msgTypes[1].OneofWrappers = []interface{}{
		(*SigningOptions_FulcioSigningMaterial)(nil),
	}
	file_sigstore_signing_proto_msgTypes[2].OneofWrappers = []interface{}{
		(*BundleContentOptions_MessageSignature_)(nil),
		(*BundleContentOptions_DsseEnvelope)(nil),
	}
	file_sigstore_signing_proto_msgTypes[4].OneofWrappers = []interface{}{}
	file_sigstore_signing_proto_msgTypes[5].OneofWrappers = []interface{}{
		(*SigningResult_Bundle)(nil),
		(*SigningResult_Error_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sigstore_signing_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   9,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sigstore_signing_proto_goTypes,
		DependencyIndexes: file_sigstore_signing_proto_depIdxs,
		MessageInfos:      file_sigstore_signing_proto_msgTypes,
	}.Build()
	File_sigstore_signing_proto = out.File
	file_sigstore_signing_proto_rawDesc = nil
	file_sigstore_signing_proto_goTypes = nil
	file_sigstore_signing_proto_depIdxs = nil
}
