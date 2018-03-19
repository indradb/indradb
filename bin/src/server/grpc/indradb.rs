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
pub struct TransactionRequest {
    // message oneof groups
    Request: ::std::option::Option<TransactionRequest_oneof_Request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionRequest {}

#[derive(Clone,PartialEq)]
pub enum TransactionRequest_oneof_Request {
    create_vertex(CreateVertexRequest),
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
        self.Request = ::std::option::Option::None;
    }

    pub fn has_create_vertex(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_vertex(&mut self, v: CreateVertexRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_vertex(&mut self) -> &mut CreateVertexRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(CreateVertexRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_vertex(&mut self) -> CreateVertexRequest {
        if self.has_create_vertex() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateVertexRequest::new()
        }
    }

    pub fn get_create_vertex(&self) -> &CreateVertexRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(ref v)) => v,
            _ => CreateVertexRequest::default_instance(),
        }
    }

    // .GetVerticesRequest get_vertices = 2;

    pub fn clear_get_vertices(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_vertices(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertices(&mut self, v: GetVerticesRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertices(&mut self) -> &mut GetVerticesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(GetVerticesRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertices(&mut self) -> GetVerticesRequest {
        if self.has_get_vertices() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVerticesRequest::new()
        }
    }

    pub fn get_get_vertices(&self) -> &GetVerticesRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(ref v)) => v,
            _ => GetVerticesRequest::default_instance(),
        }
    }

    // .DeleteVerticesRequest delete_vertices = 3;

    pub fn clear_delete_vertices(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_delete_vertices(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertices(&mut self, v: DeleteVerticesRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertices(&mut self) -> &mut DeleteVerticesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(DeleteVerticesRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertices(&mut self) -> DeleteVerticesRequest {
        if self.has_delete_vertices() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVerticesRequest::new()
        }
    }

    pub fn get_delete_vertices(&self) -> &DeleteVerticesRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(ref v)) => v,
            _ => DeleteVerticesRequest::default_instance(),
        }
    }

    // .GetVertexCountRequest get_vertex_count = 4;

    pub fn clear_get_vertex_count(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_vertex_count(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_count(&mut self, v: GetVertexCountRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_count(&mut self) -> &mut GetVertexCountRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(GetVertexCountRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_count(&mut self) -> GetVertexCountRequest {
        if self.has_get_vertex_count() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexCountRequest::new()
        }
    }

    pub fn get_get_vertex_count(&self) -> &GetVertexCountRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(ref v)) => v,
            _ => GetVertexCountRequest::default_instance(),
        }
    }

    // .CreateEdgeRequest create_edge = 5;

    pub fn clear_create_edge(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_create_edge(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_edge(&mut self, v: CreateEdgeRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_edge(&mut self) -> &mut CreateEdgeRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(CreateEdgeRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_edge(&mut self) -> CreateEdgeRequest {
        if self.has_create_edge() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateEdgeRequest::new()
        }
    }

    pub fn get_create_edge(&self) -> &CreateEdgeRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(ref v)) => v,
            _ => CreateEdgeRequest::default_instance(),
        }
    }

    // .GetEdgesRequest get_edges = 6;

    pub fn clear_get_edges(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_edges(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edges(&mut self, v: GetEdgesRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edges(&mut self) -> &mut GetEdgesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(GetEdgesRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edges(&mut self) -> GetEdgesRequest {
        if self.has_get_edges() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgesRequest::new()
        }
    }

    pub fn get_get_edges(&self) -> &GetEdgesRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(ref v)) => v,
            _ => GetEdgesRequest::default_instance(),
        }
    }

    // .DeleteEdgesRequest delete_edges = 7;

    pub fn clear_delete_edges(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_delete_edges(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edges(&mut self, v: DeleteEdgesRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edges(&mut self) -> &mut DeleteEdgesRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(DeleteEdgesRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edges(&mut self) -> DeleteEdgesRequest {
        if self.has_delete_edges() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgesRequest::new()
        }
    }

    pub fn get_delete_edges(&self) -> &DeleteEdgesRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(ref v)) => v,
            _ => DeleteEdgesRequest::default_instance(),
        }
    }

    // .GetEdgeCountRequest get_edge_count = 8;

    pub fn clear_get_edge_count(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_edge_count(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_count(&mut self, v: GetEdgeCountRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_count(&mut self) -> &mut GetEdgeCountRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(GetEdgeCountRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_count(&mut self) -> GetEdgeCountRequest {
        if self.has_get_edge_count() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeCountRequest::new()
        }
    }

    pub fn get_get_edge_count(&self) -> &GetEdgeCountRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(ref v)) => v,
            _ => GetEdgeCountRequest::default_instance(),
        }
    }

    // .GetGlobalMetadataRequest get_global_metadata = 9;

    pub fn clear_get_global_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_global_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_global_metadata(&mut self, v: GetGlobalMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_global_metadata(&mut self) -> &mut GetGlobalMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(GetGlobalMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_global_metadata(&mut self) -> GetGlobalMetadataRequest {
        if self.has_get_global_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetGlobalMetadataRequest::new()
        }
    }

    pub fn get_get_global_metadata(&self) -> &GetGlobalMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(ref v)) => v,
            _ => GetGlobalMetadataRequest::default_instance(),
        }
    }

    // .SetGlobalMetadataRequest set_global_metadata = 10;

    pub fn clear_set_global_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_set_global_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_global_metadata(&mut self, v: SetGlobalMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_global_metadata(&mut self) -> &mut SetGlobalMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(SetGlobalMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_global_metadata(&mut self) -> SetGlobalMetadataRequest {
        if self.has_set_global_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetGlobalMetadataRequest::new()
        }
    }

    pub fn get_set_global_metadata(&self) -> &SetGlobalMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(ref v)) => v,
            _ => SetGlobalMetadataRequest::default_instance(),
        }
    }

    // .DeleteGlobalMetadataRequest delete_global_metadata = 11;

    pub fn clear_delete_global_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_delete_global_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_global_metadata(&mut self, v: DeleteGlobalMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_global_metadata(&mut self) -> &mut DeleteGlobalMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(DeleteGlobalMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_global_metadata(&mut self) -> DeleteGlobalMetadataRequest {
        if self.has_delete_global_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteGlobalMetadataRequest::new()
        }
    }

    pub fn get_delete_global_metadata(&self) -> &DeleteGlobalMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(ref v)) => v,
            _ => DeleteGlobalMetadataRequest::default_instance(),
        }
    }

    // .GetVertexMetadataRequest get_vertex_metadata = 12;

    pub fn clear_get_vertex_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_vertex_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_metadata(&mut self, v: GetVertexMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_metadata(&mut self) -> &mut GetVertexMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(GetVertexMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_metadata(&mut self) -> GetVertexMetadataRequest {
        if self.has_get_vertex_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexMetadataRequest::new()
        }
    }

    pub fn get_get_vertex_metadata(&self) -> &GetVertexMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(ref v)) => v,
            _ => GetVertexMetadataRequest::default_instance(),
        }
    }

    // .SetVertexMetadataRequest set_vertex_metadata = 13;

    pub fn clear_set_vertex_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_set_vertex_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_vertex_metadata(&mut self, v: SetVertexMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_vertex_metadata(&mut self) -> &mut SetVertexMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(SetVertexMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_vertex_metadata(&mut self) -> SetVertexMetadataRequest {
        if self.has_set_vertex_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetVertexMetadataRequest::new()
        }
    }

    pub fn get_set_vertex_metadata(&self) -> &SetVertexMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(ref v)) => v,
            _ => SetVertexMetadataRequest::default_instance(),
        }
    }

    // .DeleteVertexMetadataRequest delete_vertex_metadata = 14;

    pub fn clear_delete_vertex_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_delete_vertex_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertex_metadata(&mut self, v: DeleteVertexMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertex_metadata(&mut self) -> &mut DeleteVertexMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(DeleteVertexMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertex_metadata(&mut self) -> DeleteVertexMetadataRequest {
        if self.has_delete_vertex_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVertexMetadataRequest::new()
        }
    }

    pub fn get_delete_vertex_metadata(&self) -> &DeleteVertexMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(ref v)) => v,
            _ => DeleteVertexMetadataRequest::default_instance(),
        }
    }

    // .GetEdgeMetadataRequest get_edge_metadata = 15;

    pub fn clear_get_edge_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_get_edge_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_metadata(&mut self, v: GetEdgeMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_metadata(&mut self) -> &mut GetEdgeMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(GetEdgeMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_metadata(&mut self) -> GetEdgeMetadataRequest {
        if self.has_get_edge_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeMetadataRequest::new()
        }
    }

    pub fn get_get_edge_metadata(&self) -> &GetEdgeMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(ref v)) => v,
            _ => GetEdgeMetadataRequest::default_instance(),
        }
    }

    // .SetEdgeMetadataRequest set_edge_metadata = 16;

    pub fn clear_set_edge_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_set_edge_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_edge_metadata(&mut self, v: SetEdgeMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_edge_metadata(&mut self) -> &mut SetEdgeMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(SetEdgeMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_edge_metadata(&mut self) -> SetEdgeMetadataRequest {
        if self.has_set_edge_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetEdgeMetadataRequest::new()
        }
    }

    pub fn get_set_edge_metadata(&self) -> &SetEdgeMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(ref v)) => v,
            _ => SetEdgeMetadataRequest::default_instance(),
        }
    }

    // .DeleteEdgeMetadataRequest delete_edge_metadata = 17;

    pub fn clear_delete_edge_metadata(&mut self) {
        self.Request = ::std::option::Option::None;
    }

    pub fn has_delete_edge_metadata(&self) -> bool {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edge_metadata(&mut self, v: DeleteEdgeMetadataRequest) {
        self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edge_metadata(&mut self) -> &mut DeleteEdgeMetadataRequest {
        if let ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(_)) = self.Request {
        } else {
            self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(DeleteEdgeMetadataRequest::new()));
        }
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edge_metadata(&mut self) -> DeleteEdgeMetadataRequest {
        if self.has_delete_edge_metadata() {
            match self.Request.take() {
                ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgeMetadataRequest::new()
        }
    }

    pub fn get_delete_edge_metadata(&self) -> &DeleteEdgeMetadataRequest {
        match self.Request {
            ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(ref v)) => v,
            _ => DeleteEdgeMetadataRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for TransactionRequest {
    fn is_initialized(&self) -> bool {
        if let Some(TransactionRequest_oneof_Request::create_vertex(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_vertices(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::delete_vertices(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_vertex_count(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::create_edge(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_edges(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::delete_edges(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_edge_count(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_global_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::set_global_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::delete_global_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_vertex_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::set_vertex_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::delete_vertex_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::get_edge_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::set_edge_metadata(ref v)) = self.Request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionRequest_oneof_Request::delete_edge_metadata(ref v)) = self.Request {
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
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::create_vertex(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertices(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertices(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_count(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::create_edge(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edges(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edges(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_count(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_global_metadata(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_global_metadata(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_global_metadata(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_vertex_metadata(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_vertex_metadata(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_vertex_metadata(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::get_edge_metadata(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::set_edge_metadata(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Request = ::std::option::Option::Some(TransactionRequest_oneof_Request::delete_edge_metadata(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.Request {
            match v {
                &TransactionRequest_oneof_Request::create_vertex(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::delete_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_vertex_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::create_edge(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::delete_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_edge_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::set_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::delete_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::set_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::delete_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::get_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::set_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionRequest_oneof_Request::delete_edge_metadata(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.Request {
            match v {
                &TransactionRequest_oneof_Request::create_vertex(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_vertices(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::delete_vertices(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_vertex_count(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::create_edge(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_edges(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::delete_edges(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_edge_count(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_global_metadata(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::set_global_metadata(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::delete_global_metadata(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_vertex_metadata(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::set_vertex_metadata(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::delete_vertex_metadata(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::get_edge_metadata(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::set_edge_metadata(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionRequest_oneof_Request::delete_edge_metadata(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
    pub vertex: ::protobuf::SingularPtrField<Vertex>,
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
    pub fn set_vertex(&mut self, v: Vertex) {
        self.vertex = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vertex(&mut self) -> &mut Vertex {
        if self.vertex.is_none() {
            self.vertex.set_default();
        }
        self.vertex.as_mut().unwrap()
    }

    // Take field
    pub fn take_vertex(&mut self) -> Vertex {
        self.vertex.take().unwrap_or_else(|| Vertex::new())
    }

    pub fn get_vertex(&self) -> &Vertex {
        self.vertex.as_ref().unwrap_or_else(|| Vertex::default_instance())
    }

    fn get_vertex_for_reflect(&self) -> &::protobuf::SingularPtrField<Vertex> {
        &self.vertex
    }

    fn mut_vertex_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Vertex> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Vertex>>(
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
pub struct GetVerticesRequest {
    // message fields
    pub query: ::protobuf::SingularPtrField<VertexQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexQuery>>(
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
    pub query: ::protobuf::SingularPtrField<VertexQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexQuery>>(
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
    pub key: ::protobuf::SingularPtrField<EdgeKey>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeKey>>(
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
    pub query: ::protobuf::SingularPtrField<EdgeQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeQuery>>(
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
    pub query: ::protobuf::SingularPtrField<EdgeQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeQuery>>(
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
    pub query: ::protobuf::SingularPtrField<VertexQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexQuery>>(
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
    pub query: ::protobuf::SingularPtrField<VertexQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexQuery>>(
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
    pub query: ::protobuf::SingularPtrField<VertexQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexQuery>>(
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
    pub query: ::protobuf::SingularPtrField<EdgeQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeQuery>>(
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
    pub query: ::protobuf::SingularPtrField<EdgeQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeQuery>>(
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
    pub query: ::protobuf::SingularPtrField<EdgeQuery>,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeQuery>>(
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
    pub field_type: ::std::string::String,
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
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Edge::get_field_type_for_reflect,
                    Edge::mut_field_type_for_reflect,
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
        self.clear_field_type();
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
pub struct VertexQuery {
    // message oneof groups
    Query: ::std::option::Option<VertexQuery_oneof_Query>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VertexQuery {}

#[derive(Clone,PartialEq)]
pub enum VertexQuery_oneof_Query {
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
        self.Query = ::std::option::Option::None;
    }

    pub fn has_all(&self) -> bool {
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::all(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_all(&mut self, v: AllVertexQuery) {
        self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::all(v))
    }

    // Mutable pointer to the field.
    pub fn mut_all(&mut self) -> &mut AllVertexQuery {
        if let ::std::option::Option::Some(VertexQuery_oneof_Query::all(_)) = self.Query {
        } else {
            self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::all(AllVertexQuery::new()));
        }
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::all(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_all(&mut self) -> AllVertexQuery {
        if self.has_all() {
            match self.Query.take() {
                ::std::option::Option::Some(VertexQuery_oneof_Query::all(v)) => v,
                _ => panic!(),
            }
        } else {
            AllVertexQuery::new()
        }
    }

    pub fn get_all(&self) -> &AllVertexQuery {
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::all(ref v)) => v,
            _ => AllVertexQuery::default_instance(),
        }
    }

    // .VerticesVertexQuery vertices = 2;

    pub fn clear_vertices(&mut self) {
        self.Query = ::std::option::Option::None;
    }

    pub fn has_vertices(&self) -> bool {
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: VerticesVertexQuery) {
        self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut VerticesVertexQuery {
        if let ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(_)) = self.Query {
        } else {
            self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(VerticesVertexQuery::new()));
        }
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vertices(&mut self) -> VerticesVertexQuery {
        if self.has_vertices() {
            match self.Query.take() {
                ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            VerticesVertexQuery::new()
        }
    }

    pub fn get_vertices(&self) -> &VerticesVertexQuery {
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(ref v)) => v,
            _ => VerticesVertexQuery::default_instance(),
        }
    }

    // .PipeVertexQuery pipe = 3;

    pub fn clear_pipe(&mut self) {
        self.Query = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: PipeVertexQuery) {
        self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut PipeVertexQuery {
        if let ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(_)) = self.Query {
        } else {
            self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(PipeVertexQuery::new()));
        }
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> PipeVertexQuery {
        if self.has_pipe() {
            match self.Query.take() {
                ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            PipeVertexQuery::new()
        }
    }

    pub fn get_pipe(&self) -> &PipeVertexQuery {
        match self.Query {
            ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(ref v)) => v,
            _ => PipeVertexQuery::default_instance(),
        }
    }
}

impl ::protobuf::Message for VertexQuery {
    fn is_initialized(&self) -> bool {
        if let Some(VertexQuery_oneof_Query::all(ref v)) = self.Query {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(VertexQuery_oneof_Query::vertices(ref v)) = self.Query {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(VertexQuery_oneof_Query::pipe(ref v)) = self.Query {
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
                    self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::all(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::vertices(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Query = ::std::option::Option::Some(VertexQuery_oneof_Query::pipe(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.Query {
            match v {
                &VertexQuery_oneof_Query::all(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VertexQuery_oneof_Query::vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VertexQuery_oneof_Query::pipe(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.Query {
            match v {
                &VertexQuery_oneof_Query::all(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &VertexQuery_oneof_Query::vertices(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &VertexQuery_oneof_Query::pipe(ref v) => {
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
    Query: ::std::option::Option<EdgeQuery_oneof_Query>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EdgeQuery {}

#[derive(Clone,PartialEq)]
pub enum EdgeQuery_oneof_Query {
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
        self.Query = ::std::option::Option::None;
    }

    pub fn has_edges(&self) -> bool {
        match self.Query {
            ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_edges(&mut self, v: EdgesEdgeQuery) {
        self.Query = ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_edges(&mut self) -> &mut EdgesEdgeQuery {
        if let ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(_)) = self.Query {
        } else {
            self.Query = ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(EdgesEdgeQuery::new()));
        }
        match self.Query {
            ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_edges(&mut self) -> EdgesEdgeQuery {
        if self.has_edges() {
            match self.Query.take() {
                ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(v)) => v,
                _ => panic!(),
            }
        } else {
            EdgesEdgeQuery::new()
        }
    }

    pub fn get_edges(&self) -> &EdgesEdgeQuery {
        match self.Query {
            ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(ref v)) => v,
            _ => EdgesEdgeQuery::default_instance(),
        }
    }

    // .PipeEdgeQuery pipe = 2;

    pub fn clear_pipe(&mut self) {
        self.Query = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.Query {
            ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: PipeEdgeQuery) {
        self.Query = ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut PipeEdgeQuery {
        if let ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(_)) = self.Query {
        } else {
            self.Query = ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(PipeEdgeQuery::new()));
        }
        match self.Query {
            ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> PipeEdgeQuery {
        if self.has_pipe() {
            match self.Query.take() {
                ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            PipeEdgeQuery::new()
        }
    }

    pub fn get_pipe(&self) -> &PipeEdgeQuery {
        match self.Query {
            ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(ref v)) => v,
            _ => PipeEdgeQuery::default_instance(),
        }
    }
}

impl ::protobuf::Message for EdgeQuery {
    fn is_initialized(&self) -> bool {
        if let Some(EdgeQuery_oneof_Query::edges(ref v)) = self.Query {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(EdgeQuery_oneof_Query::pipe(ref v)) = self.Query {
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
                    self.Query = ::std::option::Option::Some(EdgeQuery_oneof_Query::edges(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Query = ::std::option::Option::Some(EdgeQuery_oneof_Query::pipe(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.Query {
            match v {
                &EdgeQuery_oneof_Query::edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &EdgeQuery_oneof_Query::pipe(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.Query {
            match v {
                &EdgeQuery_oneof_Query::edges(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &EdgeQuery_oneof_Query::pipe(ref v) => {
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
    pub keys: ::protobuf::RepeatedField<EdgeKey>,
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
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<EdgeKey>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<EdgeKey> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<EdgeKey> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[EdgeKey] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<EdgeKey> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EdgeKey> {
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EdgeKey>>(
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

#[derive(PartialEq,Clone,Default)]
pub struct TransactionResponse {
    // message oneof groups
    Response: ::std::option::Option<TransactionResponse_oneof_Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionResponse {}

#[derive(Clone,PartialEq)]
pub enum TransactionResponse_oneof_Response {
    create_vertex(CreateVertexResponse),
    get_vertices(GetVerticesResponse),
    delete_vertices(DeleteVerticesResponse),
    get_vertex_count(GetVertexCountResponse),
    create_edge(CreateEdgeResponse),
    get_edges(GetEdgesResponse),
    delete_edges(DeleteEdgesResponse),
    get_edge_count(GetEdgeCountResponse),
    get_global_metadata(GetGlobalMetadataResponse),
    set_global_metadata(SetGlobalMetadataResponse),
    delete_global_metadata(DeleteGlobalMetadataResponse),
    get_vertex_metadata(GetVertexMetadataResponse),
    set_vertex_metadata(SetVertexMetadataResponse),
    delete_vertex_metadata(DeleteVertexMetadataResponse),
    get_edge_metadata(GetEdgeMetadataResponse),
    set_edge_metadata(SetEdgeMetadataResponse),
    delete_edge_metadata(DeleteEdgeMetadataResponse),
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

    // .CreateVertexResponse create_vertex = 1;

    pub fn clear_create_vertex(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_create_vertex(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_vertex(&mut self, v: CreateVertexResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_vertex(&mut self) -> &mut CreateVertexResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(CreateVertexResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_vertex(&mut self) -> CreateVertexResponse {
        if self.has_create_vertex() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateVertexResponse::new()
        }
    }

    pub fn get_create_vertex(&self) -> &CreateVertexResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(ref v)) => v,
            _ => CreateVertexResponse::default_instance(),
        }
    }

    // .GetVerticesResponse get_vertices = 2;

    pub fn clear_get_vertices(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_vertices(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertices(&mut self, v: GetVerticesResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertices(&mut self) -> &mut GetVerticesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(GetVerticesResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertices(&mut self) -> GetVerticesResponse {
        if self.has_get_vertices() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVerticesResponse::new()
        }
    }

    pub fn get_get_vertices(&self) -> &GetVerticesResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(ref v)) => v,
            _ => GetVerticesResponse::default_instance(),
        }
    }

    // .DeleteVerticesResponse delete_vertices = 3;

    pub fn clear_delete_vertices(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_delete_vertices(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertices(&mut self, v: DeleteVerticesResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertices(&mut self) -> &mut DeleteVerticesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(DeleteVerticesResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertices(&mut self) -> DeleteVerticesResponse {
        if self.has_delete_vertices() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVerticesResponse::new()
        }
    }

    pub fn get_delete_vertices(&self) -> &DeleteVerticesResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(ref v)) => v,
            _ => DeleteVerticesResponse::default_instance(),
        }
    }

    // .GetVertexCountResponse get_vertex_count = 4;

    pub fn clear_get_vertex_count(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_vertex_count(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_count(&mut self, v: GetVertexCountResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_count(&mut self) -> &mut GetVertexCountResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(GetVertexCountResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_count(&mut self) -> GetVertexCountResponse {
        if self.has_get_vertex_count() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexCountResponse::new()
        }
    }

    pub fn get_get_vertex_count(&self) -> &GetVertexCountResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(ref v)) => v,
            _ => GetVertexCountResponse::default_instance(),
        }
    }

    // .CreateEdgeResponse create_edge = 5;

    pub fn clear_create_edge(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_create_edge(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_edge(&mut self, v: CreateEdgeResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_edge(&mut self) -> &mut CreateEdgeResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(CreateEdgeResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_edge(&mut self) -> CreateEdgeResponse {
        if self.has_create_edge() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateEdgeResponse::new()
        }
    }

    pub fn get_create_edge(&self) -> &CreateEdgeResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(ref v)) => v,
            _ => CreateEdgeResponse::default_instance(),
        }
    }

    // .GetEdgesResponse get_edges = 6;

    pub fn clear_get_edges(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_edges(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edges(&mut self, v: GetEdgesResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edges(&mut self) -> &mut GetEdgesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(GetEdgesResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edges(&mut self) -> GetEdgesResponse {
        if self.has_get_edges() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgesResponse::new()
        }
    }

    pub fn get_get_edges(&self) -> &GetEdgesResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(ref v)) => v,
            _ => GetEdgesResponse::default_instance(),
        }
    }

    // .DeleteEdgesResponse delete_edges = 7;

    pub fn clear_delete_edges(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_delete_edges(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edges(&mut self, v: DeleteEdgesResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edges(&mut self) -> &mut DeleteEdgesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(DeleteEdgesResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edges(&mut self) -> DeleteEdgesResponse {
        if self.has_delete_edges() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgesResponse::new()
        }
    }

    pub fn get_delete_edges(&self) -> &DeleteEdgesResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(ref v)) => v,
            _ => DeleteEdgesResponse::default_instance(),
        }
    }

    // .GetEdgeCountResponse get_edge_count = 8;

    pub fn clear_get_edge_count(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_edge_count(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_count(&mut self, v: GetEdgeCountResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_count(&mut self) -> &mut GetEdgeCountResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(GetEdgeCountResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_count(&mut self) -> GetEdgeCountResponse {
        if self.has_get_edge_count() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeCountResponse::new()
        }
    }

    pub fn get_get_edge_count(&self) -> &GetEdgeCountResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(ref v)) => v,
            _ => GetEdgeCountResponse::default_instance(),
        }
    }

    // .GetGlobalMetadataResponse get_global_metadata = 9;

    pub fn clear_get_global_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_global_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_global_metadata(&mut self, v: GetGlobalMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_global_metadata(&mut self) -> &mut GetGlobalMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(GetGlobalMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_global_metadata(&mut self) -> GetGlobalMetadataResponse {
        if self.has_get_global_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetGlobalMetadataResponse::new()
        }
    }

    pub fn get_get_global_metadata(&self) -> &GetGlobalMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(ref v)) => v,
            _ => GetGlobalMetadataResponse::default_instance(),
        }
    }

    // .SetGlobalMetadataResponse set_global_metadata = 10;

    pub fn clear_set_global_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_set_global_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_global_metadata(&mut self, v: SetGlobalMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_global_metadata(&mut self) -> &mut SetGlobalMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(SetGlobalMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_global_metadata(&mut self) -> SetGlobalMetadataResponse {
        if self.has_set_global_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetGlobalMetadataResponse::new()
        }
    }

    pub fn get_set_global_metadata(&self) -> &SetGlobalMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(ref v)) => v,
            _ => SetGlobalMetadataResponse::default_instance(),
        }
    }

    // .DeleteGlobalMetadataResponse delete_global_metadata = 11;

    pub fn clear_delete_global_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_delete_global_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_global_metadata(&mut self, v: DeleteGlobalMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_global_metadata(&mut self) -> &mut DeleteGlobalMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(DeleteGlobalMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_global_metadata(&mut self) -> DeleteGlobalMetadataResponse {
        if self.has_delete_global_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteGlobalMetadataResponse::new()
        }
    }

    pub fn get_delete_global_metadata(&self) -> &DeleteGlobalMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(ref v)) => v,
            _ => DeleteGlobalMetadataResponse::default_instance(),
        }
    }

    // .GetVertexMetadataResponse get_vertex_metadata = 12;

    pub fn clear_get_vertex_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_vertex_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_metadata(&mut self, v: GetVertexMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_metadata(&mut self) -> &mut GetVertexMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(GetVertexMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_metadata(&mut self) -> GetVertexMetadataResponse {
        if self.has_get_vertex_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexMetadataResponse::new()
        }
    }

    pub fn get_get_vertex_metadata(&self) -> &GetVertexMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(ref v)) => v,
            _ => GetVertexMetadataResponse::default_instance(),
        }
    }

    // .SetVertexMetadataResponse set_vertex_metadata = 13;

    pub fn clear_set_vertex_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_set_vertex_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_vertex_metadata(&mut self, v: SetVertexMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_vertex_metadata(&mut self) -> &mut SetVertexMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(SetVertexMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_vertex_metadata(&mut self) -> SetVertexMetadataResponse {
        if self.has_set_vertex_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetVertexMetadataResponse::new()
        }
    }

    pub fn get_set_vertex_metadata(&self) -> &SetVertexMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(ref v)) => v,
            _ => SetVertexMetadataResponse::default_instance(),
        }
    }

    // .DeleteVertexMetadataResponse delete_vertex_metadata = 14;

    pub fn clear_delete_vertex_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_delete_vertex_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertex_metadata(&mut self, v: DeleteVertexMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertex_metadata(&mut self) -> &mut DeleteVertexMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(DeleteVertexMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertex_metadata(&mut self) -> DeleteVertexMetadataResponse {
        if self.has_delete_vertex_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVertexMetadataResponse::new()
        }
    }

    pub fn get_delete_vertex_metadata(&self) -> &DeleteVertexMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(ref v)) => v,
            _ => DeleteVertexMetadataResponse::default_instance(),
        }
    }

    // .GetEdgeMetadataResponse get_edge_metadata = 15;

    pub fn clear_get_edge_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_get_edge_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_metadata(&mut self, v: GetEdgeMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_metadata(&mut self) -> &mut GetEdgeMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(GetEdgeMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_metadata(&mut self) -> GetEdgeMetadataResponse {
        if self.has_get_edge_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeMetadataResponse::new()
        }
    }

    pub fn get_get_edge_metadata(&self) -> &GetEdgeMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(ref v)) => v,
            _ => GetEdgeMetadataResponse::default_instance(),
        }
    }

    // .SetEdgeMetadataResponse set_edge_metadata = 16;

    pub fn clear_set_edge_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_set_edge_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_edge_metadata(&mut self, v: SetEdgeMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_edge_metadata(&mut self) -> &mut SetEdgeMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(SetEdgeMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_edge_metadata(&mut self) -> SetEdgeMetadataResponse {
        if self.has_set_edge_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetEdgeMetadataResponse::new()
        }
    }

    pub fn get_set_edge_metadata(&self) -> &SetEdgeMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(ref v)) => v,
            _ => SetEdgeMetadataResponse::default_instance(),
        }
    }

    // .DeleteEdgeMetadataResponse delete_edge_metadata = 17;

    pub fn clear_delete_edge_metadata(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_delete_edge_metadata(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edge_metadata(&mut self, v: DeleteEdgeMetadataResponse) {
        self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edge_metadata(&mut self) -> &mut DeleteEdgeMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(DeleteEdgeMetadataResponse::new()));
        }
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edge_metadata(&mut self) -> DeleteEdgeMetadataResponse {
        if self.has_delete_edge_metadata() {
            match self.Response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgeMetadataResponse::new()
        }
    }

    pub fn get_delete_edge_metadata(&self) -> &DeleteEdgeMetadataResponse {
        match self.Response {
            ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(ref v)) => v,
            _ => DeleteEdgeMetadataResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for TransactionResponse {
    fn is_initialized(&self) -> bool {
        if let Some(TransactionResponse_oneof_Response::create_vertex(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_vertices(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::delete_vertices(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_vertex_count(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::create_edge(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_edges(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::delete_edges(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_edge_count(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_global_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::set_global_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::delete_global_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_vertex_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::set_vertex_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::delete_vertex_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::get_edge_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::set_edge_metadata(ref v)) = self.Response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_Response::delete_edge_metadata(ref v)) = self.Response {
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
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::create_vertex(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertices(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertices(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_count(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::create_edge(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edges(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edges(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_count(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_global_metadata(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_global_metadata(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_global_metadata(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_vertex_metadata(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_vertex_metadata(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_vertex_metadata(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::get_edge_metadata(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::set_edge_metadata(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(TransactionResponse_oneof_Response::delete_edge_metadata(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.Response {
            match v {
                &TransactionResponse_oneof_Response::create_vertex(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::delete_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_vertex_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::create_edge(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::delete_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_edge_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::set_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::delete_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::set_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::delete_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::get_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::set_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_Response::delete_edge_metadata(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.Response {
            match v {
                &TransactionResponse_oneof_Response::create_vertex(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_vertices(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::delete_vertices(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_vertex_count(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::create_edge(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_edges(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::delete_edges(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_edge_count(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_global_metadata(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::set_global_metadata(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::delete_global_metadata(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_vertex_metadata(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::set_vertex_metadata(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::delete_vertex_metadata(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::get_edge_metadata(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::set_edge_metadata(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_Response::delete_edge_metadata(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CreateVertexResponse>(
                    "create_vertex",
                    TransactionResponse::has_create_vertex,
                    TransactionResponse::get_create_vertex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetVerticesResponse>(
                    "get_vertices",
                    TransactionResponse::has_get_vertices,
                    TransactionResponse::get_get_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteVerticesResponse>(
                    "delete_vertices",
                    TransactionResponse::has_delete_vertices,
                    TransactionResponse::get_delete_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetVertexCountResponse>(
                    "get_vertex_count",
                    TransactionResponse::has_get_vertex_count,
                    TransactionResponse::get_get_vertex_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CreateEdgeResponse>(
                    "create_edge",
                    TransactionResponse::has_create_edge,
                    TransactionResponse::get_create_edge,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetEdgesResponse>(
                    "get_edges",
                    TransactionResponse::has_get_edges,
                    TransactionResponse::get_get_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteEdgesResponse>(
                    "delete_edges",
                    TransactionResponse::has_delete_edges,
                    TransactionResponse::get_delete_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetEdgeCountResponse>(
                    "get_edge_count",
                    TransactionResponse::has_get_edge_count,
                    TransactionResponse::get_get_edge_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetGlobalMetadataResponse>(
                    "get_global_metadata",
                    TransactionResponse::has_get_global_metadata,
                    TransactionResponse::get_get_global_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetGlobalMetadataResponse>(
                    "set_global_metadata",
                    TransactionResponse::has_set_global_metadata,
                    TransactionResponse::get_set_global_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteGlobalMetadataResponse>(
                    "delete_global_metadata",
                    TransactionResponse::has_delete_global_metadata,
                    TransactionResponse::get_delete_global_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetVertexMetadataResponse>(
                    "get_vertex_metadata",
                    TransactionResponse::has_get_vertex_metadata,
                    TransactionResponse::get_get_vertex_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetVertexMetadataResponse>(
                    "set_vertex_metadata",
                    TransactionResponse::has_set_vertex_metadata,
                    TransactionResponse::get_set_vertex_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteVertexMetadataResponse>(
                    "delete_vertex_metadata",
                    TransactionResponse::has_delete_vertex_metadata,
                    TransactionResponse::get_delete_vertex_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GetEdgeMetadataResponse>(
                    "get_edge_metadata",
                    TransactionResponse::has_get_edge_metadata,
                    TransactionResponse::get_get_edge_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetEdgeMetadataResponse>(
                    "set_edge_metadata",
                    TransactionResponse::has_set_edge_metadata,
                    TransactionResponse::get_set_edge_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteEdgeMetadataResponse>(
                    "delete_edge_metadata",
                    TransactionResponse::has_delete_edge_metadata,
                    TransactionResponse::get_delete_edge_metadata,
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
        self.clear_create_vertex();
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

#[derive(PartialEq,Clone,Default)]
pub struct CreateVertexResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateVertexResponse {}

impl CreateVertexResponse {
    pub fn new() -> CreateVertexResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateVertexResponse {
        static mut instance: ::protobuf::lazy::Lazy<CreateVertexResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateVertexResponse,
        };
        unsafe {
            instance.get(CreateVertexResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for CreateVertexResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for CreateVertexResponse {
    fn new() -> CreateVertexResponse {
        CreateVertexResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateVertexResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CreateVertexResponse::get_error_for_reflect,
                    CreateVertexResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateVertexResponse>(
                    "CreateVertexResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateVertexResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateVertexResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateVertexResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVerticesResponse {
    // message oneof groups
    Response: ::std::option::Option<GetVerticesResponse_oneof_Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVerticesResponse {}

#[derive(Clone,PartialEq)]
pub enum GetVerticesResponse_oneof_Response {
    vertices(Vertices),
    error(::std::string::String),
}

impl GetVerticesResponse {
    pub fn new() -> GetVerticesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVerticesResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetVerticesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVerticesResponse,
        };
        unsafe {
            instance.get(GetVerticesResponse::new)
        }
    }

    // .Vertices vertices = 1;

    pub fn clear_vertices(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_vertices(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: Vertices) {
        self.Response = ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut Vertices {
        if let ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(Vertices::new()));
        }
        match self.Response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vertices(&mut self) -> Vertices {
        if self.has_vertices() {
            match self.Response.take() {
                ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            Vertices::new()
        }
    }

    pub fn get_vertices(&self) -> &Vertices {
        match self.Response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(ref v)) => v,
            _ => Vertices::default_instance(),
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.Response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.Response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.Response = ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(_)) = self.Response {
        } else {
            self.Response = ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(::std::string::String::new()));
        }
        match self.Response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.Response.take() {
                ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.Response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for GetVerticesResponse {
    fn is_initialized(&self) -> bool {
        if let Some(GetVerticesResponse_oneof_Response::vertices(ref v)) = self.Response {
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
                    self.Response = ::std::option::Option::Some(GetVerticesResponse_oneof_Response::vertices(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Response = ::std::option::Option::Some(GetVerticesResponse_oneof_Response::error(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.Response {
            match v {
                &GetVerticesResponse_oneof_Response::vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GetVerticesResponse_oneof_Response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.Response {
            match v {
                &GetVerticesResponse_oneof_Response::vertices(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GetVerticesResponse_oneof_Response::error(ref v) => {
                    os.write_string(2, v)?;
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

impl ::protobuf::MessageStatic for GetVerticesResponse {
    fn new() -> GetVerticesResponse {
        GetVerticesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVerticesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Vertices>(
                    "vertices",
                    GetVerticesResponse::has_vertices,
                    GetVerticesResponse::get_vertices,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetVerticesResponse::has_error,
                    GetVerticesResponse::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVerticesResponse>(
                    "GetVerticesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVerticesResponse {
    fn clear(&mut self) {
        self.clear_vertices();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVerticesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVerticesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteVerticesResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteVerticesResponse {}

impl DeleteVerticesResponse {
    pub fn new() -> DeleteVerticesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteVerticesResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteVerticesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteVerticesResponse,
        };
        unsafe {
            instance.get(DeleteVerticesResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for DeleteVerticesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for DeleteVerticesResponse {
    fn new() -> DeleteVerticesResponse {
        DeleteVerticesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteVerticesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    DeleteVerticesResponse::get_error_for_reflect,
                    DeleteVerticesResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteVerticesResponse>(
                    "DeleteVerticesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteVerticesResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteVerticesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteVerticesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVertexCountResponse {
    // message fields
    pub count: u64,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVertexCountResponse {}

impl GetVertexCountResponse {
    pub fn new() -> GetVertexCountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVertexCountResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetVertexCountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVertexCountResponse,
        };
        unsafe {
            instance.get(GetVertexCountResponse::new)
        }
    }

    // uint64 count = 1;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = v;
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.count
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for GetVertexCountResponse {
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
                    let tmp = is.read_uint64()?;
                    self.count = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.count != 0 {
            os.write_uint64(1, self.count)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for GetVertexCountResponse {
    fn new() -> GetVertexCountResponse {
        GetVertexCountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVertexCountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    GetVertexCountResponse::get_count_for_reflect,
                    GetVertexCountResponse::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetVertexCountResponse::get_error_for_reflect,
                    GetVertexCountResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVertexCountResponse>(
                    "GetVertexCountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVertexCountResponse {
    fn clear(&mut self) {
        self.clear_count();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVertexCountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVertexCountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateEdgeResponse {
    // message fields
    pub created: bool,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEdgeResponse {}

impl CreateEdgeResponse {
    pub fn new() -> CreateEdgeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEdgeResponse {
        static mut instance: ::protobuf::lazy::Lazy<CreateEdgeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEdgeResponse,
        };
        unsafe {
            instance.get(CreateEdgeResponse::new)
        }
    }

    // bool created = 1;

    pub fn clear_created(&mut self) {
        self.created = false;
    }

    // Param is passed by value, moved
    pub fn set_created(&mut self, v: bool) {
        self.created = v;
    }

    pub fn get_created(&self) -> bool {
        self.created
    }

    fn get_created_for_reflect(&self) -> &bool {
        &self.created
    }

    fn mut_created_for_reflect(&mut self) -> &mut bool {
        &mut self.created
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for CreateEdgeResponse {
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
                    self.created = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if self.created != false {
            my_size += 2;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.created != false {
            os.write_bool(1, self.created)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for CreateEdgeResponse {
    fn new() -> CreateEdgeResponse {
        CreateEdgeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEdgeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "created",
                    CreateEdgeResponse::get_created_for_reflect,
                    CreateEdgeResponse::mut_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CreateEdgeResponse::get_error_for_reflect,
                    CreateEdgeResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEdgeResponse>(
                    "CreateEdgeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEdgeResponse {
    fn clear(&mut self) {
        self.clear_created();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEdgeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEdgeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEdgesResponse {
    // message fields
    pub edges: ::protobuf::RepeatedField<Edge>,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgesResponse {}

impl GetEdgesResponse {
    pub fn new() -> GetEdgesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEdgesResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetEdgesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEdgesResponse,
        };
        unsafe {
            instance.get(GetEdgesResponse::new)
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

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for GetEdgesResponse {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
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
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for GetEdgesResponse {
    fn new() -> GetEdgesResponse {
        GetEdgesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEdgesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Edge>>(
                    "edges",
                    GetEdgesResponse::get_edges_for_reflect,
                    GetEdgesResponse::mut_edges_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetEdgesResponse::get_error_for_reflect,
                    GetEdgesResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEdgesResponse>(
                    "GetEdgesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEdgesResponse {
    fn clear(&mut self) {
        self.clear_edges();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEdgesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEdgesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteEdgesResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteEdgesResponse {}

impl DeleteEdgesResponse {
    pub fn new() -> DeleteEdgesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteEdgesResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteEdgesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteEdgesResponse,
        };
        unsafe {
            instance.get(DeleteEdgesResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for DeleteEdgesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for DeleteEdgesResponse {
    fn new() -> DeleteEdgesResponse {
        DeleteEdgesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteEdgesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    DeleteEdgesResponse::get_error_for_reflect,
                    DeleteEdgesResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteEdgesResponse>(
                    "DeleteEdgesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteEdgesResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteEdgesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteEdgesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEdgeCountResponse {
    // message fields
    pub count: u64,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgeCountResponse {}

impl GetEdgeCountResponse {
    pub fn new() -> GetEdgeCountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEdgeCountResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetEdgeCountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEdgeCountResponse,
        };
        unsafe {
            instance.get(GetEdgeCountResponse::new)
        }
    }

    // uint64 count = 1;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = v;
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.count
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for GetEdgeCountResponse {
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
                    let tmp = is.read_uint64()?;
                    self.count = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.count != 0 {
            os.write_uint64(1, self.count)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for GetEdgeCountResponse {
    fn new() -> GetEdgeCountResponse {
        GetEdgeCountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEdgeCountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    GetEdgeCountResponse::get_count_for_reflect,
                    GetEdgeCountResponse::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetEdgeCountResponse::get_error_for_reflect,
                    GetEdgeCountResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEdgeCountResponse>(
                    "GetEdgeCountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEdgeCountResponse {
    fn clear(&mut self) {
        self.clear_count();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEdgeCountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEdgeCountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetGlobalMetadataResponse {
    // message fields
    pub metadata: ::std::string::String,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGlobalMetadataResponse {}

impl GetGlobalMetadataResponse {
    pub fn new() -> GetGlobalMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetGlobalMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetGlobalMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetGlobalMetadataResponse,
        };
        unsafe {
            instance.get(GetGlobalMetadataResponse::new)
        }
    }

    // string metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::string::String {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.metadata, ::std::string::String::new())
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::std::string::String {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.metadata
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for GetGlobalMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.metadata)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.metadata.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.metadata);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.metadata.is_empty() {
            os.write_string(1, &self.metadata)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for GetGlobalMetadataResponse {
    fn new() -> GetGlobalMetadataResponse {
        GetGlobalMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetGlobalMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metadata",
                    GetGlobalMetadataResponse::get_metadata_for_reflect,
                    GetGlobalMetadataResponse::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetGlobalMetadataResponse::get_error_for_reflect,
                    GetGlobalMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetGlobalMetadataResponse>(
                    "GetGlobalMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetGlobalMetadataResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetGlobalMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGlobalMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetGlobalMetadataResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetGlobalMetadataResponse {}

impl SetGlobalMetadataResponse {
    pub fn new() -> SetGlobalMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetGlobalMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetGlobalMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetGlobalMetadataResponse,
        };
        unsafe {
            instance.get(SetGlobalMetadataResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for SetGlobalMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for SetGlobalMetadataResponse {
    fn new() -> SetGlobalMetadataResponse {
        SetGlobalMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetGlobalMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    SetGlobalMetadataResponse::get_error_for_reflect,
                    SetGlobalMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetGlobalMetadataResponse>(
                    "SetGlobalMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetGlobalMetadataResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetGlobalMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetGlobalMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteGlobalMetadataResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteGlobalMetadataResponse {}

impl DeleteGlobalMetadataResponse {
    pub fn new() -> DeleteGlobalMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteGlobalMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteGlobalMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteGlobalMetadataResponse,
        };
        unsafe {
            instance.get(DeleteGlobalMetadataResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for DeleteGlobalMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for DeleteGlobalMetadataResponse {
    fn new() -> DeleteGlobalMetadataResponse {
        DeleteGlobalMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteGlobalMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    DeleteGlobalMetadataResponse::get_error_for_reflect,
                    DeleteGlobalMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteGlobalMetadataResponse>(
                    "DeleteGlobalMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteGlobalMetadataResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteGlobalMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteGlobalMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVertexMetadataResponse {
    // message fields
    pub metadata: ::protobuf::RepeatedField<VertexMetadata>,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVertexMetadataResponse {}

impl GetVertexMetadataResponse {
    pub fn new() -> GetVertexMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVertexMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetVertexMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVertexMetadataResponse,
        };
        unsafe {
            instance.get(GetVertexMetadataResponse::new)
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

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for GetVertexMetadataResponse {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
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
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for GetVertexMetadataResponse {
    fn new() -> GetVertexMetadataResponse {
        GetVertexMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVertexMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexMetadata>>(
                    "metadata",
                    GetVertexMetadataResponse::get_metadata_for_reflect,
                    GetVertexMetadataResponse::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetVertexMetadataResponse::get_error_for_reflect,
                    GetVertexMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVertexMetadataResponse>(
                    "GetVertexMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVertexMetadataResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVertexMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVertexMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetVertexMetadataResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetVertexMetadataResponse {}

impl SetVertexMetadataResponse {
    pub fn new() -> SetVertexMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetVertexMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetVertexMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetVertexMetadataResponse,
        };
        unsafe {
            instance.get(SetVertexMetadataResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for SetVertexMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for SetVertexMetadataResponse {
    fn new() -> SetVertexMetadataResponse {
        SetVertexMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetVertexMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    SetVertexMetadataResponse::get_error_for_reflect,
                    SetVertexMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetVertexMetadataResponse>(
                    "SetVertexMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetVertexMetadataResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetVertexMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetVertexMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteVertexMetadataResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteVertexMetadataResponse {}

impl DeleteVertexMetadataResponse {
    pub fn new() -> DeleteVertexMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteVertexMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteVertexMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteVertexMetadataResponse,
        };
        unsafe {
            instance.get(DeleteVertexMetadataResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for DeleteVertexMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for DeleteVertexMetadataResponse {
    fn new() -> DeleteVertexMetadataResponse {
        DeleteVertexMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteVertexMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    DeleteVertexMetadataResponse::get_error_for_reflect,
                    DeleteVertexMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteVertexMetadataResponse>(
                    "DeleteVertexMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteVertexMetadataResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteVertexMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteVertexMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEdgeMetadataResponse {
    // message fields
    pub metadata: ::protobuf::RepeatedField<VertexMetadata>,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgeMetadataResponse {}

impl GetEdgeMetadataResponse {
    pub fn new() -> GetEdgeMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEdgeMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetEdgeMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEdgeMetadataResponse,
        };
        unsafe {
            instance.get(GetEdgeMetadataResponse::new)
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

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for GetEdgeMetadataResponse {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
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
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for GetEdgeMetadataResponse {
    fn new() -> GetEdgeMetadataResponse {
        GetEdgeMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEdgeMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VertexMetadata>>(
                    "metadata",
                    GetEdgeMetadataResponse::get_metadata_for_reflect,
                    GetEdgeMetadataResponse::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetEdgeMetadataResponse::get_error_for_reflect,
                    GetEdgeMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEdgeMetadataResponse>(
                    "GetEdgeMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEdgeMetadataResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEdgeMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEdgeMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetEdgeMetadataResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetEdgeMetadataResponse {}

impl SetEdgeMetadataResponse {
    pub fn new() -> SetEdgeMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetEdgeMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetEdgeMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetEdgeMetadataResponse,
        };
        unsafe {
            instance.get(SetEdgeMetadataResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for SetEdgeMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for SetEdgeMetadataResponse {
    fn new() -> SetEdgeMetadataResponse {
        SetEdgeMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetEdgeMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    SetEdgeMetadataResponse::get_error_for_reflect,
                    SetEdgeMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetEdgeMetadataResponse>(
                    "SetEdgeMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetEdgeMetadataResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetEdgeMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetEdgeMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteEdgeMetadataResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteEdgeMetadataResponse {}

impl DeleteEdgeMetadataResponse {
    pub fn new() -> DeleteEdgeMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteEdgeMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteEdgeMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteEdgeMetadataResponse,
        };
        unsafe {
            instance.get(DeleteEdgeMetadataResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for DeleteEdgeMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

impl ::protobuf::MessageStatic for DeleteEdgeMetadataResponse {
    fn new() -> DeleteEdgeMetadataResponse {
        DeleteEdgeMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteEdgeMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    DeleteEdgeMetadataResponse::get_error_for_reflect,
                    DeleteEdgeMetadataResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteEdgeMetadataResponse>(
                    "DeleteEdgeMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteEdgeMetadataResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteEdgeMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteEdgeMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VertexMetadata {
    // message fields
    pub id: ::std::string::String,
    pub metadata: ::std::string::String,
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

    // string metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::string::String {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.metadata, ::std::string::String::new())
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::std::string::String {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.metadata
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.metadata)?;
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
        if !self.metadata.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.metadata);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.metadata.is_empty() {
            os.write_string(2, &self.metadata)?;
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
                    "metadata",
                    VertexMetadata::get_metadata_for_reflect,
                    VertexMetadata::mut_metadata_for_reflect,
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
        self.clear_metadata();
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
pub struct EdgeMetadata {
    // message fields
    pub key: ::protobuf::SingularPtrField<Edge>,
    pub metadata: ::std::string::String,
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

    // .Edge key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: Edge) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut Edge {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> Edge {
        self.key.take().unwrap_or_else(|| Edge::new())
    }

    pub fn get_key(&self) -> &Edge {
        self.key.as_ref().unwrap_or_else(|| Edge::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<Edge> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Edge> {
        &mut self.key
    }

    // string metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::string::String {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.metadata, ::std::string::String::new())
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::std::string::String {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.metadata
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.metadata)?;
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
        if !self.metadata.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.metadata);
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
        if !self.metadata.is_empty() {
            os.write_string(2, &self.metadata)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Edge>>(
                    "key",
                    EdgeMetadata::get_key_for_reflect,
                    EdgeMetadata::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metadata",
                    EdgeMetadata::get_metadata_for_reflect,
                    EdgeMetadata::mut_metadata_for_reflect,
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
        self.clear_metadata();
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11bin/indradb.proto\"\xbb\t\n\x12TransactionRequest\x12;\n\rcreate_v\
    ertex\x18\x01\x20\x01(\x0b2\x14.CreateVertexRequestH\0R\x0ccreateVertex\
    \x128\n\x0cget_vertices\x18\x02\x20\x01(\x0b2\x13.GetVerticesRequestH\0R\
    \x0bgetVertices\x12A\n\x0fdelete_vertices\x18\x03\x20\x01(\x0b2\x16.Dele\
    teVerticesRequestH\0R\x0edeleteVertices\x12B\n\x10get_vertex_count\x18\
    \x04\x20\x01(\x0b2\x16.GetVertexCountRequestH\0R\x0egetVertexCount\x125\
    \n\x0bcreate_edge\x18\x05\x20\x01(\x0b2\x12.CreateEdgeRequestH\0R\ncreat\
    eEdge\x12/\n\tget_edges\x18\x06\x20\x01(\x0b2\x10.GetEdgesRequestH\0R\
    \x08getEdges\x128\n\x0cdelete_edges\x18\x07\x20\x01(\x0b2\x13.DeleteEdge\
    sRequestH\0R\x0bdeleteEdges\x12<\n\x0eget_edge_count\x18\x08\x20\x01(\
    \x0b2\x14.GetEdgeCountRequestH\0R\x0cgetEdgeCount\x12K\n\x13get_global_m\
    etadata\x18\t\x20\x01(\x0b2\x19.GetGlobalMetadataRequestH\0R\x11getGloba\
    lMetadata\x12K\n\x13set_global_metadata\x18\n\x20\x01(\x0b2\x19.SetGloba\
    lMetadataRequestH\0R\x11setGlobalMetadata\x12T\n\x16delete_global_metada\
    ta\x18\x0b\x20\x01(\x0b2\x1c.DeleteGlobalMetadataRequestH\0R\x14deleteGl\
    obalMetadata\x12K\n\x13get_vertex_metadata\x18\x0c\x20\x01(\x0b2\x19.Get\
    VertexMetadataRequestH\0R\x11getVertexMetadata\x12K\n\x13set_vertex_meta\
    data\x18\r\x20\x01(\x0b2\x19.SetVertexMetadataRequestH\0R\x11setVertexMe\
    tadata\x12T\n\x16delete_vertex_metadata\x18\x0e\x20\x01(\x0b2\x1c.Delete\
    VertexMetadataRequestH\0R\x14deleteVertexMetadata\x12E\n\x11get_edge_met\
    adata\x18\x0f\x20\x01(\x0b2\x17.GetEdgeMetadataRequestH\0R\x0fgetEdgeMet\
    adata\x12E\n\x11set_edge_metadata\x18\x10\x20\x01(\x0b2\x17.SetEdgeMetad\
    ataRequestH\0R\x0fsetEdgeMetadata\x12N\n\x14delete_edge_metadata\x18\x11\
    \x20\x01(\x0b2\x1a.DeleteEdgeMetadataRequestH\0R\x12deleteEdgeMetadataB\
    \t\n\x07Request\"6\n\x13CreateVertexRequest\x12\x1f\n\x06vertex\x18\x01\
    \x20\x01(\x0b2\x07.VertexR\x06vertex\"8\n\x12GetVerticesRequest\x12\"\n\
    \x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\x05query\";\n\x15Delete\
    VerticesRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\
    \x05query\"\x17\n\x15GetVertexCountRequest\"/\n\x11CreateEdgeRequest\x12\
    \x1a\n\x03key\x18\x01\x20\x01(\x0b2\x08.EdgeKeyR\x03key\"3\n\x0fGetEdges\
    Request\x12\x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\"\
    6\n\x12DeleteEdgesRequest\x12\x20\n\x05query\x18\x01\x20\x01(\x0b2\n.Edg\
    eQueryR\x05query\"d\n\x13GetEdgeCountRequest\x12\x0e\n\x02id\x18\x01\x20\
    \x01(\tR\x02id\x12\x1f\n\x0btype_filter\x18\x02\x20\x01(\tR\ntypeFilter\
    \x12\x1c\n\tdirection\x18\x03\x20\x01(\tR\tdirection\".\n\x18GetGlobalMe\
    tadataRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"D\n\x18SetG\
    lobalMetadataRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\
    \x14\n\x05value\x18\x02\x20\x01(\tR\x05value\"1\n\x1bDeleteGlobalMetadat\
    aRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"R\n\x18GetVertex\
    MetadataRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\
    \x05query\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"h\n\x18SetVerte\
    xMetadataRequest\x12\"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\
    \x05query\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05val\
    ue\x18\x03\x20\x01(\tR\x05value\"U\n\x1bDeleteVertexMetadataRequest\x12\
    \"\n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\x05query\x12\x12\n\
    \x04name\x18\x02\x20\x01(\tR\x04name\"N\n\x16GetEdgeMetadataRequest\x12\
    \x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\x12\x12\n\
    \x04name\x18\x02\x20\x01(\tR\x04name\"d\n\x16SetEdgeMetadataRequest\x12\
    \x20\n\x05query\x18\x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\x12\x12\n\
    \x04name\x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x03\x20\x01(\
    \tR\x05value\"Q\n\x19DeleteEdgeMetadataRequest\x12\x20\n\x05query\x18\
    \x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\x12\x12\n\x04name\x18\x02\x20\
    \x01(\tR\x04name\",\n\x06Vertex\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02i\
    d\x12\x12\n\x04type\x18\x02\x20\x01(\tR\x04type\"/\n\x08Vertices\x12#\n\
    \x08vertices\x18\x01\x20\x03(\x0b2\x07.VertexR\x08vertices\"6\n\x04Edge\
    \x12\x1a\n\x03key\x18\x01\x20\x01(\x0b2\x08.EdgeKeyR\x03key\x12\x12\n\
    \x04type\x18\x02\x20\x01(\tR\x04type\"]\n\x07EdgeKey\x12\x1f\n\x0boutbou\
    nd_id\x18\x01\x20\x01(\tR\noutboundId\x12\x12\n\x04type\x18\x02\x20\x01(\
    \tR\x04type\x12\x1d\n\ninbound_id\x18\x03\x20\x01(\tR\tinboundId\"\x97\
    \x01\n\x0bVertexQuery\x12#\n\x03all\x18\x01\x20\x01(\x0b2\x0f.AllVertexQ\
    ueryH\0R\x03all\x122\n\x08vertices\x18\x02\x20\x01(\x0b2\x14.VerticesVer\
    texQueryH\0R\x08vertices\x12&\n\x04pipe\x18\x03\x20\x01(\x0b2\x10.PipeVe\
    rtexQueryH\0R\x04pipeB\x07\n\x05Query\"A\n\x0eAllVertexQuery\x12\x19\n\
    \x08start_id\x18\x01\x20\x01(\tR\x07startId\x12\x14\n\x05limit\x18\x02\
    \x20\x01(\rR\x05limit\"'\n\x13VerticesVertexQuery\x12\x10\n\x03ids\x18\
    \x01\x20\x03(\tR\x03ids\"g\n\x0fPipeVertexQuery\x12\x20\n\x05query\x18\
    \x01\x20\x01(\x0b2\n.EdgeQueryR\x05query\x12\x1c\n\tconverter\x18\x02\
    \x20\x01(\tR\tconverter\x12\x14\n\x05limit\x18\x03\x20\x01(\rR\x05limit\
    \"c\n\tEdgeQuery\x12'\n\x05edges\x18\x01\x20\x01(\x0b2\x0f.EdgesEdgeQuer\
    yH\0R\x05edges\x12$\n\x04pipe\x18\x02\x20\x01(\x0b2\x0e.PipeEdgeQueryH\0\
    R\x04pipeB\x07\n\x05Query\".\n\x0eEdgesEdgeQuery\x12\x1c\n\x04keys\x18\
    \x01\x20\x03(\x0b2\x08.EdgeKeyR\x04keys\"\xc8\x01\n\rPipeEdgeQuery\x12\"\
    \n\x05query\x18\x01\x20\x01(\x0b2\x0c.VertexQueryR\x05query\x12\x1c\n\tc\
    onverter\x18\x02\x20\x01(\tR\tconverter\x12\x1f\n\x0btype_filter\x18\x03\
    \x20\x01(\tR\ntypeFilter\x12\x1f\n\x0bhigh_filter\x18\x04\x20\x01(\x04R\
    \nhighFilter\x12\x1d\n\nlow_filter\x18\x05\x20\x01(\x04R\tlowFilter\x12\
    \x14\n\x05limit\x18\x06\x20\x01(\rR\x05limit\"\xce\t\n\x13TransactionRes\
    ponse\x12<\n\rcreate_vertex\x18\x01\x20\x01(\x0b2\x15.CreateVertexRespon\
    seH\0R\x0ccreateVertex\x129\n\x0cget_vertices\x18\x02\x20\x01(\x0b2\x14.\
    GetVerticesResponseH\0R\x0bgetVertices\x12B\n\x0fdelete_vertices\x18\x03\
    \x20\x01(\x0b2\x17.DeleteVerticesResponseH\0R\x0edeleteVertices\x12C\n\
    \x10get_vertex_count\x18\x04\x20\x01(\x0b2\x17.GetVertexCountResponseH\0\
    R\x0egetVertexCount\x126\n\x0bcreate_edge\x18\x05\x20\x01(\x0b2\x13.Crea\
    teEdgeResponseH\0R\ncreateEdge\x120\n\tget_edges\x18\x06\x20\x01(\x0b2\
    \x11.GetEdgesResponseH\0R\x08getEdges\x129\n\x0cdelete_edges\x18\x07\x20\
    \x01(\x0b2\x14.DeleteEdgesResponseH\0R\x0bdeleteEdges\x12=\n\x0eget_edge\
    _count\x18\x08\x20\x01(\x0b2\x15.GetEdgeCountResponseH\0R\x0cgetEdgeCoun\
    t\x12L\n\x13get_global_metadata\x18\t\x20\x01(\x0b2\x1a.GetGlobalMetadat\
    aResponseH\0R\x11getGlobalMetadata\x12L\n\x13set_global_metadata\x18\n\
    \x20\x01(\x0b2\x1a.SetGlobalMetadataResponseH\0R\x11setGlobalMetadata\
    \x12U\n\x16delete_global_metadata\x18\x0b\x20\x01(\x0b2\x1d.DeleteGlobal\
    MetadataResponseH\0R\x14deleteGlobalMetadata\x12L\n\x13get_vertex_metada\
    ta\x18\x0c\x20\x01(\x0b2\x1a.GetVertexMetadataResponseH\0R\x11getVertexM\
    etadata\x12L\n\x13set_vertex_metadata\x18\r\x20\x01(\x0b2\x1a.SetVertexM\
    etadataResponseH\0R\x11setVertexMetadata\x12U\n\x16delete_vertex_metadat\
    a\x18\x0e\x20\x01(\x0b2\x1d.DeleteVertexMetadataResponseH\0R\x14deleteVe\
    rtexMetadata\x12F\n\x11get_edge_metadata\x18\x0f\x20\x01(\x0b2\x18.GetEd\
    geMetadataResponseH\0R\x0fgetEdgeMetadata\x12F\n\x11set_edge_metadata\
    \x18\x10\x20\x01(\x0b2\x18.SetEdgeMetadataResponseH\0R\x0fsetEdgeMetadat\
    a\x12O\n\x14delete_edge_metadata\x18\x11\x20\x01(\x0b2\x1b.DeleteEdgeMet\
    adataResponseH\0R\x12deleteEdgeMetadataB\n\n\x08Response\",\n\x14CreateV\
    ertexResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"b\n\x13Ge\
    tVerticesResponse\x12'\n\x08vertices\x18\x01\x20\x01(\x0b2\t.VerticesH\0\
    R\x08vertices\x12\x16\n\x05error\x18\x02\x20\x01(\tH\0R\x05errorB\n\n\
    \x08Response\".\n\x16DeleteVerticesResponse\x12\x14\n\x05error\x18\x01\
    \x20\x01(\tR\x05error\"D\n\x16GetVertexCountResponse\x12\x14\n\x05count\
    \x18\x01\x20\x01(\x04R\x05count\x12\x14\n\x05error\x18\x02\x20\x01(\tR\
    \x05error\"D\n\x12CreateEdgeResponse\x12\x18\n\x07created\x18\x01\x20\
    \x01(\x08R\x07created\x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05error\"E\
    \n\x10GetEdgesResponse\x12\x1b\n\x05edges\x18\x01\x20\x03(\x0b2\x05.Edge\
    R\x05edges\x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05error\"+\n\x13Delet\
    eEdgesResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"B\n\x14G\
    etEdgeCountResponse\x12\x14\n\x05count\x18\x01\x20\x01(\x04R\x05count\
    \x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05error\"M\n\x19GetGlobalMetada\
    taResponse\x12\x1a\n\x08metadata\x18\x01\x20\x01(\tR\x08metadata\x12\x14\
    \n\x05error\x18\x02\x20\x01(\tR\x05error\"1\n\x19SetGlobalMetadataRespon\
    se\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"4\n\x1cDeleteGlobalM\
    etadataResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"^\n\x19\
    GetVertexMetadataResponse\x12+\n\x08metadata\x18\x01\x20\x03(\x0b2\x0f.V\
    ertexMetadataR\x08metadata\x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05err\
    or\"1\n\x19SetVertexMetadataResponse\x12\x14\n\x05error\x18\x01\x20\x01(\
    \tR\x05error\"4\n\x1cDeleteVertexMetadataResponse\x12\x14\n\x05error\x18\
    \x01\x20\x01(\tR\x05error\"\\\n\x17GetEdgeMetadataResponse\x12+\n\x08met\
    adata\x18\x01\x20\x03(\x0b2\x0f.VertexMetadataR\x08metadata\x12\x14\n\
    \x05error\x18\x02\x20\x01(\tR\x05error\"/\n\x17SetEdgeMetadataResponse\
    \x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"2\n\x1aDeleteEdgeMetad\
    ataResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"<\n\x0eVert\
    exMetadata\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x1a\n\x08metada\
    ta\x18\x02\x20\x01(\tR\x08metadata\"C\n\x0cEdgeMetadata\x12\x17\n\x03key\
    \x18\x01\x20\x01(\x0b2\x05.EdgeR\x03key\x12\x1a\n\x08metadata\x18\x02\
    \x20\x01(\tR\x08metadata2G\n\x07IndraDB\x12<\n\x0bTransaction\x12\x13.Tr\
    ansactionRequest\x1a\x14.TransactionResponse(\x010\x01J\x90I\n\x07\x12\
    \x05\0\0\x95\x02\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x06\0\x12\
    \x04\x02\0\x04\x01\n\n\n\x03\x06\0\x01\x12\x03\x02\x08\x0f\n\x0b\n\x04\
    \x06\0\x02\0\x12\x03\x03\x04U\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x03\
    \x08\x13\n\x0c\n\x05\x06\0\x02\0\x05\x12\x03\x03\x15\x1b\n\x0c\n\x05\x06\
    \0\x02\0\x02\x12\x03\x03\x1c.\n\x0c\n\x05\x06\0\x02\0\x06\x12\x03\x039?\
    \n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x03@S\n\n\n\x02\x04\0\x12\x04\x06\0\
    \x1a\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x1a\n\x0c\n\x04\x04\0\x08\0\
    \x12\x04\x07\x04\x19\x05\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x07\n\x11\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x08\x08.\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x08\x08\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\x1c)\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x08,-\n\x0b\n\x04\x04\0\x02\x01\x12\x03\t\
    \x08,\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\t\x08\x1a\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\t\x1b'\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\t*+\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\n\x082\n\x0c\n\x05\x04\0\x02\x02\x06\
    \x12\x03\n\x08\x1d\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\n\x1e-\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\n01\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0b\
    \x083\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x0b\x08\x1d\n\x0c\n\x05\x04\
    \0\x02\x03\x01\x12\x03\x0b\x1e.\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\
    \x0b12\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x0c\x08*\n\x0c\n\x05\x04\0\x02\
    \x04\x06\x12\x03\x0c\x08\x19\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x0c\
    \x1a%\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0c()\n\x0b\n\x04\x04\0\x02\
    \x05\x12\x03\r\x08&\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03\r\x08\x17\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03\r\x18!\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03\r$%\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x0e\x08,\n\x0c\n\x05\
    \x04\0\x02\x06\x06\x12\x03\x0e\x08\x1a\n\x0c\n\x05\x04\0\x02\x06\x01\x12\
    \x03\x0e\x1b'\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0e*+\n\x0b\n\x04\
    \x04\0\x02\x07\x12\x03\x0f\x08/\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03\
    \x0f\x08\x1b\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x0f\x1c*\n\x0c\n\x05\
    \x04\0\x02\x07\x03\x12\x03\x0f-.\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x10\
    \x089\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03\x10\x08\x20\n\x0c\n\x05\x04\
    \0\x02\x08\x01\x12\x03\x10!4\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x1078\
    \n\x0b\n\x04\x04\0\x02\t\x12\x03\x11\x08:\n\x0c\n\x05\x04\0\x02\t\x06\
    \x12\x03\x11\x08\x20\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03\x11!4\n\x0c\n\
    \x05\x04\0\x02\t\x03\x12\x03\x1179\n\x0b\n\x04\x04\0\x02\n\x12\x03\x12\
    \x08@\n\x0c\n\x05\x04\0\x02\n\x06\x12\x03\x12\x08#\n\x0c\n\x05\x04\0\x02\
    \n\x01\x12\x03\x12$:\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\x12=?\n\x0b\n\
    \x04\x04\0\x02\x0b\x12\x03\x13\x08:\n\x0c\n\x05\x04\0\x02\x0b\x06\x12\
    \x03\x13\x08\x20\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03\x13!4\n\x0c\n\x05\
    \x04\0\x02\x0b\x03\x12\x03\x1379\n\x0b\n\x04\x04\0\x02\x0c\x12\x03\x14\
    \x08:\n\x0c\n\x05\x04\0\x02\x0c\x06\x12\x03\x14\x08\x20\n\x0c\n\x05\x04\
    \0\x02\x0c\x01\x12\x03\x14!4\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03\x1479\
    \n\x0b\n\x04\x04\0\x02\r\x12\x03\x15\x08@\n\x0c\n\x05\x04\0\x02\r\x06\
    \x12\x03\x15\x08#\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03\x15$:\n\x0c\n\x05\
    \x04\0\x02\r\x03\x12\x03\x15=?\n\x0b\n\x04\x04\0\x02\x0e\x12\x03\x16\x08\
    6\n\x0c\n\x05\x04\0\x02\x0e\x06\x12\x03\x16\x08\x1e\n\x0c\n\x05\x04\0\
    \x02\x0e\x01\x12\x03\x16\x1f0\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03\x163\
    5\n\x0b\n\x04\x04\0\x02\x0f\x12\x03\x17\x086\n\x0c\n\x05\x04\0\x02\x0f\
    \x06\x12\x03\x17\x08\x1e\n\x0c\n\x05\x04\0\x02\x0f\x01\x12\x03\x17\x1f0\
    \n\x0c\n\x05\x04\0\x02\x0f\x03\x12\x03\x1735\n\x0b\n\x04\x04\0\x02\x10\
    \x12\x03\x18\x08<\n\x0c\n\x05\x04\0\x02\x10\x06\x12\x03\x18\x08!\n\x0c\n\
    \x05\x04\0\x02\x10\x01\x12\x03\x18\"6\n\x0c\n\x05\x04\0\x02\x10\x03\x12\
    \x03\x189;\n\n\n\x02\x04\x01\x12\x04\x1c\0\x1e\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x1c\x08\x1b\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1d\x04\x16\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x1d\x04\x1c\x1d\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03\x1d\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1d\x0b\x11\
    \n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1d\x14\x15\n\n\n\x02\x04\x02\x12\
    \x04\x20\0\"\x01\n\n\n\x03\x04\x02\x01\x12\x03\x20\x08\x1a\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03!\x04\x1a\n\r\n\x05\x04\x02\x02\0\x04\x12\x04!\x04\
    \x20\x1c\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03!\x04\x0f\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03!\x10\x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03!\
    \x18\x19\n\n\n\x02\x04\x03\x12\x04$\0&\x01\n\n\n\x03\x04\x03\x01\x12\x03\
    $\x08\x1d\n\x0b\n\x04\x04\x03\x02\0\x12\x03%\x04\x1a\n\r\n\x05\x04\x03\
    \x02\0\x04\x12\x04%\x04$\x1f\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03%\x04\
    \x0f\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03%\x10\x15\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03%\x18\x19\n\t\n\x02\x04\x04\x12\x03(\0!\n\n\n\x03\x04\
    \x04\x01\x12\x03(\x08\x1d\n\n\n\x02\x04\x05\x12\x04*\0,\x01\n\n\n\x03\
    \x04\x05\x01\x12\x03*\x08\x19\n\x0b\n\x04\x04\x05\x02\0\x12\x03+\x04\x14\
    \n\r\n\x05\x04\x05\x02\0\x04\x12\x04+\x04*\x1b\n\x0c\n\x05\x04\x05\x02\0\
    \x06\x12\x03+\x04\x0b\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03+\x0c\x0f\n\
    \x0c\n\x05\x04\x05\x02\0\x03\x12\x03+\x12\x13\n\n\n\x02\x04\x06\x12\x04.\
    \00\x01\n\n\n\x03\x04\x06\x01\x12\x03.\x08\x17\n\x0b\n\x04\x04\x06\x02\0\
    \x12\x03/\x04\x18\n\r\n\x05\x04\x06\x02\0\x04\x12\x04/\x04.\x19\n\x0c\n\
    \x05\x04\x06\x02\0\x06\x12\x03/\x04\r\n\x0c\n\x05\x04\x06\x02\0\x01\x12\
    \x03/\x0e\x13\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03/\x16\x17\n\n\n\x02\
    \x04\x07\x12\x042\04\x01\n\n\n\x03\x04\x07\x01\x12\x032\x08\x1a\n\x0b\n\
    \x04\x04\x07\x02\0\x12\x033\x04\x18\n\r\n\x05\x04\x07\x02\0\x04\x12\x043\
    \x042\x1c\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x033\x04\r\n\x0c\n\x05\x04\
    \x07\x02\0\x01\x12\x033\x0e\x13\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x033\
    \x16\x17\n\n\n\x02\x04\x08\x12\x046\0:\x01\n\n\n\x03\x04\x08\x01\x12\x03\
    6\x08\x1b\n\x0b\n\x04\x04\x08\x02\0\x12\x037\x04\x12\n\r\n\x05\x04\x08\
    \x02\0\x04\x12\x047\x046\x1d\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x037\x04\
    \n\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x037\x0b\r\n\x0c\n\x05\x04\x08\x02\
    \0\x03\x12\x037\x10\x11\n\x0b\n\x04\x04\x08\x02\x01\x12\x038\x04\x1b\n\r\
    \n\x05\x04\x08\x02\x01\x04\x12\x048\x047\x12\n\x0c\n\x05\x04\x08\x02\x01\
    \x05\x12\x038\x04\n\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x038\x0b\x16\n\
    \x0c\n\x05\x04\x08\x02\x01\x03\x12\x038\x19\x1a\n\x0b\n\x04\x04\x08\x02\
    \x02\x12\x039\x04\x19\n\r\n\x05\x04\x08\x02\x02\x04\x12\x049\x048\x1b\n\
    \x0c\n\x05\x04\x08\x02\x02\x05\x12\x039\x04\n\n\x0c\n\x05\x04\x08\x02\
    \x02\x01\x12\x039\x0b\x14\n\x0c\n\x05\x04\x08\x02\x02\x03\x12\x039\x17\
    \x18\n\n\n\x02\x04\t\x12\x04<\0>\x01\n\n\n\x03\x04\t\x01\x12\x03<\x08\
    \x20\n\x0b\n\x04\x04\t\x02\0\x12\x03=\x04\x14\n\r\n\x05\x04\t\x02\0\x04\
    \x12\x04=\x04<\"\n\x0c\n\x05\x04\t\x02\0\x05\x12\x03=\x04\n\n\x0c\n\x05\
    \x04\t\x02\0\x01\x12\x03=\x0b\x0f\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03=\
    \x12\x13\n\n\n\x02\x04\n\x12\x04@\0C\x01\n\n\n\x03\x04\n\x01\x12\x03@\
    \x08\x20\n\x0b\n\x04\x04\n\x02\0\x12\x03A\x04\x14\n\r\n\x05\x04\n\x02\0\
    \x04\x12\x04A\x04@\"\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03A\x04\n\n\x0c\n\
    \x05\x04\n\x02\0\x01\x12\x03A\x0b\x0f\n\x0c\n\x05\x04\n\x02\0\x03\x12\
    \x03A\x12\x13\n\x0b\n\x04\x04\n\x02\x01\x12\x03B\x04\x15\n\r\n\x05\x04\n\
    \x02\x01\x04\x12\x04B\x04A\x14\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03B\
    \x04\n\n\x0c\n\x05\x04\n\x02\x01\x01\x12\x03B\x0b\x10\n\x0c\n\x05\x04\n\
    \x02\x01\x03\x12\x03B\x13\x14\n\n\n\x02\x04\x0b\x12\x04E\0G\x01\n\n\n\
    \x03\x04\x0b\x01\x12\x03E\x08#\n\x0b\n\x04\x04\x0b\x02\0\x12\x03F\x04\
    \x14\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04F\x04E%\n\x0c\n\x05\x04\x0b\x02\
    \0\x05\x12\x03F\x04\n\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03F\x0b\x0f\n\
    \x0c\n\x05\x04\x0b\x02\0\x03\x12\x03F\x12\x13\n\n\n\x02\x04\x0c\x12\x04I\
    \0L\x01\n\n\n\x03\x04\x0c\x01\x12\x03I\x08\x20\n\x0b\n\x04\x04\x0c\x02\0\
    \x12\x03J\x04\x1a\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04J\x04I\"\n\x0c\n\
    \x05\x04\x0c\x02\0\x06\x12\x03J\x04\x0f\n\x0c\n\x05\x04\x0c\x02\0\x01\
    \x12\x03J\x10\x15\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\x03J\x18\x19\n\x0b\n\
    \x04\x04\x0c\x02\x01\x12\x03K\x04\x14\n\r\n\x05\x04\x0c\x02\x01\x04\x12\
    \x04K\x04J\x1a\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\x03K\x04\n\n\x0c\n\
    \x05\x04\x0c\x02\x01\x01\x12\x03K\x0b\x0f\n\x0c\n\x05\x04\x0c\x02\x01\
    \x03\x12\x03K\x12\x13\n\n\n\x02\x04\r\x12\x04N\0R\x01\n\n\n\x03\x04\r\
    \x01\x12\x03N\x08\x20\n\x0b\n\x04\x04\r\x02\0\x12\x03O\x04\x1a\n\r\n\x05\
    \x04\r\x02\0\x04\x12\x04O\x04N\"\n\x0c\n\x05\x04\r\x02\0\x06\x12\x03O\
    \x04\x0f\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03O\x10\x15\n\x0c\n\x05\x04\r\
    \x02\0\x03\x12\x03O\x18\x19\n\x0b\n\x04\x04\r\x02\x01\x12\x03P\x04\x14\n\
    \r\n\x05\x04\r\x02\x01\x04\x12\x04P\x04O\x1a\n\x0c\n\x05\x04\r\x02\x01\
    \x05\x12\x03P\x04\n\n\x0c\n\x05\x04\r\x02\x01\x01\x12\x03P\x0b\x0f\n\x0c\
    \n\x05\x04\r\x02\x01\x03\x12\x03P\x12\x13\n\x0b\n\x04\x04\r\x02\x02\x12\
    \x03Q\x04\x15\n\r\n\x05\x04\r\x02\x02\x04\x12\x04Q\x04P\x14\n\x0c\n\x05\
    \x04\r\x02\x02\x05\x12\x03Q\x04\n\n\x0c\n\x05\x04\r\x02\x02\x01\x12\x03Q\
    \x0b\x10\n\x0c\n\x05\x04\r\x02\x02\x03\x12\x03Q\x13\x14\n\n\n\x02\x04\
    \x0e\x12\x04T\0W\x01\n\n\n\x03\x04\x0e\x01\x12\x03T\x08#\n\x0b\n\x04\x04\
    \x0e\x02\0\x12\x03U\x04\x1a\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04U\x04T%\n\
    \x0c\n\x05\x04\x0e\x02\0\x06\x12\x03U\x04\x0f\n\x0c\n\x05\x04\x0e\x02\0\
    \x01\x12\x03U\x10\x15\n\x0c\n\x05\x04\x0e\x02\0\x03\x12\x03U\x18\x19\n\
    \x0b\n\x04\x04\x0e\x02\x01\x12\x03V\x04\x14\n\r\n\x05\x04\x0e\x02\x01\
    \x04\x12\x04V\x04U\x1a\n\x0c\n\x05\x04\x0e\x02\x01\x05\x12\x03V\x04\n\n\
    \x0c\n\x05\x04\x0e\x02\x01\x01\x12\x03V\x0b\x0f\n\x0c\n\x05\x04\x0e\x02\
    \x01\x03\x12\x03V\x12\x13\n\n\n\x02\x04\x0f\x12\x04Y\0\\\x01\n\n\n\x03\
    \x04\x0f\x01\x12\x03Y\x08\x1e\n\x0b\n\x04\x04\x0f\x02\0\x12\x03Z\x04\x18\
    \n\r\n\x05\x04\x0f\x02\0\x04\x12\x04Z\x04Y\x20\n\x0c\n\x05\x04\x0f\x02\0\
    \x06\x12\x03Z\x04\r\n\x0c\n\x05\x04\x0f\x02\0\x01\x12\x03Z\x0e\x13\n\x0c\
    \n\x05\x04\x0f\x02\0\x03\x12\x03Z\x16\x17\n\x0b\n\x04\x04\x0f\x02\x01\
    \x12\x03[\x04\x14\n\r\n\x05\x04\x0f\x02\x01\x04\x12\x04[\x04Z\x18\n\x0c\
    \n\x05\x04\x0f\x02\x01\x05\x12\x03[\x04\n\n\x0c\n\x05\x04\x0f\x02\x01\
    \x01\x12\x03[\x0b\x0f\n\x0c\n\x05\x04\x0f\x02\x01\x03\x12\x03[\x12\x13\n\
    \n\n\x02\x04\x10\x12\x04^\0b\x01\n\n\n\x03\x04\x10\x01\x12\x03^\x08\x1e\
    \n\x0b\n\x04\x04\x10\x02\0\x12\x03_\x04\x18\n\r\n\x05\x04\x10\x02\0\x04\
    \x12\x04_\x04^\x20\n\x0c\n\x05\x04\x10\x02\0\x06\x12\x03_\x04\r\n\x0c\n\
    \x05\x04\x10\x02\0\x01\x12\x03_\x0e\x13\n\x0c\n\x05\x04\x10\x02\0\x03\
    \x12\x03_\x16\x17\n\x0b\n\x04\x04\x10\x02\x01\x12\x03`\x04\x14\n\r\n\x05\
    \x04\x10\x02\x01\x04\x12\x04`\x04_\x18\n\x0c\n\x05\x04\x10\x02\x01\x05\
    \x12\x03`\x04\n\n\x0c\n\x05\x04\x10\x02\x01\x01\x12\x03`\x0b\x0f\n\x0c\n\
    \x05\x04\x10\x02\x01\x03\x12\x03`\x12\x13\n\x0b\n\x04\x04\x10\x02\x02\
    \x12\x03a\x04\x15\n\r\n\x05\x04\x10\x02\x02\x04\x12\x04a\x04`\x14\n\x0c\
    \n\x05\x04\x10\x02\x02\x05\x12\x03a\x04\n\n\x0c\n\x05\x04\x10\x02\x02\
    \x01\x12\x03a\x0b\x10\n\x0c\n\x05\x04\x10\x02\x02\x03\x12\x03a\x13\x14\n\
    \n\n\x02\x04\x11\x12\x04d\0g\x01\n\n\n\x03\x04\x11\x01\x12\x03d\x08!\n\
    \x0b\n\x04\x04\x11\x02\0\x12\x03e\x04\x18\n\r\n\x05\x04\x11\x02\0\x04\
    \x12\x04e\x04d#\n\x0c\n\x05\x04\x11\x02\0\x06\x12\x03e\x04\r\n\x0c\n\x05\
    \x04\x11\x02\0\x01\x12\x03e\x0e\x13\n\x0c\n\x05\x04\x11\x02\0\x03\x12\
    \x03e\x16\x17\n\x0b\n\x04\x04\x11\x02\x01\x12\x03f\x04\x14\n\r\n\x05\x04\
    \x11\x02\x01\x04\x12\x04f\x04e\x18\n\x0c\n\x05\x04\x11\x02\x01\x05\x12\
    \x03f\x04\n\n\x0c\n\x05\x04\x11\x02\x01\x01\x12\x03f\x0b\x0f\n\x0c\n\x05\
    \x04\x11\x02\x01\x03\x12\x03f\x12\x13\n\n\n\x02\x04\x12\x12\x04i\0l\x01\
    \n\n\n\x03\x04\x12\x01\x12\x03i\x08\x0e\n\x0b\n\x04\x04\x12\x02\0\x12\
    \x03j\x04\x12\n\r\n\x05\x04\x12\x02\0\x04\x12\x04j\x04i\x10\n\x0c\n\x05\
    \x04\x12\x02\0\x05\x12\x03j\x04\n\n\x0c\n\x05\x04\x12\x02\0\x01\x12\x03j\
    \x0b\r\n\x0c\n\x05\x04\x12\x02\0\x03\x12\x03j\x10\x11\n\x0b\n\x04\x04\
    \x12\x02\x01\x12\x03k\x04\x14\n\r\n\x05\x04\x12\x02\x01\x04\x12\x04k\x04\
    j\x12\n\x0c\n\x05\x04\x12\x02\x01\x05\x12\x03k\x04\n\n\x0c\n\x05\x04\x12\
    \x02\x01\x01\x12\x03k\x0b\x0f\n\x0c\n\x05\x04\x12\x02\x01\x03\x12\x03k\
    \x12\x13\n\n\n\x02\x04\x13\x12\x04n\0p\x01\n\n\n\x03\x04\x13\x01\x12\x03\
    n\x08\x10\n\x0b\n\x04\x04\x13\x02\0\x12\x03o\x04!\n\x0c\n\x05\x04\x13\
    \x02\0\x04\x12\x03o\x04\x0c\n\x0c\n\x05\x04\x13\x02\0\x06\x12\x03o\r\x13\
    \n\x0c\n\x05\x04\x13\x02\0\x01\x12\x03o\x14\x1c\n\x0c\n\x05\x04\x13\x02\
    \0\x03\x12\x03o\x1f\x20\n\n\n\x02\x04\x14\x12\x04r\0u\x01\n\n\n\x03\x04\
    \x14\x01\x12\x03r\x08\x0c\n\x0b\n\x04\x04\x14\x02\0\x12\x03s\x04\x14\n\r\
    \n\x05\x04\x14\x02\0\x04\x12\x04s\x04r\x0e\n\x0c\n\x05\x04\x14\x02\0\x06\
    \x12\x03s\x04\x0b\n\x0c\n\x05\x04\x14\x02\0\x01\x12\x03s\x0c\x0f\n\x0c\n\
    \x05\x04\x14\x02\0\x03\x12\x03s\x12\x13\n\x0b\n\x04\x04\x14\x02\x01\x12\
    \x03t\x04\x14\n\r\n\x05\x04\x14\x02\x01\x04\x12\x04t\x04s\x14\n\x0c\n\
    \x05\x04\x14\x02\x01\x05\x12\x03t\x04\n\n\x0c\n\x05\x04\x14\x02\x01\x01\
    \x12\x03t\x0b\x0f\n\x0c\n\x05\x04\x14\x02\x01\x03\x12\x03t\x12\x13\n\n\n\
    \x02\x04\x15\x12\x04x\0|\x01\n\n\n\x03\x04\x15\x01\x12\x03x\x08\x0f\n\
    \x0b\n\x04\x04\x15\x02\0\x12\x03y\x04\x1b\n\r\n\x05\x04\x15\x02\0\x04\
    \x12\x04y\x04x\x11\n\x0c\n\x05\x04\x15\x02\0\x05\x12\x03y\x04\n\n\x0c\n\
    \x05\x04\x15\x02\0\x01\x12\x03y\x0b\x16\n\x0c\n\x05\x04\x15\x02\0\x03\
    \x12\x03y\x19\x1a\n\x0b\n\x04\x04\x15\x02\x01\x12\x03z\x04\x14\n\r\n\x05\
    \x04\x15\x02\x01\x04\x12\x04z\x04y\x1b\n\x0c\n\x05\x04\x15\x02\x01\x05\
    \x12\x03z\x04\n\n\x0c\n\x05\x04\x15\x02\x01\x01\x12\x03z\x0b\x0f\n\x0c\n\
    \x05\x04\x15\x02\x01\x03\x12\x03z\x12\x13\n\x0b\n\x04\x04\x15\x02\x02\
    \x12\x03{\x04\x1a\n\r\n\x05\x04\x15\x02\x02\x04\x12\x04{\x04z\x14\n\x0c\
    \n\x05\x04\x15\x02\x02\x05\x12\x03{\x04\n\n\x0c\n\x05\x04\x15\x02\x02\
    \x01\x12\x03{\x0b\x15\n\x0c\n\x05\x04\x15\x02\x02\x03\x12\x03{\x18\x19\n\
    \x0b\n\x02\x04\x16\x12\x05~\0\x84\x01\x01\n\n\n\x03\x04\x16\x01\x12\x03~\
    \x08\x13\n\r\n\x04\x04\x16\x08\0\x12\x05\x7f\x04\x83\x01\x05\n\x0c\n\x05\
    \x04\x16\x08\0\x01\x12\x03\x7f\n\x0f\n\x0c\n\x04\x04\x16\x02\0\x12\x04\
    \x80\x01\x08\x1f\n\r\n\x05\x04\x16\x02\0\x06\x12\x04\x80\x01\x08\x16\n\r\
    \n\x05\x04\x16\x02\0\x01\x12\x04\x80\x01\x17\x1a\n\r\n\x05\x04\x16\x02\0\
    \x03\x12\x04\x80\x01\x1d\x1e\n\x0c\n\x04\x04\x16\x02\x01\x12\x04\x81\x01\
    \x08)\n\r\n\x05\x04\x16\x02\x01\x06\x12\x04\x81\x01\x08\x1b\n\r\n\x05\
    \x04\x16\x02\x01\x01\x12\x04\x81\x01\x1c$\n\r\n\x05\x04\x16\x02\x01\x03\
    \x12\x04\x81\x01'(\n\x0c\n\x04\x04\x16\x02\x02\x12\x04\x82\x01\x08!\n\r\
    \n\x05\x04\x16\x02\x02\x06\x12\x04\x82\x01\x08\x17\n\r\n\x05\x04\x16\x02\
    \x02\x01\x12\x04\x82\x01\x18\x1c\n\r\n\x05\x04\x16\x02\x02\x03\x12\x04\
    \x82\x01\x1f\x20\n\x0c\n\x02\x04\x17\x12\x06\x86\x01\0\x89\x01\x01\n\x0b\
    \n\x03\x04\x17\x01\x12\x04\x86\x01\x08\x16\n\x0c\n\x04\x04\x17\x02\0\x12\
    \x04\x87\x01\x04\x18\n\x0f\n\x05\x04\x17\x02\0\x04\x12\x06\x87\x01\x04\
    \x86\x01\x18\n\r\n\x05\x04\x17\x02\0\x05\x12\x04\x87\x01\x04\n\n\r\n\x05\
    \x04\x17\x02\0\x01\x12\x04\x87\x01\x0b\x13\n\r\n\x05\x04\x17\x02\0\x03\
    \x12\x04\x87\x01\x16\x17\n\x0c\n\x04\x04\x17\x02\x01\x12\x04\x88\x01\x04\
    \x15\n\x0f\n\x05\x04\x17\x02\x01\x04\x12\x06\x88\x01\x04\x87\x01\x18\n\r\
    \n\x05\x04\x17\x02\x01\x05\x12\x04\x88\x01\x04\n\n\r\n\x05\x04\x17\x02\
    \x01\x01\x12\x04\x88\x01\x0b\x10\n\r\n\x05\x04\x17\x02\x01\x03\x12\x04\
    \x88\x01\x13\x14\n\x0c\n\x02\x04\x18\x12\x06\x8b\x01\0\x8d\x01\x01\n\x0b\
    \n\x03\x04\x18\x01\x12\x04\x8b\x01\x08\x1b\n\x0c\n\x04\x04\x18\x02\0\x12\
    \x04\x8c\x01\x04\x1c\n\r\n\x05\x04\x18\x02\0\x04\x12\x04\x8c\x01\x04\x0c\
    \n\r\n\x05\x04\x18\x02\0\x05\x12\x04\x8c\x01\r\x13\n\r\n\x05\x04\x18\x02\
    \0\x01\x12\x04\x8c\x01\x14\x17\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\x8c\
    \x01\x1a\x1b\n\x0c\n\x02\x04\x19\x12\x06\x8f\x01\0\x93\x01\x01\n\x0b\n\
    \x03\x04\x19\x01\x12\x04\x8f\x01\x08\x17\n\x0c\n\x04\x04\x19\x02\0\x12\
    \x04\x90\x01\x04\x18\n\x0f\n\x05\x04\x19\x02\0\x04\x12\x06\x90\x01\x04\
    \x8f\x01\x19\n\r\n\x05\x04\x19\x02\0\x06\x12\x04\x90\x01\x04\r\n\r\n\x05\
    \x04\x19\x02\0\x01\x12\x04\x90\x01\x0e\x13\n\r\n\x05\x04\x19\x02\0\x03\
    \x12\x04\x90\x01\x16\x17\n\x0c\n\x04\x04\x19\x02\x01\x12\x04\x91\x01\x04\
    \x19\n\x0f\n\x05\x04\x19\x02\x01\x04\x12\x06\x91\x01\x04\x90\x01\x18\n\r\
    \n\x05\x04\x19\x02\x01\x05\x12\x04\x91\x01\x04\n\n\r\n\x05\x04\x19\x02\
    \x01\x01\x12\x04\x91\x01\x0b\x14\n\r\n\x05\x04\x19\x02\x01\x03\x12\x04\
    \x91\x01\x17\x18\n\x0c\n\x04\x04\x19\x02\x02\x12\x04\x92\x01\x04\x15\n\
    \x0f\n\x05\x04\x19\x02\x02\x04\x12\x06\x92\x01\x04\x91\x01\x19\n\r\n\x05\
    \x04\x19\x02\x02\x05\x12\x04\x92\x01\x04\n\n\r\n\x05\x04\x19\x02\x02\x01\
    \x12\x04\x92\x01\x0b\x10\n\r\n\x05\x04\x19\x02\x02\x03\x12\x04\x92\x01\
    \x13\x14\n\x0c\n\x02\x04\x1a\x12\x06\x95\x01\0\x9a\x01\x01\n\x0b\n\x03\
    \x04\x1a\x01\x12\x04\x95\x01\x08\x11\n\x0e\n\x04\x04\x1a\x08\0\x12\x06\
    \x96\x01\x04\x99\x01\x05\n\r\n\x05\x04\x1a\x08\0\x01\x12\x04\x96\x01\n\
    \x0f\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\x97\x01\x08!\n\r\n\x05\x04\x1a\
    \x02\0\x06\x12\x04\x97\x01\x08\x16\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\
    \x97\x01\x17\x1c\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\x97\x01\x1f\x20\n\
    \x0c\n\x04\x04\x1a\x02\x01\x12\x04\x98\x01\x08\x1f\n\r\n\x05\x04\x1a\x02\
    \x01\x06\x12\x04\x98\x01\x08\x15\n\r\n\x05\x04\x1a\x02\x01\x01\x12\x04\
    \x98\x01\x16\x1a\n\r\n\x05\x04\x1a\x02\x01\x03\x12\x04\x98\x01\x1d\x1e\n\
    \x0c\n\x02\x04\x1b\x12\x06\x9c\x01\0\x9e\x01\x01\n\x0b\n\x03\x04\x1b\x01\
    \x12\x04\x9c\x01\x08\x16\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\x9d\x01\x04\
    \x1e\n\r\n\x05\x04\x1b\x02\0\x04\x12\x04\x9d\x01\x04\x0c\n\r\n\x05\x04\
    \x1b\x02\0\x06\x12\x04\x9d\x01\r\x14\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\
    \x9d\x01\x15\x19\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\x9d\x01\x1c\x1d\n\
    \x0c\n\x02\x04\x1c\x12\x06\xa0\x01\0\xa7\x01\x01\n\x0b\n\x03\x04\x1c\x01\
    \x12\x04\xa0\x01\x08\x15\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\xa1\x01\x04\
    \x1a\n\x0f\n\x05\x04\x1c\x02\0\x04\x12\x06\xa1\x01\x04\xa0\x01\x17\n\r\n\
    \x05\x04\x1c\x02\0\x06\x12\x04\xa1\x01\x04\x0f\n\r\n\x05\x04\x1c\x02\0\
    \x01\x12\x04\xa1\x01\x10\x15\n\r\n\x05\x04\x1c\x02\0\x03\x12\x04\xa1\x01\
    \x18\x19\n\x0c\n\x04\x04\x1c\x02\x01\x12\x04\xa2\x01\x04\x19\n\x0f\n\x05\
    \x04\x1c\x02\x01\x04\x12\x06\xa2\x01\x04\xa1\x01\x1a\n\r\n\x05\x04\x1c\
    \x02\x01\x05\x12\x04\xa2\x01\x04\n\n\r\n\x05\x04\x1c\x02\x01\x01\x12\x04\
    \xa2\x01\x0b\x14\n\r\n\x05\x04\x1c\x02\x01\x03\x12\x04\xa2\x01\x17\x18\n\
    \x0c\n\x04\x04\x1c\x02\x02\x12\x04\xa3\x01\x04\x1b\n\x0f\n\x05\x04\x1c\
    \x02\x02\x04\x12\x06\xa3\x01\x04\xa2\x01\x19\n\r\n\x05\x04\x1c\x02\x02\
    \x05\x12\x04\xa3\x01\x04\n\n\r\n\x05\x04\x1c\x02\x02\x01\x12\x04\xa3\x01\
    \x0b\x16\n\r\n\x05\x04\x1c\x02\x02\x03\x12\x04\xa3\x01\x19\x1a\n\x0c\n\
    \x04\x04\x1c\x02\x03\x12\x04\xa4\x01\x04\x1b\n\x0f\n\x05\x04\x1c\x02\x03\
    \x04\x12\x06\xa4\x01\x04\xa3\x01\x1b\n\r\n\x05\x04\x1c\x02\x03\x05\x12\
    \x04\xa4\x01\x04\n\n\r\n\x05\x04\x1c\x02\x03\x01\x12\x04\xa4\x01\x0b\x16\
    \n\r\n\x05\x04\x1c\x02\x03\x03\x12\x04\xa4\x01\x19\x1a\n\x0c\n\x04\x04\
    \x1c\x02\x04\x12\x04\xa5\x01\x04\x1a\n\x0f\n\x05\x04\x1c\x02\x04\x04\x12\
    \x06\xa5\x01\x04\xa4\x01\x1b\n\r\n\x05\x04\x1c\x02\x04\x05\x12\x04\xa5\
    \x01\x04\n\n\r\n\x05\x04\x1c\x02\x04\x01\x12\x04\xa5\x01\x0b\x15\n\r\n\
    \x05\x04\x1c\x02\x04\x03\x12\x04\xa5\x01\x18\x19\n\x0c\n\x04\x04\x1c\x02\
    \x05\x12\x04\xa6\x01\x04\x15\n\x0f\n\x05\x04\x1c\x02\x05\x04\x12\x06\xa6\
    \x01\x04\xa5\x01\x1a\n\r\n\x05\x04\x1c\x02\x05\x05\x12\x04\xa6\x01\x04\n\
    \n\r\n\x05\x04\x1c\x02\x05\x01\x12\x04\xa6\x01\x0b\x10\n\r\n\x05\x04\x1c\
    \x02\x05\x03\x12\x04\xa6\x01\x13\x14\n\x0c\n\x02\x04\x1d\x12\x06\xa9\x01\
    \0\xbd\x01\x01\n\x0b\n\x03\x04\x1d\x01\x12\x04\xa9\x01\x08\x1b\n\x0e\n\
    \x04\x04\x1d\x08\0\x12\x06\xaa\x01\x04\xbc\x01\x05\n\r\n\x05\x04\x1d\x08\
    \0\x01\x12\x04\xaa\x01\n\x12\n\x0c\n\x04\x04\x1d\x02\0\x12\x04\xab\x01\
    \x08/\n\r\n\x05\x04\x1d\x02\0\x06\x12\x04\xab\x01\x08\x1c\n\r\n\x05\x04\
    \x1d\x02\0\x01\x12\x04\xab\x01\x1d*\n\r\n\x05\x04\x1d\x02\0\x03\x12\x04\
    \xab\x01-.\n\x0c\n\x04\x04\x1d\x02\x01\x12\x04\xac\x01\x08-\n\r\n\x05\
    \x04\x1d\x02\x01\x06\x12\x04\xac\x01\x08\x1b\n\r\n\x05\x04\x1d\x02\x01\
    \x01\x12\x04\xac\x01\x1c(\n\r\n\x05\x04\x1d\x02\x01\x03\x12\x04\xac\x01+\
    ,\n\x0c\n\x04\x04\x1d\x02\x02\x12\x04\xad\x01\x083\n\r\n\x05\x04\x1d\x02\
    \x02\x06\x12\x04\xad\x01\x08\x1e\n\r\n\x05\x04\x1d\x02\x02\x01\x12\x04\
    \xad\x01\x1f.\n\r\n\x05\x04\x1d\x02\x02\x03\x12\x04\xad\x0112\n\x0c\n\
    \x04\x04\x1d\x02\x03\x12\x04\xae\x01\x084\n\r\n\x05\x04\x1d\x02\x03\x06\
    \x12\x04\xae\x01\x08\x1e\n\r\n\x05\x04\x1d\x02\x03\x01\x12\x04\xae\x01\
    \x1f/\n\r\n\x05\x04\x1d\x02\x03\x03\x12\x04\xae\x0123\n\x0c\n\x04\x04\
    \x1d\x02\x04\x12\x04\xaf\x01\x08+\n\r\n\x05\x04\x1d\x02\x04\x06\x12\x04\
    \xaf\x01\x08\x1a\n\r\n\x05\x04\x1d\x02\x04\x01\x12\x04\xaf\x01\x1b&\n\r\
    \n\x05\x04\x1d\x02\x04\x03\x12\x04\xaf\x01)*\n\x0c\n\x04\x04\x1d\x02\x05\
    \x12\x04\xb0\x01\x08'\n\r\n\x05\x04\x1d\x02\x05\x06\x12\x04\xb0\x01\x08\
    \x18\n\r\n\x05\x04\x1d\x02\x05\x01\x12\x04\xb0\x01\x19\"\n\r\n\x05\x04\
    \x1d\x02\x05\x03\x12\x04\xb0\x01%&\n\x0c\n\x04\x04\x1d\x02\x06\x12\x04\
    \xb1\x01\x08-\n\r\n\x05\x04\x1d\x02\x06\x06\x12\x04\xb1\x01\x08\x1b\n\r\
    \n\x05\x04\x1d\x02\x06\x01\x12\x04\xb1\x01\x1c(\n\r\n\x05\x04\x1d\x02\
    \x06\x03\x12\x04\xb1\x01+,\n\x0c\n\x04\x04\x1d\x02\x07\x12\x04\xb2\x01\
    \x080\n\r\n\x05\x04\x1d\x02\x07\x06\x12\x04\xb2\x01\x08\x1c\n\r\n\x05\
    \x04\x1d\x02\x07\x01\x12\x04\xb2\x01\x1d+\n\r\n\x05\x04\x1d\x02\x07\x03\
    \x12\x04\xb2\x01./\n\x0c\n\x04\x04\x1d\x02\x08\x12\x04\xb3\x01\x08:\n\r\
    \n\x05\x04\x1d\x02\x08\x06\x12\x04\xb3\x01\x08!\n\r\n\x05\x04\x1d\x02\
    \x08\x01\x12\x04\xb3\x01\"5\n\r\n\x05\x04\x1d\x02\x08\x03\x12\x04\xb3\
    \x0189\n\x0c\n\x04\x04\x1d\x02\t\x12\x04\xb4\x01\x08;\n\r\n\x05\x04\x1d\
    \x02\t\x06\x12\x04\xb4\x01\x08!\n\r\n\x05\x04\x1d\x02\t\x01\x12\x04\xb4\
    \x01\"5\n\r\n\x05\x04\x1d\x02\t\x03\x12\x04\xb4\x018:\n\x0c\n\x04\x04\
    \x1d\x02\n\x12\x04\xb5\x01\x08A\n\r\n\x05\x04\x1d\x02\n\x06\x12\x04\xb5\
    \x01\x08$\n\r\n\x05\x04\x1d\x02\n\x01\x12\x04\xb5\x01%;\n\r\n\x05\x04\
    \x1d\x02\n\x03\x12\x04\xb5\x01>@\n\x0c\n\x04\x04\x1d\x02\x0b\x12\x04\xb6\
    \x01\x08;\n\r\n\x05\x04\x1d\x02\x0b\x06\x12\x04\xb6\x01\x08!\n\r\n\x05\
    \x04\x1d\x02\x0b\x01\x12\x04\xb6\x01\"5\n\r\n\x05\x04\x1d\x02\x0b\x03\
    \x12\x04\xb6\x018:\n\x0c\n\x04\x04\x1d\x02\x0c\x12\x04\xb7\x01\x08;\n\r\
    \n\x05\x04\x1d\x02\x0c\x06\x12\x04\xb7\x01\x08!\n\r\n\x05\x04\x1d\x02\
    \x0c\x01\x12\x04\xb7\x01\"5\n\r\n\x05\x04\x1d\x02\x0c\x03\x12\x04\xb7\
    \x018:\n\x0c\n\x04\x04\x1d\x02\r\x12\x04\xb8\x01\x08A\n\r\n\x05\x04\x1d\
    \x02\r\x06\x12\x04\xb8\x01\x08$\n\r\n\x05\x04\x1d\x02\r\x01\x12\x04\xb8\
    \x01%;\n\r\n\x05\x04\x1d\x02\r\x03\x12\x04\xb8\x01>@\n\x0c\n\x04\x04\x1d\
    \x02\x0e\x12\x04\xb9\x01\x087\n\r\n\x05\x04\x1d\x02\x0e\x06\x12\x04\xb9\
    \x01\x08\x1f\n\r\n\x05\x04\x1d\x02\x0e\x01\x12\x04\xb9\x01\x201\n\r\n\
    \x05\x04\x1d\x02\x0e\x03\x12\x04\xb9\x0146\n\x0c\n\x04\x04\x1d\x02\x0f\
    \x12\x04\xba\x01\x087\n\r\n\x05\x04\x1d\x02\x0f\x06\x12\x04\xba\x01\x08\
    \x1f\n\r\n\x05\x04\x1d\x02\x0f\x01\x12\x04\xba\x01\x201\n\r\n\x05\x04\
    \x1d\x02\x0f\x03\x12\x04\xba\x0146\n\x0c\n\x04\x04\x1d\x02\x10\x12\x04\
    \xbb\x01\x08=\n\r\n\x05\x04\x1d\x02\x10\x06\x12\x04\xbb\x01\x08\"\n\r\n\
    \x05\x04\x1d\x02\x10\x01\x12\x04\xbb\x01#7\n\r\n\x05\x04\x1d\x02\x10\x03\
    \x12\x04\xbb\x01:<\n\x0c\n\x02\x04\x1e\x12\x06\xbf\x01\0\xc1\x01\x01\n\
    \x0b\n\x03\x04\x1e\x01\x12\x04\xbf\x01\x08\x1c\n\x0c\n\x04\x04\x1e\x02\0\
    \x12\x04\xc0\x01\x04\x15\n\x0f\n\x05\x04\x1e\x02\0\x04\x12\x06\xc0\x01\
    \x04\xbf\x01\x1e\n\r\n\x05\x04\x1e\x02\0\x05\x12\x04\xc0\x01\x04\n\n\r\n\
    \x05\x04\x1e\x02\0\x01\x12\x04\xc0\x01\x0b\x10\n\r\n\x05\x04\x1e\x02\0\
    \x03\x12\x04\xc0\x01\x13\x14\n\x0c\n\x02\x04\x1f\x12\x06\xc3\x01\0\xc8\
    \x01\x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\xc3\x01\x08\x1b\n\x0e\n\x04\x04\
    \x1f\x08\0\x12\x06\xc4\x01\x04\xc7\x01\x05\n\r\n\x05\x04\x1f\x08\0\x01\
    \x12\x04\xc4\x01\n\x12\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xc5\x01\x08\x1e\
    \n\r\n\x05\x04\x1f\x02\0\x06\x12\x04\xc5\x01\x08\x10\n\r\n\x05\x04\x1f\
    \x02\0\x01\x12\x04\xc5\x01\x11\x19\n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\
    \xc5\x01\x1c\x1d\n\x0c\n\x04\x04\x1f\x02\x01\x12\x04\xc6\x01\x08\x19\n\r\
    \n\x05\x04\x1f\x02\x01\x05\x12\x04\xc6\x01\x08\x0e\n\r\n\x05\x04\x1f\x02\
    \x01\x01\x12\x04\xc6\x01\x0f\x14\n\r\n\x05\x04\x1f\x02\x01\x03\x12\x04\
    \xc6\x01\x17\x18\n\x0c\n\x02\x04\x20\x12\x06\xca\x01\0\xcc\x01\x01\n\x0b\
    \n\x03\x04\x20\x01\x12\x04\xca\x01\x08\x1e\n\x0c\n\x04\x04\x20\x02\0\x12\
    \x04\xcb\x01\x04\x15\n\x0f\n\x05\x04\x20\x02\0\x04\x12\x06\xcb\x01\x04\
    \xca\x01\x20\n\r\n\x05\x04\x20\x02\0\x05\x12\x04\xcb\x01\x04\n\n\r\n\x05\
    \x04\x20\x02\0\x01\x12\x04\xcb\x01\x0b\x10\n\r\n\x05\x04\x20\x02\0\x03\
    \x12\x04\xcb\x01\x13\x14\n\x0c\n\x02\x04!\x12\x06\xce\x01\0\xd1\x01\x01\
    \n\x0b\n\x03\x04!\x01\x12\x04\xce\x01\x08\x1e\n\x0c\n\x04\x04!\x02\0\x12\
    \x04\xcf\x01\x04\x15\n\x0f\n\x05\x04!\x02\0\x04\x12\x06\xcf\x01\x04\xce\
    \x01\x20\n\r\n\x05\x04!\x02\0\x05\x12\x04\xcf\x01\x04\n\n\r\n\x05\x04!\
    \x02\0\x01\x12\x04\xcf\x01\x0b\x10\n\r\n\x05\x04!\x02\0\x03\x12\x04\xcf\
    \x01\x13\x14\n\x0c\n\x04\x04!\x02\x01\x12\x04\xd0\x01\x04\x15\n\x0f\n\
    \x05\x04!\x02\x01\x04\x12\x06\xd0\x01\x04\xcf\x01\x15\n\r\n\x05\x04!\x02\
    \x01\x05\x12\x04\xd0\x01\x04\n\n\r\n\x05\x04!\x02\x01\x01\x12\x04\xd0\
    \x01\x0b\x10\n\r\n\x05\x04!\x02\x01\x03\x12\x04\xd0\x01\x13\x14\n\x0c\n\
    \x02\x04\"\x12\x06\xd3\x01\0\xd6\x01\x01\n\x0b\n\x03\x04\"\x01\x12\x04\
    \xd3\x01\x08\x1a\n\x0c\n\x04\x04\"\x02\0\x12\x04\xd4\x01\x04\x15\n\x0f\n\
    \x05\x04\"\x02\0\x04\x12\x06\xd4\x01\x04\xd3\x01\x1c\n\r\n\x05\x04\"\x02\
    \0\x05\x12\x04\xd4\x01\x04\x08\n\r\n\x05\x04\"\x02\0\x01\x12\x04\xd4\x01\
    \t\x10\n\r\n\x05\x04\"\x02\0\x03\x12\x04\xd4\x01\x13\x14\n\x0c\n\x04\x04\
    \"\x02\x01\x12\x04\xd5\x01\x04\x15\n\x0f\n\x05\x04\"\x02\x01\x04\x12\x06\
    \xd5\x01\x04\xd4\x01\x15\n\r\n\x05\x04\"\x02\x01\x05\x12\x04\xd5\x01\x04\
    \n\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\xd5\x01\x0b\x10\n\r\n\x05\x04\"\
    \x02\x01\x03\x12\x04\xd5\x01\x13\x14\n\x0c\n\x02\x04#\x12\x06\xd8\x01\0\
    \xdb\x01\x01\n\x0b\n\x03\x04#\x01\x12\x04\xd8\x01\x08\x18\n\x0c\n\x04\
    \x04#\x02\0\x12\x04\xd9\x01\x04\x1c\n\r\n\x05\x04#\x02\0\x04\x12\x04\xd9\
    \x01\x04\x0c\n\r\n\x05\x04#\x02\0\x06\x12\x04\xd9\x01\r\x11\n\r\n\x05\
    \x04#\x02\0\x01\x12\x04\xd9\x01\x12\x17\n\r\n\x05\x04#\x02\0\x03\x12\x04\
    \xd9\x01\x1a\x1b\n\x0c\n\x04\x04#\x02\x01\x12\x04\xda\x01\x04\x15\n\x0f\
    \n\x05\x04#\x02\x01\x04\x12\x06\xda\x01\x04\xd9\x01\x1c\n\r\n\x05\x04#\
    \x02\x01\x05\x12\x04\xda\x01\x04\n\n\r\n\x05\x04#\x02\x01\x01\x12\x04\
    \xda\x01\x0b\x10\n\r\n\x05\x04#\x02\x01\x03\x12\x04\xda\x01\x13\x14\n\
    \x0c\n\x02\x04$\x12\x06\xdd\x01\0\xdf\x01\x01\n\x0b\n\x03\x04$\x01\x12\
    \x04\xdd\x01\x08\x1b\n\x0c\n\x04\x04$\x02\0\x12\x04\xde\x01\x04\x15\n\
    \x0f\n\x05\x04$\x02\0\x04\x12\x06\xde\x01\x04\xdd\x01\x1d\n\r\n\x05\x04$\
    \x02\0\x05\x12\x04\xde\x01\x04\n\n\r\n\x05\x04$\x02\0\x01\x12\x04\xde\
    \x01\x0b\x10\n\r\n\x05\x04$\x02\0\x03\x12\x04\xde\x01\x13\x14\n\x0c\n\
    \x02\x04%\x12\x06\xe1\x01\0\xe4\x01\x01\n\x0b\n\x03\x04%\x01\x12\x04\xe1\
    \x01\x08\x1c\n\x0c\n\x04\x04%\x02\0\x12\x04\xe2\x01\x04\x15\n\x0f\n\x05\
    \x04%\x02\0\x04\x12\x06\xe2\x01\x04\xe1\x01\x1e\n\r\n\x05\x04%\x02\0\x05\
    \x12\x04\xe2\x01\x04\n\n\r\n\x05\x04%\x02\0\x01\x12\x04\xe2\x01\x0b\x10\
    \n\r\n\x05\x04%\x02\0\x03\x12\x04\xe2\x01\x13\x14\n\x0c\n\x04\x04%\x02\
    \x01\x12\x04\xe3\x01\x04\x15\n\x0f\n\x05\x04%\x02\x01\x04\x12\x06\xe3\
    \x01\x04\xe2\x01\x15\n\r\n\x05\x04%\x02\x01\x05\x12\x04\xe3\x01\x04\n\n\
    \r\n\x05\x04%\x02\x01\x01\x12\x04\xe3\x01\x0b\x10\n\r\n\x05\x04%\x02\x01\
    \x03\x12\x04\xe3\x01\x13\x14\n\x0c\n\x02\x04&\x12\x06\xe6\x01\0\xe9\x01\
    \x01\n\x0b\n\x03\x04&\x01\x12\x04\xe6\x01\x08!\n\x0c\n\x04\x04&\x02\0\
    \x12\x04\xe7\x01\x04\x18\n\x0f\n\x05\x04&\x02\0\x04\x12\x06\xe7\x01\x04\
    \xe6\x01#\n\r\n\x05\x04&\x02\0\x05\x12\x04\xe7\x01\x04\n\n\r\n\x05\x04&\
    \x02\0\x01\x12\x04\xe7\x01\x0b\x13\n\r\n\x05\x04&\x02\0\x03\x12\x04\xe7\
    \x01\x16\x17\n\x0c\n\x04\x04&\x02\x01\x12\x04\xe8\x01\x04\x15\n\x0f\n\
    \x05\x04&\x02\x01\x04\x12\x06\xe8\x01\x04\xe7\x01\x18\n\r\n\x05\x04&\x02\
    \x01\x05\x12\x04\xe8\x01\x04\n\n\r\n\x05\x04&\x02\x01\x01\x12\x04\xe8\
    \x01\x0b\x10\n\r\n\x05\x04&\x02\x01\x03\x12\x04\xe8\x01\x13\x14\n\x0c\n\
    \x02\x04'\x12\x06\xeb\x01\0\xed\x01\x01\n\x0b\n\x03\x04'\x01\x12\x04\xeb\
    \x01\x08!\n\x0c\n\x04\x04'\x02\0\x12\x04\xec\x01\x04\x15\n\x0f\n\x05\x04\
    '\x02\0\x04\x12\x06\xec\x01\x04\xeb\x01#\n\r\n\x05\x04'\x02\0\x05\x12\
    \x04\xec\x01\x04\n\n\r\n\x05\x04'\x02\0\x01\x12\x04\xec\x01\x0b\x10\n\r\
    \n\x05\x04'\x02\0\x03\x12\x04\xec\x01\x13\x14\n\x0c\n\x02\x04(\x12\x06\
    \xef\x01\0\xf1\x01\x01\n\x0b\n\x03\x04(\x01\x12\x04\xef\x01\x08$\n\x0c\n\
    \x04\x04(\x02\0\x12\x04\xf0\x01\x04\x15\n\x0f\n\x05\x04(\x02\0\x04\x12\
    \x06\xf0\x01\x04\xef\x01&\n\r\n\x05\x04(\x02\0\x05\x12\x04\xf0\x01\x04\n\
    \n\r\n\x05\x04(\x02\0\x01\x12\x04\xf0\x01\x0b\x10\n\r\n\x05\x04(\x02\0\
    \x03\x12\x04\xf0\x01\x13\x14\n\x0c\n\x02\x04)\x12\x06\xf3\x01\0\xf6\x01\
    \x01\n\x0b\n\x03\x04)\x01\x12\x04\xf3\x01\x08!\n\x0c\n\x04\x04)\x02\0\
    \x12\x04\xf4\x01\x04)\n\r\n\x05\x04)\x02\0\x04\x12\x04\xf4\x01\x04\x0c\n\
    \r\n\x05\x04)\x02\0\x06\x12\x04\xf4\x01\r\x1b\n\r\n\x05\x04)\x02\0\x01\
    \x12\x04\xf4\x01\x1c$\n\r\n\x05\x04)\x02\0\x03\x12\x04\xf4\x01'(\n\x0c\n\
    \x04\x04)\x02\x01\x12\x04\xf5\x01\x04\x15\n\x0f\n\x05\x04)\x02\x01\x04\
    \x12\x06\xf5\x01\x04\xf4\x01)\n\r\n\x05\x04)\x02\x01\x05\x12\x04\xf5\x01\
    \x04\n\n\r\n\x05\x04)\x02\x01\x01\x12\x04\xf5\x01\x0b\x10\n\r\n\x05\x04)\
    \x02\x01\x03\x12\x04\xf5\x01\x13\x14\n\x0c\n\x02\x04*\x12\x06\xf8\x01\0\
    \xfa\x01\x01\n\x0b\n\x03\x04*\x01\x12\x04\xf8\x01\x08!\n\x0c\n\x04\x04*\
    \x02\0\x12\x04\xf9\x01\x04\x15\n\x0f\n\x05\x04*\x02\0\x04\x12\x06\xf9\
    \x01\x04\xf8\x01#\n\r\n\x05\x04*\x02\0\x05\x12\x04\xf9\x01\x04\n\n\r\n\
    \x05\x04*\x02\0\x01\x12\x04\xf9\x01\x0b\x10\n\r\n\x05\x04*\x02\0\x03\x12\
    \x04\xf9\x01\x13\x14\n\x0c\n\x02\x04+\x12\x06\xfc\x01\0\xfe\x01\x01\n\
    \x0b\n\x03\x04+\x01\x12\x04\xfc\x01\x08$\n\x0c\n\x04\x04+\x02\0\x12\x04\
    \xfd\x01\x04\x15\n\x0f\n\x05\x04+\x02\0\x04\x12\x06\xfd\x01\x04\xfc\x01&\
    \n\r\n\x05\x04+\x02\0\x05\x12\x04\xfd\x01\x04\n\n\r\n\x05\x04+\x02\0\x01\
    \x12\x04\xfd\x01\x0b\x10\n\r\n\x05\x04+\x02\0\x03\x12\x04\xfd\x01\x13\
    \x14\n\x0c\n\x02\x04,\x12\x06\x80\x02\0\x83\x02\x01\n\x0b\n\x03\x04,\x01\
    \x12\x04\x80\x02\x08\x1f\n\x0c\n\x04\x04,\x02\0\x12\x04\x81\x02\x04)\n\r\
    \n\x05\x04,\x02\0\x04\x12\x04\x81\x02\x04\x0c\n\r\n\x05\x04,\x02\0\x06\
    \x12\x04\x81\x02\r\x1b\n\r\n\x05\x04,\x02\0\x01\x12\x04\x81\x02\x1c$\n\r\
    \n\x05\x04,\x02\0\x03\x12\x04\x81\x02'(\n\x0c\n\x04\x04,\x02\x01\x12\x04\
    \x82\x02\x04\x15\n\x0f\n\x05\x04,\x02\x01\x04\x12\x06\x82\x02\x04\x81\
    \x02)\n\r\n\x05\x04,\x02\x01\x05\x12\x04\x82\x02\x04\n\n\r\n\x05\x04,\
    \x02\x01\x01\x12\x04\x82\x02\x0b\x10\n\r\n\x05\x04,\x02\x01\x03\x12\x04\
    \x82\x02\x13\x14\n\x0c\n\x02\x04-\x12\x06\x85\x02\0\x87\x02\x01\n\x0b\n\
    \x03\x04-\x01\x12\x04\x85\x02\x08\x1f\n\x0c\n\x04\x04-\x02\0\x12\x04\x86\
    \x02\x04\x15\n\x0f\n\x05\x04-\x02\0\x04\x12\x06\x86\x02\x04\x85\x02!\n\r\
    \n\x05\x04-\x02\0\x05\x12\x04\x86\x02\x04\n\n\r\n\x05\x04-\x02\0\x01\x12\
    \x04\x86\x02\x0b\x10\n\r\n\x05\x04-\x02\0\x03\x12\x04\x86\x02\x13\x14\n\
    \x0c\n\x02\x04.\x12\x06\x89\x02\0\x8b\x02\x01\n\x0b\n\x03\x04.\x01\x12\
    \x04\x89\x02\x08\"\n\x0c\n\x04\x04.\x02\0\x12\x04\x8a\x02\x04\x15\n\x0f\
    \n\x05\x04.\x02\0\x04\x12\x06\x8a\x02\x04\x89\x02$\n\r\n\x05\x04.\x02\0\
    \x05\x12\x04\x8a\x02\x04\n\n\r\n\x05\x04.\x02\0\x01\x12\x04\x8a\x02\x0b\
    \x10\n\r\n\x05\x04.\x02\0\x03\x12\x04\x8a\x02\x13\x14\n\x0c\n\x02\x04/\
    \x12\x06\x8d\x02\0\x90\x02\x01\n\x0b\n\x03\x04/\x01\x12\x04\x8d\x02\x08\
    \x16\n\x0c\n\x04\x04/\x02\0\x12\x04\x8e\x02\x04\x12\n\x0f\n\x05\x04/\x02\
    \0\x04\x12\x06\x8e\x02\x04\x8d\x02\x18\n\r\n\x05\x04/\x02\0\x05\x12\x04\
    \x8e\x02\x04\n\n\r\n\x05\x04/\x02\0\x01\x12\x04\x8e\x02\x0b\r\n\r\n\x05\
    \x04/\x02\0\x03\x12\x04\x8e\x02\x10\x11\n\x0c\n\x04\x04/\x02\x01\x12\x04\
    \x8f\x02\x04\x18\n\x0f\n\x05\x04/\x02\x01\x04\x12\x06\x8f\x02\x04\x8e\
    \x02\x12\n\r\n\x05\x04/\x02\x01\x05\x12\x04\x8f\x02\x04\n\n\r\n\x05\x04/\
    \x02\x01\x01\x12\x04\x8f\x02\x0b\x13\n\r\n\x05\x04/\x02\x01\x03\x12\x04\
    \x8f\x02\x16\x17\n\x0c\n\x02\x040\x12\x06\x92\x02\0\x95\x02\x01\n\x0b\n\
    \x03\x040\x01\x12\x04\x92\x02\x08\x14\n\x0c\n\x04\x040\x02\0\x12\x04\x93\
    \x02\x04\x11\n\x0f\n\x05\x040\x02\0\x04\x12\x06\x93\x02\x04\x92\x02\x16\
    \n\r\n\x05\x040\x02\0\x06\x12\x04\x93\x02\x04\x08\n\r\n\x05\x040\x02\0\
    \x01\x12\x04\x93\x02\t\x0c\n\r\n\x05\x040\x02\0\x03\x12\x04\x93\x02\x0f\
    \x10\n\x0c\n\x04\x040\x02\x01\x12\x04\x94\x02\x04\x18\n\x0f\n\x05\x040\
    \x02\x01\x04\x12\x06\x94\x02\x04\x93\x02\x11\n\r\n\x05\x040\x02\x01\x05\
    \x12\x04\x94\x02\x04\n\n\r\n\x05\x040\x02\x01\x01\x12\x04\x94\x02\x0b\
    \x13\n\r\n\x05\x040\x02\x01\x03\x12\x04\x94\x02\x16\x17b\x06proto3\
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
