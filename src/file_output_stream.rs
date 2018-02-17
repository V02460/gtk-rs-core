// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use FileInfo;
use OutputStream;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib::Priority;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FileOutputStream(Object<ffi::GFileOutputStream, ffi::GFileOutputStreamClass>): OutputStream;

    match fn {
        get_type => || ffi::g_file_output_stream_get_type(),
    }
}

pub trait FileOutputStreamExt {
    fn get_etag(&self) -> Option<String>;

    fn query_info<'a, P: Into<Option<&'a Cancellable>>>(&self, attributes: &str, cancellable: P) -> Result<FileInfo, Error>;

    fn query_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: Priority, cancellable: P, callback: Q);
}

impl<O: IsA<FileOutputStream>> FileOutputStreamExt for O {
    fn get_etag(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_file_output_stream_get_etag(self.to_glib_none().0))
        }
    }

    fn query_info<'a, P: Into<Option<&'a Cancellable>>>(&self, attributes: &str, cancellable: P) -> Result<FileInfo, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_output_stream_query_info(self.to_glib_none().0, attributes.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn query_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn query_info_async_trampoline<Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_output_stream_query_info_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_info_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_output_stream_query_info_async(self.to_glib_none().0, attributes.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}
