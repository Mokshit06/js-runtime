declare const Runtime: {
    readFileSync(path: string): string,
    cwd(): string,
    writeTextFileSync(path: string, contents: string): void,
}

console.log(`Does this work? ${1}`);

console.log(Runtime.readFileSync('./Cargo.toml'));
console.log(Runtime.cwd());
Runtime.writeTextFileSync('./f.txt', 'Hello');
