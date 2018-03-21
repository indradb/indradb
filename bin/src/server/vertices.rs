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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0evertices.proto\",\n\x06Vertex\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\
    \x02id\x12\x12\n\x04type\x18\x02\x20\x01(\tR\x04type\"/\n\x08Vertices\
    \x12#\n\x08vertices\x18\x01\x20\x03(\x0b2\x07.VertexR\x08verticesJ\x93\
    \x02\n\x06\x12\x04\0\0\t\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\
    \x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x0e\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x12\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x03\x04\x02\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x03\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x14\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x04\x04\x03\x12\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x0f\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x12\x13\n\n\n\x02\x04\x01\x12\
    \x04\x07\0\t\x01\n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\x10\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x08\x04!\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\
    \x08\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x08\r\x13\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x08\x14\x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x08\x1f\x20b\x06proto3\
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
