//this script will read any definition of the form xdr.const("CONST_NAME", VALUE), 
//and define it on top of the .ts file for other types to use.

import fs from 'fs';
import path from 'path';


const filePath = path.join(__dirname, 'x2JavaScript/generated/stellar-xdr_generated.ts');
const content = fs.readFileSync(filePath, 'utf8');

const regex = /xdr\.const\("([A-Za-z0-9_]+)",\s*(\d+|"[^"]*")\);/g;
let match;
let constDefinitions = '';

while ((match = regex.exec(content)) !== null) {
    const [fullMatch, constName, constValue] = match;
    constDefinitions += `const ${constName} = ${constValue};\n`;
}

const newContent = constDefinitions + `\n` + content;


const newFilePath = path.join(__dirname, 'x2JavaScript/generated/stellar-xdr_generated.ts');
fs.writeFileSync(newFilePath, newContent);

