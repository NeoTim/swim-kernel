use swim_c_sys::{cchar, int};
use crate::pyconfig::Py_ssize_t;
use crate::object::{PyObject, PyTypeObject, PyType_Check, PyType_FastSubclass};

extern "C" {
    pub fn PyErr_SetNone(arg1: *mut PyObject);
    pub fn PyErr_SetObject(arg1: *mut PyObject, arg2: *mut PyObject);
    pub fn PyErr_SetString(exception: *mut PyObject, string: *const cchar);
    pub fn PyErr_Occurred() -> *mut PyObject;
    pub fn PyErr_Clear();
    pub fn PyErr_Fetch(arg1: *mut *mut PyObject, arg2: *mut *mut PyObject, arg3: *mut *mut PyObject);
    pub fn PyErr_Restore(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject);
    pub fn PyErr_GetExcInfo(arg1: *mut *mut PyObject, arg2: *mut *mut PyObject, arg3: *mut *mut PyObject);
    pub fn PyErr_SetExcInfo(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject);
    pub fn Py_FatalError(message: *const cchar) -> !;
    pub fn PyErr_GivenExceptionMatches(arg1: *mut PyObject, arg2: *mut PyObject) -> int;
    pub fn PyErr_ExceptionMatches(arg1: *mut PyObject) -> int;
    pub fn PyErr_NormalizeException(arg1: *mut *mut PyObject, arg2: *mut *mut PyObject, arg3: *mut *mut PyObject);
    pub fn PyException_SetTraceback(arg1: *mut PyObject, arg2: *mut PyObject) -> int;
    pub fn PyException_GetTraceback(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyException_GetCause(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyException_SetCause(arg1: *mut PyObject, arg2: *mut PyObject);
    pub fn PyException_GetContext(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyException_SetContext(arg1: *mut PyObject, arg2: *mut PyObject);
}

#[inline(always)]
pub unsafe fn PyExceptionClass_Check(x: *mut PyObject) -> int {
    (PyType_Check(x) != 0 && PyType_FastSubclass(x as *mut PyTypeObject, crate::object::Py_TPFLAGS_BASE_EXC_SUBCLASS) != 0) as int
}

#[inline(always)]
pub unsafe fn PyExceptionInstance_Check(x: *mut PyObject) -> int {
    PyType_FastSubclass((*x).ob_type, crate::object::Py_TPFLAGS_BASE_EXC_SUBCLASS)
}

#[inline(always)]
pub unsafe fn PyExceptionClass_Name(x: *mut PyTypeObject) -> *const cchar {
     (*x).tp_name
 }

#[inline(always)]
pub unsafe fn PyExceptionInstance_Class(x: *mut PyObject) -> *mut PyObject {
    (*x).ob_type as *mut PyObject
}

extern "C" {
    pub static mut PyExc_BaseException: *mut PyObject;
    pub static mut PyExc_Exception: *mut PyObject;
    pub static mut PyExc_StopIteration: *mut PyObject;
    pub static mut PyExc_GeneratorExit: *mut PyObject;
    pub static mut PyExc_ArithmeticError: *mut PyObject;
    pub static mut PyExc_LookupError: *mut PyObject;
    pub static mut PyExc_AssertionError: *mut PyObject;
    pub static mut PyExc_AttributeError: *mut PyObject;
    pub static mut PyExc_BufferError: *mut PyObject;
    pub static mut PyExc_EOFError: *mut PyObject;
    pub static mut PyExc_FloatingPointError: *mut PyObject;
    pub static mut PyExc_OSError: *mut PyObject;
    pub static mut PyExc_ImportError: *mut PyObject;
    #[cfg(Py_3_6)]
    pub static mut PyExc_ModuleNotFoundError: *mut PyObject;
    pub static mut PyExc_IndexError: *mut PyObject;
    pub static mut PyExc_KeyError: *mut PyObject;
    pub static mut PyExc_KeyboardInterrupt: *mut PyObject;
    pub static mut PyExc_MemoryError: *mut PyObject;
    pub static mut PyExc_NameError: *mut PyObject;
    pub static mut PyExc_OverflowError: *mut PyObject;
    pub static mut PyExc_RuntimeError: *mut PyObject;
    pub static mut PyExc_NotImplementedError: *mut PyObject;
    pub static mut PyExc_SyntaxError: *mut PyObject;
    pub static mut PyExc_IndentationError: *mut PyObject;
    pub static mut PyExc_TabError: *mut PyObject;
    pub static mut PyExc_ReferenceError: *mut PyObject;
    pub static mut PyExc_SystemError: *mut PyObject;
    pub static mut PyExc_SystemExit: *mut PyObject;
    pub static mut PyExc_TypeError: *mut PyObject;
    pub static mut PyExc_UnboundLocalError: *mut PyObject;
    pub static mut PyExc_UnicodeError: *mut PyObject;
    pub static mut PyExc_UnicodeEncodeError: *mut PyObject;
    pub static mut PyExc_UnicodeDecodeError: *mut PyObject;
    pub static mut PyExc_UnicodeTranslateError: *mut PyObject;
    pub static mut PyExc_ValueError: *mut PyObject;
    pub static mut PyExc_ZeroDivisionError: *mut PyObject;
    pub static mut PyExc_BlockingIOError: *mut PyObject;
    pub static mut PyExc_BrokenPipeError: *mut PyObject;
    pub static mut PyExc_ChildProcessError: *mut PyObject;
    pub static mut PyExc_ConnectionError: *mut PyObject;
    pub static mut PyExc_ConnectionAbortedError: *mut PyObject;
    pub static mut PyExc_ConnectionRefusedError: *mut PyObject;
    pub static mut PyExc_ConnectionResetError: *mut PyObject;
    pub static mut PyExc_FileExistsError: *mut PyObject;
    pub static mut PyExc_FileNotFoundError: *mut PyObject;
    pub static mut PyExc_InterruptedError: *mut PyObject;
    pub static mut PyExc_IsADirectoryError: *mut PyObject;
    pub static mut PyExc_NotADirectoryError: *mut PyObject;
    pub static mut PyExc_PermissionError: *mut PyObject;
    pub static mut PyExc_ProcessLookupError: *mut PyObject;
    pub static mut PyExc_TimeoutError: *mut PyObject;
    pub static mut PyExc_EnvironmentError: *mut PyObject;
    pub static mut PyExc_IOError: *mut PyObject;
    #[cfg(windows)]
    pub static mut PyExc_WindowsError: *mut PyObject;
    pub static mut PyExc_RecursionErrorInst: *mut PyObject;
    pub static mut PyExc_Warning: *mut PyObject;
    pub static mut PyExc_UserWarning: *mut PyObject;
    pub static mut PyExc_DeprecationWarning: *mut PyObject;
    pub static mut PyExc_PendingDeprecationWarning: *mut PyObject;
    pub static mut PyExc_SyntaxWarning: *mut PyObject;
    pub static mut PyExc_RuntimeWarning: *mut PyObject;
    pub static mut PyExc_FutureWarning: *mut PyObject;
    pub static mut PyExc_ImportWarning: *mut PyObject;
    pub static mut PyExc_UnicodeWarning: *mut PyObject;
    pub static mut PyExc_BytesWarning: *mut PyObject;
    pub static mut PyExc_ResourceWarning: *mut PyObject;

    pub fn PyErr_BadArgument() -> int;
    pub fn PyErr_NoMemory() -> *mut PyObject;
    pub fn PyErr_SetFromErrno(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyErr_SetFromErrnoWithFilenameObject(arg1: *mut PyObject, arg2: *mut PyObject)  -> *mut PyObject;
    #[cfg(Py_3_4)]
    pub fn PyErr_SetFromErrnoWithFilenameObjects(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> *mut PyObject;
    pub fn PyErr_SetFromErrnoWithFilename(exc: *mut PyObject, filename: *const cchar) -> *mut PyObject;
    pub fn PyErr_Format(exception: *mut PyObject, format: *const cchar, ...) -> *mut PyObject;
    #[cfg(Py_3_6)]
    pub fn PyErr_SetImportErrorSubclass(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject, arg4: *mut PyObject) -> *mut PyObject;
    pub fn PyErr_SetImportError(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> *mut PyObject;
    pub fn PyErr_BadInternalCall();
    pub fn _PyErr_BadInternalCall(filename: *const cchar, lineno: int);
    pub fn PyErr_NewException(name: *const cchar, base: *mut PyObject, dict: *mut PyObject) -> *mut PyObject;
    pub fn PyErr_NewExceptionWithDoc(name: *const cchar, doc: *const cchar, base: *mut PyObject, dict: *mut PyObject) -> *mut PyObject;
    pub fn PyErr_WriteUnraisable(arg1: *mut PyObject);
    pub fn PyErr_CheckSignals() -> int;
    pub fn PyErr_SetInterrupt();
    pub fn PyErr_SyntaxLocation(filename: *const cchar, lineno: int);
    pub fn PyErr_SyntaxLocationEx(filename: *const cchar, lineno: int, col_offset: int);
    pub fn PyErr_ProgramText(filename: *const cchar, lineno: int) -> *mut PyObject;
    pub fn PyUnicodeDecodeError_Create(encoding: *const cchar, object: *const cchar, length: Py_ssize_t, start: Py_ssize_t, end: Py_ssize_t, reason: *const cchar) -> *mut PyObject;
    pub fn PyUnicodeEncodeError_GetEncoding(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeDecodeError_GetEncoding(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeEncodeError_GetObject(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeDecodeError_GetObject(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeTranslateError_GetObject(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeEncodeError_GetStart(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> int;
    pub fn PyUnicodeDecodeError_GetStart(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> int;
    pub fn PyUnicodeTranslateError_GetStart(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> int;
    pub fn PyUnicodeEncodeError_SetStart(arg1: *mut PyObject, arg2: Py_ssize_t) -> int;
    pub fn PyUnicodeDecodeError_SetStart(arg1: *mut PyObject, arg2: Py_ssize_t) -> int;
    pub fn PyUnicodeTranslateError_SetStart(arg1: *mut PyObject, arg2: Py_ssize_t) -> int;
    pub fn PyUnicodeEncodeError_GetEnd(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> int;
    pub fn PyUnicodeDecodeError_GetEnd(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> int;
    pub fn PyUnicodeTranslateError_GetEnd(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> int;
    pub fn PyUnicodeEncodeError_SetEnd(arg1: *mut PyObject, arg2: Py_ssize_t) -> int;
    pub fn PyUnicodeDecodeError_SetEnd(arg1: *mut PyObject, arg2: Py_ssize_t) -> int;
    pub fn PyUnicodeTranslateError_SetEnd(arg1: *mut PyObject, arg2: Py_ssize_t) -> int;
    pub fn PyUnicodeEncodeError_GetReason(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeDecodeError_GetReason(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeTranslateError_GetReason(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicodeEncodeError_SetReason(exc: *mut PyObject, reason: *const cchar) -> int;
    pub fn PyUnicodeDecodeError_SetReason(exc: *mut PyObject, reason: *const cchar) -> int;
    pub fn PyUnicodeTranslateError_SetReason(exc: *mut PyObject, reason: *const cchar) -> int;
}