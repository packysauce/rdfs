// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `editlog.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct AclEditLogProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    pub entries: ::protobuf::RepeatedField<super::acl::AclEntryProto>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AclEditLogProto {
    fn default() -> &'a AclEditLogProto {
        <AclEditLogProto as ::protobuf::Message>::default_instance()
    }
}

impl AclEditLogProto {
    pub fn new() -> AclEditLogProto {
        ::std::default::Default::default()
    }

    // required string src = 1;


    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // repeated .hadoop.hdfs.AclEntryProto entries = 2;


    pub fn get_entries(&self) -> &[super::acl::AclEntryProto] {
        &self.entries
    }
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<super::acl::AclEntryProto>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<super::acl::AclEntryProto> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<super::acl::AclEntryProto> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for AclEditLogProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.entries {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> AclEditLogProto {
        AclEditLogProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "src",
                |m: &AclEditLogProto| { &m.src },
                |m: &mut AclEditLogProto| { &mut m.src },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::acl::AclEntryProto>>(
                "entries",
                |m: &AclEditLogProto| { &m.entries },
                |m: &mut AclEditLogProto| { &mut m.entries },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AclEditLogProto>(
                "AclEditLogProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AclEditLogProto {
        static instance: ::protobuf::rt::LazyV2<AclEditLogProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AclEditLogProto::new)
    }
}

impl ::protobuf::Clear for AclEditLogProto {
    fn clear(&mut self) {
        self.src.clear();
        self.entries.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AclEditLogProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AclEditLogProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct XAttrEditLogProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    pub xAttrs: ::protobuf::RepeatedField<super::xattr::XAttrProto>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a XAttrEditLogProto {
    fn default() -> &'a XAttrEditLogProto {
        <XAttrEditLogProto as ::protobuf::Message>::default_instance()
    }
}

impl XAttrEditLogProto {
    pub fn new() -> XAttrEditLogProto {
        ::std::default::Default::default()
    }

    // optional string src = 1;


    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 2;


    pub fn get_xAttrs(&self) -> &[super::xattr::XAttrProto] {
        &self.xAttrs
    }
    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<super::xattr::XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<super::xattr::XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<super::xattr::XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for XAttrEditLogProto {
    fn is_initialized(&self) -> bool {
        for v in &self.xAttrs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.xAttrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.xAttrs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> XAttrEditLogProto {
        XAttrEditLogProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "src",
                |m: &XAttrEditLogProto| { &m.src },
                |m: &mut XAttrEditLogProto| { &mut m.src },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::xattr::XAttrProto>>(
                "xAttrs",
                |m: &XAttrEditLogProto| { &m.xAttrs },
                |m: &mut XAttrEditLogProto| { &mut m.xAttrs },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<XAttrEditLogProto>(
                "XAttrEditLogProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static XAttrEditLogProto {
        static instance: ::protobuf::rt::LazyV2<XAttrEditLogProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(XAttrEditLogProto::new)
    }
}

impl ::protobuf::Clear for XAttrEditLogProto {
    fn clear(&mut self) {
        self.src.clear();
        self.xAttrs.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for XAttrEditLogProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for XAttrEditLogProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\reditlog.proto\x12\x0bhadoop.hdfs\x1a\tacl.proto\x1a\x0bxattr.proto\"\
    _\n\x0fAclEditLogProto\x12\x12\n\x03src\x18\x01\x20\x02(\tR\x03srcB\0\
    \x126\n\x07entries\x18\x02\x20\x03(\x0b2\x1a.hadoop.hdfs.AclEntryProtoR\
    \x07entriesB\0:\0\"\\\n\x11XAttrEditLogProto\x12\x12\n\x03src\x18\x01\
    \x20\x01(\tR\x03srcB\0\x121\n\x06xAttrs\x18\x02\x20\x03(\x0b2\x17.hadoop\
    .hdfs.XAttrProtoR\x06xAttrsB\0:\0B\0b\x06proto2\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}