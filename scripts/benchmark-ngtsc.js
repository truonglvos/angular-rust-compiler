const { execSync } = require('child_process');
const path = require('path');

const demoAppDir = path.join(__dirname, '../demo-app');

console.log('Starting NGTSC Benchmarking...');
console.log(`Demo App Directory: ${demoAppDir}`);

const start = process.hrtime.bigint();

try {
  const ngcPath = path.resolve(demoAppDir, 'node_modules/.bin/ngc');
  execSync(`${ngcPath} -p tsconfig.app.json`, {
    cwd: demoAppDir,
    stdio: 'inherit'
  });
} catch (error) {
  console.error('NGTSC compilation failed:', error);
  process.exit(1);
}

const end = process.hrtime.bigint();
const durationNs = end - start;
const durationMs = Number(durationNs) / 1000000;

console.log(`NGTSC Compilation Time: ${durationMs.toFixed(2)}ms`);
