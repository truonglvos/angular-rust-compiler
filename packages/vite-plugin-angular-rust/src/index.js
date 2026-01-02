import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import path from 'path';
import fs from 'fs';

const require = createRequire(import.meta.url);
// ... (rest of top of file is fine, I will target the class function or bigger block)

// I will assume lines 1-3 are correct and just adding fs. 
// But replace_file_content matches EXACT text.
// I will use a larger context or multiple replacements.
// Integrating fs import and logic changes.

// ...

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Resolve binding path dynamically or expect it to be linkable?
const bindingPath = path.resolve(__dirname, '../../binding');
let Compiler;

try {
    const binding = require(bindingPath);
    Compiler = binding.Compiler;
} catch (e) {
    console.error('[vite-plugin-angular-rust] Failed to load Rust compiler binding from:', bindingPath);
    console.error(e);
}

/**
 * @param {Object} options
 * @param {string} [options.project] Path to angular.json
 */
export default function angularRust(options = {}) {
    let bundleCache = null;
    let isBundling = false;
    let projectRoot = process.cwd();
    let globalStyles = [];

    if (!Compiler) {
        return {
            name: 'vite-plugin-angular-rust',
            buildStart() {
                this.error('Rust compiler binding not found. Please ensure it is built.');
            }
        }
    }

    const compiler = new Compiler();

    const getBundle = async () => {
        if (bundleCache) return bundleCache;
        if (isBundling) {
            while (isBundling) {
                await new Promise((r) => setTimeout(r, 100));
            }
            return bundleCache;
        }

        isBundling = true;
        // console.log('[rustBundlePlugin] Bundling project in memory...');
        const startTime = Date.now();
        try {
            const configFile = options.project
                ? path.resolve(process.cwd(), options.project)
                : process.env.ANGULAR_PROJECT_PATH
                    ? process.env.ANGULAR_PROJECT_PATH
                    : path.resolve(process.cwd(), 'angular.json');

            projectRoot = path.dirname(configFile);

            // console.log(`[rustBundlePlugin] Using project config: ${configFile}`);
            const result = compiler.bundle(configFile);

            const files = result.files || {};
            const fileCount = Object.keys(files).length;
            // console.log(`[rustBundlePlugin] Bundle generated in ${Date.now() - startTime}ms. Files: ${fileCount}`);
            if (fileCount > 0) {
                // console.log('[rustBundlePlugin] Sample keys:', Object.keys(files).slice(0, 5));
            }

            if (fileCount === 0) {
                const bundle = result.bundleJs || result.bundle_js || '';
                if (bundle.startsWith('/* Bundle Error')) {
                    console.error(bundle);
                    throw new Error('Bundling failed');
                }
            }

            bundleCache = result;
            return result;
        } finally {
            isBundling = false;
        }
    };

    return {
        name: 'vite-plugin-angular-rust',
        enforce: 'pre',
        configureServer(server) {
            // Parse angular.json to get global styles
            globalStyles = [];
            try {
                const configPath = path.resolve(projectRoot, 'angular.json');
                if (fs.existsSync(configPath)) {
                    const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
                    const project = Object.values(config.projects)[0];
                    const styles = project?.architect?.build?.options?.styles || [];
                    globalStyles = styles.map(s => typeof s === 'string' ? s : s.input);
                    // console.log('[rustBundlePlugin] Found global styles:', globalStyles);
                }
            } catch (e) {
                console.warn('Failed to parse angular.json for styles:', e);
            }
        },

        async handleHotUpdate({ file, server, modules }) {
            if (file.endsWith('.ts') || file.endsWith('.html') || file.endsWith('.css') || file.endsWith('.scss')) {
                // Check global styles
                if (globalStyles.some(style => file.endsWith(style))) {
                    // console.log('[rustBundlePlugin] Global style change. Letting Vite handle it.');
                    return; // Let Vite handle default HMR
                }

                // console.log(`[rustBundlePlugin] Hot Update: ${file}`);
                const startTime = Date.now();

                // FAST PATH: Incremental Compilation
                if (bundleCache) {
                    let targetTsFile = file;
                    if (file.endsWith('.html')) targetTsFile = file.replace(/\.html$/, '.ts');
                    else if (file.endsWith('.css')) targetTsFile = file.replace(/\.css$/, '.ts');
                    else if (file.endsWith('.scss')) targetTsFile = file.replace(/\.scss$/, '.ts');

                    if (fs.existsSync(targetTsFile)) {
                        try {
                            const content = fs.readFileSync(targetTsFile, 'utf8');
                            // console.log(`[rustBundlePlugin] Incremental compile: ${targetTsFile}`);
                            const result = compiler.compile(targetTsFile, content);

                            if (targetTsFile.endsWith('app.ts')) {
                                // console.log('[rustBundlePlugin] Compiled app.ts output:', result.code);
                            }

                            if (result.code) {
                                // Update Cache
                                const relPath = path.relative(projectRoot, targetTsFile);
                                // Normalize slashes for key match
                                // Bundler outputs to 'dist/', so we must match that prefix
                                const key = 'dist/' + relPath.replace(/\\/g, '/').replace(/\.ts$/, '.js');
                                // console.log(`[rustBundlePlugin] Updating cache key: ${key}`);

                                if (bundleCache.files) {
                                    if (bundleCache.files[key]) {
                                        bundleCache.files[key] = result.code;
                                    } else {
                                        // console.warn(`[rustBundlePlugin] Key ${key} not found in cache. Available keys:`, Object.keys(bundleCache.files).slice(0, 5));
                                        bundleCache.files[key] = result.code;
                                    }

                                    // Update modules
                                    const virtualId = '\0' + key;
                                    const updatedModules = [];

                                    const mod = server.moduleGraph.getModuleById(virtualId);
                                    if (mod) {
                                        server.moduleGraph.invalidateModule(mod);
                                        updatedModules.push(mod);
                                    }
                                    const modAbs = server.moduleGraph.getModuleById('/' + virtualId);
                                    if (modAbs) {
                                        server.moduleGraph.invalidateModule(modAbs);
                                        updatedModules.push(modAbs);
                                    }

                                    // console.log(`[rustBundlePlugin] Incrementally updated ${key} in ${Date.now() - startTime}ms`);
                                    return updatedModules.length > 0 ? updatedModules : [];
                                }
                            } else {
                                // console.warn('[rustBundlePlugin] Incremental compile failed (no code reported). Falling back to full build.');
                            }
                        } catch (e) {
                            // console.error('[rustBundlePlugin] Incremental compile error:', e);
                        }
                    }
                }

                // Fallback to full rebuild if fast path fails or cache missing
                // console.log('[rustBundlePlugin] Performing full rebuild...');

                // Store old files for diffing
                const oldFiles = bundleCache ? bundleCache.files : {};

                bundleCache = null;
                // Eagerly rebuild so fresh content is ready
                const result = await getBundle();
                const newFiles = result.files || {};

                const updatedModules = [];

                Object.keys(newFiles).forEach(key => {
                    if (oldFiles[key] !== newFiles[key]) {
                        // Invalidate module
                        const virtualId = '\0' + key;
                        const mod = server.moduleGraph.getModuleById(virtualId);
                        if (mod) {
                            server.moduleGraph.invalidateModule(mod);
                            updatedModules.push(mod);
                        }
                        // Also support absolute path variant which Vite often uses for HMR
                        const modAbs = server.moduleGraph.getModuleById('/' + virtualId);
                        if (modAbs) {
                            server.moduleGraph.invalidateModule(modAbs);
                            updatedModules.push(modAbs);
                        }
                    }
                });

                if (updatedModules.length > 0) {
                    // console.log(`[rustBundlePlugin] HMR updating ${updatedModules.length} modules: ${updatedModules.map(m => m.url).join(', ')}`);
                    return updatedModules;
                } else {
                    // console.log('[rustBundlePlugin] No changed modules found in bundle. Performing full reload.');
                    server.ws.send({ type: 'full-reload', path: '*' });
                    return [];
                }
            }
        },
        async resolveId(id, importer) {
            const cleanId = id.split('?')[0];

            // 1. Virtual modules are self-resolving
            if (cleanId.startsWith('\0')) return cleanId;

            // 2. Entry points
            if (cleanId.endsWith('bundle.js')) return '\0bundle.js';
            if (cleanId.endsWith('scripts.js')) return '\0scripts.js';

            if (!bundleCache) await getBundle();

            // 3. Resolve path relative to project or importer
            let resolvedPath = id;

            // Handle virtual importer (e.g. \0src/main.js importing ./app/app)
            if (importer && importer.startsWith('\0')) {
                const virtualImporterPath = importer.slice(1); // strip \0
                // If importer is \0bundle.js, we assume root context
                const importerDir = virtualImporterPath === 'bundle.js'
                    ? projectRoot
                    : path.dirname(path.resolve(projectRoot, virtualImporterPath));

                resolvedPath = path.resolve(importerDir, id);
            } else if (importer && !importer.startsWith(projectRoot)) {
                // Importer is likely external or absolute path from FS
                resolvedPath = path.resolve(path.dirname(importer), id);
            } else if (importer) {
                resolvedPath = path.resolve(path.dirname(importer), id);
            } else {
                resolvedPath = path.resolve(projectRoot, id);
            }

            const key = path.relative(projectRoot, resolvedPath);

            // 4. Check if we have this file in our memory cache
            if (bundleCache && bundleCache.files) {
                // Try exact match
                if (bundleCache.files[key]) return '\0' + key;
                // Try adding extension if missing/implied
                if (bundleCache.files[key + '.js']) return '\0' + key + '.js';
                if (bundleCache.files[key + '.mjs']) return '\0' + key + '.mjs';
                if (bundleCache.files[key + '/index.js']) return '\0' + key + '/index.js';
            }

            return null;
        },

        async transform(code, id) {
            // if (id.includes('button.mjs')) console.log(`[rustBundlePlugin] Checking: ${id}`);
            // Linker still needed for libraries (node_modules), but not for our compiled code 
            // because compiled code is already AOT compiled by Rust.
            if (id.includes('node_modules') && id.includes('@angular') && !id.endsWith('.css') && !id.endsWith('.scss')) {
                // console.log(`[rustBundlePlugin] Inspecting for linking: ${id}`);
                try {
                    const result = compiler.linkFile(id, code);
                    if (result.startsWith('/* Linker Error')) {
                        // console.error(`[rustBundlePlugin] Linker Error for ${id}:`, result.slice(0, 200));
                        return null; // Return null to let Vite continue with original
                    }
                    if (result !== code) {
                         // console.log(`[rustBundlePlugin] Successfully linked: ${id} (${code.length} -> ${result.length} bytes)`);
                         // Add marker to verify transform is applied
                         const markedResult = `/* LINKED BY RUST LINKER */\n${result}`;
                         return { code: markedResult, map: null };
                    } else {
                         // console.log(`[rustBundlePlugin] No linking changes for: ${id}`);
                    }
                } catch (e) {
                    // console.error(`[rustBundlePlugin] Linker exception for ${id}:`, e);
                }
            }
            return null;
        },

        async load(id) {
            if (!bundleCache) await getBundle();

            // Bootstrap Bundle
            if (id === '\0bundle.js') {
                let content = `
(function() {
  const originalWarn = console.warn;
  console.warn = function(...args) {
    if (typeof args[0] === 'string' && args[0].includes('NG0912')) return;
    originalWarn.apply(console, args);
  };
})();
`;

                // Inject global styles dynamically from angular.json
                try {
                    const configPath = path.resolve(projectRoot, 'angular.json');
                    if (fs.existsSync(configPath)) {
                        const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
                        // Use first project or default logic
                        const projectKey = Object.keys(config.projects)[0]; // naive
                        const project = config.projects[projectKey];
                        const styles = project?.architect?.build?.options?.styles || [];
                        styles.forEach(style => {
                            let stylePath = typeof style === 'string' ? style : style.input;
                            // Resolve absolute path for node_modules to bypass exports
                            if (stylePath.startsWith('node_modules/')) {
                                let currentDir = projectRoot;
                                let foundPath = null;
                                let depth = 0;
                                const maxDepth = 10;
                                
                                while (depth < maxDepth) {
                                  const tryPath = path.resolve(currentDir, stylePath);
                                  if (fs.existsSync(tryPath)) {
                                    foundPath = tryPath;
                                    break;
                                  }
                                  const parent = path.dirname(currentDir);
                                  if (parent === currentDir) break;
                                  currentDir = parent;
                                  depth++;
                                }

                                if (foundPath) {
                                     // Ensure we use a clean absolute path that Vite can serve
                                     content += ` import '${foundPath}';`;
                                } else {
                                     // console.warn(`[rustBundlePlugin] Could not find style: ${stylePath}`);
                                     // Fallback
                                     const absPath = path.resolve(projectRoot, stylePath);
                                     content += ` import '${absPath}';`;
                                }
                            } else {
                                content += ` import '/${stylePath}';`;
                            }
                        });
                    }
                } catch (e) {
                    // console.warn('[rustBundlePlugin] Failed to inject styles:', e);
                }

                // Bootstrap main entry point
                const files = bundleCache.files || {};
                const mainFile = Object.keys(files).find(f => f.endsWith('/main.js') || f === 'main.js');
                if (mainFile) {
                    // console.log(`[rustBundlePlugin] Bootstrap modular v2: import '\0${mainFile}'`);
                    // Use \0 to ensure it goes through our virtual resolution
                    content += ` import '\0${mainFile}';`;

                    // Inject HMR Accept for Main Module
                    content += `
if (import.meta.hot) {
    import.meta.hot.accept();
    import.meta.hot.dispose(() => {
        console.log('[HMR] Disposing bundle. Cleanup handled by next bootstrap.');
    });
}
`;
                } else {
                    // console.warn('[rustBundlePlugin] No main.js found. Falling back to bundle dump (legacy).');
                    return bundleCache.bundleJs || '';
                }
                return content;
            }

            if (id === '\0scripts.js') return bundleCache.scriptsJs || '';

            if (id.startsWith('\0')) {
                const key = id.slice(1);
                if (bundleCache && bundleCache.files && bundleCache.files[key]) {
                    let code = bundleCache.files[key];
                    // Intercept main.js to capture application ref
                    if (key.endsWith('main.js')) {
                        if (!code.includes('const __hmrBootstrap')) {
                            code = code.replace(/bootstrapApplication\s*\(/, '__hmrBootstrap(');
                            code += `
async function __hmrBootstrap(...args) {
  if (window.__ngAppRef) {
    try {
      const ref = await window.__ngAppRef;
      if (ref) {
          console.log('[HMR] Destroying old app...');
          ref.destroy();
      }
    } catch(e) { console.error('[HMR] Cleanup error:', e); }
  }
  
  let root = document.querySelector('app-root');
  if (!root) {
      root = document.createElement('app-root');
      document.body.appendChild(root);
  } else {
      root.innerHTML = '';
  }
  
  const promise = bootstrapApplication(...args);
  window.__ngAppRef = promise;
  return promise;
}
`;
                        }
                    }
                    return code;
                }
            }

            return null;
        },
        async transformIndexHtml(html) {
            const result = await getBundle();

            if (result.indexHtml) {
                // console.log('[rustBundlePlugin] Serving in-memory index.html');
                // Strip original main.ts script to avoid double loading/errors, as we inject bundle.js
                let cleanHtml = result.indexHtml.replace(/<script[^>]*src=["']\/?src\/main\.ts["'][^>]*>[\s\S]*?<\/script>/gi, '');
                // Remove style link key as we inject it via JS now
                cleanHtml = cleanHtml.replace(/<link[^>]*href=["']\/?styles\.css["'][^>]*>/gi, '');
                return cleanHtml;
            }
            return html;
        },
    };
}
