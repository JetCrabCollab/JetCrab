use chitin::boa_engine::Context;
use tracing::info;

pub struct PathAPI;

impl PathAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("📁 Registering Path API...");

        let path_code = r#"
        globalThis.path = {
            join: function(...paths) {
                if (paths.length === 0) return '.';
                
                let result = '';
                for (let i = 0; i < paths.length; i++) {
                    const path = paths[i];
                    if (typeof path !== 'string') {
                        throw new TypeError('Path must be a string');
                    }
                    
                    if (path === '') continue;
                    
                    if (result === '') {
                        result = path;
                    } else {
                        if (result.endsWith('/') || result.endsWith('\\')) {
                            if (path.startsWith('/') || path.startsWith('\\')) {
                                result += path.slice(1);
                            } else {
                                result += path;
                            }
                        } else {
                            if (path.startsWith('/') || path.startsWith('\\')) {
                                result += path;
                            } else {
                                result += '/' + path;
                            }
                        }
                    }
                }
                
                return result || '.';
            },
            
            resolve: function(...paths) {
                if (paths.length === 0) {
                    return process.cwd ? process.cwd() : '/';
                }
                
                let resolvedPath = '';
                for (let i = paths.length - 1; i >= 0; i--) {
                    const path = paths[i];
                    if (typeof path !== 'string') {
                        throw new TypeError('Path must be a string');
                    }
                    
                    if (path === '') continue;
                    
                    if (path.startsWith('/') || path.startsWith('\\')) {
                        resolvedPath = path;
                        break;
                    }
                    
                    if (resolvedPath === '') {
                        resolvedPath = path;
                    } else {
                        resolvedPath = path + '/' + resolvedPath;
                    }
                }
                
                return resolvedPath || '/';
            },
            
            dirname: function(path) {
                if (typeof path !== 'string') {
                    throw new TypeError('Path must be a string');
                }
                
                const lastSlash = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
                if (lastSlash === -1) return '.';
                if (lastSlash === 0) return path[0];
                return path.slice(0, lastSlash);
            },
            
            basename: function(path, ext) {
                if (typeof path !== 'string') {
                    throw new TypeError('Path must be a string');
                }
                
                const lastSlash = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
                let basename = lastSlash === -1 ? path : path.slice(lastSlash + 1);
                
                if (ext && typeof ext === 'string' && basename.endsWith(ext)) {
                    basename = basename.slice(0, -ext.length);
                }
                
                return basename;
            },
            
            extname: function(path) {
                if (typeof path !== 'string') {
                    throw new TypeError('Path must be a string');
                }
                
                const lastSlash = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
                const lastDot = path.lastIndexOf('.');
                
                if (lastDot === -1 || lastDot <= lastSlash) return '';
                return path.slice(lastDot);
            },
            
            normalize: function(path) {
                if (typeof path !== 'string') {
                    throw new TypeError('Path must be a string');
                }
                
                let normalized = path.replace(/\/+/g, '/').replace(/\\+/g, '\\');
                
                if (normalized.startsWith('./')) {
                    normalized = normalized.slice(2);
                }
                
                if (normalized.endsWith('/.')) {
                    normalized = normalized.slice(0, -2);
                }
                
                return normalized || '.';
            },
            
            isAbsolute: function(path) {
                if (typeof path !== 'string') {
                    throw new TypeError('Path must be a string');
                }
                
                return path.startsWith('/') || path.startsWith('\\') || 
                       (path.length > 2 && path[1] === ':' && (path[2] === '/' || path[2] === '\\'));
            },
            
            relative: function(from, to) {
                if (typeof from !== 'string' || typeof to !== 'string') {
                    throw new TypeError('Paths must be strings');
                }
                
                if (from === to) return '';
                
                const fromParts = from.split(/[/\\]/).filter(p => p !== '');
                const toParts = to.split(/[/\\]/).filter(p => p !== '');
                
                let commonPrefix = 0;
                for (let i = 0; i < Math.min(fromParts.length, toParts.length); i++) {
                    if (fromParts[i] === toParts[i]) {
                        commonPrefix++;
                    } else {
                        break;
                    }
                }
                
                const upCount = fromParts.length - commonPrefix;
                const relativeParts = toParts.slice(commonPrefix);
                
                let result = '';
                for (let i = 0; i < upCount; i++) {
                    result += '../';
                }
                
                return result + relativeParts.join('/');
            },
            
            parse: function(path) {
                if (typeof path !== 'string') {
                    throw new TypeError('Path must be a string');
                }
                
                const dirname = this.dirname(path);
                const basename = this.basename(path);
                const extname = this.extname(path);
                const name = basename.slice(0, -extname.length);
                
                return {
                    root: path.startsWith('/') ? '/' : '',
                    dir: dirname,
                    base: basename,
                    ext: extname,
                    name: name
                };
            },
            
            format: function(pathObject) {
                if (typeof pathObject !== 'object' || pathObject === null) {
                    throw new TypeError('Path object must be an object');
                }
                
                const root = pathObject.root || '';
                const dir = pathObject.dir || '';
                const base = pathObject.base || '';
                const ext = pathObject.ext || '';
                const name = pathObject.name || '';
                
                if (base) {
                    return root + dir + (dir && !dir.endsWith('/') ? '/' : '') + base;
                } else {
                    return root + dir + (dir && !dir.endsWith('/') ? '/' : '') + name + ext;
                }
            },
            
            sep: '/',
            delimiter: ':',
            posix: {
                sep: '/',
                delimiter: ':'
            },
            win32: {
                sep: '\\',
                delimiter: ';'
            }
        };
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(path_code))?;
        info!("✅ Path API registered successfully");
        Ok(())
    }
}
