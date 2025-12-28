/**
 * Angular Linker Plugin for Rolldown
 *
 * Use this plugin with rolldown-vite or standalone Rolldown.
 *
 * @example
 * ```js
 * import { angularLinkerRolldownPlugin } from 'angular-rust-plugins/linker/rolldown';
 * import { defineConfig } from 'vite';
 *
 * export default defineConfig({
 *   plugins: [angularLinkerRolldownPlugin()],
 *   optimizeDeps: {
 *     exclude: [
 *       '@angular/core',
 *       '@angular/common',
 *       '@angular/platform-browser',
 *       '@angular/router',
 *     ],
 *   },
 * });
 * ```
 */

import { createRequire } from "module";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import type { CompilerBinding, LinkerOptions, LinkerResult } from "./types";
import { cleanModuleId } from "./types";

let compilerInstance: CompilerBinding | null = null;

function getCompiler(options?: LinkerOptions): CompilerBinding {
  if (compilerInstance) {
    return compilerInstance;
  }

  try {
    let binding: { Compiler: new () => CompilerBinding };

    if (options?.bindingPath) {
      const require = createRequire(import.meta.url);
      binding = require(options.bindingPath);
    } else {
      // Load from bundled binding directory
      const currentFileUrl = import.meta.url;
      const currentFilePath = fileURLToPath(currentFileUrl);
      const currentDir = dirname(currentFilePath);
      const require = createRequire(currentFileUrl);

      // Try multiple possible binding locations
      const possiblePaths = [
        join(currentDir, "..", "binding"), // dist/linker/../binding
        join(currentDir, "binding"), // same directory
        join(currentDir, "..", "..", "binding"), // deeper nesting
      ];

      let loadedBinding: { Compiler: new () => CompilerBinding } | null = null;
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

export interface RolldownPlugin {
  name: string;
  transform(
    code: string,
    id: string
  ): Promise<LinkerResult | null> | LinkerResult | null;
}

/**
 * Creates a Rolldown-compatible plugin for Angular linker
 */
export function angularLinkerRolldownPlugin(
  options?: LinkerOptions
): RolldownPlugin {
  const debug = options?.debug ?? false;
  let compiler: CompilerBinding;

  return {
    name: "angular-linker-rolldown",
    async transform(code: string, id: string): Promise<LinkerResult | null> {
      // Lazy initialize compiler
      if (!compiler) {
        compiler = getCompiler(options);
      }

      // Logic from vite.config.mjs
      const isAngularPackage = id.includes('@angular') || id.includes('/@angular/');
      const isNodeModules = id.includes('node_modules');
      const cleanId = cleanModuleId(id);
      const isJsFile = cleanId.endsWith('.mjs') || cleanId.endsWith('.js');

      if (!isAngularPackage || !isNodeModules || !isJsFile) {
        return null;
      }

      // Check if file contains partial declarations
      if (!code.includes('ɵɵngDeclare')) {
        return null;
      }

      if (debug) {
        console.log(`[Angular Linker] Linking: ${cleanId}`);
      }

      try {
        const result = compiler.linkFile(cleanId, code);

        if (result.startsWith("/* Linker Error")) {
          console.error(`[Rolldown Linker Error] ${id}:\n${result}`);
          return null;
        }

        if (debug) {
          console.log(`[Angular Linker] Successfully linked: ${cleanId}`);
        }

        return { code: result, map: null };
      } catch (e) {
        console.error(`[Rolldown Linker Failed] ${id}:`, e);
        return null;
      }
    },
  };
}

export default angularLinkerRolldownPlugin;
