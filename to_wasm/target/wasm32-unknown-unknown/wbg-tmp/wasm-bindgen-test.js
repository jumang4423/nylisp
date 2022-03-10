let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextEncoder, TextDecoder } = require(`util`);

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}
function __wbg_adapter_22(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5e4fa63ab3268a75(arg0, arg1, addHeapObject(arg2));
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4);
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* Handler for `console.log` invocations.
*
* If a test is currently running it takes the `args` array and stringifies
* it and appends it to the current output of the test. Otherwise it passes
* the arguments to the original `console.log` function, psased as
* `original`.
* @param {Array<any>} args
*/
module.exports.__wbgtest_console_log = function(args) {
    try {
        wasm.__wbgtest_console_log(addBorrowedObject(args));
    } finally {
        heap[stack_pointer++] = undefined;
    }
};

/**
* Handler for `console.debug` invocations. See above.
* @param {Array<any>} args
*/
module.exports.__wbgtest_console_debug = function(args) {
    try {
        wasm.__wbgtest_console_debug(addBorrowedObject(args));
    } finally {
        heap[stack_pointer++] = undefined;
    }
};

/**
* Handler for `console.info` invocations. See above.
* @param {Array<any>} args
*/
module.exports.__wbgtest_console_info = function(args) {
    try {
        wasm.__wbgtest_console_info(addBorrowedObject(args));
    } finally {
        heap[stack_pointer++] = undefined;
    }
};

/**
* Handler for `console.warn` invocations. See above.
* @param {Array<any>} args
*/
module.exports.__wbgtest_console_warn = function(args) {
    try {
        wasm.__wbgtest_console_warn(addBorrowedObject(args));
    } finally {
        heap[stack_pointer++] = undefined;
    }
};

/**
* Handler for `console.error` invocations. See above.
* @param {Array<any>} args
*/
module.exports.__wbgtest_console_error = function(args) {
    try {
        wasm.__wbgtest_console_error(addBorrowedObject(args));
    } finally {
        heap[stack_pointer++] = undefined;
    }
};

function __wbg_adapter_53(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures__invoke0_mut__hce0a18081ce0d394(arg0, arg1);
}

function __wbg_adapter_70(arg0, arg1, arg2, arg3, arg4) {
    _assertNum(arg0);
    _assertNum(arg1);
    _assertNum(arg3);
    wasm.wasm_bindgen__convert__closures__invoke3_mut__h799477a9e6fa750f(arg0, arg1, addHeapObject(arg2), arg3, addHeapObject(arg4));
}

function __wbg_adapter_83(arg0, arg1, arg2, arg3) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h2291eaf35489aa5b(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
*/
class NyLisp {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        const obj = Object.create(NyLisp.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_nylisp_free(ptr);
    }
    /**
    * @returns {NyLisp}
    */
    static new() {
        var ret = wasm.nylisp_new();
        return NyLisp.__wrap(ret);
    }
    /**
    * @param {string} _lines
    * @returns {any[]}
    */
    run(_lines) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(_lines, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.nylisp_run(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.NyLisp = NyLisp;
/**
* Runtime test harness support instantiated in JS.
*
* The node.js entry script instantiates a `Context` here which is used to
* drive test execution.
*/
class WasmBindgenTestContext {

    static __wrap(ptr) {
        const obj = Object.create(WasmBindgenTestContext.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmbindgentestcontext_free(ptr);
    }
    /**
    * Creates a new context ready to run tests.
    *
    * A `Context` is the main structure through which test execution is
    * coordinated, and this will collect output and results for all executed
    * tests.
    */
    constructor() {
        var ret = wasm.wasmbindgentestcontext_new();
        return WasmBindgenTestContext.__wrap(ret);
    }
    /**
    * Inform this context about runtime arguments passed to the test
    * harness.
    *
    * Eventually this will be used to support flags, but for now it's just
    * used to support test filters.
    * @param {any[]} args
    */
    args(args) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passArrayJsValueToWasm0(args, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.wasmbindgentestcontext_args(this.ptr, ptr0, len0);
    }
    /**
    * Executes a list of tests, returning a promise representing their
    * eventual completion.
    *
    * This is the main entry point for executing tests. All the tests passed
    * in are the JS `Function` object that was plucked off the
    * `WebAssembly.Instance` exports list.
    *
    * The promise returned resolves to either `true` if all tests passed or
    * `false` if at least one test failed.
    * @param {any[]} tests
    * @returns {Promise<any>}
    */
    run(tests) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passArrayJsValueToWasm0(tests, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.wasmbindgentestcontext_run(this.ptr, ptr0, len0);
        return takeObject(ret);
    }
}
module.exports.WasmBindgenTestContext = WasmBindgenTestContext;

module.exports.__wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbg_self_86b4b13392c7af56 = function() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_msCrypto_9ad6677321a08dd8 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_crypto_b8c92eaac23d0d80 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).crypto;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_getRandomValues_dd27e6b0652b3236 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).getRandomValues;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_getRandomValues_e57c9b75ddead065 = function() { return logError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };

module.exports.__wbg_randomFillSync_d2ba53160aec6aba = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };

module.exports.__wbg_static_accessor_MODULE_452b4680e8614c81 = function() { return logError(function () {
    var ret = module;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_require_f5521a5b85ad2542 = function() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbg_log_60dbaec18ba13580 = function() { return logError(function (arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
}, arguments) };

module.exports.__wbg_String_2d0525b0a2d8952b = function() { return logError(function (arg0, arg1) {
    var ret = String(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

module.exports.__wbg_wbgtestinvoke_58c0b584f460c90f = function() { return handleError(function (arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = () => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_53(a, state0.b, );
            } finally {
                state0.a = a;
            }
        };
        __wbg_test_invoke(cb0);
    } finally {
        state0.a = state0.b = 0;
    }
}, arguments) };

module.exports.__wbg_static_accessor_document_39679a3ba0227499 = function() { return logError(function () {
    var ret = document;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_getElementById_ee61fa45acb16cdd = function() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_textcontent_1a07877650d85fba = function() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).textContent;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

module.exports.__wbg_settextcontent_53a1006422d713a1 = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).textContent = getStringFromWasm0(arg1, arg2);
}, arguments) };

module.exports.__wbg_stack_30cc108d2806ec8e = function() { return logError(function (arg0) {
    var ret = getObject(arg0).stack;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_number_new = function(arg0) {
    var ret = arg0;
    return addHeapObject(ret);
};

module.exports.__wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_jsval_eq = function(arg0, arg1) {
    var ret = getObject(arg0) === getObject(arg1);
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbg_self_9483edc55533b26c = function() { return logError(function (arg0) {
    var ret = getObject(arg0).self;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_stack_f5294cdf5024f4e8 = function() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

module.exports.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbg_forEach_2d9db89598d5001a = function() { return logError(function (arg0, arg1, arg2) {
    try {
        var state0 = {a: arg1, b: arg2};
        var cb0 = (arg0, arg1, arg2) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_70(a, state0.b, arg0, arg1, arg2);
            } finally {
                state0.a = a;
            }
        };
        getObject(arg0).forEach(cb0);
    } finally {
        state0.a = state0.b = 0;
    }
}, arguments) };

module.exports.__wbg_message_1dfe93b595be8811 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).message;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_name_66305ab387468967 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).name;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_newnoargs_f579424187aa1717 = function() { return logError(function (arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_call_89558c3e96703ca1 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_call_94697a95cb7e239c = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_new_4beacc9c71572250 = function() { return logError(function (arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_83(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        var ret = new Promise(cb0);
        return addHeapObject(ret);
    } finally {
        state0.a = state0.b = 0;
    }
}, arguments) };

module.exports.__wbg_resolve_4f8f547f26b30b27 = function() { return logError(function (arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_then_a6860c82b90816ca = function() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_globalThis_d61b1f48a57191ae = function() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_self_e23d74ae45fb17d1 = function() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_window_b4be7f48b24ac56e = function() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_global_e7669da72fd7f239 = function() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_new_e3b800e570795b3c = function() { return logError(function (arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_newwithlength_5f4ce114a24dfe1e = function() { return logError(function (arg0) {
    var ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_subarray_a68f835ca2af506f = function() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_length_30803400a8f15c59 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

module.exports.__wbg_set_5b8081e9d002f0df = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
}, arguments) };

module.exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

module.exports.__wbg_buffer_5e74a88a1424a2e0 = function() { return logError(function (arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_error_09919627ac0992f5 = function() { return logError(function (arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
}, arguments) };

module.exports.__wbg_new_693216e109162396 = function() { return logError(function () {
    var ret = new Error();
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_stack_0ddaca5d1abfb52f = function() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};

module.exports.__wbindgen_closure_wrapper2976 = function() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 265, __wbg_adapter_22);
    return addHeapObject(ret);
}, arguments) };

const path = require('path').join(__dirname, 'wasm-bindgen-test_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

