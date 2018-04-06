// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct PingResponse {
    // message fields
    pub ok: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingResponse {}

impl PingResponse {
    pub fn new() -> PingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingResponse {
        static mut instance: ::protobuf::lazy::Lazy<PingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingResponse,
        };
        unsafe {
            instance.get(PingResponse::new)
        }
    }

    // bool ok = 1;

    pub fn clear_ok(&mut self) {
        self.ok = false;
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.ok = v;
    }

    pub fn get_ok(&self) -> bool {
        self.ok
    }

    fn get_ok_for_reflect(&self) -> &bool {
        &self.ok
    }

    fn mut_ok_for_reflect(&mut self) -> &mut bool {
        &mut self.ok
    }
}

impl ::protobuf::Message for PingResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ok = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.ok != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ok != false {
            os.write_bool(1, self.ok)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingResponse {
    fn new() -> PingResponse {
        PingResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ok",
                    PingResponse::get_ok_for_reflect,
                    PingResponse::mut_ok_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingResponse>(
                    "PingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingResponse {
    fn clear(&mut self) {
        self.clear_ok();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionResponse {
    // message oneof groups
    response: ::std::option::Option<TransactionResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionResponse {}

#[derive(Clone,PartialEq)]
pub enum TransactionResponse_oneof_response {
    ok(bool),
    vertices(super::vertices::Vertices),
    count(u64),
    edges(super::edges::Edges),
    json(::std::string::String),
    uuid(::std::string::String),
    vertex_metadatas(super::metadata::VertexMetadatas),
    edge_metadatas(super::metadata::EdgeMetadatas),
    error(::std::string::String),
}

impl TransactionResponse {
    pub fn new() -> TransactionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionResponse {
        static mut instance: ::protobuf::lazy::Lazy<TransactionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionResponse,
        };
        unsafe {
            instance.get(TransactionResponse::new)
        }
    }

    // bool ok = 1;

    pub fn clear_ok(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_ok(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::ok(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::ok(v))
    }

    pub fn get_ok(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::ok(v)) => v,
            _ => false,
        }
    }

    // .Vertices vertices = 2;

    pub fn clear_vertices(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_vertices(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: super::vertices::Vertices) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut super::vertices::Vertices {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(super::vertices::Vertices::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vertices(&mut self) -> super::vertices::Vertices {
        if self.has_vertices() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            super::vertices::Vertices::new()
        }
    }

    pub fn get_vertices(&self) -> &super::vertices::Vertices {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(ref v)) => v,
            _ => super::vertices::Vertices::default_instance(),
        }
    }

    // uint64 count = 3;

    pub fn clear_count(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::count(v))
    }

    pub fn get_count(&self) -> u64 {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::count(v)) => v,
            _ => 0,
        }
    }

    // .Edges edges = 4;

    pub fn clear_edges(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_edges(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_edges(&mut self, v: super::edges::Edges) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_edges(&mut self) -> &mut super::edges::Edges {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::edges(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::edges(super::edges::Edges::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_edges(&mut self) -> super::edges::Edges {
        if self.has_edges() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::edges(v)) => v,
                _ => panic!(),
            }
        } else {
            super::edges::Edges::new()
        }
    }

    pub fn get_edges(&self) -> &super::edges::Edges {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::edges(ref v)) => v,
            _ => super::edges::Edges::default_instance(),
        }
    }

    // string json = 5;

    pub fn clear_json(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_json(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::json(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::json(v))
    }

    // Mutable pointer to the field.
    pub fn mut_json(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::json(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::json(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::json(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::string::String {
        if self.has_json() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::json(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_json(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::json(ref v)) => v,
            _ => "",
        }
    }

    // string uuid = 6;

    pub fn clear_uuid(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_uuid(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(v))
    }

    // Mutable pointer to the field.
    pub fn mut_uuid(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::string::String {
        if self.has_uuid() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_uuid(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(ref v)) => v,
            _ => "",
        }
    }

    // .VertexMetadatas vertex_metadatas = 7;

    pub fn clear_vertex_metadatas(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_vertex_metadatas(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vertex_metadatas(&mut self, v: super::metadata::VertexMetadatas) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vertex_metadatas(&mut self) -> &mut super::metadata::VertexMetadatas {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(super::metadata::VertexMetadatas::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vertex_metadatas(&mut self) -> super::metadata::VertexMetadatas {
        if self.has_vertex_metadatas() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(v)) => v,
                _ => panic!(),
            }
        } else {
            super::metadata::VertexMetadatas::new()
        }
    }

    pub fn get_vertex_metadatas(&self) -> &super::metadata::VertexMetadatas {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(ref v)) => v,
            _ => super::metadata::VertexMetadatas::default_instance(),
        }
    }

    // .EdgeMetadatas edge_metadatas = 8;

    pub fn clear_edge_metadatas(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_edge_metadatas(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_edge_metadatas(&mut self, v: super::metadata::EdgeMetadatas) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(v))
    }

    // Mutable pointer to the field.
    pub fn mut_edge_metadatas(&mut self) -> &mut super::metadata::EdgeMetadatas {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(super::metadata::EdgeMetadatas::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_edge_metadatas(&mut self) -> super::metadata::EdgeMetadatas {
        if self.has_edge_metadatas() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(v)) => v,
                _ => panic!(),
            }
        } else {
            super::metadata::EdgeMetadatas::new()
        }
    }

    pub fn get_edge_metadatas(&self) -> &super::metadata::EdgeMetadatas {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(ref v)) => v,
            _ => super::metadata::EdgeMetadatas::default_instance(),
        }
    }

    // string error = 9;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for TransactionResponse {
    fn is_initialized(&self) -> bool {
        if let Some(TransactionResponse_oneof_response::vertices(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::edges(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::vertex_metadatas(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::edge_metadatas(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::ok(is.read_bool()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::vertices(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::count(is.read_uint64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::edges(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::json(is.read_string()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::uuid(is.read_string()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::vertex_metadatas(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::edge_metadatas(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::error(is.read_string()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &TransactionResponse_oneof_response::ok(v) => {
                    my_size += 2;
                },
                &TransactionResponse_oneof_response::vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::count(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TransactionResponse_oneof_response::edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::json(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
                &TransactionResponse_oneof_response::uuid(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
                &TransactionResponse_oneof_response::vertex_metadatas(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::edge_metadatas(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &TransactionResponse_oneof_response::ok(v) => {
                    os.write_bool(1, v)?;
                },
                &TransactionResponse_oneof_response::vertices(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::count(v) => {
                    os.write_uint64(3, v)?;
                },
                &TransactionResponse_oneof_response::edges(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::json(ref v) => {
                    os.write_string(5, v)?;
                },
                &TransactionResponse_oneof_response::uuid(ref v) => {
                    os.write_string(6, v)?;
                },
                &TransactionResponse_oneof_response::vertex_metadatas(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::edge_metadatas(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::error(ref v) => {
                    os.write_string(9, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionResponse {
    fn new() -> TransactionResponse {
        TransactionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "ok",
                    TransactionResponse::has_ok,
                    TransactionResponse::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::vertices::Vertices>(
                    "vertices",
                    TransactionResponse::has_vertices,
                    TransactionResponse::get_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "count",
                    TransactionResponse::has_count,
                    TransactionResponse::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::edges::Edges>(
                    "edges",
                    TransactionResponse::has_edges,
                    TransactionResponse::get_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "json",
                    TransactionResponse::has_json,
                    TransactionResponse::get_json,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "uuid",
                    TransactionResponse::has_uuid,
                    TransactionResponse::get_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::metadata::VertexMetadatas>(
                    "vertex_metadatas",
                    TransactionResponse::has_vertex_metadatas,
                    TransactionResponse::get_vertex_metadatas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::metadata::EdgeMetadatas>(
                    "edge_metadatas",
                    TransactionResponse::has_edge_metadatas,
                    TransactionResponse::get_edge_metadatas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    TransactionResponse::has_error,
                    TransactionResponse::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionResponse>(
                    "TransactionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionResponse {
    fn clear(&mut self) {
        self.clear_ok();
        self.clear_vertices();
        self.clear_count();
        self.clear_edges();
        self.clear_json();
        self.clear_uuid();
        self.clear_vertex_metadatas();
        self.clear_edge_metadatas();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14proto/response.proto\x1a\x14proto/vertices.proto\x1a\x11proto/edge\
    s.proto\x1a\x14proto/metadata.proto\"\x1e\n\x0cPingResponse\x12\x0e\n\
    \x02ok\x18\x01\x20\x01(\x08R\x02ok\"\xd0\x02\n\x13TransactionResponse\
    \x12\x10\n\x02ok\x18\x01\x20\x01(\x08H\0R\x02ok\x12'\n\x08vertices\x18\
    \x02\x20\x01(\x0b2\t.VerticesH\0R\x08vertices\x12\x16\n\x05count\x18\x03\
    \x20\x01(\x04H\0R\x05count\x12\x1e\n\x05edges\x18\x04\x20\x01(\x0b2\x06.\
    EdgesH\0R\x05edges\x12\x14\n\x04json\x18\x05\x20\x01(\tH\0R\x04json\x12\
    \x14\n\x04uuid\x18\x06\x20\x01(\tH\0R\x04uuid\x12=\n\x10vertex_metadatas\
    \x18\x07\x20\x01(\x0b2\x10.VertexMetadatasH\0R\x0fvertexMetadatas\x127\n\
    \x0eedge_metadatas\x18\x08\x20\x01(\x0b2\x0e.EdgeMetadatasH\0R\redgeMeta\
    datas\x12\x16\n\x05error\x18\t\x20\x01(\tH\0R\x05errorB\n\n\x08responseJ\
    \xb4\x05\n\x06\x12\x04\0\0\x16\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\
    \x02\x03\0\x12\x03\x02\x07\x1d\n\t\n\x02\x03\x01\x12\x03\x03\x07\x1a\n\t\
    \n\x02\x03\x02\x12\x03\x04\x07\x1d\n\n\n\x02\x04\0\x12\x04\x06\0\x08\x01\
    \n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x07\x04\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x07\x04\x06\x16\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x07\x04\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x07\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x0e\x0f\n\n\n\x02\
    \x04\x01\x12\x04\n\0\x16\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\x1b\n\
    \x0c\n\x04\x04\x01\x08\0\x12\x04\x0b\x04\x15\x05\n\x0c\n\x05\x04\x01\x08\
    \0\x01\x12\x03\x0b\n\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0c\x08\x14\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0c\x08\x0c\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x0c\r\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0c\x12\
    \x13\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\r\x08\x1e\n\x0c\n\x05\x04\x01\
    \x02\x01\x06\x12\x03\r\x08\x10\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\r\
    \x11\x19\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\r\x1c\x1d\n\x0b\n\x04\
    \x04\x01\x02\x02\x12\x03\x0e\x08\x19\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\
    \x03\x0e\x08\x0e\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0e\x0f\x14\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0e\x17\x18\n\x0b\n\x04\x04\x01\
    \x02\x03\x12\x03\x0f\x08\x18\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x0f\
    \x08\r\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0f\x0e\x13\n\x0c\n\x05\
    \x04\x01\x02\x03\x03\x12\x03\x0f\x16\x17\n\x0b\n\x04\x04\x01\x02\x04\x12\
    \x03\x10\x08\x18\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\x10\x08\x0e\n\
    \x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x10\x0f\x13\n\x0c\n\x05\x04\x01\
    \x02\x04\x03\x12\x03\x10\x16\x17\n\x0b\n\x04\x04\x01\x02\x05\x12\x03\x11\
    \x08\x18\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x03\x11\x08\x0e\n\x0c\n\x05\
    \x04\x01\x02\x05\x01\x12\x03\x11\x0f\x13\n\x0c\n\x05\x04\x01\x02\x05\x03\
    \x12\x03\x11\x16\x17\n\x0b\n\x04\x04\x01\x02\x06\x12\x03\x12\x08-\n\x0c\
    \n\x05\x04\x01\x02\x06\x06\x12\x03\x12\x08\x17\n\x0c\n\x05\x04\x01\x02\
    \x06\x01\x12\x03\x12\x18(\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\x12+,\
    \n\x0b\n\x04\x04\x01\x02\x07\x12\x03\x13\x08)\n\x0c\n\x05\x04\x01\x02\
    \x07\x06\x12\x03\x13\x08\x15\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\x13\
    \x16$\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03\x13'(\n\x0b\n\x04\x04\x01\
    \x02\x08\x12\x03\x14\x08\x19\n\x0c\n\x05\x04\x01\x02\x08\x05\x12\x03\x14\
    \x08\x0e\n\x0c\n\x05\x04\x01\x02\x08\x01\x12\x03\x14\x0f\x14\n\x0c\n\x05\
    \x04\x01\x02\x08\x03\x12\x03\x14\x17\x18b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
