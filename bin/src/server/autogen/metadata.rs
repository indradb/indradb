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
    pub values: ::protobuf::RepeatedField<VertexMetadata>,
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

    // repeated .VertexMetadata values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<VertexMetadata>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<VertexMetadata> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<VertexMetadata> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[VertexMetadata] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::protobuf::RepeatedField<VertexMetadata> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VertexMetadata> {
        &mut self.values
    }
}

impl ::protobuf::Message for VertexMetadatas {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
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
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
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
                    "values",
                    VertexMetadatas::get_values_for_reflect,
                    VertexMetadatas::mut_values_for_reflect,
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
        self.clear_values();
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
    pub key: ::protobuf::SingularPtrField<super::edges::EdgeKey>,
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
    pub fn set_key(&mut self, v: super::edges::EdgeKey) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut super::edges::EdgeKey {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> super::edges::EdgeKey {
        self.key.take().unwrap_or_else(|| super::edges::EdgeKey::new())
    }

    pub fn get_key(&self) -> &super::edges::EdgeKey {
        self.key.as_ref().unwrap_or_else(|| super::edges::EdgeKey::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<super::edges::EdgeKey> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::edges::EdgeKey> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::edges::EdgeKey>>(
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
    pub values: ::protobuf::RepeatedField<EdgeMetadata>,
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

    // repeated .EdgeMetadata values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<EdgeMetadata>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<EdgeMetadata> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<EdgeMetadata> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[EdgeMetadata] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::protobuf::RepeatedField<EdgeMetadata> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EdgeMetadata> {
        &mut self.values
    }
}

impl ::protobuf::Message for EdgeMetadatas {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
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
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
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
                    "values",
                    EdgeMetadatas::get_values_for_reflect,
                    EdgeMetadatas::mut_values_for_reflect,
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
        self.clear_values();
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
    \n\x14proto/metadata.proto\x1a\x11proto/edges.proto\"6\n\x0eVertexMetada\
    ta\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x14\n\x05value\x18\x02\
    \x20\x01(\tR\x05value\":\n\x0fVertexMetadatas\x12'\n\x06values\x18\x01\
    \x20\x03(\x0b2\x0f.VertexMetadataR\x06values\"@\n\x0cEdgeMetadata\x12\
    \x1a\n\x03key\x18\x01\x20\x01(\x0b2\x08.EdgeKeyR\x03key\x12\x14\n\x05val\
    ue\x18\x02\x20\x01(\tR\x05value\"6\n\rEdgeMetadatas\x12%\n\x06values\x18\
    \x01\x20\x03(\x0b2\r.EdgeMetadataR\x06valuesJ\x9f\x04\n\x06\x12\x04\0\0\
    \x14\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\
    \x1a\n\n\n\x02\x04\0\x12\x04\x04\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x04\x08\x16\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x12\n\r\n\x05\x04\0\
    \x02\0\x04\x12\x04\x05\x04\x04\x18\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x05\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\x0b\r\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\x05\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x06\
    \x04\x15\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x06\x04\x05\x12\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x06\x0b\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x13\x14\n\n\n\
    \x02\x04\x01\x12\x04\t\0\x0b\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x17\
    \n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x04'\n\x0c\n\x05\x04\x01\x02\0\x04\
    \x12\x03\n\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\n\r\x1b\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\n\x1c\"\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\n%&\n\n\n\x02\x04\x02\x12\x04\r\0\x10\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\r\x08\x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0e\x04\x14\n\r\n\x05\
    \x04\x02\x02\0\x04\x12\x04\x0e\x04\r\x16\n\x0c\n\x05\x04\x02\x02\0\x06\
    \x12\x03\x0e\x04\x0b\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0e\x0c\x0f\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0e\x12\x13\n\x0b\n\x04\x04\x02\x02\
    \x01\x12\x03\x0f\x04\x15\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x0f\x04\
    \x0e\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03\x0f\x0b\x10\n\x0c\n\x05\x04\x02\x02\x01\x03\
    \x12\x03\x0f\x13\x14\n\n\n\x02\x04\x03\x12\x04\x12\0\x14\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03\x12\x08\x15\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x13\
    \x04%\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x13\x04\x0c\n\x0c\n\x05\x04\
    \x03\x02\0\x06\x12\x03\x13\r\x19\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\
    \x13\x1a\x20\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x13#$b\x06proto3\
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
