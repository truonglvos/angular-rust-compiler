import { defineConfig } from 'vite';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import path from 'path';
import fs from 'fs';

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Resolve binding path relative to this file
// demo-app/vite.config.mjs -> packages/binding
const bindingPath = path.resolve(__dirname, '../packages/binding');
const { Compiler } = require(bindingPath);

const compiler = new Compiler();

// Esbuild plugin for Angular linker during pre-bundling (for regular Vite with esbuild)
function angularLinkerEsbuildPlugin() {
  return {
    name: 'angular-linker-esbuild',
    setup(build) {
      // Handle all .mjs and .js files in @angular packages
      // Broadened filter to handle absolute paths correctly across OSes
      build.onLoad({ filter: /@angular\/.*\.(mjs|js)$/ }, async (args) => {
        // console.log(`[Linker] Attempting to process: ${args.path}`);
        const code = await fs.promises.readFile(args.path, 'utf8');

        // Check if file contains partial declarations
        if (!code.includes('ɵɵngDeclare')) {
          return { contents: code, loader: 'js' };
        }

        try {
          const result = compiler.linkFile(args.path, code);
          if (result.startsWith('/* Linker Error')) {
            // console.error(`[Pre-bundle Linker Error] ${args.path}:\n${result}`);
            return { contents: code, loader: 'js' };
          }
          return { contents: result, loader: 'js' };
        } catch (e) {
          // console.error(`[Pre-bundle Linker Failed] ${args.path}:`, e);
          return { contents: code, loader: 'js' };
        }
      });
    },
  };
}

// Rollup/Rolldown-compatible plugin for Angular linker during pre-bundling
function angularLinkerRolldownPlugin() {
  return {
    name: 'angular-linker-rolldown',
    async transform(code, id) {
      // Debug: Log all processed files to understand what's being transformed
      if (id.includes('angular')) {
        console.log(`[Linker Debug] Checking: ${id}`);
      }

      // Only process @angular packages with .mjs or .js extensions
      // Use more flexible matching for different path formats
      const isAngularPackage = id.includes('@angular') || id.includes('/@angular/');
      const isNodeModules = id.includes('node_modules');
      // Strip query string before checking extension
      const cleanId = id.split('?')[0];
      const isJsFile = cleanId.endsWith('.mjs') || cleanId.endsWith('.js');

      if (!isAngularPackage || !isNodeModules || !isJsFile) {
        return null;
      }

      // Check if file contains partial declarations
      if (!code.includes('ɵɵngDeclare')) {
        return null;
      }

      console.log(`[Rust Linker] Linking: ${cleanId}`);

      try {
        const result = compiler.linkFile(cleanId, code);

        if (result.startsWith('/* Linker Error')) {
          console.error(`[Rolldown Linker Error] ${id}:\n${result}`);
          return null;
        }
        return { code: result, map: null };
      } catch (e) {
        console.error(`[Rolldown Linker Failed] ${id}:`, e);
        return null;
      }
    },
  };
}

function rustNgcPlugin() {
  return {
    name: 'rust-ngc-plugin',
    enforce: 'pre',
    transform(code, id) {
      // Skip pre-bundled dependencies (handled by esbuild plugin)
      if (id.includes('node_modules')) {
        // Only log if it's an Angular file that somehow wasn't pre-bundled
        if (id.includes('@angular') && id.includes('ɵɵngDeclare')) {
          console.log(`[Vite Plugin] Fallback linking for: ${id}`);
          const cleanId = id.split('?')[0];
          if (cleanId.endsWith('.mjs') || cleanId.endsWith('.js')) {
            try {
              const result = compiler.linkFile(id, code);
              if (result.startsWith('/* Linker Error')) {
                console.error(result);
                return null;
              }
              return { code: result, map: null };
            } catch (e) {
              console.error(`Linker failed for ${id}:`, e);
              return null;
            }
          }
        }
        return null;
      }

      if (!id.endsWith('.ts') || id.endsWith('.d.ts')) {
        return null;
      }

      try {
        const result = compiler.compile(id, code);

        if (result.startsWith('/* Error')) {
          console.error(result);
          throw new Error(`Rust Compilation Failed for ${id}`);
        }

        return {
          code: result,
          map: null,
        };
      } catch (err) {
        console.error('Compilation error:', err);
        throw err;
      }
    },
    handleHotUpdate({ file, server, modules }) {
      if (file.endsWith('.html')) {
        const tsFile = file.replace(/\.html$/, '.ts');
        console.log(`[HMR] HTML changed: ${file}`);
        console.log(`[HMR] Invalidate TS: ${tsFile}`);

        const mod = server.moduleGraph.getModuleById(tsFile);
        if (mod) {
          console.log(`[HMR] Found module, invalidating...`);
          server.moduleGraph.invalidateModule(mod);

          server.ws.send({
            type: 'full-reload',
            path: '*',
          });

          return [];
        } else {
          console.log(`[HMR] Module not found in graph`);
          server.ws.send({
            type: 'full-reload',
            path: '*',
          });
          return [];
        }
      }
    },
  };
}

export default defineConfig({
  plugins: [angularLinkerRolldownPlugin(), rustNgcPlugin()],
  resolve: {
    extensions: ['.ts', '.js', '.json'],
  },
  server: {
    port: 4300,
  },
  optimizeDeps: {
    // Exclude Angular packages from pre-bundling so linker plugin can process them
    exclude: [
      '@angular/core',
      '@angular/common',
      '@angular/platform-browser',
      '@angular/router',
      '@angular/forms',
    ],
    // Still include zone.js and rxjs which don't need linking
    include: ['zone.js', 'rxjs', 'rxjs/operators'],
  },
});
