// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Cancellable;
use Error;
use InputStream;
use OutputStreamSpliceFlags;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct OutputStream(Object<ffi::GOutputStream, ffi::GOutputStreamClass>);

    match fn {
        get_type => || ffi::g_output_stream_get_type(),
    }
}

pub trait OutputStreamExt {
    fn clear_pending(&self);

    fn close<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn close_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q);

    fn flush<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn flush_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q);

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn is_closing(&self) -> bool;

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn printf<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, error: &mut Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<usize>;

    fn set_pending(&self) -> Result<(), Error>;

    fn splice<'a, P: IsA<InputStream>, Q: Into<Option<&'a Cancellable>>>(&self, source: &P, flags: OutputStreamSpliceFlags, cancellable: Q) -> Result<isize, Error>;

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn vprintf<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, error: &mut Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize>;

    fn write<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &[u8], cancellable: P) -> Result<isize, Error>;

    fn write_all<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &[u8], cancellable: P) -> Result<usize, Error>;

    fn write_bytes<'a, P: Into<Option<&'a Cancellable>>>(&self, bytes: &glib::Bytes, cancellable: P) -> Result<isize, Error>;
}

impl<O: IsA<OutputStream>> OutputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_output_stream_clear_pending(self.to_glib_none().0);
        }
    }

    fn close<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_close(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn close_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_output_stream_close_async(self.to_glib_none().0, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn flush<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_flush(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn flush_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn flush_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_flush_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = flush_async_trampoline::<Q>;
        unsafe {
            ffi::g_output_stream_flush_async(self.to_glib_none().0, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_has_pending(self.to_glib_none().0))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_is_closed(self.to_glib_none().0))
        }
    }

    fn is_closing(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_is_closing(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn printf<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, error: &mut Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<usize> {
    //    unsafe { TODO: call ffi::g_output_stream_printf() }
    //}

    fn set_pending(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_set_pending(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn splice<'a, P: IsA<InputStream>, Q: Into<Option<&'a Cancellable>>>(&self, source: &P, flags: OutputStreamSpliceFlags, cancellable: Q) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_splice(self.to_glib_none().0, source.to_glib_none().0, flags.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn vprintf<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, error: &mut Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize> {
    //    unsafe { TODO: call ffi::g_output_stream_vprintf() }
    //}

    fn write<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &[u8], cancellable: P) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write(self.to_glib_none().0, buffer.to_glib_none().0, count, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn write_all<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &[u8], cancellable: P) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut bytes_written = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_write_all(self.to_glib_none().0, buffer.to_glib_none().0, count, &mut bytes_written, cancellable.0, &mut error);
            if error.is_null() { Ok(bytes_written) } else { Err(from_glib_full(error)) }
        }
    }

    fn write_bytes<'a, P: Into<Option<&'a Cancellable>>>(&self, bytes: &glib::Bytes, cancellable: P) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write_bytes(self.to_glib_none().0, bytes.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }
}
