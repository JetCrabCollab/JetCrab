use boa_engine::Context;
use tracing::info;

pub struct BufferAPI;

impl BufferAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("📦 Registering Buffer API...");

        let buffer_code = r#"
        globalThis.Buffer = {
            from: function(data, encoding) {
                if (typeof data === 'string') {
                    return new Uint8Array(new TextEncoder().encode(data));
                } else if (Array.isArray(data)) {
                    return new Uint8Array(data);
                } else if (data instanceof ArrayBuffer) {
                    return new Uint8Array(data);
                }
                throw new Error('Invalid data type for Buffer.from');
            },
            
            alloc: function(size, fill, encoding) {
                const buffer = new Uint8Array(size);
                if (fill !== undefined) {
                    if (typeof fill === 'number') {
                        buffer.fill(fill);
                    } else if (typeof fill === 'string') {
                        const encoded = new TextEncoder().encode(fill);
                        for (let i = 0; i < size; i++) {
                            buffer[i] = encoded[i % encoded.length];
                        }
                    }
                }
                return buffer;
            },
            
            allocUnsafe: function(size) {
                return new Uint8Array(size);
            },
            
            allocUnsafeSlow: function(size) {
                return new Uint8Array(size);
            },
            
            byteLength: function(string, encoding) {
                if (typeof string === 'string') {
                    return new TextEncoder().encode(string).length;
                } else if (string instanceof Uint8Array) {
                    return string.length;
                }
                return 0;
            },
            
            compare: function(buf1, buf2) {
                const a = new Uint8Array(buf1);
                const b = new Uint8Array(buf2);
                const minLength = Math.min(a.length, b.length);
                
                for (let i = 0; i < minLength; i++) {
                    if (a[i] < b[i]) return -1;
                    if (a[i] > b[i]) return 1;
                }
                
                if (a.length < b.length) return -1;
                if (a.length > b.length) return 1;
                return 0;
            },
            
            concat: function(list, length) {
                if (!Array.isArray(list)) {
                    throw new Error('First argument must be an array');
                }
                
                let totalLength = 0;
                for (const buf of list) {
                    totalLength += buf.length;
                }
                
                if (length !== undefined) {
                    totalLength = Math.min(totalLength, length);
                }
                
                const result = new Uint8Array(totalLength);
                let offset = 0;
                
                for (const buf of list) {
                    const copyLength = Math.min(buf.length, totalLength - offset);
                    result.set(buf.slice(0, copyLength), offset);
                    offset += copyLength;
                    if (offset >= totalLength) break;
                }
                
                return result;
            },
            
            isBuffer: function(obj) {
                return obj instanceof Uint8Array;
            },
            
            isEncoding: function(encoding) {
                const validEncodings = ['utf8', 'utf-8', 'ascii', 'latin1', 'binary', 'base64', 'hex'];
                return validEncodings.includes(encoding);
            }
        };
        
        Uint8Array.prototype.toString = function(encoding) {
            if (encoding === 'hex') {
                return Array.from(this).map(b => b.toString(16).padStart(2, '0')).join('');
            } else if (encoding === 'base64') {
                return btoa(String.fromCharCode.apply(null, this));
            } else {
                return new TextDecoder().decode(this);
            }
        };
        
        Uint8Array.prototype.toJSON = function() {
            return Array.from(this);
        };
        
        Uint8Array.prototype.equals = function(other) {
            if (!(other instanceof Uint8Array)) return false;
            if (this.length !== other.length) return false;
            for (let i = 0; i < this.length; i++) {
                if (this[i] !== other[i]) return false;
            }
            return true;
        };
        
        Uint8Array.prototype.copy = function(target, targetStart, sourceStart, sourceEnd) {
            targetStart = targetStart || 0;
            sourceStart = sourceStart || 0;
            sourceEnd = sourceEnd || this.length;
            
            const length = Math.min(sourceEnd - sourceStart, target.length - targetStart);
            for (let i = 0; i < length; i++) {
                target[targetStart + i] = this[sourceStart + i];
            }
            return length;
        };
        
        Uint8Array.prototype.slice = function(start, end) {
            start = start || 0;
            end = end || this.length;
            
            if (start < 0) start = this.length + start;
            if (end < 0) end = this.length + end;
            
            start = Math.max(0, Math.min(start, this.length));
            end = Math.max(start, Math.min(end, this.length));
            
            return new Uint8Array(this.buffer, this.byteOffset + start, end - start);
        };
        
        Uint8Array.prototype.subarray = function(start, end) {
            return this.slice(start, end);
        };
        
        Uint8Array.prototype.fill = function(value, start, end) {
            start = start || 0;
            end = end || this.length;
            
            if (start < 0) start = this.length + start;
            if (end < 0) end = this.length + end;
            
            start = Math.max(0, Math.min(start, this.length));
            end = Math.max(start, Math.min(end, this.length));
            
            for (let i = start; i < end; i++) {
                this[i] = value;
            }
            return this;
        };
        
        Uint8Array.prototype.write = function(string, offset, length, encoding) {
            offset = offset || 0;
            length = length || this.length - offset;
            encoding = encoding || 'utf8';
            
            const encoder = new TextEncoder();
            const encoded = encoder.encode(string);
            const writeLength = Math.min(length, encoded.length, this.length - offset);
            
            for (let i = 0; i < writeLength; i++) {
                this[offset + i] = encoded[i];
            }
            return writeLength;
        };
        
        Uint8Array.prototype.readUInt8 = function(offset) {
            return this[offset || 0];
        };
        
        Uint8Array.prototype.readUInt16LE = function(offset) {
            offset = offset || 0;
            return this[offset] | (this[offset + 1] << 8);
        };
        
        Uint8Array.prototype.readUInt16BE = function(offset) {
            offset = offset || 0;
            return (this[offset] << 8) | this[offset + 1];
        };
        
        Uint8Array.prototype.readUInt32LE = function(offset) {
            offset = offset || 0;
            return this[offset] | (this[offset + 1] << 8) | (this[offset + 2] << 16) | (this[offset + 3] << 24);
        };
        
        Uint8Array.prototype.readUInt32BE = function(offset) {
            offset = offset || 0;
            return (this[offset] << 24) | (this[offset + 1] << 16) | (this[offset + 2] << 8) | this[offset + 3];
        };
        
        Uint8Array.prototype.writeUInt8 = function(value, offset) {
            this[offset || 0] = value;
        };
        
        Uint8Array.prototype.writeUInt16LE = function(value, offset) {
            offset = offset || 0;
            this[offset] = value & 0xff;
            this[offset + 1] = (value >> 8) & 0xff;
        };
        
        Uint8Array.prototype.writeUInt16BE = function(value, offset) {
            offset = offset || 0;
            this[offset] = (value >> 8) & 0xff;
            this[offset + 1] = value & 0xff;
        };
        
        Uint8Array.prototype.writeUInt32LE = function(value, offset) {
            offset = offset || 0;
            this[offset] = value & 0xff;
            this[offset + 1] = (value >> 8) & 0xff;
            this[offset + 2] = (value >> 16) & 0xff;
            this[offset + 3] = (value >> 24) & 0xff;
        };
        
        Uint8Array.prototype.writeUInt32BE = function(value, offset) {
            offset = offset || 0;
            this[offset] = (value >> 24) & 0xff;
            this[offset + 1] = (value >> 16) & 0xff;
            this[offset + 2] = (value >> 8) & 0xff;
            this[offset + 3] = value & 0xff;
        };
        "#;

        context.eval(boa_engine::Source::from_bytes(buffer_code))?;
        info!("✅ Buffer API registered successfully");
        Ok(())
    }
}
