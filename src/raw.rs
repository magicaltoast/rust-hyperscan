/* automatically generated by rust-bindgen */

pub enum Struct_hs_database { }
pub type hs_database_t = Struct_hs_database;
pub type hs_error_t = ::std::os::raw::c_int;
pub type hs_alloc_t =
    ::std::option::Option<extern "C" fn(size: size_t)
                              -> *mut ::std::os::raw::c_void>;
pub type hs_free_t =
    ::std::option::Option<unsafe extern "C" fn(ptr:
                                                   *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_hs_compile_error {
    pub message: *mut ::std::os::raw::c_char,
    pub expression: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_hs_compile_error {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_hs_compile_error {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type hs_compile_error_t = Struct_hs_compile_error;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_hs_platform_info {
    pub tune: ::std::os::raw::c_uint,
    pub cpu_features: ::std::os::raw::c_ulonglong,
    pub reserved1: ::std::os::raw::c_ulonglong,
    pub reserved2: ::std::os::raw::c_ulonglong,
}
impl ::std::clone::Clone for Struct_hs_platform_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_hs_platform_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type hs_platform_info_t = Struct_hs_platform_info;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_hs_expr_info {
    pub min_width: ::std::os::raw::c_uint,
    pub max_width: ::std::os::raw::c_uint,
    pub unordered_matches: ::std::os::raw::c_char,
    pub matches_at_eod: ::std::os::raw::c_char,
    pub matches_only_at_eod: ::std::os::raw::c_char,
}
impl ::std::clone::Clone for Struct_hs_expr_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_hs_expr_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type hs_expr_info_t = Struct_hs_expr_info;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_hs_expr_ext {
    pub flags: ::std::os::raw::c_ulonglong,
    pub min_offset: ::std::os::raw::c_ulonglong,
    pub max_offset: ::std::os::raw::c_ulonglong,
    pub min_length: ::std::os::raw::c_ulonglong,
}
impl ::std::clone::Clone for Struct_hs_expr_ext {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_hs_expr_ext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type hs_expr_ext_t = Struct_hs_expr_ext;
pub enum Struct_hs_stream { }
pub type hs_stream_t = Struct_hs_stream;
pub enum Struct_hs_scratch { }
pub type hs_scratch_t = Struct_hs_scratch;
pub type match_event_handler =
    ::std::option::Option<unsafe extern "C" fn(id: ::std::os::raw::c_uint,
                                               from:
                                                   ::std::os::raw::c_ulonglong,
                                               to:
                                                   ::std::os::raw::c_ulonglong,
                                               flags: ::std::os::raw::c_uint,
                                               context:
                                                   *mut ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
#[link(name = "hs", kind = "static")]
extern "C" {
    pub fn hs_free_database(db: *mut hs_database_t) -> hs_error_t;
    pub fn hs_serialize_database(db: *const hs_database_t,
                                 bytes: *mut *mut ::std::os::raw::c_char,
                                 length: *mut size_t) -> hs_error_t;
    pub fn hs_deserialize_database(bytes: *const ::std::os::raw::c_char,
                                   length: size_t,
                                   db: *mut *mut hs_database_t) -> hs_error_t;
    pub fn hs_deserialize_database_at(bytes: *const ::std::os::raw::c_char,
                                      length: size_t, db: *mut hs_database_t)
     -> hs_error_t;
    pub fn hs_stream_size(database: *const hs_database_t,
                          stream_size: *mut size_t) -> hs_error_t;
    pub fn hs_database_size(database: *const hs_database_t,
                            database_size: *mut size_t) -> hs_error_t;
    pub fn hs_serialized_database_size(bytes: *const ::std::os::raw::c_char,
                                       length: size_t,
                                       deserialized_size: *mut size_t)
     -> hs_error_t;
    pub fn hs_database_info(database: *const hs_database_t,
                            info: *mut *mut ::std::os::raw::c_char)
     -> hs_error_t;
    pub fn hs_serialized_database_info(bytes: *const ::std::os::raw::c_char,
                                       length: size_t,
                                       info: *mut *mut ::std::os::raw::c_char)
     -> hs_error_t;
    pub fn hs_set_allocator(alloc_func: hs_alloc_t, free_func: hs_free_t)
     -> hs_error_t;
    pub fn hs_set_database_allocator(alloc_func: hs_alloc_t,
                                     free_func: hs_free_t) -> hs_error_t;
    pub fn hs_set_misc_allocator(alloc_func: hs_alloc_t, free_func: hs_free_t)
     -> hs_error_t;
    pub fn hs_set_scratch_allocator(alloc_func: hs_alloc_t,
                                    free_func: hs_free_t) -> hs_error_t;
    pub fn hs_set_stream_allocator(alloc_func: hs_alloc_t,
                                   free_func: hs_free_t) -> hs_error_t;
    pub fn hs_version() -> *const ::std::os::raw::c_char;
    pub fn hs_compile(expression: *const ::std::os::raw::c_char,
                      flags: ::std::os::raw::c_uint,
                      mode: ::std::os::raw::c_uint,
                      platform: *const hs_platform_info_t,
                      db: *mut *mut hs_database_t,
                      error: *mut *mut hs_compile_error_t) -> hs_error_t;
    pub fn hs_compile_multi(expressions: *const *const ::std::os::raw::c_char,
                            flags: *const ::std::os::raw::c_uint,
                            ids: *const ::std::os::raw::c_uint,
                            elements: ::std::os::raw::c_uint,
                            mode: ::std::os::raw::c_uint,
                            platform: *const hs_platform_info_t,
                            db: *mut *mut hs_database_t,
                            error: *mut *mut hs_compile_error_t)
     -> hs_error_t;
    pub fn hs_compile_ext_multi(expressions:
                                    *const *const ::std::os::raw::c_char,
                                flags: *const ::std::os::raw::c_uint,
                                ids: *const ::std::os::raw::c_uint,
                                ext: *const *const hs_expr_ext_t,
                                elements: ::std::os::raw::c_uint,
                                mode: ::std::os::raw::c_uint,
                                platform: *const hs_platform_info_t,
                                db: *mut *mut hs_database_t,
                                error: *mut *mut hs_compile_error_t)
     -> hs_error_t;
    pub fn hs_free_compile_error(error: *mut hs_compile_error_t)
     -> hs_error_t;
    pub fn hs_expression_info(expression: *const ::std::os::raw::c_char,
                              flags: ::std::os::raw::c_uint,
                              info: *mut *mut hs_expr_info_t,
                              error: *mut *mut hs_compile_error_t)
     -> hs_error_t;
    pub fn hs_populate_platform(platform: *mut hs_platform_info_t)
     -> hs_error_t;
    pub fn hs_open_stream(db: *const hs_database_t,
                          flags: ::std::os::raw::c_uint,
                          stream: *mut *mut hs_stream_t) -> hs_error_t;
    pub fn hs_scan_stream(id: *mut hs_stream_t,
                          data: *const ::std::os::raw::c_char,
                          length: ::std::os::raw::c_uint,
                          flags: ::std::os::raw::c_uint,
                          scratch: *mut hs_scratch_t,
                          onEvent: match_event_handler,
                          ctxt: *mut ::std::os::raw::c_void) -> hs_error_t;
    pub fn hs_close_stream(id: *mut hs_stream_t, scratch: *mut hs_scratch_t,
                           onEvent: match_event_handler,
                           ctxt: *mut ::std::os::raw::c_void) -> hs_error_t;
    pub fn hs_reset_stream(id: *mut hs_stream_t,
                           flags: ::std::os::raw::c_uint,
                           scratch: *mut hs_scratch_t,
                           onEvent: match_event_handler,
                           context: *mut ::std::os::raw::c_void)
     -> hs_error_t;
    pub fn hs_copy_stream(to_id: *mut *mut hs_stream_t,
                          from_id: *const hs_stream_t) -> hs_error_t;
    pub fn hs_reset_and_copy_stream(to_id: *mut hs_stream_t,
                                    from_id: *const hs_stream_t,
                                    scratch: *mut hs_scratch_t,
                                    onEvent: match_event_handler,
                                    context: *mut ::std::os::raw::c_void)
     -> hs_error_t;
    pub fn hs_scan(db: *const hs_database_t,
                   data: *const ::std::os::raw::c_char,
                   length: ::std::os::raw::c_uint,
                   flags: ::std::os::raw::c_uint, scratch: *mut hs_scratch_t,
                   onEvent: match_event_handler,
                   context: *mut ::std::os::raw::c_void) -> hs_error_t;
    pub fn hs_scan_vector(db: *const hs_database_t,
                          data: *const *const ::std::os::raw::c_char,
                          length: *const ::std::os::raw::c_uint,
                          count: ::std::os::raw::c_uint,
                          flags: ::std::os::raw::c_uint,
                          scratch: *mut hs_scratch_t,
                          onEvent: match_event_handler,
                          context: *mut ::std::os::raw::c_void) -> hs_error_t;
    pub fn hs_alloc_scratch(db: *const hs_database_t,
                            scratch: *mut *mut hs_scratch_t) -> hs_error_t;
    pub fn hs_clone_scratch(src: *const hs_scratch_t,
                            dest: *mut *mut hs_scratch_t) -> hs_error_t;
    pub fn hs_scratch_size(scratch: *const hs_scratch_t,
                           scratch_size: *mut size_t) -> hs_error_t;
    pub fn hs_free_scratch(scratch: *mut hs_scratch_t) -> hs_error_t;
}
