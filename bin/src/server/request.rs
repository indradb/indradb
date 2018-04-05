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
pub struct PingRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingRequest {}

impl PingRequest {
    pub fn new() -> PingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingRequest {
        static mut instance: ::protobuf::lazy::Lazy<PingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingRequest,
        };
        unsafe {
            instance.get(PingRequest::new)
        }
    }
}

impl ::protobuf::Message for PingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for PingRequest {
    fn new() -> PingRequest {
        PingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PingRequest>(
                    "PingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionRequest {
    // message oneof groups
    request: ::std::option::Option<TransactionRequest_oneof_request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionRequest {}

#[derive(Clone,PartialEq)]
pub enum TransactionRequest_oneof_request {
    create_vertex(CreateVertexRequest),
    create_vertex_from_type(CreateVertexFromTypeRequest),
    get_vertices(GetVerticesRequest),
    delete_vertices(DeleteVerticesRequest),
    get_vertex_count(GetVertexCountRequest),
    create_edge(CreateEdgeRequest),
    get_edges(GetEdgesRequest),
    delete_edges(DeleteEdgesRequest),
    get_edge_count(GetEdgeCountRequest),
    get_global_metadata(GetGlobalMetadataRequest),
    set_global_metadata(SetGlobalMetadataRequest),
    delete_global_metadata(DeleteGlobalMetadataRequest),
    get_vertex_metadata(GetVertexMetadataRequest),
    set_vertex_metadata(SetVertexMetadataRequest),
    delete_vertex_metadata(DeleteVertexMetadataRequest),
    get_edge_metadata(GetEdgeMetadataRequest),
    set_edge_metadata(SetEdgeMetadataRequest),
    delete_edge_metadata(DeleteEdgeMetadataRequest),
}

impl TransactionRequest {
    pub fn new() -> TransactionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionRequest {
        static mut instance: ::protobuf::lazy::Lazy<TransactionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionRequest,
        };
        unsafe {
            instance.get(TransactionRequest::new)
        }
    }

    // .CreateVertexRequest create_vertex = 1;

    pub fn clear_create_vertex(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_create_vertex(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_vertex(&mut self, v: CreateVertexRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_vertex(&mut self) -> &mut CreateVertexRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(CreateVertexRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_vertex(&mut self) -> CreateVertexRequest {
        if self.has_create_vertex() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateVertexRequest::new()
        }
    }

    pub fn get_create_vertex(&self) -> &CreateVertexRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(ref v)) => v,
            _ => CreateVertexRequest::default_instance(),
        }
    }

    // .CreateVertexFromTypeRequest create_vertex_from_type = 2;

    pub fn clear_create_vertex_from_type(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_create_vertex_from_type(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_vertex_from_type(&mut self, v: CreateVertexFromTypeRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_vertex_from_type(&mut self) -> &mut CreateVertexFromTypeRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(CreateVertexFromTypeRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_vertex_from_type(&mut self) -> CreateVertexFromTypeRequest {
        if self.has_create_vertex_from_type() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateVertexFromTypeRequest::new()
        }
    }

    pub fn get_create_vertex_from_type(&self) -> &CreateVertexFromTypeRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(ref v)) => v,
            _ => CreateVertexFromTypeRequest::default_instance(),
        }
    }

    // .GetVerticesRequest get_vertices = 3;

    pub fn clear_get_vertices(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_vertices(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertices(&mut self, v: GetVerticesRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertices(&mut self) -> &mut GetVerticesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(GetVerticesRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertices(&mut self) -> GetVerticesRequest {
        if self.has_get_vertices() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVerticesRequest::new()
        }
    }

    pub fn get_get_vertices(&self) -> &GetVerticesRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(ref v)) => v,
            _ => GetVerticesRequest::default_instance(),
        }
    }

    // .DeleteVerticesRequest delete_vertices = 4;

    pub fn clear_delete_vertices(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_delete_vertices(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertices(&mut self, v: DeleteVerticesRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertices(&mut self) -> &mut DeleteVerticesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(DeleteVerticesRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertices(&mut self) -> DeleteVerticesRequest {
        if self.has_delete_vertices() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVerticesRequest::new()
        }
    }

    pub fn get_delete_vertices(&self) -> &DeleteVerticesRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(ref v)) => v,
            _ => DeleteVerticesRequest::default_instance(),
        }
    }

    // .GetVertexCountRequest get_vertex_count = 5;

    pub fn clear_get_vertex_count(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_vertex_count(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_count(&mut self, v: GetVertexCountRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_count(&mut self) -> &mut GetVertexCountRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(GetVertexCountRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_count(&mut self) -> GetVertexCountRequest {
        if self.has_get_vertex_count() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexCountRequest::new()
        }
    }

    pub fn get_get_vertex_count(&self) -> &GetVertexCountRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(ref v)) => v,
            _ => GetVertexCountRequest::default_instance(),
        }
    }

    // .CreateEdgeRequest create_edge = 6;

    pub fn clear_create_edge(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_create_edge(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_edge(&mut self, v: CreateEdgeRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_edge(&mut self) -> &mut CreateEdgeRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(CreateEdgeRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_edge(&mut self) -> CreateEdgeRequest {
        if self.has_create_edge() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateEdgeRequest::new()
        }
    }

    pub fn get_create_edge(&self) -> &CreateEdgeRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(ref v)) => v,
            _ => CreateEdgeRequest::default_instance(),
        }
    }

    // .GetEdgesRequest get_edges = 7;

    pub fn clear_get_edges(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_edges(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edges(&mut self, v: GetEdgesRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edges(&mut self) -> &mut GetEdgesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(GetEdgesRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edges(&mut self) -> GetEdgesRequest {
        if self.has_get_edges() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgesRequest::new()
        }
    }

    pub fn get_get_edges(&self) -> &GetEdgesRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(ref v)) => v,
            _ => GetEdgesRequest::default_instance(),
        }
    }

    // .DeleteEdgesRequest delete_edges = 8;

    pub fn clear_delete_edges(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_delete_edges(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edges(&mut self, v: DeleteEdgesRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edges(&mut self) -> &mut DeleteEdgesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(DeleteEdgesRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edges(&mut self) -> DeleteEdgesRequest {
        if self.has_delete_edges() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgesRequest::new()
        }
    }

    pub fn get_delete_edges(&self) -> &DeleteEdgesRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(ref v)) => v,
            _ => DeleteEdgesRequest::default_instance(),
        }
    }

    // .GetEdgeCountRequest get_edge_count = 9;

    pub fn clear_get_edge_count(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_edge_count(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_count(&mut self, v: GetEdgeCountRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_count(&mut self) -> &mut GetEdgeCountRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(GetEdgeCountRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_count(&mut self) -> GetEdgeCountRequest {
        if self.has_get_edge_count() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeCountRequest::new()
        }
    }

    pub fn get_get_edge_count(&self) -> &GetEdgeCountRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(ref v)) => v,
            _ => GetEdgeCountRequest::default_instance(),
        }
    }

    // .GetGlobalMetadataRequest get_global_metadata = 10;

    pub fn clear_get_global_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_global_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_global_metadata(&mut self, v: GetGlobalMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_global_metadata(&mut self) -> &mut GetGlobalMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(GetGlobalMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_global_metadata(&mut self) -> GetGlobalMetadataRequest {
        if self.has_get_global_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetGlobalMetadataRequest::new()
        }
    }

    pub fn get_get_global_metadata(&self) -> &GetGlobalMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(ref v)) => v,
            _ => GetGlobalMetadataRequest::default_instance(),
        }
    }

    // .SetGlobalMetadataRequest set_global_metadata = 11;

    pub fn clear_set_global_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_set_global_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_global_metadata(&mut self, v: SetGlobalMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_global_metadata(&mut self) -> &mut SetGlobalMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(SetGlobalMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_global_metadata(&mut self) -> SetGlobalMetadataRequest {
        if self.has_set_global_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetGlobalMetadataRequest::new()
        }
    }

    pub fn get_set_global_metadata(&self) -> &SetGlobalMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(ref v)) => v,
            _ => SetGlobalMetadataRequest::default_instance(),
        }
    }

    // .DeleteGlobalMetadataRequest delete_global_metadata = 12;

    pub fn clear_delete_global_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_delete_global_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_global_metadata(&mut self, v: DeleteGlobalMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_global_metadata(&mut self) -> &mut DeleteGlobalMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(DeleteGlobalMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_global_metadata(&mut self) -> DeleteGlobalMetadataRequest {
        if self.has_delete_global_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteGlobalMetadataRequest::new()
        }
    }

    pub fn get_delete_global_metadata(&self) -> &DeleteGlobalMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(ref v)) => v,
            _ => DeleteGlobalMetadataRequest::default_instance(),
        }
    }

    // .GetVertexMetadataRequest get_vertex_metadata = 13;

    pub fn clear_get_vertex_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_vertex_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_metadata(&mut self, v: GetVertexMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_metadata(&mut self) -> &mut GetVertexMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(GetVertexMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_metadata(&mut self) -> GetVertexMetadataRequest {
        if self.has_get_vertex_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexMetadataRequest::new()
        }
    }

    pub fn get_get_vertex_metadata(&self) -> &GetVertexMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(ref v)) => v,
            _ => GetVertexMetadataRequest::default_instance(),
        }
    }

    // .SetVertexMetadataRequest set_vertex_metadata = 14;

    pub fn clear_set_vertex_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_set_vertex_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_vertex_metadata(&mut self, v: SetVertexMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_vertex_metadata(&mut self) -> &mut SetVertexMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(SetVertexMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_vertex_metadata(&mut self) -> SetVertexMetadataRequest {
        if self.has_set_vertex_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetVertexMetadataRequest::new()
        }
    }

    pub fn get_set_vertex_metadata(&self) -> &SetVertexMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(ref v)) => v,
            _ => SetVertexMetadataRequest::default_instance(),
        }
    }

    // .DeleteVertexMetadataRequest delete_vertex_metadata = 15;

    pub fn clear_delete_vertex_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_delete_vertex_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertex_metadata(&mut self, v: DeleteVertexMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertex_metadata(&mut self) -> &mut DeleteVertexMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(DeleteVertexMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertex_metadata(&mut self) -> DeleteVertexMetadataRequest {
        if self.has_delete_vertex_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVertexMetadataRequest::new()
        }
    }

    pub fn get_delete_vertex_metadata(&self) -> &DeleteVertexMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(ref v)) => v,
            _ => DeleteVertexMetadataRequest::default_instance(),
        }
    }

    // .GetEdgeMetadataRequest get_edge_metadata = 16;

    pub fn clear_get_edge_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_edge_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_metadata(&mut self, v: GetEdgeMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_metadata(&mut self) -> &mut GetEdgeMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(GetEdgeMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_metadata(&mut self) -> GetEdgeMetadataRequest {
        if self.has_get_edge_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeMetadataRequest::new()
        }
    }

    pub fn get_get_edge_metadata(&self) -> &GetEdgeMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(ref v)) => v,
            _ => GetEdgeMetadataRequest::default_instance(),
        }
    }

    // .SetEdgeMetadataRequest set_edge_metadata = 17;

    pub fn clear_set_edge_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_set_edge_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_edge_metadata(&mut self, v: SetEdgeMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_edge_metadata(&mut self) -> &mut SetEdgeMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(SetEdgeMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_edge_metadata(&mut self) -> SetEdgeMetadataRequest {
        if self.has_set_edge_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetEdgeMetadataRequest::new()
        }
    }

    pub fn get_set_edge_metadata(&self) -> &SetEdgeMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(ref v)) => v,
            _ => SetEdgeMetadataRequest::default_instance(),
        }
    }

    // .DeleteEdgeMetadataRequest delete_edge_metadata = 18;

    pub fn clear_delete_edge_metadata(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_delete_edge_metadata(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edge_metadata(&mut self, v: DeleteEdgeMetadataRequest) {
        self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edge_metadata(&mut self) -> &mut DeleteEdgeMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(DeleteEdgeMetadataRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edge_metadata(&mut self) -> DeleteEdgeMetadataRequest {
        if self.has_delete_edge_metadata() {
            match self.request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgeMetadataRequest::new()
        }
    }

    pub fn get_delete_edge_metadata(&self) -> &DeleteEdgeMetadataRequest {
        match self.request {
            ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(ref v)) => v,
            _ => DeleteEdgeMetadataRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for TransactionRequest {
    fn is_initialized(&self) -> bool {
        if let Some(TransactionRequest_oneof_request::create_vertex(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::create_vertex_from_type(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_vertices(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::delete_vertices(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_vertex_count(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::create_edge(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_edges(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::delete_edges(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_edge_count(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_global_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::set_global_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::delete_global_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_vertex_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::set_vertex_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::delete_vertex_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::get_edge_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::set_edge_metadata(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_request::delete_edge_metadata(ref v)) = self.request {
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
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_vertex_from_type(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertices(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertices(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_count(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::create_edge(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edges(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edges(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_count(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_global_metadata(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_global_metadata(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_global_metadata(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_vertex_metadata(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_vertex_metadata(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_vertex_metadata(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::get_edge_metadata(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::set_edge_metadata(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(TransactionRequest_oneof_request::delete_edge_metadata(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &TransactionRequest_oneof_request::create_vertex(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::create_vertex_from_type(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::delete_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_vertex_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::create_edge(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::delete_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_edge_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::set_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::delete_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::set_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::delete_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::get_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::set_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_request::delete_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &TransactionRequest_oneof_request::create_vertex(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::create_vertex_from_type(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_vertices(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::delete_vertices(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_vertex_count(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::create_edge(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_edges(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::delete_edges(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_edge_count(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_global_metadata(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::set_global_metadata(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::delete_global_metadata(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_vertex_metadata(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::set_vertex_metadata(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::delete_vertex_metadata(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::get_edge_metadata(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::set_edge_metadata(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_request::delete_edge_metadata(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for TransactionRequest {
    fn new() -> TransactionRequest {
        TransactionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CreateVertexRequest>(
                    "create_vertex",
                    TransactionRequest::has_create_vertex,
                    TransactionRequest::get_create_vertex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CreateVertexFromTypeRequest>(
                    "create_vertex_from_type",
                    TransactionRequest::has_create_vertex_from_type,
                    TransactionRequest::get_create_vertex_from_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetVerticesRequest>(
                    "get_vertices",
                    TransactionRequest::has_get_vertices,
                    TransactionRequest::get_get_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteVerticesRequest>(
                    "delete_vertices",
                    TransactionRequest::has_delete_vertices,
                    TransactionRequest::get_delete_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetVertexCountRequest>(
                    "get_vertex_count",
                    TransactionRequest::has_get_vertex_count,
                    TransactionRequest::get_get_vertex_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CreateEdgeRequest>(
                    "create_edge",
                    TransactionRequest::has_create_edge,
                    TransactionRequest::get_create_edge,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetEdgesRequest>(
                    "get_edges",
                    TransactionRequest::has_get_edges,
                    TransactionRequest::get_get_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteEdgesRequest>(
                    "delete_edges",
                    TransactionRequest::has_delete_edges,
                    TransactionRequest::get_delete_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetEdgeCountRequest>(
                    "get_edge_count",
                    TransactionRequest::has_get_edge_count,
                    TransactionRequest::get_get_edge_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetGlobalMetadataRequest>(
                    "get_global_metadata",
                    TransactionRequest::has_get_global_metadata,
                    TransactionRequest::get_get_global_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetGlobalMetadataRequest>(
                    "set_global_metadata",
                    TransactionRequest::has_set_global_metadata,
                    TransactionRequest::get_set_global_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteGlobalMetadataRequest>(
                    "delete_global_metadata",
                    TransactionRequest::has_delete_global_metadata,
                    TransactionRequest::get_delete_global_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetVertexMetadataRequest>(
                    "get_vertex_metadata",
                    TransactionRequest::has_get_vertex_metadata,
                    TransactionRequest::get_get_vertex_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetVertexMetadataRequest>(
                    "set_vertex_metadata",
                    TransactionRequest::has_set_vertex_metadata,
                    TransactionRequest::get_set_vertex_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteVertexMetadataRequest>(
                    "delete_vertex_metadata",
                    TransactionRequest::has_delete_vertex_metadata,
                    TransactionRequest::get_delete_vertex_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetEdgeMetadataRequest>(
                    "get_edge_metadata",
                    TransactionRequest::has_get_edge_metadata,
                    TransactionRequest::get_get_edge_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetEdgeMetadataRequest>(
                    "set_edge_metadata",
                    TransactionRequest::has_set_edge_metadata,
                    TransactionRequest::get_set_edge_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteEdgeMetadataRequest>(
                    "delete_edge_metadata",
                    TransactionRequest::has_delete_edge_metadata,
                    TransactionRequest::get_delete_edge_metadata,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionRequest>(
                    "TransactionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionRequest {
    fn clear(&mut self) {
        self.clear_create_vertex();
        self.clear_create_vertex_from_type();
        self.clear_get_vertices();
        self.clear_delete_vertices();
        self.clear_get_vertex_count();
        self.clear_create_edge();
        self.clear_get_edges();
        self.clear_delete_edges();
        self.clear_get_edge_count();
        self.clear_get_global_metadata();
        self.clear_set_global_metadata();
        self.clear_delete_global_metadata();
        self.clear_get_vertex_metadata();
        self.clear_set_vertex_metadata();
        self.clear_delete_vertex_metadata();
        self.clear_get_edge_metadata();
        self.clear_set_edge_metadata();
        self.clear_delete_edge_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateVertexRequest {
    // message fields
    pub vertex: ::protobuf::SingularPtrField<super::vertices::Vertex>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateVertexRequest {}

impl CreateVertexRequest {
    pub fn new() -> CreateVertexRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateVertexRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateVertexRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateVertexRequest,
        };
        unsafe {
            instance.get(CreateVertexRequest::new)
        }
    }

    // .Vertex vertex = 1;

    pub fn clear_vertex(&mut self) {
        self.vertex.clear();
    }

    pub fn has_vertex(&self) -> bool {
        self.vertex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertex(&mut self, v: super::vertices::Vertex) {
        self.vertex = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vertex(&mut self) -> &mut super::vertices::Vertex {
        if self.vertex.is_none() {
            self.vertex.set_default();
        }
        self.vertex.as_mut().unwrap()
    }

    // Take field
    pub fn take_vertex(&mut self) -> super::vertices::Vertex {
        self.vertex.take().unwrap_or_else(|| super::vertices::Vertex::new())
    }

    pub fn get_vertex(&self) -> &super::vertices::Vertex {
        self.vertex.as_ref().unwrap_or_else(|| super::vertices::Vertex::default_instance())
    }

    fn get_vertex_for_reflect(&self) -> &::protobuf::SingularPtrField<super::vertices::Vertex> {
        &self.vertex
    }

    fn mut_vertex_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::vertices::Vertex> {
        &mut self.vertex
    }
}

impl ::protobuf::Message for CreateVertexRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.vertex {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.vertex)?;
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
        if let Some(ref v) = self.vertex.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.vertex.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for CreateVertexRequest {
    fn new() -> CreateVertexRequest {
        CreateVertexRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateVertexRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::vertices::Vertex>>(
                    "vertex",
                    CreateVertexRequest::get_vertex_for_reflect,
                    CreateVertexRequest::mut_vertex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateVertexRequest>(
                    "CreateVertexRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateVertexRequest {
    fn clear(&mut self) {
        self.clear_vertex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateVertexRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateVertexRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateVertexFromTypeRequest {
    // message fields
    pub field_type: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateVertexFromTypeRequest {}

impl CreateVertexFromTypeRequest {
    pub fn new() -> CreateVertexFromTypeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateVertexFromTypeRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateVertexFromTypeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateVertexFromTypeRequest,
        };
        unsafe {
            instance.get(CreateVertexFromTypeRequest::new)
        }
    }

    // string type = 1;

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

impl ::protobuf::Message for CreateVertexFromTypeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
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

impl ::protobuf::MessageStatic for CreateVertexFromTypeRequest {
    fn new() -> CreateVertexFromTypeRequest {
        CreateVertexFromTypeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateVertexFromTypeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    CreateVertexFromTypeRequest::get_field_type_for_reflect,
                    CreateVertexFromTypeRequest::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateVertexFromTypeRequest>(
                    "CreateVertexFromTypeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateVertexFromTypeRequest {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateVertexFromTypeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateVertexFromTypeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVerticesRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::VertexQuery>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVerticesRequest {}

impl GetVerticesRequest {
    pub fn new() -> GetVerticesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVerticesRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetVerticesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVerticesRequest,
        };
        unsafe {
            instance.get(GetVerticesRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::VertexQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::VertexQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::VertexQuery {
        self.query.take().unwrap_or_else(|| super::queries::VertexQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::VertexQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::VertexQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &mut self.query
    }
}

impl ::protobuf::Message for GetVerticesRequest {
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

impl ::protobuf::MessageStatic for GetVerticesRequest {
    fn new() -> GetVerticesRequest {
        GetVerticesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVerticesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::VertexQuery>>(
                    "query",
                    GetVerticesRequest::get_query_for_reflect,
                    GetVerticesRequest::mut_query_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVerticesRequest>(
                    "GetVerticesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVerticesRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVerticesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVerticesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteVerticesRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::VertexQuery>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteVerticesRequest {}

impl DeleteVerticesRequest {
    pub fn new() -> DeleteVerticesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteVerticesRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteVerticesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteVerticesRequest,
        };
        unsafe {
            instance.get(DeleteVerticesRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::VertexQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::VertexQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::VertexQuery {
        self.query.take().unwrap_or_else(|| super::queries::VertexQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::VertexQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::VertexQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &mut self.query
    }
}

impl ::protobuf::Message for DeleteVerticesRequest {
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

impl ::protobuf::MessageStatic for DeleteVerticesRequest {
    fn new() -> DeleteVerticesRequest {
        DeleteVerticesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteVerticesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::VertexQuery>>(
                    "query",
                    DeleteVerticesRequest::get_query_for_reflect,
                    DeleteVerticesRequest::mut_query_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteVerticesRequest>(
                    "DeleteVerticesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteVerticesRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteVerticesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteVerticesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVertexCountRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVertexCountRequest {}

impl GetVertexCountRequest {
    pub fn new() -> GetVertexCountRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVertexCountRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetVertexCountRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVertexCountRequest,
        };
        unsafe {
            instance.get(GetVertexCountRequest::new)
        }
    }
}

impl ::protobuf::Message for GetVertexCountRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for GetVertexCountRequest {
    fn new() -> GetVertexCountRequest {
        GetVertexCountRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVertexCountRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetVertexCountRequest>(
                    "GetVertexCountRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVertexCountRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVertexCountRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVertexCountRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateEdgeRequest {
    // message fields
    pub key: ::protobuf::SingularPtrField<super::edges::EdgeKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEdgeRequest {}

impl CreateEdgeRequest {
    pub fn new() -> CreateEdgeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEdgeRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateEdgeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEdgeRequest,
        };
        unsafe {
            instance.get(CreateEdgeRequest::new)
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
}

impl ::protobuf::Message for CreateEdgeRequest {
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

impl ::protobuf::MessageStatic for CreateEdgeRequest {
    fn new() -> CreateEdgeRequest {
        CreateEdgeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEdgeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::edges::EdgeKey>>(
                    "key",
                    CreateEdgeRequest::get_key_for_reflect,
                    CreateEdgeRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEdgeRequest>(
                    "CreateEdgeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEdgeRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEdgeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEdgeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEdgesRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::EdgeQuery>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgesRequest {}

impl GetEdgesRequest {
    pub fn new() -> GetEdgesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEdgesRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetEdgesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEdgesRequest,
        };
        unsafe {
            instance.get(GetEdgesRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::EdgeQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::EdgeQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::EdgeQuery {
        self.query.take().unwrap_or_else(|| super::queries::EdgeQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::EdgeQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::EdgeQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &mut self.query
    }
}

impl ::protobuf::Message for GetEdgesRequest {
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

impl ::protobuf::MessageStatic for GetEdgesRequest {
    fn new() -> GetEdgesRequest {
        GetEdgesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEdgesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::EdgeQuery>>(
                    "query",
                    GetEdgesRequest::get_query_for_reflect,
                    GetEdgesRequest::mut_query_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEdgesRequest>(
                    "GetEdgesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEdgesRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEdgesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEdgesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteEdgesRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::EdgeQuery>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteEdgesRequest {}

impl DeleteEdgesRequest {
    pub fn new() -> DeleteEdgesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteEdgesRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteEdgesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteEdgesRequest,
        };
        unsafe {
            instance.get(DeleteEdgesRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::EdgeQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::EdgeQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::EdgeQuery {
        self.query.take().unwrap_or_else(|| super::queries::EdgeQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::EdgeQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::EdgeQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &mut self.query
    }
}

impl ::protobuf::Message for DeleteEdgesRequest {
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

impl ::protobuf::MessageStatic for DeleteEdgesRequest {
    fn new() -> DeleteEdgesRequest {
        DeleteEdgesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteEdgesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::EdgeQuery>>(
                    "query",
                    DeleteEdgesRequest::get_query_for_reflect,
                    DeleteEdgesRequest::mut_query_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteEdgesRequest>(
                    "DeleteEdgesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteEdgesRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteEdgesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteEdgesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEdgeCountRequest {
    // message fields
    pub id: ::std::string::String,
    pub type_filter: ::std::string::String,
    pub direction: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgeCountRequest {}

impl GetEdgeCountRequest {
    pub fn new() -> GetEdgeCountRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEdgeCountRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetEdgeCountRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEdgeCountRequest,
        };
        unsafe {
            instance.get(GetEdgeCountRequest::new)
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

    // string type_filter = 2;

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

    // string direction = 3;

    pub fn clear_direction(&mut self) {
        self.direction.clear();
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: ::std::string::String) {
        self.direction = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_direction(&mut self) -> &mut ::std::string::String {
        &mut self.direction
    }

    // Take field
    pub fn take_direction(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.direction, ::std::string::String::new())
    }

    pub fn get_direction(&self) -> &str {
        &self.direction
    }

    fn get_direction_for_reflect(&self) -> &::std::string::String {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.direction
    }
}

impl ::protobuf::Message for GetEdgeCountRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_filter)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.direction)?;
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
        if !self.type_filter.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.type_filter);
        }
        if !self.direction.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.direction);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.type_filter.is_empty() {
            os.write_string(2, &self.type_filter)?;
        }
        if !self.direction.is_empty() {
            os.write_string(3, &self.direction)?;
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

impl ::protobuf::MessageStatic for GetEdgeCountRequest {
    fn new() -> GetEdgeCountRequest {
        GetEdgeCountRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEdgeCountRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    GetEdgeCountRequest::get_id_for_reflect,
                    GetEdgeCountRequest::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_filter",
                    GetEdgeCountRequest::get_type_filter_for_reflect,
                    GetEdgeCountRequest::mut_type_filter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "direction",
                    GetEdgeCountRequest::get_direction_for_reflect,
                    GetEdgeCountRequest::mut_direction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEdgeCountRequest>(
                    "GetEdgeCountRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEdgeCountRequest {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_type_filter();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEdgeCountRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEdgeCountRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetGlobalMetadataRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGlobalMetadataRequest {}

impl GetGlobalMetadataRequest {
    pub fn new() -> GetGlobalMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetGlobalMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetGlobalMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetGlobalMetadataRequest,
        };
        unsafe {
            instance.get(GetGlobalMetadataRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for GetGlobalMetadataRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for GetGlobalMetadataRequest {
    fn new() -> GetGlobalMetadataRequest {
        GetGlobalMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetGlobalMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GetGlobalMetadataRequest::get_name_for_reflect,
                    GetGlobalMetadataRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetGlobalMetadataRequest>(
                    "GetGlobalMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetGlobalMetadataRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetGlobalMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGlobalMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetGlobalMetadataRequest {
    // message fields
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetGlobalMetadataRequest {}

impl SetGlobalMetadataRequest {
    pub fn new() -> SetGlobalMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetGlobalMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetGlobalMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetGlobalMetadataRequest,
        };
        unsafe {
            instance.get(SetGlobalMetadataRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
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

impl ::protobuf::Message for SetGlobalMetadataRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for SetGlobalMetadataRequest {
    fn new() -> SetGlobalMetadataRequest {
        SetGlobalMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetGlobalMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    SetGlobalMetadataRequest::get_name_for_reflect,
                    SetGlobalMetadataRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    SetGlobalMetadataRequest::get_value_for_reflect,
                    SetGlobalMetadataRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetGlobalMetadataRequest>(
                    "SetGlobalMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetGlobalMetadataRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetGlobalMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetGlobalMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteGlobalMetadataRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteGlobalMetadataRequest {}

impl DeleteGlobalMetadataRequest {
    pub fn new() -> DeleteGlobalMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteGlobalMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteGlobalMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteGlobalMetadataRequest,
        };
        unsafe {
            instance.get(DeleteGlobalMetadataRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for DeleteGlobalMetadataRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for DeleteGlobalMetadataRequest {
    fn new() -> DeleteGlobalMetadataRequest {
        DeleteGlobalMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteGlobalMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeleteGlobalMetadataRequest::get_name_for_reflect,
                    DeleteGlobalMetadataRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteGlobalMetadataRequest>(
                    "DeleteGlobalMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteGlobalMetadataRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteGlobalMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteGlobalMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVertexMetadataRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::VertexQuery>,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVertexMetadataRequest {}

impl GetVertexMetadataRequest {
    pub fn new() -> GetVertexMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVertexMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetVertexMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVertexMetadataRequest,
        };
        unsafe {
            instance.get(GetVertexMetadataRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::VertexQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::VertexQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::VertexQuery {
        self.query.take().unwrap_or_else(|| super::queries::VertexQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::VertexQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::VertexQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &mut self.query
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for GetVertexMetadataRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
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

impl ::protobuf::MessageStatic for GetVertexMetadataRequest {
    fn new() -> GetVertexMetadataRequest {
        GetVertexMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVertexMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::VertexQuery>>(
                    "query",
                    GetVertexMetadataRequest::get_query_for_reflect,
                    GetVertexMetadataRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GetVertexMetadataRequest::get_name_for_reflect,
                    GetVertexMetadataRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVertexMetadataRequest>(
                    "GetVertexMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVertexMetadataRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVertexMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVertexMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetVertexMetadataRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::VertexQuery>,
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetVertexMetadataRequest {}

impl SetVertexMetadataRequest {
    pub fn new() -> SetVertexMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetVertexMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetVertexMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetVertexMetadataRequest,
        };
        unsafe {
            instance.get(SetVertexMetadataRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::VertexQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::VertexQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::VertexQuery {
        self.query.take().unwrap_or_else(|| super::queries::VertexQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::VertexQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::VertexQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &mut self.query
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string value = 3;

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

impl ::protobuf::Message for SetVertexMetadataRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
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
        if let Some(ref v) = self.query.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.value);
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.value.is_empty() {
            os.write_string(3, &self.value)?;
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

impl ::protobuf::MessageStatic for SetVertexMetadataRequest {
    fn new() -> SetVertexMetadataRequest {
        SetVertexMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetVertexMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::VertexQuery>>(
                    "query",
                    SetVertexMetadataRequest::get_query_for_reflect,
                    SetVertexMetadataRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    SetVertexMetadataRequest::get_name_for_reflect,
                    SetVertexMetadataRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    SetVertexMetadataRequest::get_value_for_reflect,
                    SetVertexMetadataRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetVertexMetadataRequest>(
                    "SetVertexMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetVertexMetadataRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetVertexMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetVertexMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteVertexMetadataRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::VertexQuery>,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteVertexMetadataRequest {}

impl DeleteVertexMetadataRequest {
    pub fn new() -> DeleteVertexMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteVertexMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteVertexMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteVertexMetadataRequest,
        };
        unsafe {
            instance.get(DeleteVertexMetadataRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::VertexQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::VertexQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::VertexQuery {
        self.query.take().unwrap_or_else(|| super::queries::VertexQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::VertexQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::VertexQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::VertexQuery> {
        &mut self.query
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for DeleteVertexMetadataRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
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

impl ::protobuf::MessageStatic for DeleteVertexMetadataRequest {
    fn new() -> DeleteVertexMetadataRequest {
        DeleteVertexMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteVertexMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::VertexQuery>>(
                    "query",
                    DeleteVertexMetadataRequest::get_query_for_reflect,
                    DeleteVertexMetadataRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeleteVertexMetadataRequest::get_name_for_reflect,
                    DeleteVertexMetadataRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteVertexMetadataRequest>(
                    "DeleteVertexMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteVertexMetadataRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteVertexMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteVertexMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEdgeMetadataRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::EdgeQuery>,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgeMetadataRequest {}

impl GetEdgeMetadataRequest {
    pub fn new() -> GetEdgeMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEdgeMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetEdgeMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEdgeMetadataRequest,
        };
        unsafe {
            instance.get(GetEdgeMetadataRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::EdgeQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::EdgeQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::EdgeQuery {
        self.query.take().unwrap_or_else(|| super::queries::EdgeQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::EdgeQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::EdgeQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &mut self.query
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for GetEdgeMetadataRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
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

impl ::protobuf::MessageStatic for GetEdgeMetadataRequest {
    fn new() -> GetEdgeMetadataRequest {
        GetEdgeMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEdgeMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::EdgeQuery>>(
                    "query",
                    GetEdgeMetadataRequest::get_query_for_reflect,
                    GetEdgeMetadataRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GetEdgeMetadataRequest::get_name_for_reflect,
                    GetEdgeMetadataRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEdgeMetadataRequest>(
                    "GetEdgeMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEdgeMetadataRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEdgeMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEdgeMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetEdgeMetadataRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::EdgeQuery>,
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetEdgeMetadataRequest {}

impl SetEdgeMetadataRequest {
    pub fn new() -> SetEdgeMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetEdgeMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetEdgeMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetEdgeMetadataRequest,
        };
        unsafe {
            instance.get(SetEdgeMetadataRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::EdgeQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::EdgeQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::EdgeQuery {
        self.query.take().unwrap_or_else(|| super::queries::EdgeQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::EdgeQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::EdgeQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &mut self.query
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string value = 3;

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

impl ::protobuf::Message for SetEdgeMetadataRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
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
        if let Some(ref v) = self.query.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.value);
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.value.is_empty() {
            os.write_string(3, &self.value)?;
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

impl ::protobuf::MessageStatic for SetEdgeMetadataRequest {
    fn new() -> SetEdgeMetadataRequest {
        SetEdgeMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetEdgeMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::EdgeQuery>>(
                    "query",
                    SetEdgeMetadataRequest::get_query_for_reflect,
                    SetEdgeMetadataRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    SetEdgeMetadataRequest::get_name_for_reflect,
                    SetEdgeMetadataRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    SetEdgeMetadataRequest::get_value_for_reflect,
                    SetEdgeMetadataRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetEdgeMetadataRequest>(
                    "SetEdgeMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetEdgeMetadataRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetEdgeMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetEdgeMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteEdgeMetadataRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<super::queries::EdgeQuery>,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteEdgeMetadataRequest {}

impl DeleteEdgeMetadataRequest {
    pub fn new() -> DeleteEdgeMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteEdgeMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteEdgeMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteEdgeMetadataRequest,
        };
        unsafe {
            instance.get(DeleteEdgeMetadataRequest::new)
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
    pub fn set_query(&mut self, v: super::queries::EdgeQuery) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut super::queries::EdgeQuery {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> super::queries::EdgeQuery {
        self.query.take().unwrap_or_else(|| super::queries::EdgeQuery::new())
    }

    pub fn get_query(&self) -> &super::queries::EdgeQuery {
        self.query.as_ref().unwrap_or_else(|| super::queries::EdgeQuery::default_instance())
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::queries::EdgeQuery> {
        &mut self.query
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for DeleteEdgeMetadataRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
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
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
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

impl ::protobuf::MessageStatic for DeleteEdgeMetadataRequest {
    fn new() -> DeleteEdgeMetadataRequest {
        DeleteEdgeMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteEdgeMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::queries::EdgeQuery>>(
                    "query",
                    DeleteEdgeMetadataRequest::get_query_for_reflect,
                    DeleteEdgeMetadataRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeleteEdgeMetadataRequest::get_name_for_reflect,
                    DeleteEdgeMetadataRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteEdgeMetadataRequest>(
                    "DeleteEdgeMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteEdgeMetadataRequest {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteEdgeMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteEdgeMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13proto/request.proto\x1a\x11proto/edges.proto\x1a\x13proto/queries.\
    proto\x1a\x14proto/vertices.proto\"\r\n\x0bPingRequest\"\x92\n\n\x12Tran\
    sactionRequest\x12;\n\rcreate_vertex\x18\x01\x20\x01(\x0b2\x14.CreateVer\
    texRequestH\0R\x0ccreateVertex\x12U\n\x17create_vertex_from_type\x18\x02\
    \x20\x01(\x0b2\x1c.CreateVertexFromTypeRequestH\0R\x14createVertexFromTy\
    pe\x128\n\x0cget_vertices\x18\x03\x20\x01(\x0b2\x13.GetVerticesRequestH\
    \0R\x0bgetVertices\x12A\n\x0fdelete_vertices\x18\x04\x20\x01(\x0b2\x16.D\
    eleteVerticesRequestH\0R\x0edeleteVertices\x12B\n\x10get_vertex_count\
    \x18\x05\x20\x01(\x0b2\x16.GetVertexCountRequestH\0R\x0egetVertexCount\
    \x125\n\x0bcreate_edge\x18\x06\x20\x01(\x0b2\x12.CreateEdgeRequestH\0R\n\
    createEdge\x12/\n\tget_edges\x18\x07\x20\x01(\x0b2\x10.GetEdgesRequestH\
    \0R\x08getEdges\x128\n\x0cdelete_edges\x18\x08\x20\x01(\x0b2\x13.DeleteE\
    dgesRequestH\0R\x0bdeleteEdges\x12<\n\x0eget_edge_count\x18\t\x20\x01(\
    \x0b2\x14.GetEdgeCountRequestH\0R\x0cgetEdgeCount\x12K\n\x13get_global_m\
    etadata\x18\n\x20\x01(\x0b2\x19.GetGlobalMetadataRequestH\0R\x11getGloba\
    lMetadata\x12K\n\x13set_global_metadata\x18\x0b\x20\x01(\x0b2\x19.SetGlo\
    balMetadataRequestH\0R\x11setGlobalMetadata\x12T\n\x16delete_global_meta\
    data\x18\x0c\x20\x01(\x0b2\x1c.DeleteGlobalMetadataRequestH\0R\x14delete\
    GlobalMetadata\x12K\n\x13get_vertex_metadata\x18\r\x20\x01(\x0b2\x19.Get\
    VertexMetadataRequestH\0R\x11getVertexMetadata\x12K\n\x13set_vertex_meta\
    data\x18\x0e\x20\x01(\x0b2\x19.SetVertexMetadataRequestH\0R\x11setVertex\
    Metadata\x12T\n\x16delete_vertex_metadata\x18\x0f\x20\x01(\x0b2\x1c.Dele\
    teVertexMetadataRequestH\0R\x14deleteVertexMetadata\x12E\n\x11get_edge_m\
    etadata\x18\x10\x20\x01(\x0b2\x17.GetEdgeMetadataRequestH\0R\x0fgetEdgeM\
    etadata\x12E\n\x11set_edge_metadata\x18\x11\x20\x01(\x0b2\x17.SetEdgeMet\
    adataRequestH\0R\x0fsetEdgeMetadata\x12N\n\x14delete_edge_metadata\x18\
    \x12\x20\x01(\x0b2\x1a.DeleteEdgeMetadataRequestH\0R\x12deleteEdgeMetada\
    taB\t\n\x07request\"6\n\x13CreateVertexRequest\x12\x1f\n\x06vertex\x18\
    \x01\x20\x01(\x0b2\x07.VertexR\x06vertex\"1\n\x1bCreateVertexFromTypeReq\
    uest\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\"8\n\x12GetVerticesRe\
    quest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\x05query\"\
    ;\n\x15DeleteVerticesRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.\
    VertexQueryR\x05query\"\x17\n\x15GetVertexCountRequest\"/\n\x11CreateEdg\
    eRequest\x12\x1a\n\x03key\x18\x01\x20\x01(\x0b2\x08.EdgeKeyR\x03key\"3\n\
    \x0fGetEdgesRequest\x12\x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQuery\
    R\x05query\"6\n\x12DeleteEdgesRequest\x12\x20\n\x05query\x18\x01\x20\x01\
    (\x0b2\n.EdgeQueryR\x05query\"d\n\x13GetEdgeCountRequest\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\tR\x02id\x12\x1f\n\x0btype_filter\x18\x02\x20\x01(\tR\
    \ntypeFilter\x12\x1c\n\tdirection\x18\x03\x20\x01(\tR\tdirection\".\n\
    \x18GetGlobalMetadataRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04na\
    me\"D\n\x18SetGlobalMetadataRequest\x12\x12\n\x04name\x18\x01\x20\x01(\t\
    R\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value\"1\n\x1bDelete\
    GlobalMetadataRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"R\n\
    \x18GetVertexMetadataRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.\
    VertexQueryR\x05query\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"h\n\
    \x18SetVertexMetadataRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.\
    VertexQueryR\x05query\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\
    \x14\n\x05value\x18\x03\x20\x01(\tR\x05value\"U\n\x1bDeleteVertexMetadat\
    aRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\x05quer\
    y\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"N\n\x16GetEdgeMetadataR\
    equest\x12\x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\
    \x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"d\n\x16SetEdgeMetadataRe\
    quest\x12\x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\x12\
    \x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x03\
    \x20\x01(\tR\x05value\"Q\n\x19DeleteEdgeMetadataRequest\x12\x20\n\x05que\
    ry\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\x12\x12\n\x04name\x18\x02\
    \x20\x01(\tR\x04nameJ\xb4\x1b\n\x06\x12\x04\0\0o\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x1a\n\t\n\x02\x03\x01\x12\
    \x03\x03\x07\x1c\n\t\n\x02\x03\x02\x12\x03\x04\x07\x1d\n\n\n\x02\x04\0\
    \x12\x04\x06\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x13\n\n\n\x02\
    \x04\x01\x12\x04\t\0\x1e\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x1a\n\
    \x0c\n\x04\x04\x01\x08\0\x12\x04\n\x04\x1d\x05\n\x0c\n\x05\x04\x01\x08\0\
    \x01\x12\x03\n\n\x11\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x08.\n\x0c\n\
    \x05\x04\x01\x02\0\x06\x12\x03\x0b\x08\x1b\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x0b\x1c)\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b,-\n\x0b\n\
    \x04\x04\x01\x02\x01\x12\x03\x0c\x08@\n\x0c\n\x05\x04\x01\x02\x01\x06\
    \x12\x03\x0c\x08#\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c$;\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x03\x0c>?\n\x0b\n\x04\x04\x01\x02\x02\x12\
    \x03\r\x08,\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\r\x08\x1a\n\x0c\n\
    \x05\x04\x01\x02\x02\x01\x12\x03\r\x1b'\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x03\r*+\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x0e\x082\n\x0c\n\x05\
    \x04\x01\x02\x03\x06\x12\x03\x0e\x08\x1d\n\x0c\n\x05\x04\x01\x02\x03\x01\
    \x12\x03\x0e\x1e-\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x0e01\n\x0b\n\
    \x04\x04\x01\x02\x04\x12\x03\x0f\x083\n\x0c\n\x05\x04\x01\x02\x04\x06\
    \x12\x03\x0f\x08\x1d\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x0f\x1e.\n\
    \x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x0f12\n\x0b\n\x04\x04\x01\x02\x05\
    \x12\x03\x10\x08*\n\x0c\n\x05\x04\x01\x02\x05\x06\x12\x03\x10\x08\x19\n\
    \x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x10\x1a%\n\x0c\n\x05\x04\x01\x02\
    \x05\x03\x12\x03\x10()\n\x0b\n\x04\x04\x01\x02\x06\x12\x03\x11\x08&\n\
    \x0c\n\x05\x04\x01\x02\x06\x06\x12\x03\x11\x08\x17\n\x0c\n\x05\x04\x01\
    \x02\x06\x01\x12\x03\x11\x18!\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\
    \x11$%\n\x0b\n\x04\x04\x01\x02\x07\x12\x03\x12\x08,\n\x0c\n\x05\x04\x01\
    \x02\x07\x06\x12\x03\x12\x08\x1a\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\
    \x12\x1b'\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03\x12*+\n\x0b\n\x04\x04\
    \x01\x02\x08\x12\x03\x13\x08/\n\x0c\n\x05\x04\x01\x02\x08\x06\x12\x03\
    \x13\x08\x1b\n\x0c\n\x05\x04\x01\x02\x08\x01\x12\x03\x13\x1c*\n\x0c\n\
    \x05\x04\x01\x02\x08\x03\x12\x03\x13-.\n\x0b\n\x04\x04\x01\x02\t\x12\x03\
    \x14\x08:\n\x0c\n\x05\x04\x01\x02\t\x06\x12\x03\x14\x08\x20\n\x0c\n\x05\
    \x04\x01\x02\t\x01\x12\x03\x14!4\n\x0c\n\x05\x04\x01\x02\t\x03\x12\x03\
    \x1479\n\x0b\n\x04\x04\x01\x02\n\x12\x03\x15\x08:\n\x0c\n\x05\x04\x01\
    \x02\n\x06\x12\x03\x15\x08\x20\n\x0c\n\x05\x04\x01\x02\n\x01\x12\x03\x15\
    !4\n\x0c\n\x05\x04\x01\x02\n\x03\x12\x03\x1579\n\x0b\n\x04\x04\x01\x02\
    \x0b\x12\x03\x16\x08@\n\x0c\n\x05\x04\x01\x02\x0b\x06\x12\x03\x16\x08#\n\
    \x0c\n\x05\x04\x01\x02\x0b\x01\x12\x03\x16$:\n\x0c\n\x05\x04\x01\x02\x0b\
    \x03\x12\x03\x16=?\n\x0b\n\x04\x04\x01\x02\x0c\x12\x03\x17\x08:\n\x0c\n\
    \x05\x04\x01\x02\x0c\x06\x12\x03\x17\x08\x20\n\x0c\n\x05\x04\x01\x02\x0c\
    \x01\x12\x03\x17!4\n\x0c\n\x05\x04\x01\x02\x0c\x03\x12\x03\x1779\n\x0b\n\
    \x04\x04\x01\x02\r\x12\x03\x18\x08:\n\x0c\n\x05\x04\x01\x02\r\x06\x12\
    \x03\x18\x08\x20\n\x0c\n\x05\x04\x01\x02\r\x01\x12\x03\x18!4\n\x0c\n\x05\
    \x04\x01\x02\r\x03\x12\x03\x1879\n\x0b\n\x04\x04\x01\x02\x0e\x12\x03\x19\
    \x08@\n\x0c\n\x05\x04\x01\x02\x0e\x06\x12\x03\x19\x08#\n\x0c\n\x05\x04\
    \x01\x02\x0e\x01\x12\x03\x19$:\n\x0c\n\x05\x04\x01\x02\x0e\x03\x12\x03\
    \x19=?\n\x0b\n\x04\x04\x01\x02\x0f\x12\x03\x1a\x086\n\x0c\n\x05\x04\x01\
    \x02\x0f\x06\x12\x03\x1a\x08\x1e\n\x0c\n\x05\x04\x01\x02\x0f\x01\x12\x03\
    \x1a\x1f0\n\x0c\n\x05\x04\x01\x02\x0f\x03\x12\x03\x1a35\n\x0b\n\x04\x04\
    \x01\x02\x10\x12\x03\x1b\x086\n\x0c\n\x05\x04\x01\x02\x10\x06\x12\x03\
    \x1b\x08\x1e\n\x0c\n\x05\x04\x01\x02\x10\x01\x12\x03\x1b\x1f0\n\x0c\n\
    \x05\x04\x01\x02\x10\x03\x12\x03\x1b35\n\x0b\n\x04\x04\x01\x02\x11\x12\
    \x03\x1c\x08<\n\x0c\n\x05\x04\x01\x02\x11\x06\x12\x03\x1c\x08!\n\x0c\n\
    \x05\x04\x01\x02\x11\x01\x12\x03\x1c\"6\n\x0c\n\x05\x04\x01\x02\x11\x03\
    \x12\x03\x1c9;\n\n\n\x02\x04\x02\x12\x04\x20\0\"\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x20\x08\x1b\n\x0b\n\x04\x04\x02\x02\0\x12\x03!\x04\x16\n\r\
    \n\x05\x04\x02\x02\0\x04\x12\x04!\x04\x20\x1d\n\x0c\n\x05\x04\x02\x02\0\
    \x06\x12\x03!\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03!\x0b\x11\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03!\x14\x15\n\n\n\x02\x04\x03\x12\x04$\0&\
    \x01\n\n\n\x03\x04\x03\x01\x12\x03$\x08#\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03%\x04\x14\n\r\n\x05\x04\x03\x02\0\x04\x12\x04%\x04$%\n\x0c\n\x05\x04\
    \x03\x02\0\x05\x12\x03%\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03%\x0b\
    \x0f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03%\x12\x13\n\n\n\x02\x04\x04\
    \x12\x04(\0*\x01\n\n\n\x03\x04\x04\x01\x12\x03(\x08\x1a\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03)\x04\x1a\n\r\n\x05\x04\x04\x02\0\x04\x12\x04)\x04(\
    \x1c\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03)\x04\x0f\n\x0c\n\x05\x04\x04\
    \x02\0\x01\x12\x03)\x10\x15\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03)\x18\
    \x19\n\n\n\x02\x04\x05\x12\x04,\0.\x01\n\n\n\x03\x04\x05\x01\x12\x03,\
    \x08\x1d\n\x0b\n\x04\x04\x05\x02\0\x12\x03-\x04\x1a\n\r\n\x05\x04\x05\
    \x02\0\x04\x12\x04-\x04,\x1f\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03-\x04\
    \x0f\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03-\x10\x15\n\x0c\n\x05\x04\x05\
    \x02\0\x03\x12\x03-\x18\x19\n\t\n\x02\x04\x06\x12\x030\0!\n\n\n\x03\x04\
    \x06\x01\x12\x030\x08\x1d\n\n\n\x02\x04\x07\x12\x042\04\x01\n\n\n\x03\
    \x04\x07\x01\x12\x032\x08\x19\n\x0b\n\x04\x04\x07\x02\0\x12\x033\x04\x14\
    \n\r\n\x05\x04\x07\x02\0\x04\x12\x043\x042\x1b\n\x0c\n\x05\x04\x07\x02\0\
    \x06\x12\x033\x04\x0b\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x033\x0c\x0f\n\
    \x0c\n\x05\x04\x07\x02\0\x03\x12\x033\x12\x13\n\n\n\x02\x04\x08\x12\x046\
    \08\x01\n\n\n\x03\x04\x08\x01\x12\x036\x08\x17\n\x0b\n\x04\x04\x08\x02\0\
    \x12\x037\x04\x18\n\r\n\x05\x04\x08\x02\0\x04\x12\x047\x046\x19\n\x0c\n\
    \x05\x04\x08\x02\0\x06\x12\x037\x04\r\n\x0c\n\x05\x04\x08\x02\0\x01\x12\
    \x037\x0e\x13\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x037\x16\x17\n\n\n\x02\
    \x04\t\x12\x04:\0<\x01\n\n\n\x03\x04\t\x01\x12\x03:\x08\x1a\n\x0b\n\x04\
    \x04\t\x02\0\x12\x03;\x04\x18\n\r\n\x05\x04\t\x02\0\x04\x12\x04;\x04:\
    \x1c\n\x0c\n\x05\x04\t\x02\0\x06\x12\x03;\x04\r\n\x0c\n\x05\x04\t\x02\0\
    \x01\x12\x03;\x0e\x13\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03;\x16\x17\n\n\n\
    \x02\x04\n\x12\x04>\0B\x01\n\n\n\x03\x04\n\x01\x12\x03>\x08\x1b\n\x0b\n\
    \x04\x04\n\x02\0\x12\x03?\x04\x12\n\r\n\x05\x04\n\x02\0\x04\x12\x04?\x04\
    >\x1d\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03?\x04\n\n\x0c\n\x05\x04\n\x02\0\
    \x01\x12\x03?\x0b\r\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03?\x10\x11\n\x0b\n\
    \x04\x04\n\x02\x01\x12\x03@\x04\x1b\n\r\n\x05\x04\n\x02\x01\x04\x12\x04@\
    \x04?\x12\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03@\x04\n\n\x0c\n\x05\x04\n\
    \x02\x01\x01\x12\x03@\x0b\x16\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03@\x19\
    \x1a\n\x0b\n\x04\x04\n\x02\x02\x12\x03A\x04\x19\n\r\n\x05\x04\n\x02\x02\
    \x04\x12\x04A\x04@\x1b\n\x0c\n\x05\x04\n\x02\x02\x05\x12\x03A\x04\n\n\
    \x0c\n\x05\x04\n\x02\x02\x01\x12\x03A\x0b\x14\n\x0c\n\x05\x04\n\x02\x02\
    \x03\x12\x03A\x17\x18\n\n\n\x02\x04\x0b\x12\x04D\0F\x01\n\n\n\x03\x04\
    \x0b\x01\x12\x03D\x08\x20\n\x0b\n\x04\x04\x0b\x02\0\x12\x03E\x04\x14\n\r\
    \n\x05\x04\x0b\x02\0\x04\x12\x04E\x04D\"\n\x0c\n\x05\x04\x0b\x02\0\x05\
    \x12\x03E\x04\n\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03E\x0b\x0f\n\x0c\n\
    \x05\x04\x0b\x02\0\x03\x12\x03E\x12\x13\n\n\n\x02\x04\x0c\x12\x04H\0K\
    \x01\n\n\n\x03\x04\x0c\x01\x12\x03H\x08\x20\n\x0b\n\x04\x04\x0c\x02\0\
    \x12\x03I\x04\x14\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04I\x04H\"\n\x0c\n\
    \x05\x04\x0c\x02\0\x05\x12\x03I\x04\n\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\
    \x03I\x0b\x0f\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\x03I\x12\x13\n\x0b\n\x04\
    \x04\x0c\x02\x01\x12\x03J\x04\x15\n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04J\
    \x04I\x14\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\x03J\x04\n\n\x0c\n\x05\x04\
    \x0c\x02\x01\x01\x12\x03J\x0b\x10\n\x0c\n\x05\x04\x0c\x02\x01\x03\x12\
    \x03J\x13\x14\n\n\n\x02\x04\r\x12\x04M\0O\x01\n\n\n\x03\x04\r\x01\x12\
    \x03M\x08#\n\x0b\n\x04\x04\r\x02\0\x12\x03N\x04\x14\n\r\n\x05\x04\r\x02\
    \0\x04\x12\x04N\x04M%\n\x0c\n\x05\x04\r\x02\0\x05\x12\x03N\x04\n\n\x0c\n\
    \x05\x04\r\x02\0\x01\x12\x03N\x0b\x0f\n\x0c\n\x05\x04\r\x02\0\x03\x12\
    \x03N\x12\x13\n\n\n\x02\x04\x0e\x12\x04Q\0T\x01\n\n\n\x03\x04\x0e\x01\
    \x12\x03Q\x08\x20\n\x0b\n\x04\x04\x0e\x02\0\x12\x03R\x04\x1a\n\r\n\x05\
    \x04\x0e\x02\0\x04\x12\x04R\x04Q\"\n\x0c\n\x05\x04\x0e\x02\0\x06\x12\x03\
    R\x04\x0f\n\x0c\n\x05\x04\x0e\x02\0\x01\x12\x03R\x10\x15\n\x0c\n\x05\x04\
    \x0e\x02\0\x03\x12\x03R\x18\x19\n\x0b\n\x04\x04\x0e\x02\x01\x12\x03S\x04\
    \x14\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04S\x04R\x1a\n\x0c\n\x05\x04\x0e\
    \x02\x01\x05\x12\x03S\x04\n\n\x0c\n\x05\x04\x0e\x02\x01\x01\x12\x03S\x0b\
    \x0f\n\x0c\n\x05\x04\x0e\x02\x01\x03\x12\x03S\x12\x13\n\n\n\x02\x04\x0f\
    \x12\x04V\0Z\x01\n\n\n\x03\x04\x0f\x01\x12\x03V\x08\x20\n\x0b\n\x04\x04\
    \x0f\x02\0\x12\x03W\x04\x1a\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04W\x04V\"\
    \n\x0c\n\x05\x04\x0f\x02\0\x06\x12\x03W\x04\x0f\n\x0c\n\x05\x04\x0f\x02\
    \0\x01\x12\x03W\x10\x15\n\x0c\n\x05\x04\x0f\x02\0\x03\x12\x03W\x18\x19\n\
    \x0b\n\x04\x04\x0f\x02\x01\x12\x03X\x04\x14\n\r\n\x05\x04\x0f\x02\x01\
    \x04\x12\x04X\x04W\x1a\n\x0c\n\x05\x04\x0f\x02\x01\x05\x12\x03X\x04\n\n\
    \x0c\n\x05\x04\x0f\x02\x01\x01\x12\x03X\x0b\x0f\n\x0c\n\x05\x04\x0f\x02\
    \x01\x03\x12\x03X\x12\x13\n\x0b\n\x04\x04\x0f\x02\x02\x12\x03Y\x04\x15\n\
    \r\n\x05\x04\x0f\x02\x02\x04\x12\x04Y\x04X\x14\n\x0c\n\x05\x04\x0f\x02\
    \x02\x05\x12\x03Y\x04\n\n\x0c\n\x05\x04\x0f\x02\x02\x01\x12\x03Y\x0b\x10\
    \n\x0c\n\x05\x04\x0f\x02\x02\x03\x12\x03Y\x13\x14\n\n\n\x02\x04\x10\x12\
    \x04\\\0_\x01\n\n\n\x03\x04\x10\x01\x12\x03\\\x08#\n\x0b\n\x04\x04\x10\
    \x02\0\x12\x03]\x04\x1a\n\r\n\x05\x04\x10\x02\0\x04\x12\x04]\x04\\%\n\
    \x0c\n\x05\x04\x10\x02\0\x06\x12\x03]\x04\x0f\n\x0c\n\x05\x04\x10\x02\0\
    \x01\x12\x03]\x10\x15\n\x0c\n\x05\x04\x10\x02\0\x03\x12\x03]\x18\x19\n\
    \x0b\n\x04\x04\x10\x02\x01\x12\x03^\x04\x14\n\r\n\x05\x04\x10\x02\x01\
    \x04\x12\x04^\x04]\x1a\n\x0c\n\x05\x04\x10\x02\x01\x05\x12\x03^\x04\n\n\
    \x0c\n\x05\x04\x10\x02\x01\x01\x12\x03^\x0b\x0f\n\x0c\n\x05\x04\x10\x02\
    \x01\x03\x12\x03^\x12\x13\n\n\n\x02\x04\x11\x12\x04a\0d\x01\n\n\n\x03\
    \x04\x11\x01\x12\x03a\x08\x1e\n\x0b\n\x04\x04\x11\x02\0\x12\x03b\x04\x18\
    \n\r\n\x05\x04\x11\x02\0\x04\x12\x04b\x04a\x20\n\x0c\n\x05\x04\x11\x02\0\
    \x06\x12\x03b\x04\r\n\x0c\n\x05\x04\x11\x02\0\x01\x12\x03b\x0e\x13\n\x0c\
    \n\x05\x04\x11\x02\0\x03\x12\x03b\x16\x17\n\x0b\n\x04\x04\x11\x02\x01\
    \x12\x03c\x04\x14\n\r\n\x05\x04\x11\x02\x01\x04\x12\x04c\x04b\x18\n\x0c\
    \n\x05\x04\x11\x02\x01\x05\x12\x03c\x04\n\n\x0c\n\x05\x04\x11\x02\x01\
    \x01\x12\x03c\x0b\x0f\n\x0c\n\x05\x04\x11\x02\x01\x03\x12\x03c\x12\x13\n\
    \n\n\x02\x04\x12\x12\x04f\0j\x01\n\n\n\x03\x04\x12\x01\x12\x03f\x08\x1e\
    \n\x0b\n\x04\x04\x12\x02\0\x12\x03g\x04\x18\n\r\n\x05\x04\x12\x02\0\x04\
    \x12\x04g\x04f\x20\n\x0c\n\x05\x04\x12\x02\0\x06\x12\x03g\x04\r\n\x0c\n\
    \x05\x04\x12\x02\0\x01\x12\x03g\x0e\x13\n\x0c\n\x05\x04\x12\x02\0\x03\
    \x12\x03g\x16\x17\n\x0b\n\x04\x04\x12\x02\x01\x12\x03h\x04\x14\n\r\n\x05\
    \x04\x12\x02\x01\x04\x12\x04h\x04g\x18\n\x0c\n\x05\x04\x12\x02\x01\x05\
    \x12\x03h\x04\n\n\x0c\n\x05\x04\x12\x02\x01\x01\x12\x03h\x0b\x0f\n\x0c\n\
    \x05\x04\x12\x02\x01\x03\x12\x03h\x12\x13\n\x0b\n\x04\x04\x12\x02\x02\
    \x12\x03i\x04\x15\n\r\n\x05\x04\x12\x02\x02\x04\x12\x04i\x04h\x14\n\x0c\
    \n\x05\x04\x12\x02\x02\x05\x12\x03i\x04\n\n\x0c\n\x05\x04\x12\x02\x02\
    \x01\x12\x03i\x0b\x10\n\x0c\n\x05\x04\x12\x02\x02\x03\x12\x03i\x13\x14\n\
    \n\n\x02\x04\x13\x12\x04l\0o\x01\n\n\n\x03\x04\x13\x01\x12\x03l\x08!\n\
    \x0b\n\x04\x04\x13\x02\0\x12\x03m\x04\x18\n\r\n\x05\x04\x13\x02\0\x04\
    \x12\x04m\x04l#\n\x0c\n\x05\x04\x13\x02\0\x06\x12\x03m\x04\r\n\x0c\n\x05\
    \x04\x13\x02\0\x01\x12\x03m\x0e\x13\n\x0c\n\x05\x04\x13\x02\0\x03\x12\
    \x03m\x16\x17\n\x0b\n\x04\x04\x13\x02\x01\x12\x03n\x04\x14\n\r\n\x05\x04\
    \x13\x02\x01\x04\x12\x04n\x04m\x18\n\x0c\n\x05\x04\x13\x02\x01\x05\x12\
    \x03n\x04\n\n\x0c\n\x05\x04\x13\x02\x01\x01\x12\x03n\x0b\x0f\n\x0c\n\x05\
    \x04\x13\x02\x01\x03\x12\x03n\x12\x13b\x06proto3\
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
