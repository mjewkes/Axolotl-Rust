// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct TSProtoWhisperMessage {
    // message fields
    ratchetKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    counter: ::std::option::Option<u32>,
    previousCounter: ::std::option::Option<u32>,
    ciphertext: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TSProtoWhisperMessage {
    pub fn new() -> TSProtoWhisperMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSProtoWhisperMessage {
        static mut instance: ::protobuf::lazy::Lazy<TSProtoWhisperMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSProtoWhisperMessage,
        };
        unsafe {
            instance.get(|| {
                TSProtoWhisperMessage {
                    ratchetKey: ::protobuf::SingularField::none(),
                    counter: ::std::option::Option::None,
                    previousCounter: ::std::option::Option::None,
                    ciphertext: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes ratchetKey = 1;

    pub fn clear_ratchetKey(&mut self) {
        self.ratchetKey.clear();
    }

    pub fn has_ratchetKey(&self) -> bool {
        self.ratchetKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ratchetKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.ratchetKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ratchetKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.ratchetKey.is_none() {
            self.ratchetKey.set_default();
        };
        self.ratchetKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_ratchetKey(&mut self) -> ::std::vec::Vec<u8> {
        self.ratchetKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ratchetKey<'a>(&'a self) -> &'a [u8] {
        match self.ratchetKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 counter = 2;

    pub fn clear_counter(&mut self) {
        self.counter = ::std::option::Option::None;
    }

    pub fn has_counter(&self) -> bool {
        self.counter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter(&mut self, v: u32) {
        self.counter = ::std::option::Option::Some(v);
    }

    pub fn get_counter<'a>(&self) -> u32 {
        self.counter.unwrap_or(0)
    }

    // optional uint32 previousCounter = 3;

    pub fn clear_previousCounter(&mut self) {
        self.previousCounter = ::std::option::Option::None;
    }

    pub fn has_previousCounter(&self) -> bool {
        self.previousCounter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previousCounter(&mut self, v: u32) {
        self.previousCounter = ::std::option::Option::Some(v);
    }

    pub fn get_previousCounter<'a>(&self) -> u32 {
        self.previousCounter.unwrap_or(0)
    }

    // optional bytes ciphertext = 4;

    pub fn clear_ciphertext(&mut self) {
        self.ciphertext.clear();
    }

    pub fn has_ciphertext(&self) -> bool {
        self.ciphertext.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ciphertext(&mut self, v: ::std::vec::Vec<u8>) {
        self.ciphertext = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphertext<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.ciphertext.is_none() {
            self.ciphertext.set_default();
        };
        self.ciphertext.as_mut().unwrap()
    }

    // Take field
    pub fn take_ciphertext(&mut self) -> ::std::vec::Vec<u8> {
        self.ciphertext.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ciphertext<'a>(&'a self) -> &'a [u8] {
        match self.ciphertext.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TSProtoWhisperMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ratchetKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.counter = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.previousCounter = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ciphertext.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ratchetKey.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.counter.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.previousCounter.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ciphertext.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ratchetKey.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.counter {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.previousCounter {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.ciphertext.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSProtoWhisperMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSProtoWhisperMessage {
    fn new() -> TSProtoWhisperMessage {
        TSProtoWhisperMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSProtoWhisperMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ratchetKey",
                    TSProtoWhisperMessage::has_ratchetKey,
                    TSProtoWhisperMessage::get_ratchetKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "counter",
                    TSProtoWhisperMessage::has_counter,
                    TSProtoWhisperMessage::get_counter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "previousCounter",
                    TSProtoWhisperMessage::has_previousCounter,
                    TSProtoWhisperMessage::get_previousCounter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ciphertext",
                    TSProtoWhisperMessage::has_ciphertext,
                    TSProtoWhisperMessage::get_ciphertext,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSProtoWhisperMessage>(
                    "TSProtoWhisperMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSProtoWhisperMessage {
    fn clear(&mut self) {
        self.clear_ratchetKey();
        self.clear_counter();
        self.clear_previousCounter();
        self.clear_ciphertext();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSProtoWhisperMessage {
    fn eq(&self, other: &TSProtoWhisperMessage) -> bool {
        self.ratchetKey == other.ratchetKey &&
        self.counter == other.counter &&
        self.previousCounter == other.previousCounter &&
        self.ciphertext == other.ciphertext &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSProtoWhisperMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSProtoPreKeyWhisperMessage {
    // message fields
    registrationId: ::std::option::Option<u32>,
    preKeyId: ::std::option::Option<u32>,
    signedPreKeyId: ::std::option::Option<u32>,
    baseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    identityKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    message: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TSProtoPreKeyWhisperMessage {
    pub fn new() -> TSProtoPreKeyWhisperMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSProtoPreKeyWhisperMessage {
        static mut instance: ::protobuf::lazy::Lazy<TSProtoPreKeyWhisperMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSProtoPreKeyWhisperMessage,
        };
        unsafe {
            instance.get(|| {
                TSProtoPreKeyWhisperMessage {
                    registrationId: ::std::option::Option::None,
                    preKeyId: ::std::option::Option::None,
                    signedPreKeyId: ::std::option::Option::None,
                    baseKey: ::protobuf::SingularField::none(),
                    identityKey: ::protobuf::SingularField::none(),
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 registrationId = 5;

    pub fn clear_registrationId(&mut self) {
        self.registrationId = ::std::option::Option::None;
    }

    pub fn has_registrationId(&self) -> bool {
        self.registrationId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registrationId(&mut self, v: u32) {
        self.registrationId = ::std::option::Option::Some(v);
    }

    pub fn get_registrationId<'a>(&self) -> u32 {
        self.registrationId.unwrap_or(0)
    }

    // optional uint32 preKeyId = 1;

    pub fn clear_preKeyId(&mut self) {
        self.preKeyId = ::std::option::Option::None;
    }

    pub fn has_preKeyId(&self) -> bool {
        self.preKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preKeyId(&mut self, v: u32) {
        self.preKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_preKeyId<'a>(&self) -> u32 {
        self.preKeyId.unwrap_or(0)
    }

    // optional uint32 signedPreKeyId = 6;

    pub fn clear_signedPreKeyId(&mut self) {
        self.signedPreKeyId = ::std::option::Option::None;
    }

    pub fn has_signedPreKeyId(&self) -> bool {
        self.signedPreKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signedPreKeyId(&mut self, v: u32) {
        self.signedPreKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_signedPreKeyId<'a>(&self) -> u32 {
        self.signedPreKeyId.unwrap_or(0)
    }

    // optional bytes baseKey = 2;

    pub fn clear_baseKey(&mut self) {
        self.baseKey.clear();
    }

    pub fn has_baseKey(&self) -> bool {
        self.baseKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.baseKey.is_none() {
            self.baseKey.set_default();
        };
        self.baseKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseKey(&mut self) -> ::std::vec::Vec<u8> {
        self.baseKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseKey<'a>(&'a self) -> &'a [u8] {
        match self.baseKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes identityKey = 3;

    pub fn clear_identityKey(&mut self) {
        self.identityKey.clear();
    }

    pub fn has_identityKey(&self) -> bool {
        self.identityKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identityKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.identityKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identityKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.identityKey.is_none() {
            self.identityKey.set_default();
        };
        self.identityKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_identityKey(&mut self) -> ::std::vec::Vec<u8> {
        self.identityKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_identityKey<'a>(&'a self) -> &'a [u8] {
        match self.identityKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes message = 4;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::vec::Vec<u8> {
        self.message.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a [u8] {
        match self.message.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TSProtoPreKeyWhisperMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.registrationId = ::std::option::Option::Some(tmp);
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.preKeyId = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.signedPreKeyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.baseKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.identityKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.registrationId.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.preKeyId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.signedPreKeyId.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.baseKey.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.identityKey.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.message.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.registrationId {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.preKeyId {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.signedPreKeyId {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.baseKey.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.identityKey.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.message.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSProtoPreKeyWhisperMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSProtoPreKeyWhisperMessage {
    fn new() -> TSProtoPreKeyWhisperMessage {
        TSProtoPreKeyWhisperMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSProtoPreKeyWhisperMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "registrationId",
                    TSProtoPreKeyWhisperMessage::has_registrationId,
                    TSProtoPreKeyWhisperMessage::get_registrationId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "preKeyId",
                    TSProtoPreKeyWhisperMessage::has_preKeyId,
                    TSProtoPreKeyWhisperMessage::get_preKeyId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "signedPreKeyId",
                    TSProtoPreKeyWhisperMessage::has_signedPreKeyId,
                    TSProtoPreKeyWhisperMessage::get_signedPreKeyId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "baseKey",
                    TSProtoPreKeyWhisperMessage::has_baseKey,
                    TSProtoPreKeyWhisperMessage::get_baseKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "identityKey",
                    TSProtoPreKeyWhisperMessage::has_identityKey,
                    TSProtoPreKeyWhisperMessage::get_identityKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "message",
                    TSProtoPreKeyWhisperMessage::has_message,
                    TSProtoPreKeyWhisperMessage::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSProtoPreKeyWhisperMessage>(
                    "TSProtoPreKeyWhisperMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSProtoPreKeyWhisperMessage {
    fn clear(&mut self) {
        self.clear_registrationId();
        self.clear_preKeyId();
        self.clear_signedPreKeyId();
        self.clear_baseKey();
        self.clear_identityKey();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSProtoPreKeyWhisperMessage {
    fn eq(&self, other: &TSProtoPreKeyWhisperMessage) -> bool {
        self.registrationId == other.registrationId &&
        self.preKeyId == other.preKeyId &&
        self.signedPreKeyId == other.signedPreKeyId &&
        self.baseKey == other.baseKey &&
        self.identityKey == other.identityKey &&
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSProtoPreKeyWhisperMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSProtoKeyExchangeMessage {
    // message fields
    id: ::std::option::Option<u32>,
    baseKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ratchetKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    identityKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    baseKeySignature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TSProtoKeyExchangeMessage {
    pub fn new() -> TSProtoKeyExchangeMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSProtoKeyExchangeMessage {
        static mut instance: ::protobuf::lazy::Lazy<TSProtoKeyExchangeMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSProtoKeyExchangeMessage,
        };
        unsafe {
            instance.get(|| {
                TSProtoKeyExchangeMessage {
                    id: ::std::option::Option::None,
                    baseKey: ::protobuf::SingularField::none(),
                    ratchetKey: ::protobuf::SingularField::none(),
                    identityKey: ::protobuf::SingularField::none(),
                    baseKeySignature: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // optional bytes baseKey = 2;

    pub fn clear_baseKey(&mut self) {
        self.baseKey.clear();
    }

    pub fn has_baseKey(&self) -> bool {
        self.baseKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.baseKey.is_none() {
            self.baseKey.set_default();
        };
        self.baseKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseKey(&mut self) -> ::std::vec::Vec<u8> {
        self.baseKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseKey<'a>(&'a self) -> &'a [u8] {
        match self.baseKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes ratchetKey = 3;

    pub fn clear_ratchetKey(&mut self) {
        self.ratchetKey.clear();
    }

    pub fn has_ratchetKey(&self) -> bool {
        self.ratchetKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ratchetKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.ratchetKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ratchetKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.ratchetKey.is_none() {
            self.ratchetKey.set_default();
        };
        self.ratchetKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_ratchetKey(&mut self) -> ::std::vec::Vec<u8> {
        self.ratchetKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ratchetKey<'a>(&'a self) -> &'a [u8] {
        match self.ratchetKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes identityKey = 4;

    pub fn clear_identityKey(&mut self) {
        self.identityKey.clear();
    }

    pub fn has_identityKey(&self) -> bool {
        self.identityKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identityKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.identityKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identityKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.identityKey.is_none() {
            self.identityKey.set_default();
        };
        self.identityKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_identityKey(&mut self) -> ::std::vec::Vec<u8> {
        self.identityKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_identityKey<'a>(&'a self) -> &'a [u8] {
        match self.identityKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes baseKeySignature = 5;

    pub fn clear_baseKeySignature(&mut self) {
        self.baseKeySignature.clear();
    }

    pub fn has_baseKeySignature(&self) -> bool {
        self.baseKeySignature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseKeySignature(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseKeySignature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseKeySignature<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.baseKeySignature.is_none() {
            self.baseKeySignature.set_default();
        };
        self.baseKeySignature.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseKeySignature(&mut self) -> ::std::vec::Vec<u8> {
        self.baseKeySignature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseKeySignature<'a>(&'a self) -> &'a [u8] {
        match self.baseKeySignature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TSProtoKeyExchangeMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.baseKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ratchetKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.identityKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.baseKeySignature.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.baseKey.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.ratchetKey.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.identityKey.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.baseKeySignature.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.baseKey.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.ratchetKey.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.identityKey.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.baseKeySignature.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSProtoKeyExchangeMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSProtoKeyExchangeMessage {
    fn new() -> TSProtoKeyExchangeMessage {
        TSProtoKeyExchangeMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSProtoKeyExchangeMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    TSProtoKeyExchangeMessage::has_id,
                    TSProtoKeyExchangeMessage::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "baseKey",
                    TSProtoKeyExchangeMessage::has_baseKey,
                    TSProtoKeyExchangeMessage::get_baseKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ratchetKey",
                    TSProtoKeyExchangeMessage::has_ratchetKey,
                    TSProtoKeyExchangeMessage::get_ratchetKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "identityKey",
                    TSProtoKeyExchangeMessage::has_identityKey,
                    TSProtoKeyExchangeMessage::get_identityKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "baseKeySignature",
                    TSProtoKeyExchangeMessage::has_baseKeySignature,
                    TSProtoKeyExchangeMessage::get_baseKeySignature,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSProtoKeyExchangeMessage>(
                    "TSProtoKeyExchangeMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSProtoKeyExchangeMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_baseKey();
        self.clear_ratchetKey();
        self.clear_identityKey();
        self.clear_baseKeySignature();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSProtoKeyExchangeMessage {
    fn eq(&self, other: &TSProtoKeyExchangeMessage) -> bool {
        self.id == other.id &&
        self.baseKey == other.baseKey &&
        self.ratchetKey == other.ratchetKey &&
        self.identityKey == other.identityKey &&
        self.baseKeySignature == other.baseKeySignature &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSProtoKeyExchangeMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSProtoSenderKeyMessage {
    // message fields
    id: ::std::option::Option<u32>,
    iteration: ::std::option::Option<u32>,
    ciphertext: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TSProtoSenderKeyMessage {
    pub fn new() -> TSProtoSenderKeyMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSProtoSenderKeyMessage {
        static mut instance: ::protobuf::lazy::Lazy<TSProtoSenderKeyMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSProtoSenderKeyMessage,
        };
        unsafe {
            instance.get(|| {
                TSProtoSenderKeyMessage {
                    id: ::std::option::Option::None,
                    iteration: ::std::option::Option::None,
                    ciphertext: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // optional uint32 iteration = 2;

    pub fn clear_iteration(&mut self) {
        self.iteration = ::std::option::Option::None;
    }

    pub fn has_iteration(&self) -> bool {
        self.iteration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iteration(&mut self, v: u32) {
        self.iteration = ::std::option::Option::Some(v);
    }

    pub fn get_iteration<'a>(&self) -> u32 {
        self.iteration.unwrap_or(0)
    }

    // optional bytes ciphertext = 3;

    pub fn clear_ciphertext(&mut self) {
        self.ciphertext.clear();
    }

    pub fn has_ciphertext(&self) -> bool {
        self.ciphertext.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ciphertext(&mut self, v: ::std::vec::Vec<u8>) {
        self.ciphertext = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphertext<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.ciphertext.is_none() {
            self.ciphertext.set_default();
        };
        self.ciphertext.as_mut().unwrap()
    }

    // Take field
    pub fn take_ciphertext(&mut self) -> ::std::vec::Vec<u8> {
        self.ciphertext.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ciphertext<'a>(&'a self) -> &'a [u8] {
        match self.ciphertext.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TSProtoSenderKeyMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.iteration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ciphertext.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.iteration.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ciphertext.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.iteration {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.ciphertext.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSProtoSenderKeyMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSProtoSenderKeyMessage {
    fn new() -> TSProtoSenderKeyMessage {
        TSProtoSenderKeyMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSProtoSenderKeyMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    TSProtoSenderKeyMessage::has_id,
                    TSProtoSenderKeyMessage::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "iteration",
                    TSProtoSenderKeyMessage::has_iteration,
                    TSProtoSenderKeyMessage::get_iteration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ciphertext",
                    TSProtoSenderKeyMessage::has_ciphertext,
                    TSProtoSenderKeyMessage::get_ciphertext,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSProtoSenderKeyMessage>(
                    "TSProtoSenderKeyMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSProtoSenderKeyMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_iteration();
        self.clear_ciphertext();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSProtoSenderKeyMessage {
    fn eq(&self, other: &TSProtoSenderKeyMessage) -> bool {
        self.id == other.id &&
        self.iteration == other.iteration &&
        self.ciphertext == other.ciphertext &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSProtoSenderKeyMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSProtoSenderKeyDistributionMessage {
    // message fields
    id: ::std::option::Option<u32>,
    iteration: ::std::option::Option<u32>,
    chainKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signingKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TSProtoSenderKeyDistributionMessage {
    pub fn new() -> TSProtoSenderKeyDistributionMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSProtoSenderKeyDistributionMessage {
        static mut instance: ::protobuf::lazy::Lazy<TSProtoSenderKeyDistributionMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSProtoSenderKeyDistributionMessage,
        };
        unsafe {
            instance.get(|| {
                TSProtoSenderKeyDistributionMessage {
                    id: ::std::option::Option::None,
                    iteration: ::std::option::Option::None,
                    chainKey: ::protobuf::SingularField::none(),
                    signingKey: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // optional uint32 iteration = 2;

    pub fn clear_iteration(&mut self) {
        self.iteration = ::std::option::Option::None;
    }

    pub fn has_iteration(&self) -> bool {
        self.iteration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iteration(&mut self, v: u32) {
        self.iteration = ::std::option::Option::Some(v);
    }

    pub fn get_iteration<'a>(&self) -> u32 {
        self.iteration.unwrap_or(0)
    }

    // optional bytes chainKey = 3;

    pub fn clear_chainKey(&mut self) {
        self.chainKey.clear();
    }

    pub fn has_chainKey(&self) -> bool {
        self.chainKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chainKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.chainKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chainKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.chainKey.is_none() {
            self.chainKey.set_default();
        };
        self.chainKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_chainKey(&mut self) -> ::std::vec::Vec<u8> {
        self.chainKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_chainKey<'a>(&'a self) -> &'a [u8] {
        match self.chainKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes signingKey = 4;

    pub fn clear_signingKey(&mut self) {
        self.signingKey.clear();
    }

    pub fn has_signingKey(&self) -> bool {
        self.signingKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signingKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.signingKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signingKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.signingKey.is_none() {
            self.signingKey.set_default();
        };
        self.signingKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_signingKey(&mut self) -> ::std::vec::Vec<u8> {
        self.signingKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signingKey<'a>(&'a self) -> &'a [u8] {
        match self.signingKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TSProtoSenderKeyDistributionMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.iteration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.chainKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.signingKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.iteration.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.chainKey.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.signingKey.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.iteration {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.chainKey.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.signingKey.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSProtoSenderKeyDistributionMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSProtoSenderKeyDistributionMessage {
    fn new() -> TSProtoSenderKeyDistributionMessage {
        TSProtoSenderKeyDistributionMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSProtoSenderKeyDistributionMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    TSProtoSenderKeyDistributionMessage::has_id,
                    TSProtoSenderKeyDistributionMessage::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "iteration",
                    TSProtoSenderKeyDistributionMessage::has_iteration,
                    TSProtoSenderKeyDistributionMessage::get_iteration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "chainKey",
                    TSProtoSenderKeyDistributionMessage::has_chainKey,
                    TSProtoSenderKeyDistributionMessage::get_chainKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "signingKey",
                    TSProtoSenderKeyDistributionMessage::has_signingKey,
                    TSProtoSenderKeyDistributionMessage::get_signingKey,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSProtoSenderKeyDistributionMessage>(
                    "TSProtoSenderKeyDistributionMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSProtoSenderKeyDistributionMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_iteration();
        self.clear_chainKey();
        self.clear_signingKey();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSProtoSenderKeyDistributionMessage {
    fn eq(&self, other: &TSProtoSenderKeyDistributionMessage) -> bool {
        self.id == other.id &&
        self.iteration == other.iteration &&
        self.chainKey == other.chainKey &&
        self.signingKey == other.signingKey &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSProtoSenderKeyDistributionMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x57, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x54, 0x65, 0x78, 0x74, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x74, 0x65, 0x78,
    0x74, 0x73, 0x65, 0x63, 0x75, 0x72, 0x65, 0x22, 0x69, 0x0a, 0x15, 0x54, 0x53, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x57, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x12, 0x0a, 0x0a, 0x72, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75,
    0x73, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x12,
    0x0a, 0x0a, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x96, 0x01, 0x0a, 0x1b, 0x54, 0x53, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x72,
    0x65, 0x4b, 0x65, 0x79, 0x57, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x16, 0x0a, 0x0e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x49, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x72,
    0x65, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e,
    0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x7b, 0x0a, 0x19, 0x54,
    0x53, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x4b, 0x65, 0x79, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x61, 0x74, 0x63, 0x68, 0x65, 0x74,
    0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x4b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x18,
    0x0a, 0x10, 0x62, 0x61, 0x73, 0x65, 0x4b, 0x65, 0x79, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75,
    0x72, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4c, 0x0a, 0x17, 0x54, 0x53, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x11, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x6a, 0x0a, 0x23, 0x54, 0x53, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x44, 0x69, 0x73, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x0a, 0x0a,
    0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x09, 0x69, 0x74, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08,
    0x63, 0x68, 0x61, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12,
    0x0a, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x4b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0c, 0x4a, 0x8a, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02,
    0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x03, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x05, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x05,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x12, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x06, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x06, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x10, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0c,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x12,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0e, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0e, 0x23, 0x24, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x02,
    0x25, 0x22, 0x10, 0x20, 0x57, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0f, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x12, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x14, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x15, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x15, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x02, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x16, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x17, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x17,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x12, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x1a, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x1a, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x12, 0x03, 0x1c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1c,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1d, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x20, 0x00, 0x25, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x20, 0x08, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21,
    0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x22, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x22, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x23,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x03, 0x12, 0x03, 0x24, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x24, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x12,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x1f, 0x20,
];

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
