const fs = require('fs');

console.log(`Does this work? ${1}`);

console.log(fs.readFileSync('./Cargo.toml', 'utf8'));
console.log(process.cwd());
fs.writeFileSync('./f.txt', 'Hello');
