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
pub struct VertexQuery {
    // message oneof groups
    query: ::std::option::Option<VertexQuery_oneof_query>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VertexQuery {}

#[derive(Clone,PartialEq)]
pub enum VertexQuery_oneof_query {
    all(AllVertexQuery),
    vertices(VerticesVertexQuery),
    pipe(PipeVertexQuery),
}

impl VertexQuery {
    pub fn new() -> VertexQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VertexQuery {
        static mut instance: ::protobuf::lazy::Lazy<VertexQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VertexQuery,
        };
        unsafe {
            instance.get(VertexQuery::new)
        }
    }

    // .AllVertexQuery all = 1;

    pub fn clear_all(&mut self) {
        self.query = ::std::option::Option::None;
    }

    pub fn has_all(&self) -> bool {
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::all(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_all(&mut self, v: AllVertexQuery) {
        self.query = ::std::option::Option::Some(VertexQuery_oneof_query::all(v))
    }

    // Mutable pointer to the field.
    pub fn mut_all(&mut self) -> &mut AllVertexQuery {
        if let ::std::option::Option::Some(VertexQuery_oneof_query::all(_)) = self.query {
        } else {
            self.query = ::std::option::Option::Some(VertexQuery_oneof_query::all(AllVertexQuery::new()));
        }
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::all(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_all(&mut self) -> AllVertexQuery {
        if self.has_all() {
            match self.query.take() {
                ::std::option::Option::Some(VertexQuery_oneof_query::all(v)) => v,
                _ => panic!(),
            }
        } else {
            AllVertexQuery::new()
        }
    }

    pub fn get_all(&self) -> &AllVertexQuery {
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::all(ref v)) => v,
            _ => AllVertexQuery::default_instance(),
        }
    }

    // .VerticesVertexQuery vertices = 2;

    pub fn clear_vertices(&mut self) {
        self.query = ::std::option::Option::None;
    }

    pub fn has_vertices(&self) -> bool {
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: VerticesVertexQuery) {
        self.query = ::std::option::Option::Some(VertexQuery_oneof_query::vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut VerticesVertexQuery {
        if let ::std::option::Option::Some(VertexQuery_oneof_query::vertices(_)) = self.query {
        } else {
            self.query = ::std::option::Option::Some(VertexQuery_oneof_query::vertices(VerticesVertexQuery::new()));
        }
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vertices(&mut self) -> VerticesVertexQuery {
        if self.has_vertices() {
            match self.query.take() {
                ::std::option::Option::Some(VertexQuery_oneof_query::vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            VerticesVertexQuery::new()
        }
    }

    pub fn get_vertices(&self) -> &VerticesVertexQuery {
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::vertices(ref v)) => v,
            _ => VerticesVertexQuery::default_instance(),
        }
    }

    // .PipeVertexQuery pipe = 3;

    pub fn clear_pipe(&mut self) {
        self.query = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: PipeVertexQuery) {
        self.query = ::std::option::Option::Some(VertexQuery_oneof_query::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut PipeVertexQuery {
        if let ::std::option::Option::Some(VertexQuery_oneof_query::pipe(_)) = self.query {
        } else {
            self.query = ::std::option::Option::Some(VertexQuery_oneof_query::pipe(PipeVertexQuery::new()));
        }
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> PipeVertexQuery {
        if self.has_pipe() {
            match self.query.take() {
                ::std::option::Option::Some(VertexQuery_oneof_query::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            PipeVertexQuery::new()
        }
    }

    pub fn get_pipe(&self) -> &PipeVertexQuery {
        match self.query {
            ::std::option::Option::Some(VertexQuery_oneof_query::pipe(ref v)) => v,
            _ => PipeVertexQuery::default_instance(),
        }
    }
}

impl ::protobuf::Message for VertexQuery {
    fn is_initialized(&self) -> bool {
        if let Some(VertexQuery_oneof_query::all(ref v)) = self.query {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(VertexQuery_oneof_query::vertices(ref v)) = self.query {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(VertexQuery_oneof_query::pipe(ref v)) = self.query {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query = ::std::option::Option::Some(VertexQuery_oneof_query::all(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query = ::std::option::Option::Some(VertexQuery_oneof_query::vertices(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query = ::std::option::Option::Some(VertexQuery_oneof_query::pipe(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.query {
            match v {
                &VertexQuery_oneof_query::all(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VertexQuery_oneof_query::vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VertexQuery_oneof_query::pipe(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.query {
            match v {
                &VertexQuery_oneof_query::all(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &VertexQuery_oneof_query::vertices(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &VertexQuery_oneof_query::pipe(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for VertexQuery {
    fn new() -> VertexQuery {
        VertexQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<VertexQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, AllVertexQuery>(
                    "all",
                    VertexQuery::has_all,
                    VertexQuery::get_all,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, VerticesVertexQuery>(
                    "vertices",
                    VertexQuery::has_vertices,
                    VertexQuery::get_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PipeVertexQuery>(
                    "pipe",
                    VertexQuery::has_pipe,
                    VertexQuery::get_pipe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VertexQuery>(
                    "VertexQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VertexQuery {
    fn clear(&mut self) {
        self.clear_all();
        self.clear_vertices();
        self.clear_pipe();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VertexQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VertexQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AllVertexQuery {
    // message fields
    pub start_id: ::std::string::String,
    pub limit: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AllVertexQuery {}

impl AllVertexQuery {
    pub fn new() -> AllVertexQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AllVertexQuery {
        static mut instance: ::protobuf::lazy::Lazy<AllVertexQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AllVertexQuery,
        };
        unsafe {
            instance.get(AllVertexQuery::new)
        }
    }

    // string start_id = 1;

    pub fn clear_start_id(&mut self) {
        self.start_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_start_id(&mut self, v: ::std::string::String) {
        self.start_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_id(&mut self) -> &mut ::std::string::String {
        &mut self.start_id
    }

    // Take field
    pub fn take_start_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.start_id, ::std::string::String::new())
    }

    pub fn get_start_id(&self) -> &str {
        &self.start_id
    }

    fn get_start_id_for_reflect(&self) -> &::std::string::String {
        &self.start_id
    }

    fn mut_start_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.start_id
    }

    // uint32 limit = 2;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &u32 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut u32 {
        &mut self.limit
    }
}

impl ::protobuf::Message for AllVertexQuery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.start_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.limit = tmp;
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
        if !self.start_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.start_id);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(2, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.start_id.is_empty() {
            os.write_string(1, &self.start_id)?;
        }
        if self.limit != 0 {
            os.write_uint32(2, self.limit)?;
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

impl ::protobuf::MessageStatic for AllVertexQuery {
    fn new() -> AllVertexQuery {
        AllVertexQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<AllVertexQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "start_id",
                    AllVertexQuery::get_start_id_for_reflect,
                    AllVertexQuery::mut_start_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "limit",
                    AllVertexQuery::get_limit_for_reflect,
                    AllVertexQuery::mut_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AllVertexQuery>(
                    "AllVertexQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AllVertexQuery {
    fn clear(&mut self) {
        self.clear_start_id();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AllVertexQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AllVertexQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerticesVertexQuery {
    // message fields
    pub ids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerticesVertexQuery {}

impl VerticesVertexQuery {
    pub fn new() -> VerticesVertexQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerticesVertexQuery {
        static mut instance: ::protobuf::lazy::Lazy<VerticesVertexQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerticesVertexQuery,
        };
        unsafe {
            instance.get(VerticesVertexQuery::new)
        }
    }

    // repeated string ids = 1;

    pub fn clear_ids(&mut self) {
        self.ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ids
    }

    // Take field
    pub fn take_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_ids(&self) -> &[::std::string::String] {
        &self.ids
    }

    fn get_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.ids
    }

    fn mut_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ids
    }
}

impl ::protobuf::Message for VerticesVertexQuery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.ids)?;
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
        for value in &self.ids {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ids {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for VerticesVertexQuery {
    fn new() -> VerticesVertexQuery {
        VerticesVertexQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerticesVertexQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ids",
                    VerticesVertexQuery::get_ids_for_reflect,
                    VerticesVertexQuery::mut_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerticesVertexQuery>(
                    "VerticesVertexQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerticesVertexQuery {
    fn clear(&mut self) {
        self.clear_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerticesVertexQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerticesVertexQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PipeVertexQuery {
    // message fields
    pub query: ::protobuf::SingularPtrField<EdgeQuery>,
    pub converter: ::std::string::String,
    pub limit: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PipeVertexQuery {}

impl PipeVertexQuery {
    pub fn new() -> PipeVertexQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PipeVertexQuery {
        static mut instance: ::protobuf::lazy::Lazy<PipeVertexQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PipeVertexQuery,
        };
        unsafe {
            instance.get(PipeVertexQuery::new)
        }
    }

    // .EdgeQuery query = 1;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: EdgeQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut EdgeQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> EdgeQuery {
        self.query.take().unwrap_or_else(|| EdgeQuery::new())
    }

    pub fn get_query(&self) -> &EdgeQuery {
        self.query.as_ref().unwrap_or_else(|| EdgeQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<EdgeQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EdgeQuery> {
        &mut self.query
    }

    // string converter = 2;

    pub fn clear_converter(&mut self) {
        self.converter.clear();
    }

    // Param is passed by value, moved
    pub fn set_converter(&mut self, v: ::std::string::String) {
        self.converter = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_converter(&mut self) -> &mut ::std::string::String {
        &mut self.converter
    }

    // Take field
    pub fn take_converter(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.converter, ::std::string::String::new())
    }

    pub fn get_converter(&self) -> &str {
        &self.converter
    }

    fn get_converter_for_reflect(&self) -> &::std::string::String {
        &self.converter
    }

    fn mut_converter_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.converter
    }

    // uint32 limit = 3;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &u32 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut u32 {
        &mut self.limit
    }
}

impl ::protobuf::Message for PipeVertexQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.query {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.converter)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.limit = tmp;
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
        if let Some(ref v) = self.query.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.converter.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.converter);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.query.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.converter.is_empty() {
            os.write_string(2, &self.converter)?;
        }
        if self.limit != 0 {
            os.write_uint32(3, self.limit)?;
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

impl ::protobuf::MessageStatic for PipeVertexQuery {
    fn new() -> PipeVertexQuery {
        PipeVertexQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<PipeVertexQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeQuery>>(
                    "query",
                    PipeVertexQuery::get_query_for_reflect,
                    PipeVertexQuery::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "converter",
                    PipeVertexQuery::get_converter_for_reflect,
                    PipeVertexQuery::mut_converter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "limit",
                    PipeVertexQuery::get_limit_for_reflect,
                    PipeVertexQuery::mut_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PipeVertexQuery>(
                    "PipeVertexQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PipeVertexQuery {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_converter();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PipeVertexQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PipeVertexQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EdgeQuery {
    // message oneof groups
    query: ::std::option::Option<EdgeQuery_oneof_query>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EdgeQuery {}

#[derive(Clone,PartialEq)]
pub enum EdgeQuery_oneof_query {
    edges(EdgesEdgeQuery),
    pipe(PipeEdgeQuery),
}

impl EdgeQuery {
    pub fn new() -> EdgeQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EdgeQuery {
        static mut instance: ::protobuf::lazy::Lazy<EdgeQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EdgeQuery,
        };
        unsafe {
            instance.get(EdgeQuery::new)
        }
    }

    // .EdgesEdgeQuery edges = 1;

    pub fn clear_edges(&mut self) {
        self.query = ::std::option::Option::None;
    }

    pub fn has_edges(&self) -> bool {
        match self.query {
            ::std::option::Option::Some(EdgeQuery_oneof_query::edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_edges(&mut self, v: EdgesEdgeQuery) {
        self.query = ::std::option::Option::Some(EdgeQuery_oneof_query::edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_edges(&mut self) -> &mut EdgesEdgeQuery {
        if let ::std::option::Option::Some(EdgeQuery_oneof_query::edges(_)) = self.query {
        } else {
            self.query = ::std::option::Option::Some(EdgeQuery_oneof_query::edges(EdgesEdgeQuery::new()));
        }
        match self.query {
            ::std::option::Option::Some(EdgeQuery_oneof_query::edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_edges(&mut self) -> EdgesEdgeQuery {
        if self.has_edges() {
            match self.query.take() {
                ::std::option::Option::Some(EdgeQuery_oneof_query::edges(v)) => v,
                _ => panic!(),
            }
        } else {
            EdgesEdgeQuery::new()
        }
    }

    pub fn get_edges(&self) -> &EdgesEdgeQuery {
        match self.query {
            ::std::option::Option::Some(EdgeQuery_oneof_query::edges(ref v)) => v,
            _ => EdgesEdgeQuery::default_instance(),
        }
    }

    // .PipeEdgeQuery pipe = 2;

    pub fn clear_pipe(&mut self) {
        self.query = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.query {
            ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: PipeEdgeQuery) {
        self.query = ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut PipeEdgeQuery {
        if let ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(_)) = self.query {
        } else {
            self.query = ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(PipeEdgeQuery::new()));
        }
        match self.query {
            ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> PipeEdgeQuery {
        if self.has_pipe() {
            match self.query.take() {
                ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            PipeEdgeQuery::new()
        }
    }

    pub fn get_pipe(&self) -> &PipeEdgeQuery {
        match self.query {
            ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(ref v)) => v,
            _ => PipeEdgeQuery::default_instance(),
        }
    }
}

impl ::protobuf::Message for EdgeQuery {
    fn is_initialized(&self) -> bool {
        if let Some(EdgeQuery_oneof_query::edges(ref v)) = self.query {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(EdgeQuery_oneof_query::pipe(ref v)) = self.query {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query = ::std::option::Option::Some(EdgeQuery_oneof_query::edges(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query = ::std::option::Option::Some(EdgeQuery_oneof_query::pipe(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.query {
            match v {
                &EdgeQuery_oneof_query::edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &EdgeQuery_oneof_query::pipe(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.query {
            match v {
                &EdgeQuery_oneof_query::edges(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &EdgeQuery_oneof_query::pipe(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for EdgeQuery {
    fn new() -> EdgeQuery {
        EdgeQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<EdgeQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, EdgesEdgeQuery>(
                    "edges",
                    EdgeQuery::has_edges,
                    EdgeQuery::get_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PipeEdgeQuery>(
                    "pipe",
                    EdgeQuery::has_pipe,
                    EdgeQuery::get_pipe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EdgeQuery>(
                    "EdgeQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EdgeQuery {
    fn clear(&mut self) {
        self.clear_edges();
        self.clear_pipe();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EdgeQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EdgeQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EdgesEdgeQuery {
    // message fields
    pub keys: ::protobuf::RepeatedField<super::edges::EdgeKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EdgesEdgeQuery {}

impl EdgesEdgeQuery {
    pub fn new() -> EdgesEdgeQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EdgesEdgeQuery {
        static mut instance: ::protobuf::lazy::Lazy<EdgesEdgeQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EdgesEdgeQuery,
        };
        unsafe {
            instance.get(EdgesEdgeQuery::new)
        }
    }

    // repeated .EdgeKey keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<super::edges::EdgeKey>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<super::edges::EdgeKey> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<super::edges::EdgeKey> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[super::edges::EdgeKey] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<super::edges::EdgeKey> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::edges::EdgeKey> {
        &mut self.keys
    }
}

impl ::protobuf::Message for EdgesEdgeQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.keys {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
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
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
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

impl ::protobuf::MessageStatic for EdgesEdgeQuery {
    fn new() -> EdgesEdgeQuery {
        EdgesEdgeQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<EdgesEdgeQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::edges::EdgeKey>>(
                    "keys",
                    EdgesEdgeQuery::get_keys_for_reflect,
                    EdgesEdgeQuery::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EdgesEdgeQuery>(
                    "EdgesEdgeQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EdgesEdgeQuery {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EdgesEdgeQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EdgesEdgeQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PipeEdgeQuery {
    // message fields
    pub query: ::protobuf::SingularPtrField<VertexQuery>,
    pub converter: ::std::string::String,
    pub type_filter: ::std::string::String,
    pub high_filter: u64,
    pub low_filter: u64,
    pub limit: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PipeEdgeQuery {}

impl PipeEdgeQuery {
    pub fn new() -> PipeEdgeQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PipeEdgeQuery {
        static mut instance: ::protobuf::lazy::Lazy<PipeEdgeQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PipeEdgeQuery,
        };
        unsafe {
            instance.get(PipeEdgeQuery::new)
        }
    }

    // .VertexQuery query = 1;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: VertexQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut VertexQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> VertexQuery {
        self.query.take().unwrap_or_else(|| VertexQuery::new())
    }

    pub fn get_query(&self) -> &VertexQuery {
        self.query.as_ref().unwrap_or_else(|| VertexQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<VertexQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VertexQuery> {
        &mut self.query
    }

    // string converter = 2;

    pub fn clear_converter(&mut self) {
        self.converter.clear();
    }

    // Param is passed by value, moved
    pub fn set_converter(&mut self, v: ::std::string::String) {
        self.converter = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_converter(&mut self) -> &mut ::std::string::String {
        &mut self.converter
    }

    // Take field
    pub fn take_converter(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.converter, ::std::string::String::new())
    }

    pub fn get_converter(&self) -> &str {
        &self.converter
    }

    fn get_converter_for_reflect(&self) -> &::std::string::String {
        &self.converter
    }

    fn mut_converter_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.converter
    }

    // string type_filter = 3;

    pub fn clear_type_filter(&mut self) {
        self.type_filter.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_filter(&mut self, v: ::std::string::String) {
        self.type_filter = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_filter(&mut self) -> &mut ::std::string::String {
        &mut self.type_filter
    }

    // Take field
    pub fn take_type_filter(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_filter, ::std::string::String::new())
    }

    pub fn get_type_filter(&self) -> &str {
        &self.type_filter
    }

    fn get_type_filter_for_reflect(&self) -> &::std::string::String {
        &self.type_filter
    }

    fn mut_type_filter_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_filter
    }

    // uint64 high_filter = 4;

    pub fn clear_high_filter(&mut self) {
        self.high_filter = 0;
    }

    // Param is passed by value, moved
    pub fn set_high_filter(&mut self, v: u64) {
        self.high_filter = v;
    }

    pub fn get_high_filter(&self) -> u64 {
        self.high_filter
    }

    fn get_high_filter_for_reflect(&self) -> &u64 {
        &self.high_filter
    }

    fn mut_high_filter_for_reflect(&mut self) -> &mut u64 {
        &mut self.high_filter
    }

    // uint64 low_filter = 5;

    pub fn clear_low_filter(&mut self) {
        self.low_filter = 0;
    }

    // Param is passed by value, moved
    pub fn set_low_filter(&mut self, v: u64) {
        self.low_filter = v;
    }

    pub fn get_low_filter(&self) -> u64 {
        self.low_filter
    }

    fn get_low_filter_for_reflect(&self) -> &u64 {
        &self.low_filter
    }

    fn mut_low_filter_for_reflect(&mut self) -> &mut u64 {
        &mut self.low_filter
    }

    // uint32 limit = 6;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &u32 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut u32 {
        &mut self.limit
    }
}

impl ::protobuf::Message for PipeEdgeQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.query {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.converter)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_filter)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.high_filter = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.low_filter = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.limit = tmp;
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
        if let Some(ref v) = self.query.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.converter.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.converter);
        }
        if !self.type_filter.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.type_filter);
        }
        if self.high_filter != 0 {
            my_size += ::protobuf::rt::value_size(4, self.high_filter, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.low_filter != 0 {
            my_size += ::protobuf::rt::value_size(5, self.low_filter, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(6, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.query.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.converter.is_empty() {
            os.write_string(2, &self.converter)?;
        }
        if !self.type_filter.is_empty() {
            os.write_string(3, &self.type_filter)?;
        }
        if self.high_filter != 0 {
            os.write_uint64(4, self.high_filter)?;
        }
        if self.low_filter != 0 {
            os.write_uint64(5, self.low_filter)?;
        }
        if self.limit != 0 {
            os.write_uint32(6, self.limit)?;
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

impl ::protobuf::MessageStatic for PipeEdgeQuery {
    fn new() -> PipeEdgeQuery {
        PipeEdgeQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<PipeEdgeQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexQuery>>(
                    "query",
                    PipeEdgeQuery::get_query_for_reflect,
                    PipeEdgeQuery::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "converter",
                    PipeEdgeQuery::get_converter_for_reflect,
                    PipeEdgeQuery::mut_converter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_filter",
                    PipeEdgeQuery::get_type_filter_for_reflect,
                    PipeEdgeQuery::mut_type_filter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "high_filter",
                    PipeEdgeQuery::get_high_filter_for_reflect,
                    PipeEdgeQuery::mut_high_filter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "low_filter",
                    PipeEdgeQuery::get_low_filter_for_reflect,
                    PipeEdgeQuery::mut_low_filter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "limit",
                    PipeEdgeQuery::get_limit_for_reflect,
                    PipeEdgeQuery::mut_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PipeEdgeQuery>(
                    "PipeEdgeQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PipeEdgeQuery {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_converter();
        self.clear_type_filter();
        self.clear_high_filter();
        self.clear_low_filter();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PipeEdgeQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PipeEdgeQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rqueries.proto\x1a\x0bedges.proto\"\x97\x01\n\x0bVertexQuery\x12#\n\
    \x03all\x18\x01\x20\x01(\x0b2\x0f.AllVertexQueryH\0R\x03all\x122\n\x08ve\
    rtices\x18\x02\x20\x01(\x0b2\x14.VerticesVertexQueryH\0R\x08vertices\x12\
    &\n\x04pipe\x18\x03\x20\x01(\x0b2\x10.PipeVertexQueryH\0R\x04pipeB\x07\n\
    \x05query\"A\n\x0eAllVertexQuery\x12\x19\n\x08start_id\x18\x01\x20\x01(\
    \tR\x07startId\x12\x14\n\x05limit\x18\x02\x20\x01(\rR\x05limit\"'\n\x13V\
    erticesVertexQuery\x12\x10\n\x03ids\x18\x01\x20\x03(\tR\x03ids\"g\n\x0fP\
    ipeVertexQuery\x12\x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05\
    query\x12\x1c\n\tconverter\x18\x02\x20\x01(\tR\tconverter\x12\x14\n\x05l\
    imit\x18\x03\x20\x01(\rR\x05limit\"c\n\tEdgeQuery\x12'\n\x05edges\x18\
    \x01\x20\x01(\x0b2\x0f.EdgesEdgeQueryH\0R\x05edges\x12$\n\x04pipe\x18\
    \x02\x20\x01(\x0b2\x0e.PipeEdgeQueryH\0R\x04pipeB\x07\n\x05query\".\n\
    \x0eEdgesEdgeQuery\x12\x1c\n\x04keys\x18\x01\x20\x03(\x0b2\x08.EdgeKeyR\
    \x04keys\"\xc8\x01\n\rPipeEdgeQuery\x12\"\n\x05query\x18\x01\x20\x01(\
    \x0b2\x0c.VertexQueryR\x05query\x12\x1c\n\tconverter\x18\x02\x20\x01(\tR\
    \tconverter\x12\x1f\n\x0btype_filter\x18\x03\x20\x01(\tR\ntypeFilter\x12\
    \x1f\n\x0bhigh_filter\x18\x04\x20\x01(\x04R\nhighFilter\x12\x1d\n\nlow_f\
    ilter\x18\x05\x20\x01(\x04R\tlowFilter\x12\x14\n\x05limit\x18\x06\x20\
    \x01(\rR\x05limitJ\x9c\x0b\n\x06\x12\x04\0\0-\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x14\n\n\n\x02\x04\0\x12\x04\
    \x04\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x13\n\x0c\n\x04\x04\0\
    \x08\0\x12\x04\x05\x04\t\x05\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x05\n\
    \x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x08\x1f\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03\x06\x08\x16\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x17\x1a\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x1d\x1e\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x03\x07\x08)\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x07\x08\x1b\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x1c$\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x07'(\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x08\x08!\n\x0c\
    \n\x05\x04\0\x02\x02\x06\x12\x03\x08\x08\x17\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x03\x08\x18\x1c\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x08\x1f\
    \x20\n\n\n\x02\x04\x01\x12\x04\x0c\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\x0c\x08\x16\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x04\x18\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\r\x04\x0c\x18\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\r\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\x0b\x13\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\r\x16\x17\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x0e\x04\x15\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x0e\x04\r\x18\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03\x0e\x0b\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0e\
    \x13\x14\n\n\n\x02\x04\x02\x12\x04\x11\0\x13\x01\n\n\n\x03\x04\x02\x01\
    \x12\x03\x11\x08\x1b\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x04\x1c\n\x0c\
    \n\x05\x04\x02\x02\0\x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03\x12\r\x13\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x12\x14\x17\
    \n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x12\x1a\x1b\n\n\n\x02\x04\x03\x12\
    \x04\x15\0\x19\x01\n\n\n\x03\x04\x03\x01\x12\x03\x15\x08\x17\n\x0b\n\x04\
    \x04\x03\x02\0\x12\x03\x16\x04\x18\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\
    \x16\x04\x15\x19\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x16\x04\r\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x03\x16\x0e\x13\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03\x16\x16\x17\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x17\x04\x19\n\r\
    \n\x05\x04\x03\x02\x01\x04\x12\x04\x17\x04\x16\x18\n\x0c\n\x05\x04\x03\
    \x02\x01\x05\x12\x03\x17\x04\n\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\
    \x17\x0b\x14\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x17\x17\x18\n\x0b\n\
    \x04\x04\x03\x02\x02\x12\x03\x18\x04\x15\n\r\n\x05\x04\x03\x02\x02\x04\
    \x12\x04\x18\x04\x17\x19\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03\x18\x04\
    \n\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x18\x0b\x10\n\x0c\n\x05\x04\
    \x03\x02\x02\x03\x12\x03\x18\x13\x14\n\n\n\x02\x04\x04\x12\x04\x1b\0\x20\
    \x01\n\n\n\x03\x04\x04\x01\x12\x03\x1b\x08\x11\n\x0c\n\x04\x04\x04\x08\0\
    \x12\x04\x1c\x04\x1f\x05\n\x0c\n\x05\x04\x04\x08\0\x01\x12\x03\x1c\n\x0f\
    \n\x0b\n\x04\x04\x04\x02\0\x12\x03\x1d\x08!\n\x0c\n\x05\x04\x04\x02\0\
    \x06\x12\x03\x1d\x08\x16\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x1d\x17\
    \x1c\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1d\x1f\x20\n\x0b\n\x04\x04\
    \x04\x02\x01\x12\x03\x1e\x08\x1f\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03\
    \x1e\x08\x15\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1e\x16\x1a\n\x0c\n\
    \x05\x04\x04\x02\x01\x03\x12\x03\x1e\x1d\x1e\n\n\n\x02\x04\x05\x12\x04\"\
    \0$\x01\n\n\n\x03\x04\x05\x01\x12\x03\"\x08\x16\n\x0b\n\x04\x04\x05\x02\
    \0\x12\x03#\x04\x1e\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03#\x04\x0c\n\x0c\
    \n\x05\x04\x05\x02\0\x06\x12\x03#\r\x14\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x03#\x15\x19\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03#\x1c\x1d\n\n\n\
    \x02\x04\x06\x12\x04&\0-\x01\n\n\n\x03\x04\x06\x01\x12\x03&\x08\x15\n\
    \x0b\n\x04\x04\x06\x02\0\x12\x03'\x04\x1a\n\r\n\x05\x04\x06\x02\0\x04\
    \x12\x04'\x04&\x17\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03'\x04\x0f\n\x0c\
    \n\x05\x04\x06\x02\0\x01\x12\x03'\x10\x15\n\x0c\n\x05\x04\x06\x02\0\x03\
    \x12\x03'\x18\x19\n\x0b\n\x04\x04\x06\x02\x01\x12\x03(\x04\x19\n\r\n\x05\
    \x04\x06\x02\x01\x04\x12\x04(\x04'\x1a\n\x0c\n\x05\x04\x06\x02\x01\x05\
    \x12\x03(\x04\n\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03(\x0b\x14\n\x0c\n\
    \x05\x04\x06\x02\x01\x03\x12\x03(\x17\x18\n\x0b\n\x04\x04\x06\x02\x02\
    \x12\x03)\x04\x1b\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04)\x04(\x19\n\x0c\
    \n\x05\x04\x06\x02\x02\x05\x12\x03)\x04\n\n\x0c\n\x05\x04\x06\x02\x02\
    \x01\x12\x03)\x0b\x16\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03)\x19\x1a\n\
    \x0b\n\x04\x04\x06\x02\x03\x12\x03*\x04\x1b\n\r\n\x05\x04\x06\x02\x03\
    \x04\x12\x04*\x04)\x1b\n\x0c\n\x05\x04\x06\x02\x03\x05\x12\x03*\x04\n\n\
    \x0c\n\x05\x04\x06\x02\x03\x01\x12\x03*\x0b\x16\n\x0c\n\x05\x04\x06\x02\
    \x03\x03\x12\x03*\x19\x1a\n\x0b\n\x04\x04\x06\x02\x04\x12\x03+\x04\x1a\n\
    \r\n\x05\x04\x06\x02\x04\x04\x12\x04+\x04*\x1b\n\x0c\n\x05\x04\x06\x02\
    \x04\x05\x12\x03+\x04\n\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\x03+\x0b\x15\
    \n\x0c\n\x05\x04\x06\x02\x04\x03\x12\x03+\x18\x19\n\x0b\n\x04\x04\x06\
    \x02\x05\x12\x03,\x04\x15\n\r\n\x05\x04\x06\x02\x05\x04\x12\x04,\x04+\
    \x1a\n\x0c\n\x05\x04\x06\x02\x05\x05\x12\x03,\x04\n\n\x0c\n\x05\x04\x06\
    \x02\x05\x01\x12\x03,\x0b\x10\n\x0c\n\x05\x04\x06\x02\x05\x03\x12\x03,\
    \x13\x14b\x06proto3\
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
