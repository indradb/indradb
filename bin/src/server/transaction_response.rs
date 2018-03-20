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
        self.response = ::std::option::Option::None;
    }

    pub fn has_create_vertex(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_vertex(&mut self, v: CreateVertexResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_vertex(&mut self) -> &mut CreateVertexResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(CreateVertexResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_vertex(&mut self) -> CreateVertexResponse {
        if self.has_create_vertex() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateVertexResponse::new()
        }
    }

    pub fn get_create_vertex(&self) -> &CreateVertexResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(ref v)) => v,
            _ => CreateVertexResponse::default_instance(),
        }
    }

    // .GetVerticesResponse get_vertices = 2;

    pub fn clear_get_vertices(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_vertices(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertices(&mut self, v: GetVerticesResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertices(&mut self) -> &mut GetVerticesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(GetVerticesResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertices(&mut self) -> GetVerticesResponse {
        if self.has_get_vertices() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVerticesResponse::new()
        }
    }

    pub fn get_get_vertices(&self) -> &GetVerticesResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(ref v)) => v,
            _ => GetVerticesResponse::default_instance(),
        }
    }

    // .DeleteVerticesResponse delete_vertices = 3;

    pub fn clear_delete_vertices(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_delete_vertices(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertices(&mut self, v: DeleteVerticesResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertices(&mut self) -> &mut DeleteVerticesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(DeleteVerticesResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertices(&mut self) -> DeleteVerticesResponse {
        if self.has_delete_vertices() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVerticesResponse::new()
        }
    }

    pub fn get_delete_vertices(&self) -> &DeleteVerticesResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(ref v)) => v,
            _ => DeleteVerticesResponse::default_instance(),
        }
    }

    // .GetVertexCountResponse get_vertex_count = 4;

    pub fn clear_get_vertex_count(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_vertex_count(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_count(&mut self, v: GetVertexCountResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_count(&mut self) -> &mut GetVertexCountResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(GetVertexCountResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_count(&mut self) -> GetVertexCountResponse {
        if self.has_get_vertex_count() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexCountResponse::new()
        }
    }

    pub fn get_get_vertex_count(&self) -> &GetVertexCountResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(ref v)) => v,
            _ => GetVertexCountResponse::default_instance(),
        }
    }

    // .CreateEdgeResponse create_edge = 5;

    pub fn clear_create_edge(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_create_edge(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_edge(&mut self, v: CreateEdgeResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_edge(&mut self) -> &mut CreateEdgeResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(CreateEdgeResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_edge(&mut self) -> CreateEdgeResponse {
        if self.has_create_edge() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateEdgeResponse::new()
        }
    }

    pub fn get_create_edge(&self) -> &CreateEdgeResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(ref v)) => v,
            _ => CreateEdgeResponse::default_instance(),
        }
    }

    // .GetEdgesResponse get_edges = 6;

    pub fn clear_get_edges(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_edges(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edges(&mut self, v: GetEdgesResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edges(&mut self) -> &mut GetEdgesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(GetEdgesResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edges(&mut self) -> GetEdgesResponse {
        if self.has_get_edges() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgesResponse::new()
        }
    }

    pub fn get_get_edges(&self) -> &GetEdgesResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(ref v)) => v,
            _ => GetEdgesResponse::default_instance(),
        }
    }

    // .DeleteEdgesResponse delete_edges = 7;

    pub fn clear_delete_edges(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_delete_edges(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edges(&mut self, v: DeleteEdgesResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edges(&mut self) -> &mut DeleteEdgesResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(DeleteEdgesResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edges(&mut self) -> DeleteEdgesResponse {
        if self.has_delete_edges() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgesResponse::new()
        }
    }

    pub fn get_delete_edges(&self) -> &DeleteEdgesResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(ref v)) => v,
            _ => DeleteEdgesResponse::default_instance(),
        }
    }

    // .GetEdgeCountResponse get_edge_count = 8;

    pub fn clear_get_edge_count(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_edge_count(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_count(&mut self, v: GetEdgeCountResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_count(&mut self) -> &mut GetEdgeCountResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(GetEdgeCountResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_count(&mut self) -> GetEdgeCountResponse {
        if self.has_get_edge_count() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeCountResponse::new()
        }
    }

    pub fn get_get_edge_count(&self) -> &GetEdgeCountResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(ref v)) => v,
            _ => GetEdgeCountResponse::default_instance(),
        }
    }

    // .GetGlobalMetadataResponse get_global_metadata = 9;

    pub fn clear_get_global_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_global_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_global_metadata(&mut self, v: GetGlobalMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_global_metadata(&mut self) -> &mut GetGlobalMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(GetGlobalMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_global_metadata(&mut self) -> GetGlobalMetadataResponse {
        if self.has_get_global_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetGlobalMetadataResponse::new()
        }
    }

    pub fn get_get_global_metadata(&self) -> &GetGlobalMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(ref v)) => v,
            _ => GetGlobalMetadataResponse::default_instance(),
        }
    }

    // .SetGlobalMetadataResponse set_global_metadata = 10;

    pub fn clear_set_global_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_set_global_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_global_metadata(&mut self, v: SetGlobalMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_global_metadata(&mut self) -> &mut SetGlobalMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(SetGlobalMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_global_metadata(&mut self) -> SetGlobalMetadataResponse {
        if self.has_set_global_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetGlobalMetadataResponse::new()
        }
    }

    pub fn get_set_global_metadata(&self) -> &SetGlobalMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(ref v)) => v,
            _ => SetGlobalMetadataResponse::default_instance(),
        }
    }

    // .DeleteGlobalMetadataResponse delete_global_metadata = 11;

    pub fn clear_delete_global_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_delete_global_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_global_metadata(&mut self, v: DeleteGlobalMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_global_metadata(&mut self) -> &mut DeleteGlobalMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(DeleteGlobalMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_global_metadata(&mut self) -> DeleteGlobalMetadataResponse {
        if self.has_delete_global_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteGlobalMetadataResponse::new()
        }
    }

    pub fn get_delete_global_metadata(&self) -> &DeleteGlobalMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(ref v)) => v,
            _ => DeleteGlobalMetadataResponse::default_instance(),
        }
    }

    // .GetVertexMetadataResponse get_vertex_metadata = 12;

    pub fn clear_get_vertex_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_vertex_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_vertex_metadata(&mut self, v: GetVertexMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_vertex_metadata(&mut self) -> &mut GetVertexMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(GetVertexMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_vertex_metadata(&mut self) -> GetVertexMetadataResponse {
        if self.has_get_vertex_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetVertexMetadataResponse::new()
        }
    }

    pub fn get_get_vertex_metadata(&self) -> &GetVertexMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(ref v)) => v,
            _ => GetVertexMetadataResponse::default_instance(),
        }
    }

    // .SetVertexMetadataResponse set_vertex_metadata = 13;

    pub fn clear_set_vertex_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_set_vertex_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_vertex_metadata(&mut self, v: SetVertexMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_vertex_metadata(&mut self) -> &mut SetVertexMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(SetVertexMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_vertex_metadata(&mut self) -> SetVertexMetadataResponse {
        if self.has_set_vertex_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetVertexMetadataResponse::new()
        }
    }

    pub fn get_set_vertex_metadata(&self) -> &SetVertexMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(ref v)) => v,
            _ => SetVertexMetadataResponse::default_instance(),
        }
    }

    // .DeleteVertexMetadataResponse delete_vertex_metadata = 14;

    pub fn clear_delete_vertex_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_delete_vertex_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_vertex_metadata(&mut self, v: DeleteVertexMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_vertex_metadata(&mut self) -> &mut DeleteVertexMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(DeleteVertexMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_vertex_metadata(&mut self) -> DeleteVertexMetadataResponse {
        if self.has_delete_vertex_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteVertexMetadataResponse::new()
        }
    }

    pub fn get_delete_vertex_metadata(&self) -> &DeleteVertexMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(ref v)) => v,
            _ => DeleteVertexMetadataResponse::default_instance(),
        }
    }

    // .GetEdgeMetadataResponse get_edge_metadata = 15;

    pub fn clear_get_edge_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_get_edge_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_edge_metadata(&mut self, v: GetEdgeMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_edge_metadata(&mut self) -> &mut GetEdgeMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(GetEdgeMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_edge_metadata(&mut self) -> GetEdgeMetadataResponse {
        if self.has_get_edge_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            GetEdgeMetadataResponse::new()
        }
    }

    pub fn get_get_edge_metadata(&self) -> &GetEdgeMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(ref v)) => v,
            _ => GetEdgeMetadataResponse::default_instance(),
        }
    }

    // .SetEdgeMetadataResponse set_edge_metadata = 16;

    pub fn clear_set_edge_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_set_edge_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_edge_metadata(&mut self, v: SetEdgeMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_edge_metadata(&mut self) -> &mut SetEdgeMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(SetEdgeMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_edge_metadata(&mut self) -> SetEdgeMetadataResponse {
        if self.has_set_edge_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            SetEdgeMetadataResponse::new()
        }
    }

    pub fn get_set_edge_metadata(&self) -> &SetEdgeMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(ref v)) => v,
            _ => SetEdgeMetadataResponse::default_instance(),
        }
    }

    // .DeleteEdgeMetadataResponse delete_edge_metadata = 17;

    pub fn clear_delete_edge_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_delete_edge_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_edge_metadata(&mut self, v: DeleteEdgeMetadataResponse) {
        self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_edge_metadata(&mut self) -> &mut DeleteEdgeMetadataResponse {
        if let ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(DeleteEdgeMetadataResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_edge_metadata(&mut self) -> DeleteEdgeMetadataResponse {
        if self.has_delete_edge_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteEdgeMetadataResponse::new()
        }
    }

    pub fn get_delete_edge_metadata(&self) -> &DeleteEdgeMetadataResponse {
        match self.response {
            ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(ref v)) => v,
            _ => DeleteEdgeMetadataResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for TransactionResponse {
    fn is_initialized(&self) -> bool {
        if let Some(TransactionResponse_oneof_response::create_vertex(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_vertices(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::delete_vertices(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_vertex_count(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::create_edge(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_edges(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::delete_edges(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_edge_count(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_global_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::set_global_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::delete_global_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_vertex_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::set_vertex_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::delete_vertex_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::get_edge_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::set_edge_metadata(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(TransactionResponse_oneof_response::delete_edge_metadata(ref v)) = self.response {
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
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::create_vertex(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertices(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertices(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_count(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::create_edge(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edges(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edges(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_count(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_global_metadata(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_global_metadata(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_global_metadata(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_vertex_metadata(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_vertex_metadata(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_vertex_metadata(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::get_edge_metadata(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::set_edge_metadata(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(TransactionResponse_oneof_response::delete_edge_metadata(is.read_message()?));
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
                &TransactionResponse_oneof_response::create_vertex(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::delete_vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_vertex_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::create_edge(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::delete_edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_edge_count(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::set_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::delete_global_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::set_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::delete_vertex_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::get_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::set_edge_metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TransactionResponse_oneof_response::delete_edge_metadata(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &TransactionResponse_oneof_response::create_vertex(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_vertices(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::delete_vertices(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_vertex_count(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::create_edge(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_edges(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::delete_edges(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_edge_count(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_global_metadata(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::set_global_metadata(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::delete_global_metadata(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_vertex_metadata(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::set_vertex_metadata(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::delete_vertex_metadata(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::get_edge_metadata(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::set_edge_metadata(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TransactionResponse_oneof_response::delete_edge_metadata(ref v) => {
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
    response: ::std::option::Option<GetVerticesResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVerticesResponse {}

#[derive(Clone,PartialEq)]
pub enum GetVerticesResponse_oneof_response {
    vertices(super::models::Vertices),
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
        self.response = ::std::option::Option::None;
    }

    pub fn has_vertices(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: super::models::Vertices) {
        self.response = ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(v))
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut super::models::Vertices {
        if let ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(super::models::Vertices::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vertices(&mut self) -> super::models::Vertices {
        if self.has_vertices() {
            match self.response.take() {
                ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(v)) => v,
                _ => panic!(),
            }
        } else {
            super::models::Vertices::new()
        }
    }

    pub fn get_vertices(&self) -> &super::models::Vertices {
        match self.response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(ref v)) => v,
            _ => super::models::Vertices::default_instance(),
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for GetVerticesResponse {
    fn is_initialized(&self) -> bool {
        if let Some(GetVerticesResponse_oneof_response::vertices(ref v)) = self.response {
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
                    self.response = ::std::option::Option::Some(GetVerticesResponse_oneof_response::vertices(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetVerticesResponse_oneof_response::error(is.read_string()?));
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
                &GetVerticesResponse_oneof_response::vertices(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GetVerticesResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetVerticesResponse_oneof_response::vertices(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GetVerticesResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::models::Vertices>(
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
    // message oneof groups
    response: ::std::option::Option<GetVertexCountResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVertexCountResponse {}

#[derive(Clone,PartialEq)]
pub enum GetVertexCountResponse_oneof_response {
    count(u64),
    error(::std::string::String),
}

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
        self.response = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetVertexCountResponse_oneof_response::count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.response = ::std::option::Option::Some(GetVertexCountResponse_oneof_response::count(v))
    }

    pub fn get_count(&self) -> u64 {
        match self.response {
            ::std::option::Option::Some(GetVertexCountResponse_oneof_response::count(v)) => v,
            _ => 0,
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
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
                    self.response = ::std::option::Option::Some(GetVertexCountResponse_oneof_response::count(is.read_uint64()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetVertexCountResponse_oneof_response::error(is.read_string()?));
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
                &GetVertexCountResponse_oneof_response::count(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &GetVertexCountResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetVertexCountResponse_oneof_response::count(v) => {
                    os.write_uint64(1, v)?;
                },
                &GetVertexCountResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "count",
                    GetVertexCountResponse::has_count,
                    GetVertexCountResponse::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetVertexCountResponse::has_error,
                    GetVertexCountResponse::get_error,
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
    // message oneof groups
    response: ::std::option::Option<CreateEdgeResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEdgeResponse {}

#[derive(Clone,PartialEq)]
pub enum CreateEdgeResponse_oneof_response {
    created(bool),
    error(::std::string::String),
}

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
        self.response = ::std::option::Option::None;
    }

    pub fn has_created(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(CreateEdgeResponse_oneof_response::created(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_created(&mut self, v: bool) {
        self.response = ::std::option::Option::Some(CreateEdgeResponse_oneof_response::created(v))
    }

    pub fn get_created(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(CreateEdgeResponse_oneof_response::created(v)) => v,
            _ => false,
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
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
                    self.response = ::std::option::Option::Some(CreateEdgeResponse_oneof_response::created(is.read_bool()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(CreateEdgeResponse_oneof_response::error(is.read_string()?));
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
                &CreateEdgeResponse_oneof_response::created(v) => {
                    my_size += 2;
                },
                &CreateEdgeResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &CreateEdgeResponse_oneof_response::created(v) => {
                    os.write_bool(1, v)?;
                },
                &CreateEdgeResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "created",
                    CreateEdgeResponse::has_created,
                    CreateEdgeResponse::get_created,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    CreateEdgeResponse::has_error,
                    CreateEdgeResponse::get_error,
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
    // message oneof groups
    response: ::std::option::Option<GetEdgesResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgesResponse {}

#[derive(Clone,PartialEq)]
pub enum GetEdgesResponse_oneof_response {
    edges(super::models::Edges),
    error(::std::string::String),
}

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

    // .Edges edges = 1;

    pub fn clear_edges(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_edges(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_edges(&mut self, v: super::models::Edges) {
        self.response = ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(v))
    }

    // Mutable pointer to the field.
    pub fn mut_edges(&mut self) -> &mut super::models::Edges {
        if let ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(super::models::Edges::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_edges(&mut self) -> super::models::Edges {
        if self.has_edges() {
            match self.response.take() {
                ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(v)) => v,
                _ => panic!(),
            }
        } else {
            super::models::Edges::new()
        }
    }

    pub fn get_edges(&self) -> &super::models::Edges {
        match self.response {
            ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(ref v)) => v,
            _ => super::models::Edges::default_instance(),
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for GetEdgesResponse {
    fn is_initialized(&self) -> bool {
        if let Some(GetEdgesResponse_oneof_response::edges(ref v)) = self.response {
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
                    self.response = ::std::option::Option::Some(GetEdgesResponse_oneof_response::edges(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetEdgesResponse_oneof_response::error(is.read_string()?));
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
                &GetEdgesResponse_oneof_response::edges(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GetEdgesResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetEdgesResponse_oneof_response::edges(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GetEdgesResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::models::Edges>(
                    "edges",
                    GetEdgesResponse::has_edges,
                    GetEdgesResponse::get_edges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetEdgesResponse::has_error,
                    GetEdgesResponse::get_error,
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
    // message oneof groups
    response: ::std::option::Option<GetEdgeCountResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgeCountResponse {}

#[derive(Clone,PartialEq)]
pub enum GetEdgeCountResponse_oneof_response {
    count(u64),
    error(::std::string::String),
}

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
        self.response = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.response = ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::count(v))
    }

    pub fn get_count(&self) -> u64 {
        match self.response {
            ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::count(v)) => v,
            _ => 0,
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
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
                    self.response = ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::count(is.read_uint64()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetEdgeCountResponse_oneof_response::error(is.read_string()?));
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
                &GetEdgeCountResponse_oneof_response::count(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &GetEdgeCountResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetEdgeCountResponse_oneof_response::count(v) => {
                    os.write_uint64(1, v)?;
                },
                &GetEdgeCountResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "count",
                    GetEdgeCountResponse::has_count,
                    GetEdgeCountResponse::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetEdgeCountResponse::has_error,
                    GetEdgeCountResponse::get_error,
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
    // message oneof groups
    response: ::std::option::Option<GetGlobalMetadataResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGlobalMetadataResponse {}

#[derive(Clone,PartialEq)]
pub enum GetGlobalMetadataResponse_oneof_response {
    metadata(::std::string::String),
    error(::std::string::String),
}

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
        self.response = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::string::String {
        if self.has_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_metadata(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(ref v)) => v,
            _ => "",
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::metadata(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetGlobalMetadataResponse_oneof_response::error(is.read_string()?));
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
                &GetGlobalMetadataResponse_oneof_response::metadata(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &GetGlobalMetadataResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetGlobalMetadataResponse_oneof_response::metadata(ref v) => {
                    os.write_string(1, v)?;
                },
                &GetGlobalMetadataResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "metadata",
                    GetGlobalMetadataResponse::has_metadata,
                    GetGlobalMetadataResponse::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetGlobalMetadataResponse::has_error,
                    GetGlobalMetadataResponse::get_error,
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
    // message oneof groups
    response: ::std::option::Option<GetVertexMetadataResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVertexMetadataResponse {}

#[derive(Clone,PartialEq)]
pub enum GetVertexMetadataResponse_oneof_response {
    metadata(super::models::VertexMetadatas),
    error(::std::string::String),
}

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

    // .VertexMetadatas metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: super::models::VertexMetadatas) {
        self.response = ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut super::models::VertexMetadatas {
        if let ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(super::models::VertexMetadatas::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> super::models::VertexMetadatas {
        if self.has_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            super::models::VertexMetadatas::new()
        }
    }

    pub fn get_metadata(&self) -> &super::models::VertexMetadatas {
        match self.response {
            ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(ref v)) => v,
            _ => super::models::VertexMetadatas::default_instance(),
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for GetVertexMetadataResponse {
    fn is_initialized(&self) -> bool {
        if let Some(GetVertexMetadataResponse_oneof_response::metadata(ref v)) = self.response {
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
                    self.response = ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::metadata(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetVertexMetadataResponse_oneof_response::error(is.read_string()?));
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
                &GetVertexMetadataResponse_oneof_response::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GetVertexMetadataResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetVertexMetadataResponse_oneof_response::metadata(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GetVertexMetadataResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::models::VertexMetadatas>(
                    "metadata",
                    GetVertexMetadataResponse::has_metadata,
                    GetVertexMetadataResponse::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetVertexMetadataResponse::has_error,
                    GetVertexMetadataResponse::get_error,
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
    // message oneof groups
    response: ::std::option::Option<GetEdgeMetadataResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEdgeMetadataResponse {}

#[derive(Clone,PartialEq)]
pub enum GetEdgeMetadataResponse_oneof_response {
    metadata(super::models::VertexMetadatas),
    error(::std::string::String),
}

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

    // .VertexMetadatas metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: super::models::VertexMetadatas) {
        self.response = ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut super::models::VertexMetadatas {
        if let ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(super::models::VertexMetadatas::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> super::models::VertexMetadatas {
        if self.has_metadata() {
            match self.response.take() {
                ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            super::models::VertexMetadatas::new()
        }
    }

    pub fn get_metadata(&self) -> &super::models::VertexMetadatas {
        match self.response {
            ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(ref v)) => v,
            _ => super::models::VertexMetadatas::default_instance(),
        }
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.response = ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(::std::string::String::new()));
        }
        match self.response {
            ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.response {
            ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for GetEdgeMetadataResponse {
    fn is_initialized(&self) -> bool {
        if let Some(GetEdgeMetadataResponse_oneof_response::metadata(ref v)) = self.response {
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
                    self.response = ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::metadata(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(GetEdgeMetadataResponse_oneof_response::error(is.read_string()?));
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
                &GetEdgeMetadataResponse_oneof_response::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GetEdgeMetadataResponse_oneof_response::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
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
                &GetEdgeMetadataResponse_oneof_response::metadata(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GetEdgeMetadataResponse_oneof_response::error(ref v) => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::models::VertexMetadatas>(
                    "metadata",
                    GetEdgeMetadataResponse::has_metadata,
                    GetEdgeMetadataResponse::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error",
                    GetEdgeMetadataResponse::has_error,
                    GetEdgeMetadataResponse::get_error,
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1atransaction_response.proto\x1a\x0cmodels.proto\"\xce\t\n\x13Transa\
    ctionResponse\x12<\n\rcreate_vertex\x18\x01\x20\x01(\x0b2\x15.CreateVert\
    exResponseH\0R\x0ccreateVertex\x129\n\x0cget_vertices\x18\x02\x20\x01(\
    \x0b2\x14.GetVerticesResponseH\0R\x0bgetVertices\x12B\n\x0fdelete_vertic\
    es\x18\x03\x20\x01(\x0b2\x17.DeleteVerticesResponseH\0R\x0edeleteVertice\
    s\x12C\n\x10get_vertex_count\x18\x04\x20\x01(\x0b2\x17.GetVertexCountRes\
    ponseH\0R\x0egetVertexCount\x126\n\x0bcreate_edge\x18\x05\x20\x01(\x0b2\
    \x13.CreateEdgeResponseH\0R\ncreateEdge\x120\n\tget_edges\x18\x06\x20\
    \x01(\x0b2\x11.GetEdgesResponseH\0R\x08getEdges\x129\n\x0cdelete_edges\
    \x18\x07\x20\x01(\x0b2\x14.DeleteEdgesResponseH\0R\x0bdeleteEdges\x12=\n\
    \x0eget_edge_count\x18\x08\x20\x01(\x0b2\x15.GetEdgeCountResponseH\0R\
    \x0cgetEdgeCount\x12L\n\x13get_global_metadata\x18\t\x20\x01(\x0b2\x1a.G\
    etGlobalMetadataResponseH\0R\x11getGlobalMetadata\x12L\n\x13set_global_m\
    etadata\x18\n\x20\x01(\x0b2\x1a.SetGlobalMetadataResponseH\0R\x11setGlob\
    alMetadata\x12U\n\x16delete_global_metadata\x18\x0b\x20\x01(\x0b2\x1d.De\
    leteGlobalMetadataResponseH\0R\x14deleteGlobalMetadata\x12L\n\x13get_ver\
    tex_metadata\x18\x0c\x20\x01(\x0b2\x1a.GetVertexMetadataResponseH\0R\x11\
    getVertexMetadata\x12L\n\x13set_vertex_metadata\x18\r\x20\x01(\x0b2\x1a.\
    SetVertexMetadataResponseH\0R\x11setVertexMetadata\x12U\n\x16delete_vert\
    ex_metadata\x18\x0e\x20\x01(\x0b2\x1d.DeleteVertexMetadataResponseH\0R\
    \x14deleteVertexMetadata\x12F\n\x11get_edge_metadata\x18\x0f\x20\x01(\
    \x0b2\x18.GetEdgeMetadataResponseH\0R\x0fgetEdgeMetadata\x12F\n\x11set_e\
    dge_metadata\x18\x10\x20\x01(\x0b2\x18.SetEdgeMetadataResponseH\0R\x0fse\
    tEdgeMetadata\x12O\n\x14delete_edge_metadata\x18\x11\x20\x01(\x0b2\x1b.D\
    eleteEdgeMetadataResponseH\0R\x12deleteEdgeMetadataB\n\n\x08response\",\
    \n\x14CreateVertexResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05err\
    or\"b\n\x13GetVerticesResponse\x12'\n\x08vertices\x18\x01\x20\x01(\x0b2\
    \t.VerticesH\0R\x08vertices\x12\x16\n\x05error\x18\x02\x20\x01(\tH\0R\
    \x05errorB\n\n\x08response\".\n\x16DeleteVerticesResponse\x12\x14\n\x05e\
    rror\x18\x01\x20\x01(\tR\x05error\"T\n\x16GetVertexCountResponse\x12\x16\
    \n\x05count\x18\x01\x20\x01(\x04H\0R\x05count\x12\x16\n\x05error\x18\x02\
    \x20\x01(\tH\0R\x05errorB\n\n\x08response\"T\n\x12CreateEdgeResponse\x12\
    \x1a\n\x07created\x18\x01\x20\x01(\x08H\0R\x07created\x12\x16\n\x05error\
    \x18\x02\x20\x01(\tH\0R\x05errorB\n\n\x08response\"V\n\x10GetEdgesRespon\
    se\x12\x1e\n\x05edges\x18\x01\x20\x01(\x0b2\x06.EdgesH\0R\x05edges\x12\
    \x16\n\x05error\x18\x02\x20\x01(\tH\0R\x05errorB\n\n\x08response\"+\n\
    \x13DeleteEdgesResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\
    \"R\n\x14GetEdgeCountResponse\x12\x16\n\x05count\x18\x01\x20\x01(\x04H\0\
    R\x05count\x12\x16\n\x05error\x18\x02\x20\x01(\tH\0R\x05errorB\n\n\x08re\
    sponse\"]\n\x19GetGlobalMetadataResponse\x12\x1c\n\x08metadata\x18\x01\
    \x20\x01(\tH\0R\x08metadata\x12\x16\n\x05error\x18\x02\x20\x01(\tH\0R\
    \x05errorB\n\n\x08response\"1\n\x19SetGlobalMetadataResponse\x12\x14\n\
    \x05error\x18\x01\x20\x01(\tR\x05error\"4\n\x1cDeleteGlobalMetadataRespo\
    nse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\x05error\"o\n\x19GetVertexMet\
    adataResponse\x12.\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.VertexMetadat\
    asH\0R\x08metadata\x12\x16\n\x05error\x18\x02\x20\x01(\tH\0R\x05errorB\n\
    \n\x08response\"1\n\x19SetVertexMetadataResponse\x12\x14\n\x05error\x18\
    \x01\x20\x01(\tR\x05error\"4\n\x1cDeleteVertexMetadataResponse\x12\x14\n\
    \x05error\x18\x01\x20\x01(\tR\x05error\"m\n\x17GetEdgeMetadataResponse\
    \x12.\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.VertexMetadatasH\0R\x08met\
    adata\x12\x16\n\x05error\x18\x02\x20\x01(\tH\0R\x05errorB\n\n\x08respons\
    e\"/\n\x17SetEdgeMetadataResponse\x12\x14\n\x05error\x18\x01\x20\x01(\tR\
    \x05error\"2\n\x1aDeleteEdgeMetadataResponse\x12\x14\n\x05error\x18\x01\
    \x20\x01(\tR\x05errorJ\xd6\x18\n\x06\x12\x04\0\0t\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x15\n\n\n\x02\x04\0\x12\
    \x04\x04\0\x18\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x1b\n\x0c\n\x04\
    \x04\0\x08\0\x12\x04\x05\x04\x17\x05\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\
    \x05\n\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x08/\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03\x06\x08\x1c\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\
    \x1d*\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06-.\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x03\x07\x08-\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x07\x08\x1b\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x1c(\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x07+,\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x08\x083\n\x0c\
    \n\x05\x04\0\x02\x02\x06\x12\x03\x08\x08\x1e\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x03\x08\x1f.\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0812\n\x0b\
    \n\x04\x04\0\x02\x03\x12\x03\t\x084\n\x0c\n\x05\x04\0\x02\x03\x06\x12\
    \x03\t\x08\x1e\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\t\x1f/\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x03\t23\n\x0b\n\x04\x04\0\x02\x04\x12\x03\n\x08+\
    \n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03\n\x08\x1a\n\x0c\n\x05\x04\0\x02\
    \x04\x01\x12\x03\n\x1b&\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\n)*\n\x0b\
    \n\x04\x04\0\x02\x05\x12\x03\x0b\x08'\n\x0c\n\x05\x04\0\x02\x05\x06\x12\
    \x03\x0b\x08\x18\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0b\x19\"\n\x0c\n\
    \x05\x04\0\x02\x05\x03\x12\x03\x0b%&\n\x0b\n\x04\x04\0\x02\x06\x12\x03\
    \x0c\x08-\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03\x0c\x08\x1b\n\x0c\n\x05\
    \x04\0\x02\x06\x01\x12\x03\x0c\x1c(\n\x0c\n\x05\x04\0\x02\x06\x03\x12\
    \x03\x0c+,\n\x0b\n\x04\x04\0\x02\x07\x12\x03\r\x080\n\x0c\n\x05\x04\0\
    \x02\x07\x06\x12\x03\r\x08\x1c\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\r\
    \x1d+\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\r./\n\x0b\n\x04\x04\0\x02\
    \x08\x12\x03\x0e\x08:\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03\x0e\x08!\n\
    \x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x0e\"5\n\x0c\n\x05\x04\0\x02\x08\
    \x03\x12\x03\x0e89\n\x0b\n\x04\x04\0\x02\t\x12\x03\x0f\x08;\n\x0c\n\x05\
    \x04\0\x02\t\x06\x12\x03\x0f\x08!\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03\
    \x0f\"5\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\x0f8:\n\x0b\n\x04\x04\0\x02\
    \n\x12\x03\x10\x08A\n\x0c\n\x05\x04\0\x02\n\x06\x12\x03\x10\x08$\n\x0c\n\
    \x05\x04\0\x02\n\x01\x12\x03\x10%;\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\
    \x10>@\n\x0b\n\x04\x04\0\x02\x0b\x12\x03\x11\x08;\n\x0c\n\x05\x04\0\x02\
    \x0b\x06\x12\x03\x11\x08!\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03\x11\"5\n\
    \x0c\n\x05\x04\0\x02\x0b\x03\x12\x03\x118:\n\x0b\n\x04\x04\0\x02\x0c\x12\
    \x03\x12\x08;\n\x0c\n\x05\x04\0\x02\x0c\x06\x12\x03\x12\x08!\n\x0c\n\x05\
    \x04\0\x02\x0c\x01\x12\x03\x12\"5\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03\
    \x128:\n\x0b\n\x04\x04\0\x02\r\x12\x03\x13\x08A\n\x0c\n\x05\x04\0\x02\r\
    \x06\x12\x03\x13\x08$\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03\x13%;\n\x0c\n\
    \x05\x04\0\x02\r\x03\x12\x03\x13>@\n\x0b\n\x04\x04\0\x02\x0e\x12\x03\x14\
    \x087\n\x0c\n\x05\x04\0\x02\x0e\x06\x12\x03\x14\x08\x1f\n\x0c\n\x05\x04\
    \0\x02\x0e\x01\x12\x03\x14\x201\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03\
    \x1446\n\x0b\n\x04\x04\0\x02\x0f\x12\x03\x15\x087\n\x0c\n\x05\x04\0\x02\
    \x0f\x06\x12\x03\x15\x08\x1f\n\x0c\n\x05\x04\0\x02\x0f\x01\x12\x03\x15\
    \x201\n\x0c\n\x05\x04\0\x02\x0f\x03\x12\x03\x1546\n\x0b\n\x04\x04\0\x02\
    \x10\x12\x03\x16\x08=\n\x0c\n\x05\x04\0\x02\x10\x06\x12\x03\x16\x08\"\n\
    \x0c\n\x05\x04\0\x02\x10\x01\x12\x03\x16#7\n\x0c\n\x05\x04\0\x02\x10\x03\
    \x12\x03\x16:<\n\n\n\x02\x04\x01\x12\x04\x1a\0\x1c\x01\n\n\n\x03\x04\x01\
    \x01\x12\x03\x1a\x08\x1c\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1b\x04\x15\n\
    \r\n\x05\x04\x01\x02\0\x04\x12\x04\x1b\x04\x1a\x1e\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\x1b\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1b\
    \x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1b\x13\x14\n\n\n\x02\x04\
    \x02\x12\x04\x1e\0#\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1e\x08\x1b\n\x0c\
    \n\x04\x04\x02\x08\0\x12\x04\x1f\x04\"\x05\n\x0c\n\x05\x04\x02\x08\0\x01\
    \x12\x03\x1f\n\x12\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x20\x08\x1e\n\x0c\n\
    \x05\x04\x02\x02\0\x06\x12\x03\x20\x08\x10\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03\x20\x11\x19\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x20\x1c\x1d\n\
    \x0b\n\x04\x04\x02\x02\x01\x12\x03!\x08\x19\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03!\x08\x0e\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03!\x0f\x14\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03!\x17\x18\n\n\n\x02\x04\x03\x12\
    \x04%\0'\x01\n\n\n\x03\x04\x03\x01\x12\x03%\x08\x1e\n\x0b\n\x04\x04\x03\
    \x02\0\x12\x03&\x04\x15\n\r\n\x05\x04\x03\x02\0\x04\x12\x04&\x04%\x20\n\
    \x0c\n\x05\x04\x03\x02\0\x05\x12\x03&\x04\n\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03&\x0b\x10\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03&\x13\x14\n\n\
    \n\x02\x04\x04\x12\x04)\0.\x01\n\n\n\x03\x04\x04\x01\x12\x03)\x08\x1e\n\
    \x0c\n\x04\x04\x04\x08\0\x12\x04*\x04-\x05\n\x0c\n\x05\x04\x04\x08\0\x01\
    \x12\x03*\n\x12\n\x0b\n\x04\x04\x04\x02\0\x12\x03+\x08\x19\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03+\x08\x0e\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03+\x0f\x14\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03+\x17\x18\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03,\x08\x19\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\
    \x03,\x08\x0e\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03,\x0f\x14\n\x0c\n\
    \x05\x04\x04\x02\x01\x03\x12\x03,\x17\x18\n\n\n\x02\x04\x05\x12\x040\05\
    \x01\n\n\n\x03\x04\x05\x01\x12\x030\x08\x1a\n\x0c\n\x04\x04\x05\x08\0\
    \x12\x041\x044\x05\n\x0c\n\x05\x04\x05\x08\0\x01\x12\x031\n\x12\n\x0b\n\
    \x04\x04\x05\x02\0\x12\x032\x08\x19\n\x0c\n\x05\x04\x05\x02\0\x05\x12\
    \x032\x08\x0c\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x032\r\x14\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x032\x17\x18\n\x0b\n\x04\x04\x05\x02\x01\x12\x033\
    \x08\x19\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x033\x08\x0e\n\x0c\n\x05\
    \x04\x05\x02\x01\x01\x12\x033\x0f\x14\n\x0c\n\x05\x04\x05\x02\x01\x03\
    \x12\x033\x17\x18\n\n\n\x02\x04\x06\x12\x047\0<\x01\n\n\n\x03\x04\x06\
    \x01\x12\x037\x08\x18\n\x0c\n\x04\x04\x06\x08\0\x12\x048\x04;\x05\n\x0c\
    \n\x05\x04\x06\x08\0\x01\x12\x038\n\x12\n\x0b\n\x04\x04\x06\x02\0\x12\
    \x039\x08\x18\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x039\x08\r\n\x0c\n\x05\
    \x04\x06\x02\0\x01\x12\x039\x0e\x13\n\x0c\n\x05\x04\x06\x02\0\x03\x12\
    \x039\x16\x17\n\x0b\n\x04\x04\x06\x02\x01\x12\x03:\x08\x19\n\x0c\n\x05\
    \x04\x06\x02\x01\x05\x12\x03:\x08\x0e\n\x0c\n\x05\x04\x06\x02\x01\x01\
    \x12\x03:\x0f\x14\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03:\x17\x18\n\n\n\
    \x02\x04\x07\x12\x04>\0@\x01\n\n\n\x03\x04\x07\x01\x12\x03>\x08\x1b\n\
    \x0b\n\x04\x04\x07\x02\0\x12\x03?\x04\x15\n\r\n\x05\x04\x07\x02\0\x04\
    \x12\x04?\x04>\x1d\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03?\x04\n\n\x0c\n\
    \x05\x04\x07\x02\0\x01\x12\x03?\x0b\x10\n\x0c\n\x05\x04\x07\x02\0\x03\
    \x12\x03?\x13\x14\n\n\n\x02\x04\x08\x12\x04B\0G\x01\n\n\n\x03\x04\x08\
    \x01\x12\x03B\x08\x1c\n\x0c\n\x04\x04\x08\x08\0\x12\x04C\x04F\x05\n\x0c\
    \n\x05\x04\x08\x08\0\x01\x12\x03C\n\x12\n\x0b\n\x04\x04\x08\x02\0\x12\
    \x03D\x08\x19\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03D\x08\x0e\n\x0c\n\x05\
    \x04\x08\x02\0\x01\x12\x03D\x0f\x14\n\x0c\n\x05\x04\x08\x02\0\x03\x12\
    \x03D\x17\x18\n\x0b\n\x04\x04\x08\x02\x01\x12\x03E\x08\x19\n\x0c\n\x05\
    \x04\x08\x02\x01\x05\x12\x03E\x08\x0e\n\x0c\n\x05\x04\x08\x02\x01\x01\
    \x12\x03E\x0f\x14\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03E\x17\x18\n\n\n\
    \x02\x04\t\x12\x04I\0N\x01\n\n\n\x03\x04\t\x01\x12\x03I\x08!\n\x0c\n\x04\
    \x04\t\x08\0\x12\x04J\x04M\x05\n\x0c\n\x05\x04\t\x08\0\x01\x12\x03J\n\
    \x12\n\x0b\n\x04\x04\t\x02\0\x12\x03K\x08\x1c\n\x0c\n\x05\x04\t\x02\0\
    \x05\x12\x03K\x08\x0e\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03K\x0f\x17\n\x0c\
    \n\x05\x04\t\x02\0\x03\x12\x03K\x1a\x1b\n\x0b\n\x04\x04\t\x02\x01\x12\
    \x03L\x08\x19\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03L\x08\x0e\n\x0c\n\x05\
    \x04\t\x02\x01\x01\x12\x03L\x0f\x14\n\x0c\n\x05\x04\t\x02\x01\x03\x12\
    \x03L\x17\x18\n\n\n\x02\x04\n\x12\x04P\0R\x01\n\n\n\x03\x04\n\x01\x12\
    \x03P\x08!\n\x0b\n\x04\x04\n\x02\0\x12\x03Q\x04\x15\n\r\n\x05\x04\n\x02\
    \0\x04\x12\x04Q\x04P#\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03Q\x04\n\n\x0c\n\
    \x05\x04\n\x02\0\x01\x12\x03Q\x0b\x10\n\x0c\n\x05\x04\n\x02\0\x03\x12\
    \x03Q\x13\x14\n\n\n\x02\x04\x0b\x12\x04T\0V\x01\n\n\n\x03\x04\x0b\x01\
    \x12\x03T\x08$\n\x0b\n\x04\x04\x0b\x02\0\x12\x03U\x04\x15\n\r\n\x05\x04\
    \x0b\x02\0\x04\x12\x04U\x04T&\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03U\x04\
    \n\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03U\x0b\x10\n\x0c\n\x05\x04\x0b\
    \x02\0\x03\x12\x03U\x13\x14\n\n\n\x02\x04\x0c\x12\x04X\0]\x01\n\n\n\x03\
    \x04\x0c\x01\x12\x03X\x08!\n\x0c\n\x04\x04\x0c\x08\0\x12\x04Y\x04\\\x05\
    \n\x0c\n\x05\x04\x0c\x08\0\x01\x12\x03Y\n\x12\n\x0b\n\x04\x04\x0c\x02\0\
    \x12\x03Z\x08%\n\x0c\n\x05\x04\x0c\x02\0\x06\x12\x03Z\x08\x17\n\x0c\n\
    \x05\x04\x0c\x02\0\x01\x12\x03Z\x18\x20\n\x0c\n\x05\x04\x0c\x02\0\x03\
    \x12\x03Z#$\n\x0b\n\x04\x04\x0c\x02\x01\x12\x03[\x08\x19\n\x0c\n\x05\x04\
    \x0c\x02\x01\x05\x12\x03[\x08\x0e\n\x0c\n\x05\x04\x0c\x02\x01\x01\x12\
    \x03[\x0f\x14\n\x0c\n\x05\x04\x0c\x02\x01\x03\x12\x03[\x17\x18\n\n\n\x02\
    \x04\r\x12\x04_\0a\x01\n\n\n\x03\x04\r\x01\x12\x03_\x08!\n\x0b\n\x04\x04\
    \r\x02\0\x12\x03`\x04\x15\n\r\n\x05\x04\r\x02\0\x04\x12\x04`\x04_#\n\x0c\
    \n\x05\x04\r\x02\0\x05\x12\x03`\x04\n\n\x0c\n\x05\x04\r\x02\0\x01\x12\
    \x03`\x0b\x10\n\x0c\n\x05\x04\r\x02\0\x03\x12\x03`\x13\x14\n\n\n\x02\x04\
    \x0e\x12\x04c\0e\x01\n\n\n\x03\x04\x0e\x01\x12\x03c\x08$\n\x0b\n\x04\x04\
    \x0e\x02\0\x12\x03d\x04\x15\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04d\x04c&\n\
    \x0c\n\x05\x04\x0e\x02\0\x05\x12\x03d\x04\n\n\x0c\n\x05\x04\x0e\x02\0\
    \x01\x12\x03d\x0b\x10\n\x0c\n\x05\x04\x0e\x02\0\x03\x12\x03d\x13\x14\n\n\
    \n\x02\x04\x0f\x12\x04g\0l\x01\n\n\n\x03\x04\x0f\x01\x12\x03g\x08\x1f\n\
    \x0c\n\x04\x04\x0f\x08\0\x12\x04h\x04k\x05\n\x0c\n\x05\x04\x0f\x08\0\x01\
    \x12\x03h\n\x12\n\x0b\n\x04\x04\x0f\x02\0\x12\x03i\x08%\n\x0c\n\x05\x04\
    \x0f\x02\0\x06\x12\x03i\x08\x17\n\x0c\n\x05\x04\x0f\x02\0\x01\x12\x03i\
    \x18\x20\n\x0c\n\x05\x04\x0f\x02\0\x03\x12\x03i#$\n\x0b\n\x04\x04\x0f\
    \x02\x01\x12\x03j\x08\x19\n\x0c\n\x05\x04\x0f\x02\x01\x05\x12\x03j\x08\
    \x0e\n\x0c\n\x05\x04\x0f\x02\x01\x01\x12\x03j\x0f\x14\n\x0c\n\x05\x04\
    \x0f\x02\x01\x03\x12\x03j\x17\x18\n\n\n\x02\x04\x10\x12\x04n\0p\x01\n\n\
    \n\x03\x04\x10\x01\x12\x03n\x08\x1f\n\x0b\n\x04\x04\x10\x02\0\x12\x03o\
    \x04\x15\n\r\n\x05\x04\x10\x02\0\x04\x12\x04o\x04n!\n\x0c\n\x05\x04\x10\
    \x02\0\x05\x12\x03o\x04\n\n\x0c\n\x05\x04\x10\x02\0\x01\x12\x03o\x0b\x10\
    \n\x0c\n\x05\x04\x10\x02\0\x03\x12\x03o\x13\x14\n\n\n\x02\x04\x11\x12\
    \x04r\0t\x01\n\n\n\x03\x04\x11\x01\x12\x03r\x08\"\n\x0b\n\x04\x04\x11\
    \x02\0\x12\x03s\x04\x15\n\r\n\x05\x04\x11\x02\0\x04\x12\x04s\x04r$\n\x0c\
    \n\x05\x04\x11\x02\0\x05\x12\x03s\x04\n\n\x0c\n\x05\x04\x11\x02\0\x01\
    \x12\x03s\x0b\x10\n\x0c\n\x05\x04\x11\x02\0\x03\x12\x03s\x13\x14b\x06pro\
    to3\
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
