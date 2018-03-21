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
pub struct Edge {
    // message fields
    pub key: ::protobuf::SingularPtrField<EdgeKey>,
    pub created_datetime: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Edge {}

impl Edge {
    pub fn new() -> Edge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Edge {
        static mut instance: ::protobuf::lazy::Lazy<Edge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Edge,
        };
        unsafe {
            instance.get(Edge::new)
        }
    }

    // .EdgeKey key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: EdgeKey) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut EdgeKey {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> EdgeKey {
        self.key.take().unwrap_or_else(|| EdgeKey::new())
    }

    pub fn get_key(&self) -> &EdgeKey {
        self.key.as_ref().unwrap_or_else(|| EdgeKey::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<EdgeKey> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EdgeKey> {
        &mut self.key
    }

    // uint32 created_datetime = 2;

    pub fn clear_created_datetime(&mut self) {
        self.created_datetime = 0;
    }

    // Param is passed by value, moved
    pub fn set_created_datetime(&mut self, v: u32) {
        self.created_datetime = v;
    }

    pub fn get_created_datetime(&self) -> u32 {
        self.created_datetime
    }

    fn get_created_datetime_for_reflect(&self) -> &u32 {
        &self.created_datetime
    }

    fn mut_created_datetime_for_reflect(&mut self) -> &mut u32 {
        &mut self.created_datetime
    }
}

impl ::protobuf::Message for Edge {
    fn is_initialized(&self) -> bool {
        for v in &self.key {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.created_datetime = tmp;
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
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.created_datetime != 0 {
            my_size += ::protobuf::rt::value_size(2, self.created_datetime, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.created_datetime != 0 {
            os.write_uint32(2, self.created_datetime)?;
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

impl ::protobuf::MessageStatic for Edge {
    fn new() -> Edge {
        Edge::new()
    }

    fn descriptor_static(_: ::std::option::Option<Edge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeKey>>(
                    "key",
                    Edge::get_key_for_reflect,
                    Edge::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "created_datetime",
                    Edge::get_created_datetime_for_reflect,
                    Edge::mut_created_datetime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Edge>(
                    "Edge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Edge {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_created_datetime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Edge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Edges {
    // message fields
    pub edges: ::protobuf::RepeatedField<Edge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Edges {}

impl Edges {
    pub fn new() -> Edges {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Edges {
        static mut instance: ::protobuf::lazy::Lazy<Edges> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Edges,
        };
        unsafe {
            instance.get(Edges::new)
        }
    }

    // repeated .Edge edges = 1;

    pub fn clear_edges(&mut self) {
        self.edges.clear();
    }

    // Param is passed by value, moved
    pub fn set_edges(&mut self, v: ::protobuf::RepeatedField<Edge>) {
        self.edges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_edges(&mut self) -> &mut ::protobuf::RepeatedField<Edge> {
        &mut self.edges
    }

    // Take field
    pub fn take_edges(&mut self) -> ::protobuf::RepeatedField<Edge> {
        ::std::mem::replace(&mut self.edges, ::protobuf::RepeatedField::new())
    }

    pub fn get_edges(&self) -> &[Edge] {
        &self.edges
    }

    fn get_edges_for_reflect(&self) -> &::protobuf::RepeatedField<Edge> {
        &self.edges
    }

    fn mut_edges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Edge> {
        &mut self.edges
    }
}

impl ::protobuf::Message for Edges {
    fn is_initialized(&self) -> bool {
        for v in &self.edges {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.edges)?;
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
        for value in &self.edges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.edges {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for Edges {
    fn new() -> Edges {
        Edges::new()
    }

    fn descriptor_static(_: ::std::option::Option<Edges>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Edge>>(
                    "edges",
                    Edges::get_edges_for_reflect,
                    Edges::mut_edges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Edges>(
                    "Edges",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Edges {
    fn clear(&mut self) {
        self.clear_edges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Edges {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Edges {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EdgeKey {
    // message fields
    pub outbound_id: ::std::string::String,
    pub field_type: ::std::string::String,
    pub inbound_id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EdgeKey {}

impl EdgeKey {
    pub fn new() -> EdgeKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EdgeKey {
        static mut instance: ::protobuf::lazy::Lazy<EdgeKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EdgeKey,
        };
        unsafe {
            instance.get(EdgeKey::new)
        }
    }

    // string outbound_id = 1;

    pub fn clear_outbound_id(&mut self) {
        self.outbound_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_outbound_id(&mut self, v: ::std::string::String) {
        self.outbound_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outbound_id(&mut self) -> &mut ::std::string::String {
        &mut self.outbound_id
    }

    // Take field
    pub fn take_outbound_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.outbound_id, ::std::string::String::new())
    }

    pub fn get_outbound_id(&self) -> &str {
        &self.outbound_id
    }

    fn get_outbound_id_for_reflect(&self) -> &::std::string::String {
        &self.outbound_id
    }

    fn mut_outbound_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.outbound_id
    }

    // string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // string inbound_id = 3;

    pub fn clear_inbound_id(&mut self) {
        self.inbound_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_inbound_id(&mut self, v: ::std::string::String) {
        self.inbound_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inbound_id(&mut self) -> &mut ::std::string::String {
        &mut self.inbound_id
    }

    // Take field
    pub fn take_inbound_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inbound_id, ::std::string::String::new())
    }

    pub fn get_inbound_id(&self) -> &str {
        &self.inbound_id
    }

    fn get_inbound_id_for_reflect(&self) -> &::std::string::String {
        &self.inbound_id
    }

    fn mut_inbound_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.inbound_id
    }
}

impl ::protobuf::Message for EdgeKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.outbound_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inbound_id)?;
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
        if !self.outbound_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.outbound_id);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
        }
        if !self.inbound_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.inbound_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.outbound_id.is_empty() {
            os.write_string(1, &self.outbound_id)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(2, &self.field_type)?;
        }
        if !self.inbound_id.is_empty() {
            os.write_string(3, &self.inbound_id)?;
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

impl ::protobuf::MessageStatic for EdgeKey {
    fn new() -> EdgeKey {
        EdgeKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<EdgeKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "outbound_id",
                    EdgeKey::get_outbound_id_for_reflect,
                    EdgeKey::mut_outbound_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    EdgeKey::get_field_type_for_reflect,
                    EdgeKey::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "inbound_id",
                    EdgeKey::get_inbound_id_for_reflect,
                    EdgeKey::mut_inbound_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EdgeKey>(
                    "EdgeKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EdgeKey {
    fn clear(&mut self) {
        self.clear_outbound_id();
        self.clear_field_type();
        self.clear_inbound_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EdgeKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EdgeKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bedges.proto\"M\n\x04Edge\x12\x1a\n\x03key\x18\x01\x20\x01(\x0b2\
    \x08.EdgeKeyR\x03key\x12)\n\x10created_datetime\x18\x02\x20\x01(\rR\x0fc\
    reatedDatetime\"$\n\x05Edges\x12\x1b\n\x05edges\x18\x01\x20\x03(\x0b2\
    \x05.EdgeR\x05edges\"]\n\x07EdgeKey\x12\x1f\n\x0boutbound_id\x18\x01\x20\
    \x01(\tR\noutboundId\x12\x12\n\x04type\x18\x02\x20\x01(\tR\x04type\x12\
    \x1d\n\ninbound_id\x18\x03\x20\x01(\tR\tinboundIdJ\xfd\x03\n\x06\x12\x04\
    \0\0\x0f\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\
    \0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x0c\n\x0b\n\x04\x04\0\x02\
    \0\x12\x03\x03\x04\x14\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x03\x04\x02\x0e\
    \n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x03\x04\x0b\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x03\x0c\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x12\x13\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x20\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x04\x04\x03\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\
    \x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x1b\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x04\x1e\x1f\n\n\n\x02\x04\x01\x12\x04\x07\0\t\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\r\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x08\x04\x1c\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x08\x04\x0c\n\x0c\
    \n\x05\x04\x01\x02\0\x06\x12\x03\x08\r\x11\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x08\x12\x17\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x08\x1a\x1b\n\
    \n\n\x02\x04\x02\x12\x04\x0b\0\x0f\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0b\
    \x08\x0f\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0c\x04\x1b\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04\x0c\x04\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\
    \x0c\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0c\x0b\x16\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03\x0c\x19\x1a\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x03\r\x04\x14\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\r\x04\x0c\x1b\n\x0c\
    \n\x05\x04\x02\x02\x01\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x01\x12\x03\r\x0b\x0f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\r\x12\x13\
    \n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x0e\x04\x1a\n\r\n\x05\x04\x02\x02\
    \x02\x04\x12\x04\x0e\x04\r\x14\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\
    \x0e\x04\n\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x0e\x0b\x15\n\x0c\n\
    \x05\x04\x02\x02\x02\x03\x12\x03\x0e\x18\x19b\x06proto3\
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
