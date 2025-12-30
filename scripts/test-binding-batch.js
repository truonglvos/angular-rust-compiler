const { Compiler } = require('../packages/binding');
const fs = require('fs');
const path = require('path');

console.log('Loaded compiler binding from packages/binding');

const compiler = new Compiler();
const srcDir = path.resolve(__dirname, '../demo-app/src');
const outDir = path.resolve(__dirname, '../dist-binding-batch');

if (fs.existsSync(outDir)) {
    fs.rmSync(outDir, { recursive: true, force: true });
}
fs.mkdirSync(outDir, { recursive: true });

// 1. Collect all files in memory
const filesToCompile = [];

function collectFiles(dir) {
    const items = fs.readdirSync(dir);
    for (const item of items) {
        const fullPath = path.join(dir, item);
        const stat = fs.statSync(fullPath);
        if (stat.isDirectory()) {
            collectFiles(fullPath);
        } else if (item.endsWith('.ts') && !item.endsWith('.d.ts')) {
            const content = fs.readFileSync(fullPath, 'utf8');
            // Binding expects 'filename' and 'content' for FileEntry
            filesToCompile.push({ 
                filename: fullPath,
                content: content
            });
        }
    }
}

console.log(`Reading files from ${srcDir}...`);
collectFiles(srcDir);
console.log(`Found ${filesToCompile.length} files.`);

if (typeof compiler.compileBatch !== 'function') {
    console.error('Error: compiler.compileBatch is not a function. binding interface update might not have propagated.');
    console.log('Available methods:', Object.getOwnPropertyNames(Object.getPrototypeOf(compiler)));
    process.exit(1);
}

// 2. Measure Batch Compilation Time
console.log('Starting Batch Compilation...');
const start = Date.now();

let results;
try {
    results = compiler.compileBatch(filesToCompile);
} catch (e) {
    console.error('Batch compilation failed:', e);
    process.exit(1);
}

const end = Date.now();
const duration = end - start;
const avg = (duration / filesToCompile.length).toFixed(2);

console.log(`\nBatch compilation time: ${duration}ms`);
console.log(`Average per file: ${avg}ms`);
console.log(`Total Results: ${results.length}`);

// 3. Write Outputs
console.log('Writing outputs...');
for (const entry of results) {
    // entry.filename is absolute path
    const relativePath = path.relative(srcDir, entry.filename);
    const outPath = path.join(outDir, relativePath.replace('.ts', '.js'));
    const outDirPath = path.dirname(outPath);
    
    if (!fs.existsSync(outDirPath)) {
        fs.mkdirSync(outDirPath, { recursive: true });
    }
    
    if (entry.code) {
        fs.writeFileSync(outPath, entry.code);
    }
    
    if (entry.diagnostics && entry.diagnostics.length > 0) {
        console.log(`[Diagnostic] ${relativePath}:`);
        entry.diagnostics.forEach(d => console.log(`  - ${d.message}`));
    }
}

console.log(`Output written to: ${outDir}`);
