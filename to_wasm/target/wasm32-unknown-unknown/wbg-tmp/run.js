
        const { exit } = require('process');
        const wasm = require("./wasm-bindgen-test");

        
const handlers = {};

const wrap = method => {
    const og = console[method];
    const on_method = `on_console_${method}`;
    console[method] = function (...args) {
        og.apply(this, args);
        if (handlers[on_method]) {
            handlers[on_method](args);
        }
    };
};

// override `console.log` and `console.error` etc... before we import tests to
// ensure they're bound correctly in wasm. This'll allow us to intercept
// all these calls and capture the output of tests
wrap("debug");
wrap("log");
wrap("info");
wrap("warn");
wrap("error");

cx = new wasm.WasmBindgenTestContext();
handlers.on_console_debug = wasm.__wbgtest_console_debug;
handlers.on_console_log = wasm.__wbgtest_console_log;
handlers.on_console_info = wasm.__wbgtest_console_info;
handlers.on_console_warn = wasm.__wbgtest_console_warn;
handlers.on_console_error = wasm.__wbgtest_console_error;


        global.__wbg_test_invoke = f => f();

        async function main(tests) {
            // Forward runtime arguments. These arguments are also arguments to the
            // `wasm-bindgen-test-runner` which forwards them to node which we
            // forward to the test harness. this is basically only used for test
            // filters for now.
            cx.args(process.argv.slice(2));

            const ok = await cx.run(tests.map(n => wasm.__wasm[n]));
            if (!ok)
                exit(1);
        }

        const tests = [];
    tests.push('__wbgt_test_run_0')

        main(tests)
            .catch(e => {
                console.error(e);
                exit(1);
            });
    