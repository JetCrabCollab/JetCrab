use boa_engine::Context;
use tracing::info;

pub struct AssertAPI;

impl AssertAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔍 Registering Assert API...");

        let assert_code = r#"
        globalThis.assert = function(value, message) {
            if (!value) {
                throw new Error(message || 'Assertion failed');
            }
        };
        
        globalThis.assert.ok = function(value, message) {
            assert(value, message);
        };
        
        globalThis.assert.equal = function(actual, expected, message) {
            if (actual !== expected) {
                throw new Error(message || `Expected ${expected} but got ${actual}`);
            }
        };
        
        globalThis.assert.notEqual = function(actual, expected, message) {
            if (actual === expected) {
                throw new Error(message || `Expected not ${expected} but got ${actual}`);
            }
        };
        
        globalThis.assert.strictEqual = function(actual, expected, message) {
            if (actual !== expected) {
                throw new Error(message || `Expected ${expected} but got ${actual}`);
            }
        };
        
        globalThis.assert.notStrictEqual = function(actual, expected, message) {
            if (actual === expected) {
                throw new Error(message || `Expected not ${expected} but got ${actual}`);
            }
        };
        
        globalThis.assert.deepEqual = function(actual, expected, message) {
            if (JSON.stringify(actual) !== JSON.stringify(expected)) {
                throw new Error(message || `Expected ${JSON.stringify(expected)} but got ${JSON.stringify(actual)}`);
            }
        };
        
        globalThis.assert.notDeepEqual = function(actual, expected, message) {
            if (JSON.stringify(actual) === JSON.stringify(expected)) {
                throw new Error(message || `Expected not ${JSON.stringify(expected)} but got ${JSON.stringify(actual)}`);
            }
        };
        
        globalThis.assert.throws = function(fn, error, message) {
            try {
                fn();
                throw new Error(message || 'Expected function to throw');
            } catch (e) {
                if (error) {
                    if (typeof error === 'string' && !e.message.includes(error)) {
                        throw new Error(message || `Expected error message to include "${error}" but got "${e.message}"`);
                    }
                    if (error instanceof RegExp && !error.test(e.message)) {
                        throw new Error(message || `Expected error message to match ${error} but got "${e.message}"`);
                    }
                }
            }
        };
        
        globalThis.assert.doesNotThrow = function(fn, error, message) {
            try {
                fn();
            } catch (e) {
                if (error) {
                    if (typeof error === 'string' && e.message.includes(error)) {
                        throw new Error(message || `Expected function not to throw error containing "${error}" but got "${e.message}"`);
                    }
                    if (error instanceof RegExp && error.test(e.message)) {
                        throw new Error(message || `Expected function not to throw error matching ${error} but got "${e.message}"`);
                    }
                }
                throw e;
            }
        };
        
        globalThis.assert.fail = function(message) {
            throw new Error(message || 'Assertion failed');
        };
        
        globalThis.assert.ifError = function(err) {
            if (err) {
                throw err;
            }
        };
        
        globalThis.assert.match = function(string, regexp, message) {
            if (!regexp.test(string)) {
                throw new Error(message || `Expected ${string} to match ${regexp}`);
            }
        };
        
        globalThis.assert.notMatch = function(string, regexp, message) {
            if (regexp.test(string)) {
                throw new Error(message || `Expected ${string} not to match ${regexp}`);
            }
        };
        
        globalThis.assert.approximately = function(actual, expected, delta, message) {
            if (Math.abs(actual - expected) > delta) {
                throw new Error(message || `Expected ${actual} to be approximately ${expected} ± ${delta}`);
            }
        };
        
        globalThis.assert.isTrue = function(value, message) {
            assert(value === true, message || `Expected true but got ${value}`);
        };
        
        globalThis.assert.isFalse = function(value, message) {
            assert(value === false, message || `Expected false but got ${value}`);
        };
        
        globalThis.assert.isNull = function(value, message) {
            assert(value === null, message || `Expected null but got ${value}`);
        };
        
        globalThis.assert.isNotNull = function(value, message) {
            assert(value !== null, message || `Expected not null but got ${value}`);
        };
        
        globalThis.assert.isUndefined = function(value, message) {
            assert(value === undefined, message || `Expected undefined but got ${value}`);
        };
        
        globalThis.assert.isDefined = function(value, message) {
            assert(value !== undefined, message || `Expected defined but got ${value}`);
        };
        
        globalThis.assert.isFunction = function(value, message) {
            assert(typeof value === 'function', message || `Expected function but got ${typeof value}`);
        };
        
        globalThis.assert.isNotFunction = function(value, message) {
            assert(typeof value !== 'function', message || `Expected not function but got ${typeof value}`);
        };
        
        globalThis.assert.isObject = function(value, message) {
            assert(typeof value === 'object' && value !== null, message || `Expected object but got ${typeof value}`);
        };
        
        globalThis.assert.isNotObject = function(value, message) {
            assert(typeof value !== 'object' || value === null, message || `Expected not object but got ${typeof value}`);
        };
        
        globalThis.assert.isArray = function(value, message) {
            assert(Array.isArray(value), message || `Expected array but got ${typeof value}`);
        };
        
        globalThis.assert.isNotArray = function(value, message) {
            assert(!Array.isArray(value), message || `Expected not array but got ${typeof value}`);
        };
        
        globalThis.assert.isString = function(value, message) {
            assert(typeof value === 'string', message || `Expected string but got ${typeof value}`);
        };
        
        globalThis.assert.isNotString = function(value, message) {
            assert(typeof value !== 'string', message || `Expected not string but got ${typeof value}`);
        };
        
        globalThis.assert.isNumber = function(value, message) {
            assert(typeof value === 'number', message || `Expected number but got ${typeof value}`);
        };
        
        globalThis.assert.isNotNumber = function(value, message) {
            assert(typeof value !== 'number', message || `Expected not number but got ${typeof value}`);
        };
        
        globalThis.assert.isBoolean = function(value, message) {
            assert(typeof value === 'boolean', message || `Expected boolean but got ${typeof value}`);
        };
        
        globalThis.assert.isNotBoolean = function(value, message) {
            assert(typeof value !== 'boolean', message || `Expected not boolean but got ${typeof value}`);
        };
        
        globalThis.assert.typeOf = function(value, type, message) {
            assert(typeof value === type, message || `Expected ${type} but got ${typeof value}`);
        };
        
        globalThis.assert.instanceOf = function(object, constructor, message) {
            assert(object instanceof constructor, message || `Expected ${object} to be instance of ${constructor.name}`);
        };
        
        globalThis.assert.notInstanceOf = function(object, constructor, message) {
            assert(!(object instanceof constructor), message || `Expected ${object} not to be instance of ${constructor.name}`);
        };
        
        globalThis.assert.include = function(haystack, needle, message) {
            if (typeof haystack === 'string') {
                assert(haystack.includes(needle), message || `Expected "${haystack}" to include "${needle}"`);
            } else if (Array.isArray(haystack)) {
                assert(haystack.includes(needle), message || `Expected [${haystack}] to include ${needle}`);
            } else {
                assert(haystack.hasOwnProperty(needle), message || `Expected ${haystack} to include property ${needle}`);
            }
        };
        
        globalThis.assert.notInclude = function(haystack, needle, message) {
            if (typeof haystack === 'string') {
                assert(!haystack.includes(needle), message || `Expected "${haystack}" not to include "${needle}"`);
            } else if (Array.isArray(haystack)) {
                assert(!haystack.includes(needle), message || `Expected [${haystack}] not to include ${needle}`);
            } else {
                assert(!haystack.hasOwnProperty(needle), message || `Expected ${haystack} not to include property ${needle}`);
            }
        };
        "#;

        context.eval(boa_engine::Source::from_bytes(assert_code))?;
        info!("✅ Assert API registered successfully");
        Ok(())
    }
}
