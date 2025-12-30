const { Compiler } = require('../packages/binding');
const fs = require('fs');
const path = require('path');

// Ensure binding is loaded
console.log('Loaded compiler binding form packages/binding');

const compiler = new Compiler();
const start = Date.now();
let count = 0;

const rootDir = path.resolve(__dirname, '..');
const srcDir = path.join(rootDir, 'demo-app/src');
const outDir = path.join(rootDir, 'dist-binding'); // Output separate from normal build

if (fs.existsSync(outDir)) {
    fs.rmSync(outDir, { recursive: true, force: true });
}
fs.mkdirSync(outDir, { recursive: true });

// Buffer all files first to exclude Read I/O from timing
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
            const relativePath = path.relative(srcDir, fullPath);
            filesToCompile.push({ fullPath, content, relativePath });
        }
    }
}

console.log(`Reading files from ${srcDir}...`);
collectFiles(srcDir);
console.log(`Found ${filesToCompile.length} files.`);

// Measure Pure Compilation Time
console.log('Starting pure compilation...');
const compileStart = Date.now();
const results = [];

for (const file of filesToCompile) {
    try {
        // Only measure the Rust compile call
        const result = compiler.compile(file.fullPath, file.content);
        results.push({ file, result });
        count++;
    } catch (err) {
        console.error(`Failed to compile ${file.relativePath}:`, err);
    }
}

const compileEnd = Date.now();
const duration = compileEnd - compileStart;
const avg = (duration / count).toFixed(2);

console.log(`\nPure compilation time: ${duration}ms`);
console.log(`Average per file: ${avg}ms`);

// Write Phase
console.log('Writing outputs...');
for (const { file, result } of results) {
    const outPath = path.join(outDir, file.relativePath.replace('.ts', '.js'));
    const outDirPath = path.dirname(outPath);
    
    if (!fs.existsSync(outDirPath)) {
        fs.mkdirSync(outDirPath, { recursive: true });
    }
    
    fs.writeFileSync(outPath, result.code);
    
    if (result.diagnostics.length > 0) {
        console.log(`[Diagnostic] ${file.relativePath}:`);
        result.diagnostics.forEach(d => console.log(`  - ${d.message}`));
    }
}

console.log(`Output written to: ${outDir}`);
