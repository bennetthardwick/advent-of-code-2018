#!/usr/bin/env node

const fs = require('fs');
const input = fs.readFileSync('./input', { encoding: 'utf8' });

// Use function constructor because gotta go fast
console.log('Part one!');
console.log(new Function(`return ${input}`)() + '\n');

console.log('Part two!');

let seen = {};
let total = 0;
let frequency = '';
let commands = input.split('\n');

for (let i = 0; i < commands.length; i++) {
  total = eval(`${total} ${commands[i]}`);

  frequency = String(total);

  if (seen[frequency]) {
    break;
  } else {
    seen[frequency] = true;
  }

  if (i >= commands.length - 1) i = -1;

}

console.log(frequency);
