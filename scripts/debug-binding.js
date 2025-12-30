const { Compiler } = require('../packages/binding');
console.log('Compiler loaded');
console.log('Prototype methods:', Object.getOwnPropertyNames(Compiler.prototype));

try {
    const c = new Compiler();
    console.log('Instance created');
    console.log('compileBatch exists?', typeof c.compileBatch);
    console.log('compile_batch exists?', typeof c.compile_batch);
} catch (e) {
    console.error('Error creating instance:', e);
}
