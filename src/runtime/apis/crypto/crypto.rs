use boa_engine::{Context, JsValue, NativeFunction};
use tracing::info;

pub struct CryptoAPI;

impl CryptoAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔐 Registering Crypto API...");

        let crypto_code = r#"
        globalThis.crypto = {
            createHash: function(algorithm) {
                return {
                    algorithm: algorithm,
                    data: new Uint8Array(),
                    
                    update: function(data, encoding) {
                        if (typeof data === 'string') {
                            const encoder = new TextEncoder();
                            const newData = encoder.encode(data);
                            const combined = new Uint8Array(this.data.length + newData.length);
                            combined.set(this.data);
                            combined.set(newData, this.data.length);
                            this.data = combined;
                        } else if (data instanceof Uint8Array) {
                            const combined = new Uint8Array(this.data.length + data.length);
                            combined.set(this.data);
                            combined.set(data, this.data.length);
                            this.data = combined;
                        }
                        return this;
                    },
                    
                    digest: function(encoding) {
                        let hash = 0;
                        for (let i = 0; i < this.data.length; i++) {
                            hash = ((hash << 5) - hash + this.data[i]) & 0xffffffff;
                        }
                        
                        if (encoding === 'hex') {
                            return Math.abs(hash).toString(16).padStart(8, '0');
                        } else if (encoding === 'base64') {
                            const bytes = new Uint8Array(4);
                            bytes[0] = (hash >> 24) & 0xFF;
                            bytes[1] = (hash >> 16) & 0xFF;
                            bytes[2] = (hash >> 8) & 0xFF;
                            bytes[3] = hash & 0xFF;
                            return btoa(String.fromCharCode.apply(null, bytes));
                        } else {
                            return new Uint8Array([
                                (hash >> 24) & 0xFF,
                                (hash >> 16) & 0xFF,
                                (hash >> 8) & 0xFF,
                                hash & 0xFF
                            ]);
                        }
                    }
                };
            },
            
            createHmac: function(algorithm, key) {
                return {
                    algorithm: algorithm,
                    key: key,
                    data: new Uint8Array(),
                    
                    update: function(data, encoding) {
                        if (typeof data === 'string') {
                            const encoder = new TextEncoder();
                            const newData = encoder.encode(data);
                            const combined = new Uint8Array(this.data.length + newData.length);
                            combined.set(this.data);
                            combined.set(newData, this.data.length);
                            this.data = combined;
                        } else if (data instanceof Uint8Array) {
                            const combined = new Uint8Array(this.data.length + data.length);
                            combined.set(this.data);
                            combined.set(data, this.data.length);
                            this.data = combined;
                        }
                        return this;
                    },
                    
                    digest: function(encoding) {
                        let hash = 0;
                        for (let i = 0; i < this.data.length; i++) {
                            hash = ((hash << 5) - hash + this.data[i]) & 0xffffffff;
                        }
                        
                        if (typeof this.key === 'string') {
                            const keyBytes = new TextEncoder().encode(this.key);
                            for (let i = 0; i < keyBytes.length; i++) {
                                hash ^= keyBytes[i];
                            }
                        }
                        
                        if (encoding === 'hex') {
                            return Math.abs(hash).toString(16).padStart(8, '0');
                        } else if (encoding === 'base64') {
                            const bytes = new Uint8Array(4);
                            bytes[0] = (hash >> 24) & 0xFF;
                            bytes[1] = (hash >> 16) & 0xFF;
                            bytes[2] = (hash >> 8) & 0xFF;
                            bytes[3] = hash & 0xFF;
                            return btoa(String.fromCharCode.apply(null, bytes));
                        } else {
                            return new Uint8Array([
                                (hash >> 24) & 0xFF,
                                (hash >> 16) & 0xFF,
                                (hash >> 8) & 0xFF,
                                hash & 0xFF
                            ]);
                        }
                    }
                };
            },
            
            randomBytes: function(size) {
                if (typeof size !== 'number' || size < 0) {
                    throw new TypeError('Size must be a positive number');
                }
                
                const bytes = new Uint8Array(size);
                for (let i = 0; i < size; i++) {
                    bytes[i] = Math.floor(Math.random() * 256);
                }
                return bytes;
            },
            
            randomFill: function(buffer, offset, size) {
                if (!(buffer instanceof Uint8Array)) {
                    throw new TypeError('Buffer must be a Uint8Array');
                }
                
                offset = offset || 0;
                size = size || buffer.length - offset;
                
                for (let i = offset; i < offset + size; i++) {
                    buffer[i] = Math.floor(Math.random() * 256);
                }
                
                return buffer;
            },
            
            randomInt: function(min, max) {
                if (typeof min !== 'number' || typeof max !== 'number') {
                    throw new TypeError('Min and max must be numbers');
                }
                
                return Math.floor(Math.random() * (max - min + 1)) + min;
            },
            
            getRandomValues: function(array) {
                if (!(array instanceof Uint8Array) && !(array instanceof Uint32Array)) {
                    throw new TypeError('Array must be a Uint8Array or Uint32Array');
                }
                
                for (let i = 0; i < array.length; i++) {
                    array[i] = Math.floor(Math.random() * 256);
                }
                
                return array;
            },
            
            subtle: {
                generateKey: function(algorithm, extractable, keyUsages) {
                    return Promise.resolve({
                        type: 'secret',
                        extractable: extractable,
                        algorithm: algorithm,
                        usages: keyUsages
                    });
                },
                
                encrypt: function(algorithm, key, data) {
                    return Promise.resolve(new Uint8Array(data.length));
                },
                
                decrypt: function(algorithm, key, data) {
                    return Promise.resolve(new Uint8Array(data.length));
                },
                
                sign: function(algorithm, key, data) {
                    return Promise.resolve(new Uint8Array(64));
                },
                
                verify: function(algorithm, key, signature, data) {
                    return Promise.resolve(true);
                },
                
                digest: function(algorithm, data) {
                    return Promise.resolve(new Uint8Array(32));
                },
                
                deriveKey: function(algorithm, baseKey, derivedKeyAlgorithm, extractable, keyUsages) {
                    return Promise.resolve({
                        type: 'secret',
                        extractable: extractable,
                        algorithm: derivedKeyAlgorithm,
                        usages: keyUsages
                    });
                },
                
                deriveBits: function(algorithm, baseKey, length) {
                    return Promise.resolve(new Uint8Array(length / 8));
                },
                
                importKey: function(format, keyData, algorithm, extractable, keyUsages) {
                    return Promise.resolve({
                        type: 'secret',
                        extractable: extractable,
                        algorithm: algorithm,
                        usages: keyUsages
                    });
                },
                
                exportKey: function(format, key) {
                    return Promise.resolve(new Uint8Array(32));
                },
                
                wrapKey: function(format, key, wrappingKey, wrapAlgorithm) {
                    return Promise.resolve(new Uint8Array(32));
                },
                
                unwrapKey: function(format, wrappedKey, unwrappingKey, unwrapAlgorithm, unwrappedKeyAlgorithm, extractable, keyUsages) {
                    return Promise.resolve({
                        type: 'secret',
                        extractable: extractable,
                        algorithm: unwrappedKeyAlgorithm,
                        usages: keyUsages
                    });
                }
            }
        };
        
        globalThis.crypto_module = {
            createHash: globalThis.crypto.createHash,
            createHmac: globalThis.crypto.createHmac,
            randomBytes: globalThis.crypto.randomBytes,
            randomFill: globalThis.crypto.randomFill,
            randomInt: globalThis.crypto.randomInt,
            getRandomValues: globalThis.crypto.getRandomValues,
            subtle: globalThis.crypto.subtle,
            
            constants: {
                OPENSSL_VERSION_NUMBER: 0x10100000,
                SSL_OP_NO_SSLv2: 0x01000000,
                SSL_OP_NO_SSLv3: 0x02000000,
                SSL_OP_NO_TLSv1: 0x04000000,
                SSL_OP_NO_TLSv1_1: 0x08000000,
                SSL_OP_NO_TLSv1_2: 0x10000000,
                SSL_OP_NO_TLSv1_3: 0x20000000,
                SSL_OP_NO_RENEGOTIATION: 0x40000000,
                SSL_OP_CIPHER_SERVER_PREFERENCE: 0x00400000,
                SSL_OP_TLS_ROLLBACK_BUG: 0x00200000,
                SSL_OP_SINGLE_DH_USE: 0x00100000,
                SSL_OP_SINGLE_ECDH_USE: 0x00080000,
                SSL_OP_NO_COMPRESSION: 0x00020000,
                SSL_OP_NO_TICKET: 0x00004000,
                SSL_OP_ALL: 0x80000BFF,
                SSL_OP_PRIORITIZE_CHACHA: 0x00000020,
                SSL_OP_CIPHER_SERVER_PREFERENCE: 0x00400000,
                SSL_OP_TLS_ROLLBACK_BUG: 0x00200000,
                SSL_OP_SINGLE_DH_USE: 0x00100000,
                SSL_OP_SINGLE_ECDH_USE: 0x00080000,
                SSL_OP_NO_COMPRESSION: 0x00020000,
                SSL_OP_NO_TICKET: 0x00004000,
                SSL_OP_ALL: 0x80000BFF,
                SSL_OP_PRIORITIZE_CHACHA: 0x00000020
            }
        };
        "#;

        context.eval(boa_engine::Source::from_bytes(crypto_code))?;
        info!("✅ Crypto API registered successfully");
        Ok(())
    }
}
