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
pub struct Feed {
    // message fields
    discoveryKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Feed {}

impl Feed {
    pub fn new() -> Feed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Feed {
        static mut instance: ::protobuf::lazy::Lazy<Feed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Feed,
        };
        unsafe {
            instance.get(Feed::new)
        }
    }

    // required bytes discoveryKey = 1;

    pub fn clear_discoveryKey(&mut self) {
        self.discoveryKey.clear();
    }

    pub fn has_discoveryKey(&self) -> bool {
        self.discoveryKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_discoveryKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.discoveryKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_discoveryKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.discoveryKey.is_none() {
            self.discoveryKey.set_default();
        }
        self.discoveryKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_discoveryKey(&mut self) -> ::std::vec::Vec<u8> {
        self.discoveryKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_discoveryKey(&self) -> &[u8] {
        match self.discoveryKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_discoveryKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.discoveryKey
    }

    fn mut_discoveryKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.discoveryKey
    }

    // optional bytes nonce = 2;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.nonce.is_none() {
            self.nonce.set_default();
        }
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_nonce(&self) -> &[u8] {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.nonce
    }
}

impl ::protobuf::Message for Feed {
    fn is_initialized(&self) -> bool {
        if self.discoveryKey.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.discoveryKey)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.nonce)?;
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
        if let Some(ref v) = self.discoveryKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.discoveryKey.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.nonce.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for Feed {
    fn new() -> Feed {
        Feed::new()
    }

    fn descriptor_static(_: ::std::option::Option<Feed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "discoveryKey",
                    Feed::get_discoveryKey_for_reflect,
                    Feed::mut_discoveryKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nonce",
                    Feed::get_nonce_for_reflect,
                    Feed::mut_nonce_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Feed>(
                    "Feed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Feed {
    fn clear(&mut self) {
        self.clear_discoveryKey();
        self.clear_nonce();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Feed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Feed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nfeed.proto\"@\n\x04Feed\x12\"\n\x0cdiscoveryKey\x18\x01\x20\x02(\x0c\
    R\x0cdiscoveryKey\x12\x14\n\x05nonce\x18\x02\x20\x01(\x0cR\x05nonce\
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
