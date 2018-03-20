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
pub struct Vertex {
    // message fields
    pub id: ::std::string::String,
    pub field_type: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Vertex {}

impl Vertex {
    pub fn new() -> Vertex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Vertex {
        static mut instance: ::protobuf::lazy::Lazy<Vertex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vertex,
        };
        unsafe {
            instance.get(Vertex::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
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
}

impl ::protobuf::Message for Vertex {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(2, &self.field_type)?;
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

impl ::protobuf::MessageStatic for Vertex {
    fn new() -> Vertex {
        Vertex::new()
    }

    fn descriptor_static(_: ::std::option::Option<Vertex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Vertex::get_id_for_reflect,
                    Vertex::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Vertex::get_field_type_for_reflect,
                    Vertex::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vertex>(
                    "Vertex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Vertex {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vertex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Vertices {
    // message fields
    pub vertices: ::protobuf::RepeatedField<Vertex>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Vertices {}

impl Vertices {
    pub fn new() -> Vertices {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Vertices {
        static mut instance: ::protobuf::lazy::Lazy<Vertices> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vertices,
        };
        unsafe {
            instance.get(Vertices::new)
        }
    }

    // repeated .Vertex vertices = 1;

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: ::protobuf::RepeatedField<Vertex>) {
        self.vertices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut ::protobuf::RepeatedField<Vertex> {
        &mut self.vertices
    }

    // Take field
    pub fn take_vertices(&mut self) -> ::protobuf::RepeatedField<Vertex> {
        ::std::mem::replace(&mut self.vertices, ::protobuf::RepeatedField::new())
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn get_vertices_for_reflect(&self) -> &::protobuf::RepeatedField<Vertex> {
        &self.vertices
    }

    fn mut_vertices_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Vertex> {
        &mut self.vertices
    }
}

impl ::protobuf::Message for Vertices {
    fn is_initialized(&self) -> bool {
        for v in &self.vertices {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.vertices)?;
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
        for value in &self.vertices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.vertices {
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

impl ::protobuf::MessageStatic for Vertices {
    fn new() -> Vertices {
        Vertices::new()
    }

    fn descriptor_static(_: ::std::option::Option<Vertices>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Vertex>>(
                    "vertices",
                    Vertices::get_vertices_for_reflect,
                    Vertices::mut_vertices_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vertices>(
                    "Vertices",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Vertices {
    fn clear(&mut self) {
        self.clear_vertices();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Vertices {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vertices {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

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

#[derive(PartialEq,Clone,Default)]
pub struct VertexMetadata {
    // message fields
    pub id: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VertexMetadata {}

impl VertexMetadata {
    pub fn new() -> VertexMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VertexMetadata {
        static mut instance: ::protobuf::lazy::Lazy<VertexMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VertexMetadata,
        };
        unsafe {
            instance.get(VertexMetadata::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for VertexMetadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
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

impl ::protobuf::MessageStatic for VertexMetadata {
    fn new() -> VertexMetadata {
        VertexMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<VertexMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    VertexMetadata::get_id_for_reflect,
                    VertexMetadata::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    VertexMetadata::get_value_for_reflect,
                    VertexMetadata::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VertexMetadata>(
                    "VertexMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VertexMetadata {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VertexMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VertexMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VertexMetadatas {
    // message fields
    pub metadata: ::protobuf::RepeatedField<VertexMetadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VertexMetadatas {}

impl VertexMetadatas {
    pub fn new() -> VertexMetadatas {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VertexMetadatas {
        static mut instance: ::protobuf::lazy::Lazy<VertexMetadatas> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VertexMetadatas,
        };
        unsafe {
            instance.get(VertexMetadatas::new)
        }
    }

    // repeated .VertexMetadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::protobuf::RepeatedField<VertexMetadata>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::protobuf::RepeatedField<VertexMetadata> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::protobuf::RepeatedField<VertexMetadata> {
        ::std::mem::replace(&mut self.metadata, ::protobuf::RepeatedField::new())
    }

    pub fn get_metadata(&self) -> &[VertexMetadata] {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::RepeatedField<VertexMetadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VertexMetadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for VertexMetadatas {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metadata)?;
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
        for value in &self.metadata {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.metadata {
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

impl ::protobuf::MessageStatic for VertexMetadatas {
    fn new() -> VertexMetadatas {
        VertexMetadatas::new()
    }

    fn descriptor_static(_: ::std::option::Option<VertexMetadatas>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexMetadata>>(
                    "metadata",
                    VertexMetadatas::get_metadata_for_reflect,
                    VertexMetadatas::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VertexMetadatas>(
                    "VertexMetadatas",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VertexMetadatas {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VertexMetadatas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VertexMetadatas {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EdgeMetadata {
    // message fields
    pub key: ::protobuf::SingularPtrField<EdgeKey>,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EdgeMetadata {}

impl EdgeMetadata {
    pub fn new() -> EdgeMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EdgeMetadata {
        static mut instance: ::protobuf::lazy::Lazy<EdgeMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EdgeMetadata,
        };
        unsafe {
            instance.get(EdgeMetadata::new)
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

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for EdgeMetadata {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
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
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
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

impl ::protobuf::MessageStatic for EdgeMetadata {
    fn new() -> EdgeMetadata {
        EdgeMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<EdgeMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeKey>>(
                    "key",
                    EdgeMetadata::get_key_for_reflect,
                    EdgeMetadata::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    EdgeMetadata::get_value_for_reflect,
                    EdgeMetadata::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EdgeMetadata>(
                    "EdgeMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EdgeMetadata {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EdgeMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EdgeMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EdgeMetadatas {
    // message fields
    pub metadata: ::protobuf::RepeatedField<EdgeMetadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EdgeMetadatas {}

impl EdgeMetadatas {
    pub fn new() -> EdgeMetadatas {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EdgeMetadatas {
        static mut instance: ::protobuf::lazy::Lazy<EdgeMetadatas> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EdgeMetadatas,
        };
        unsafe {
            instance.get(EdgeMetadatas::new)
        }
    }

    // repeated .EdgeMetadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::protobuf::RepeatedField<EdgeMetadata>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::protobuf::RepeatedField<EdgeMetadata> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::protobuf::RepeatedField<EdgeMetadata> {
        ::std::mem::replace(&mut self.metadata, ::protobuf::RepeatedField::new())
    }

    pub fn get_metadata(&self) -> &[EdgeMetadata] {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::RepeatedField<EdgeMetadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EdgeMetadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for EdgeMetadatas {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metadata)?;
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
        for value in &self.metadata {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.metadata {
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

impl ::protobuf::MessageStatic for EdgeMetadatas {
    fn new() -> EdgeMetadatas {
        EdgeMetadatas::new()
    }

    fn descriptor_static(_: ::std::option::Option<EdgeMetadatas>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeMetadata>>(
                    "metadata",
                    EdgeMetadatas::get_metadata_for_reflect,
                    EdgeMetadatas::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EdgeMetadatas>(
                    "EdgeMetadatas",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EdgeMetadatas {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EdgeMetadatas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EdgeMetadatas {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cmodels.proto\",\n\x06Vertex\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\
    \x02id\x12\x12\n\x04type\x18\x02\x20\x01(\tR\x04type\"/\n\x08Vertices\
    \x12#\n\x08vertices\x18\x01\x20\x03(\x0b2\x07.VertexR\x08vertices\"M\n\
    \x04Edge\x12\x1a\n\x03key\x18\x01\x20\x01(\x0b2\x08.EdgeKeyR\x03key\x12)\
    \n\x10created_datetime\x18\x02\x20\x01(\rR\x0fcreatedDatetime\"$\n\x05Ed\
    ges\x12\x1b\n\x05edges\x18\x01\x20\x03(\x0b2\x05.EdgeR\x05edges\"]\n\x07\
    EdgeKey\x12\x1f\n\x0boutbound_id\x18\x01\x20\x01(\tR\noutboundId\x12\x12\
    \n\x04type\x18\x02\x20\x01(\tR\x04type\x12\x1d\n\ninbound_id\x18\x03\x20\
    \x01(\tR\tinboundId\"6\n\x0eVertexMetadata\x12\x0e\n\x02id\x18\x01\x20\
    \x01(\tR\x02id\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value\">\n\x0fV\
    ertexMetadatas\x12+\n\x08metadata\x18\x01\x20\x03(\x0b2\x0f.VertexMetada\
    taR\x08metadata\"@\n\x0cEdgeMetadata\x12\x1a\n\x03key\x18\x01\x20\x01(\
    \x0b2\x08.EdgeKeyR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05valu\
    e\":\n\rEdgeMetadatas\x12)\n\x08metadata\x18\x01\x20\x03(\x0b2\r.EdgeMet\
    adataR\x08metadataJ\x80\n\n\x06\x12\x04\0\0*\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x02\x08\x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x12\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\x03\x04\x02\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\r\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x03\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x04\x14\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x04\x04\x03\x12\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x04\x0b\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x12\x13\n\
    \n\n\x02\x04\x01\x12\x04\x07\0\t\x01\n\n\n\x03\x04\x01\x01\x12\x03\x07\
    \x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x08\x04!\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03\x08\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x08\
    \r\x13\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x08\x14\x1c\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x08\x1f\x20\n\n\n\x02\x04\x02\x12\x04\x0b\0\x0e\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x0b\x08\x0c\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x0c\x04\x14\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x0c\x04\x0b\x0e\
    \n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x0c\x04\x0b\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03\x0c\x0c\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0c\
    \x12\x13\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\r\x04\x20\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x04\r\x04\x0c\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03\r\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\r\x0b\x1b\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03\r\x1e\x1f\n\n\n\x02\x04\x03\x12\x04\x10\
    \0\x12\x01\n\n\n\x03\x04\x03\x01\x12\x03\x10\x08\r\n\x0b\n\x04\x04\x03\
    \x02\0\x12\x03\x11\x04\x1c\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x11\x04\
    \x0c\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x11\r\x11\n\x0c\n\x05\x04\x03\
    \x02\0\x01\x12\x03\x11\x12\x17\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x11\
    \x1a\x1b\n\n\n\x02\x04\x04\x12\x04\x14\0\x18\x01\n\n\n\x03\x04\x04\x01\
    \x12\x03\x14\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x15\x04\x1b\n\r\n\
    \x05\x04\x04\x02\0\x04\x12\x04\x15\x04\x14\x11\n\x0c\n\x05\x04\x04\x02\0\
    \x05\x12\x03\x15\x04\n\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x15\x0b\x16\
    \n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x15\x19\x1a\n\x0b\n\x04\x04\x04\
    \x02\x01\x12\x03\x16\x04\x14\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04\x16\
    \x04\x15\x1b\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x16\x04\n\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x03\x16\x0b\x0f\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x03\x16\x12\x13\n\x0b\n\x04\x04\x04\x02\x02\x12\x03\x17\x04\x1a\
    \n\r\n\x05\x04\x04\x02\x02\x04\x12\x04\x17\x04\x16\x14\n\x0c\n\x05\x04\
    \x04\x02\x02\x05\x12\x03\x17\x04\n\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\
    \x03\x17\x0b\x15\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03\x17\x18\x19\n\n\
    \n\x02\x04\x05\x12\x04\x1a\0\x1d\x01\n\n\n\x03\x04\x05\x01\x12\x03\x1a\
    \x08\x16\n\x0b\n\x04\x04\x05\x02\0\x12\x03\x1b\x04\x12\n\r\n\x05\x04\x05\
    \x02\0\x04\x12\x04\x1b\x04\x1a\x18\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03\
    \x1b\x04\n\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03\x1b\x0b\r\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03\x1b\x10\x11\n\x0b\n\x04\x04\x05\x02\x01\x12\
    \x03\x1c\x04\x15\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04\x1c\x04\x1b\x12\n\
    \x0c\n\x05\x04\x05\x02\x01\x05\x12\x03\x1c\x04\n\n\x0c\n\x05\x04\x05\x02\
    \x01\x01\x12\x03\x1c\x0b\x10\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03\x1c\
    \x13\x14\n\n\n\x02\x04\x06\x12\x04\x1f\0!\x01\n\n\n\x03\x04\x06\x01\x12\
    \x03\x1f\x08\x17\n\x0b\n\x04\x04\x06\x02\0\x12\x03\x20\x04)\n\x0c\n\x05\
    \x04\x06\x02\0\x04\x12\x03\x20\x04\x0c\n\x0c\n\x05\x04\x06\x02\0\x06\x12\
    \x03\x20\r\x1b\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03\x20\x1c$\n\x0c\n\
    \x05\x04\x06\x02\0\x03\x12\x03\x20'(\n\n\n\x02\x04\x07\x12\x04#\0&\x01\n\
    \n\n\x03\x04\x07\x01\x12\x03#\x08\x14\n\x0b\n\x04\x04\x07\x02\0\x12\x03$\
    \x04\x14\n\r\n\x05\x04\x07\x02\0\x04\x12\x04$\x04#\x16\n\x0c\n\x05\x04\
    \x07\x02\0\x06\x12\x03$\x04\x0b\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03$\
    \x0c\x0f\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03$\x12\x13\n\x0b\n\x04\x04\
    \x07\x02\x01\x12\x03%\x04\x15\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04%\x04\
    $\x14\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03%\x04\n\n\x0c\n\x05\x04\x07\
    \x02\x01\x01\x12\x03%\x0b\x10\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03%\
    \x13\x14\n\n\n\x02\x04\x08\x12\x04(\0*\x01\n\n\n\x03\x04\x08\x01\x12\x03\
    (\x08\x15\n\x0b\n\x04\x04\x08\x02\0\x12\x03)\x04'\n\x0c\n\x05\x04\x08\
    \x02\0\x04\x12\x03)\x04\x0c\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03)\r\x19\
    \n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03)\x1a\"\n\x0c\n\x05\x04\x08\x02\0\
    \x03\x12\x03)%&b\x06proto3\
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
