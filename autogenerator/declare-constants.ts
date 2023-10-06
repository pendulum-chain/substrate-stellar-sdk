//this scripts looks for the declared constants in the generated stellar-xdr_generated.ts file
//it will declare them at the top of the file for it to be executed properly

import fs from 'fs';
import path from 'path';

const filePath = path.join(__dirname, 'x2JavaScript/generated/stellar-xdr_generated.ts');
let content = fs.readFileSync(filePath, 'utf8');


const beginFlag = '// begin constants';
const endFlag = '// end constants';

//we look for the section of the code that defines the constants
//we will replace this section if the constants have already been defined to 
//avoid multiple definition
const constantsRegex = new RegExp(`${beginFlag}[\\s\\S]*${endFlag}\n?`, 'm');
content = content.replace(constantsRegex, '');

//we find every declared contsant of the form `xdr.const("CONS_NAME", VALUE);`
const regex = /xdr\.const\("([A-Za-z0-9_]+)",\s*(\d+|"[^"]*")\);/g;
let match;
let constDefinitions = `${beginFlag}\n`;

// for each find, we declare it.
while ((match = regex.exec(content)) !== null) {
    const [fullMatch, constName, constValue] = match;
    constDefinitions += `const ${constName} = ${constValue};\n`;
}
constDefinitions += `${endFlag}\n`;

const newContent = constDefinitions + content;

fs.writeFileSync(filePath, newContent);