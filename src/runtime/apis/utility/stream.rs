use chitin::boa_engine::Context;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Debug, Clone)]
pub enum StreamState {
    Readable,
    Writable,
    Duplex,
    Transform,
}

#[derive(Debug, Clone)]
pub struct StreamOptions {
    pub high_water_mark: usize,
    pub object_mode: bool,
    pub encoding: Option<String>,
    pub auto_destroy: bool,
}

impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            high_water_mark: 16 * 1024,
            object_mode: false,
            encoding: None,
            auto_destroy: true,
        }
    }
}

#[derive(Debug)]
pub struct ReadableStream {
    pub buffer: VecDeque<u8>,
    pub state: StreamState,
    pub options: StreamOptions,
    pub destroyed: bool,
    pub readable: bool,
    pub readable_ended: bool,
    pub readable_destroyed: bool,
}

impl ReadableStream {
    pub fn new(options: StreamOptions) -> Self {
        Self {
            buffer: VecDeque::new(),
            state: StreamState::Readable,
            options,
            destroyed: false,
            readable: true,
            readable_ended: false,
            readable_destroyed: false,
        }
    }

    pub fn push(&mut self, chunk: Vec<u8>) -> bool {
        if self.readable_ended || self.destroyed {
            return false;
        }

        self.buffer.extend(chunk);
        self.readable = true;
        true
    }

    pub fn read(&mut self, size: Option<usize>) -> Option<Vec<u8>> {
        if self.buffer.is_empty() || self.destroyed {
            return None;
        }

        let read_size = size.unwrap_or(self.buffer.len());
        let mut result = Vec::new();

        for _ in 0..read_size {
            if let Some(byte) = self.buffer.pop_front() {
                result.push(byte);
            } else {
                break;
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
        self.readable = false;
        self.readable_destroyed = true;
        self.buffer.clear();
    }

    pub fn end(&mut self) {
        self.readable_ended = true;
        self.readable = false;
    }
}

#[derive(Debug)]
pub struct WritableStream {
    pub state: StreamState,
    pub options: StreamOptions,
    pub destroyed: bool,
    pub writable: bool,
    pub writable_ended: bool,
    pub writable_destroyed: bool,
}

impl WritableStream {
    pub fn new(options: StreamOptions) -> Self {
        Self {
            state: StreamState::Writable,
            options,
            destroyed: false,
            writable: true,
            writable_ended: false,
            writable_destroyed: false,
        }
    }

    pub fn write(&mut self, chunk: Vec<u8>) -> bool {
        if self.writable_ended || self.destroyed {
            return false;
        }
        true
    }

    pub fn end(&mut self) {
        self.writable_ended = true;
        self.writable = false;
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
        self.writable = false;
        self.writable_destroyed = true;
    }
}

#[derive(Debug)]
pub struct DuplexStream {
    pub readable: ReadableStream,
    pub writable: WritableStream,
    pub state: StreamState,
    pub options: StreamOptions,
    pub destroyed: bool,
}

impl DuplexStream {
    pub fn new(options: StreamOptions) -> Self {
        Self {
            readable: ReadableStream::new(options.clone()),
            writable: WritableStream::new(options.clone()),
            state: StreamState::Duplex,
            options,
            destroyed: false,
        }
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
        self.readable.destroy();
        self.writable.destroy();
    }
}

pub struct StreamAPI;

impl StreamAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("🌊 Registering Stream API...");

        let stream_code = r#"
        class Readable {
            constructor(options = {}) {
                this.readable = true;
                this.readableEnded = false;
                this.readableDestroyed = false;
                this.destroyed = false;
                this.highWaterMark = options.highWaterMark || 16384;
                this.objectMode = options.objectMode || false;
                this.encoding = options.encoding || null;
                this.autoDestroy = options.autoDestroy !== false;
                this.buffer = [];
                
                console.log('🌊 Readable stream created');
            }
            
            read(size) {
                if (this.buffer.length === 0) {
                    return null;
                }
                
                const chunk = this.buffer.shift();
                console.log(`🌊 Readable stream read: ${chunk.length} bytes`);
                return chunk;
            }
            
            push(chunk, encoding) {
                if (this.readableEnded || this.destroyed) {
                    return false;
                }
                
                this.buffer.push(chunk);
                console.log(`🌊 Readable stream pushed: ${chunk.length} bytes`);
                return true;
            }
            
            unshift(chunk, encoding) {
                if (this.readableEnded || this.destroyed) {
                    return false;
                }
                
                this.buffer.unshift(chunk);
                console.log(`🌊 Readable stream unshifted: ${chunk.length} bytes`);
                return true;
            }
            
            destroy(error) {
                this.destroyed = true;
                this.readable = false;
                this.readableDestroyed = true;
                this.buffer = [];
                console.log('🌊 Readable stream destroyed');
            }
            
            end(chunk, encoding) {
                this.readableEnded = true;
                this.readable = false;
                console.log('🌊 Readable stream ended');
            }
            
            isPaused() {
                return false;
            }
            
            pause() {
                console.log('🌊 Readable stream paused');
            }
            
            resume() {
                console.log('🌊 Readable stream resumed');
            }
            
            pipe(destination, options) {
                console.log('🌊 Readable stream piped to destination');
                return destination;
            }
            
            unpipe(destination) {
                console.log('🌊 Readable stream unpiped from destination');
                return this;
            }
            
            wrap(oldStream) {
                console.log('🌊 Readable stream wrapped');
                return this;
            }
        }

        class Writable {
            constructor(options = {}) {
                this.writable = true;
                this.writableEnded = false;
                this.writableDestroyed = false;
                this.destroyed = false;
                this.highWaterMark = options.highWaterMark || 16384;
                this.objectMode = options.objectMode || false;
                this.autoDestroy = options.autoDestroy !== false;
                
                console.log('🌊 Writable stream created');
            }
            
            write(chunk, encoding, callback) {
                if (this.writableEnded || this.destroyed) {
                    return false;
                }
                
                console.log(`🌊 Writable stream write: ${chunk.length} bytes`);
                
                if (callback) {
                    callback();
                }
                
                return true;
            }
            
            end(chunk, encoding, callback) {
                this.writableEnded = true;
                this.writable = false;
                console.log('🌊 Writable stream ended');
                
                if (callback) {
                    callback();
                }
            }
            
            destroy(error) {
                this.destroyed = true;
                this.writable = false;
                this.writableDestroyed = true;
                console.log('🌊 Writable stream destroyed');
            }
            
            cork() {
                console.log('🌊 Writable stream corked');
            }
            
            uncork() {
                console.log('🌊 Writable stream uncorked');
            }
            
            setDefaultEncoding(encoding) {
                console.log(`🌊 Writable stream default encoding set: ${encoding}`);
                return this;
            }
        }

        class Duplex extends Readable {
            constructor(options = {}) {
                super(options);
                this.writable = true;
                this.writableEnded = false;
                this.writableDestroyed = false;
                this.highWaterMark = options.highWaterMark || 16384;
                this.objectMode = options.objectMode || false;
                this.autoDestroy = options.autoDestroy !== false;
                
                console.log('🌊 Duplex stream created');
            }
            
            write(chunk, encoding, callback) {
                if (this.writableEnded || this.destroyed) {
                    return false;
                }
                
                console.log(`🌊 Duplex stream write: ${chunk.length} bytes`);
                
                if (callback) {
                    callback();
                }
                
                return true;
            }
            
            end(chunk, encoding, callback) {
                this.writableEnded = true;
                this.writable = false;
                console.log('🌊 Duplex stream ended');
                
                if (callback) {
                    callback();
                }
            }
            
            destroy(error) {
                this.destroyed = true;
                this.readable = false;
                this.writable = false;
                this.readableDestroyed = true;
                this.writableDestroyed = true;
                console.log('🌊 Duplex stream destroyed');
            }
        }

        class Transform extends Duplex {
            constructor(options = {}) {
                super(options);
                console.log('🌊 Transform stream created');
            }
            
            _transform(chunk, encoding, callback) {
                console.log(`🌊 Transform stream transform: ${chunk.length} bytes`);
                callback(null, chunk);
            }
            
            _flush(callback) {
                console.log('🌊 Transform stream flush');
                callback();
            }
        }

        class PassThrough extends Transform {
            constructor(options = {}) {
                super(options);
                console.log('🌊 PassThrough stream created');
            }
            
            _transform(chunk, encoding, callback) {
                console.log(`🌊 PassThrough stream transform: ${chunk.length} bytes`);
                callback(null, chunk);
            }
        }

        globalThis.stream = {
            Readable: Readable,
            Writable: Writable,
            Duplex: Duplex,
            Transform: Transform,
            PassThrough: PassThrough,
            
            finished: function(stream, callback) {
                console.log('🌊 Stream finished');
                if (callback) {
                    callback();
                }
            },
            
            pipeline: function(...streams) {
                console.log('🌊 Stream pipeline created');
                return streams[streams.length - 1];
            },
            
            compose: function(...streams) {
                console.log('🌊 Stream compose');
                return streams[0];
            },
            
            addAbortSignal: function(signal, stream) {
                console.log('🌊 Stream abort signal added');
                return stream;
            },
            
            getDefaultHighWaterMark: function(objectMode) {
                return objectMode ? 16 : 16384;
            },
            
            setDefaultHighWaterMark: function(objectMode, value) {
                console.log(`🌊 Stream default high water mark set: ${value}`);
            },
            
            isDisturbed: function(stream) {
                return false;
            },
            
            isErrored: function(stream) {
                return false;
            },
            
            isReadable: function(stream) {
                return stream.readable;
            },
            
            isWritable: function(stream) {
                return stream.writable;
            },
            
            isDuplex: function(stream) {
                return stream instanceof Duplex;
            },
            
            isTransform: function(stream) {
                return stream instanceof Transform;
            },
            
            isPassThrough: function(stream) {
                return stream instanceof PassThrough;
            },
            
            isStream: function(stream) {
                return stream instanceof Readable || stream instanceof Writable;
            },
            
            constants: {
                kReadableStream: 'readable',
                kWritableStream: 'writable',
                kDuplexStream: 'duplex',
                kTransformStream: 'transform',
                kPassThroughStream: 'passthrough'
            }
        };

        globalThis.Readable = Readable;
        globalThis.Writable = Writable;
        globalThis.Duplex = Duplex;
        globalThis.Transform = Transform;
        globalThis.PassThrough = PassThrough;
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(stream_code))?;
        info!("✅ Stream API registered successfully");
        Ok(())
    }
}
