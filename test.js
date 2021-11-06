console.log(`Does this work? ${1}`);

console.log(Runtime.readFileSync('./Cargo.toml'));
console.log(Runtime.cwd());
Runtime.writeTextFileSync('./f.txt', 'Hello');
