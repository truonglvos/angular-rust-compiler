/**
 * Angular Compiler Plugin for Vite
 *
 * This plugin compiles Angular TypeScript files using the Rust-based Angular compiler.
 * Use with the linker plugin for a complete Angular build solution.
 *
 * @example
 * ```js
 * import { angularCompilerVitePlugin } from 'angular-rust-plugins/compiler/vite';
 * import { angularLinkerVitePlugin } from 'angular-rust-plugins/linker/vite';
 * import { defineConfig } from 'vite';
 *
 * export default defineConfig({
 *   plugins: [
 *     angularLinkerVitePlugin(),
 *     angularCompilerVitePlugin(),
 *   ],
 * });
 * ```
 */

import type { Plugin, HmrContext } from "vite";
import { createRequire } from "module";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import { Compiler } from "../binding";

let compilerInstance: Compiler | null = null;

export interface CompilerOptions {
  /**
   * Enable debug logging
   * @default false
   */
  debug?: boolean;

  /**
   * Custom path to the Angular Rust binding package
   */
  bindingPath?: string;
}

// ============ Diagnostic Formatting ============
const RED = '\x1b[31m';
const YELLOW = '\x1b[33m';
const CYAN = '\x1b[36m';
const BOLD = '\x1b[1m';
const RESET = '\x1b[0m';

function formatDiagnostic(diag: any, sourceCode: string): string {
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

function getCompiler(options?: CompilerOptions): Compiler {
  if (compilerInstance) {
    return compilerInstance;
  }

  try {
    let binding: { Compiler: new () => Compiler };

    if (options?.bindingPath) {
      const require = createRequire(import.meta.url);
      binding = require(options.bindingPath);
    } else {
      // Load from bundled binding directory
      // Use import.meta.url to get the actual location of this file
      const currentFileUrl = import.meta.url;
      const currentFilePath = fileURLToPath(currentFileUrl);
      const currentDir = dirname(currentFilePath);
      const require = createRequire(currentFileUrl);

      // Try multiple possible binding locations
      const possiblePaths = [
        join(currentDir, "..", "binding"), // dist/compiler/../binding
        join(currentDir, "..", "..", "binding"), // in case of deeper nesting
        join(currentDir, "binding"), // same directory
      ];

      let loadedBinding: { Compiler: new () => Compiler } | null = null;
      let lastError: unknown = null;

      for (const bindingPath of possiblePaths) {
        try {
          loadedBinding = require(bindingPath);
          break;
        } catch (e) {
          lastError = e;
        }
      }

      if (!loadedBinding) {
        throw (
          lastError ||
          new Error("Could not find binding in any expected location")
        );
      }

      binding = loadedBinding;
    }

    compilerInstance = new binding.Compiler();
    return compilerInstance;
  } catch (e) {
    throw new Error(`Failed to load Angular Rust binding. Error: ${e}`);
  }
}

/**
 * Creates a Vite plugin for Angular Rust compiler
 * Compiles .ts files (except .d.ts) using the Rust compiler
 */
export function angularCompilerVitePlugin(options?: CompilerOptions): Plugin {
  const debug = options?.debug ?? false;
  let compiler: Compiler;

  return {
    name: "angular-rust-compiler",
    enforce: "pre",

    transform(code: string, id: string) {
      // Lazy initialize compiler
      if (!compiler) {
        compiler = getCompiler(options);
      }

      // Skip node_modules but check for Angular packages that need linking
      if (id.includes('node_modules')) {
        if (id.includes('@angular') && code.includes('ɵɵngDeclare')) {
          const cleanId = id.split('?')[0];
          if (cleanId.endsWith('.mjs') || cleanId.endsWith('.js')) {
            try {
              const result = compiler.linkFile(id, code);
              if (result.startsWith('/* Linker Error')) {
                 if (debug) console.error(`[Linker Error] ${id}: ${result}`);
                 return null;
              }
              return { code: result, map: null };
            } catch (e) {
               if (debug) console.error(`Linker failed for ${id}:`, e);
              return null;
            }
          }
        }
        return null;
      }

      // Only process TypeScript files, skip declaration files
      const cleanId = id.split('?')[0];
      if (!cleanId.endsWith('.ts') || cleanId.endsWith('.d.ts')) {
        return null;
      }

      if (debug) {
        console.log(`[Angular Compiler] Compiling: ${id}`);
      }

      try {
        const result = compiler.compile(id, code);

        // Handle structured result
        const { code: compiledCode, diagnostics } = result;

        if (compiledCode.startsWith('/* Error')) {
          console.error(`[Angular Compiler Error] ${id}:\n${compiledCode}`);
          throw new Error(`Rust Compilation Failed for ${id}`);
        }

        if (diagnostics && diagnostics.length > 0) {
          diagnostics.forEach((diag: any) => {
            console.warn(formatDiagnostic(diag, code));
          });
        }

        if (debug) {
          console.log(`[Angular Compiler] Successfully compiled: ${id}`);
        }

        return { code: compiledCode, map: null };
      } catch (e) {
        console.error(`[Angular Compiler Failed] ${id}:`, e);
        throw e;
      }
    },

    handleHotUpdate({ file, server }: HmrContext) {
      // When HTML template changes, invalidate the corresponding TS file
      if (file.endsWith(".html")) {
        const tsFile = file.replace(/\.html$/, ".ts");

        if (debug) {
          console.log(`[HMR] HTML changed: ${file}`);
          console.log(`[HMR] Invalidating TS: ${tsFile}`);
        }

        const mod = server.moduleGraph.getModuleById(tsFile);
        if (mod) {
          server.moduleGraph.invalidateModule(mod);
          server.ws.send({ type: "full-reload", path: "*" });
          return [];
        } else {
          server.ws.send({ type: "full-reload", path: "*" });
          return [];
        }
      }
    },
  };
}

export default angularCompilerVitePlugin;
