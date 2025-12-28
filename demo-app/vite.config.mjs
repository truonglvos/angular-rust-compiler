import { defineConfig } from 'vite';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import path from 'path';
import fs from 'fs';

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Resolve binding path relative to this file
const bindingPath = path.resolve(__dirname, '../packages/binding');
const { Compiler } = require(bindingPath);

const compiler = new Compiler();

// ============ Diagnostic Formatting ============
const RED = '\x1b[31m';
const YELLOW = '\x1b[33m';
const CYAN = '\x1b[36m';
const BOLD = '\x1b[1m';
const RESET = '\x1b[0m';

function formatDiagnostic(diag, sourceCode) {
  const level = 'WARNING';
  const codeStr = `NG${diag.code}`;
  const file = diag.file || 'unknown';

  let line = 1;
  let col = 0;
  let lineStartPos = 0;

  if (diag.start !== undefined && diag.start !== null) {
    for (let i = 0; i < diag.start && i < sourceCode.length; i++) {
      if (sourceCode[i] === '\n') {
        line++;
        col = 0;
        lineStartPos = i + 1;
      } else {
        col++;
      }
    }
  }

  let output = `\n${BOLD}${YELLOW}▲ [${level}] ${RED}${codeStr}${RESET}${BOLD}: ${diag.message}${RESET} ${YELLOW}[plugin rust-ngc-plugin]${RESET}\n`;

  const lineStr = line.toString();
  const colStr = (col + 1).toString();
  output += `\n    ${CYAN}${file}:${lineStr}:${colStr}:${RESET}\n`;

  let lineEndPos = sourceCode.indexOf('\n', lineStartPos);
  if (lineEndPos === -1) lineEndPos = sourceCode.length;
  const lineContent = sourceCode.substring(lineStartPos, lineEndPos);

  output += `      ${BOLD}${lineStr} │ ${RESET}${lineContent}\n`;

  const gutterWidth = lineStr.length + 3;
  const gutterEmpty = ' '.repeat(gutterWidth);
  const length = diag.length || 1;
  const underline = '~'.repeat(length);

  output += `      ${gutterEmpty}${' '.repeat(col)}${RED}${underline}${RESET}`;

  return output;
}

// ============ Vite Plugins ============

// Angular Linker plugin for @angular packages
function angularLinkerRolldownPlugin() {
  return {
    name: 'angular-linker-rolldown',
    async transform(code, id) {
      const isAngularPackage = id.includes('@angular') || id.includes('/@angular/');
      const isNodeModules = id.includes('node_modules');
      const cleanId = id.split('?')[0];
      const isJsFile = cleanId.endsWith('.mjs') || cleanId.endsWith('.js');

      if (!isAngularPackage || !isNodeModules || !isJsFile) {
        return null;
      }

      if (!code.includes('ɵɵngDeclare')) {
        return null;
      }

      try {
        // Cache is handled in Rust side
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

// Rust NGC plugin for .ts files
function rustNgcPlugin() {
  return {
    name: 'rust-ngc-plugin',
    enforce: 'pre',
    transform(code, id) {
      // Skip pre-bundled dependencies
      if (id.includes('node_modules')) {
        if (id.includes('@angular') && code.includes('ɵɵngDeclare')) {
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

      const cleanId = id.split('?')[0];
      if (!cleanId.endsWith('.ts') || cleanId.endsWith('.d.ts')) {
        return null;
      }

      try {
        // Cache is handled in Rust side
        const result = compiler.compile(id, code);

        if (typeof result === 'string') {
          if (result.startsWith('/* Error')) {
            console.error(result);
            throw new Error(`Rust Compilation Failed for ${id}`);
          }
          return { code: result, map: null };
        }

        const { code: compiledCode, diagnostics } = result;

        if (compiledCode.startsWith('/* Error')) {
          console.error(compiledCode);
          throw new Error(`Rust Compilation Failed for ${id}`);
        }

        if (diagnostics && diagnostics.length > 0) {
          diagnostics.forEach((diag) => {
            console.warn(formatDiagnostic(diag, code));
          });
        }

        return { code: compiledCode, map: null };
      } catch (err) {
        console.error('Compilation error:', err);
        throw err;
      }
    },
    handleHotUpdate({ file, server }) {
      if (file.endsWith('.html')) {
        const tsFile = file.replace(/\.html$/, '.ts');
        const mod = server.moduleGraph.getModuleById(tsFile);
        if (mod) {
          server.moduleGraph.invalidateModule(mod);
          server.ws.send({ type: 'full-reload', path: '*' });
          return [];
        } else {
          server.ws.send({ type: 'full-reload', path: '*' });
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
    exclude: [
      '@angular/core',
      '@angular/common',
      '@angular/platform-browser',
      '@angular/router',
      '@angular/forms',
    ],
    include: ['zone.js', 'rxjs', 'rxjs/operators'],
  },
});
