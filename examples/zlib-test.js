// Zlib Module Test
console.log('=== Zlib Module Test ===');

// Test compression constants
console.log('Testing compression constants:');
console.log('Z_NO_FLUSH:', zlib.constants.Z_NO_FLUSH);
console.log('Z_OK:', zlib.constants.Z_OK);
console.log('Z_BEST_COMPRESSION:', zlib.constants.Z_BEST_COMPRESSION);

// Test gzip compression/decompression
console.log('Testing gzip compression:');
const testData = 'Hello, World! This is a test string for compression.';
const gzipped = zlib.gzip(testData);
console.log('Original:', testData);
console.log('Gzipped:', gzipped);

const gunzipped = zlib.gunzip(gzipped);
console.log('Gunzipped:', gunzipped);

// Test deflate compression/decompression
console.log('Testing deflate compression:');
const deflated = zlib.deflate(testData);
console.log('Deflated:', deflated);

const inflated = zlib.inflate(deflated);
console.log('Inflated:', inflated);

// Test deflate raw compression/decompression
console.log('Testing deflate raw compression:');
const deflateRaw = zlib.deflateRaw(testData);
console.log('Deflate Raw:', deflateRaw);

const inflateRaw = zlib.inflateRaw(deflateRaw);
console.log('Inflate Raw:', inflateRaw);

// Test brotli compression/decompression
console.log('Testing brotli compression:');
const brotliCompressed = zlib.brotliCompress(testData);
console.log('Brotli Compressed:', brotliCompressed);

const brotliDecompressed = zlib.brotliDecompress(brotliCompressed);
console.log('Brotli Decompressed:', brotliDecompressed);

// Test stream creation
console.log('Testing stream creation:');
const gzipStream = zlib.createGzip();
console.log('Gzip stream type:', gzipStream.type);

const gunzipStream = zlib.createGunzip();
console.log('Gunzip stream type:', gunzipStream.type);

const deflateStream = zlib.createDeflate();
console.log('Deflate stream type:', deflateStream.type);

const inflateStream = zlib.createInflate();
console.log('Inflate stream type:', inflateStream.type);

const brotliCompressStream = zlib.createBrotliCompress();
console.log('Brotli compress stream type:', brotliCompressStream.type);

const brotliDecompressStream = zlib.createBrotliDecompress();
console.log('Brotli decompress stream type:', brotliDecompressStream.type);

// Test stream operations
console.log('Testing stream operations:');
gzipStream.write('test data');
gzipStream.end();

console.log('=== Zlib Module Test Completed Successfully ===');


