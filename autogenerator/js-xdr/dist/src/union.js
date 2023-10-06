"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.processUnion = void 0;
const pascal_case_1 = require("pascal-case");
const types_1 = require("../types/types");
function processUnion(name, unionDefinition, resolvedSwitchType) {
    const subTypes = [];
    const subReaders = [];
    const subWriters = [];
    let defaultReader;
    unionDefinition.switches.forEach((switchSpec) => {
        const armOrVoid = switchSpec[1];
        const switchValue = switchSpec[0];
        const caseIdentifier = typeof switchValue === "string" ? (0, pascal_case_1.pascalCase)(switchValue) : `V${switchValue}`;
        const fieldName = typeof switchValue !== "string"
            ? `(${switchValue} as ${(0, types_1.determineTypeReference)(unionDefinition.switchOn)})`
            : `${(0, types_1.determineTypeReference)(unionDefinition.switchOn)}::${caseIdentifier}`;
        const simpleFieldName = typeof switchValue !== "string"
            ? `${switchValue}`
            : `${(0, types_1.determineTypeReference)(unionDefinition.switchOn)}::${caseIdentifier}`;
        if (typeof armOrVoid === "string") {
            const type = unionDefinition.arms[armOrVoid];
            if (type === undefined) {
                throw new Error(`Union definition "${name}" has a switch "${caseIdentifier}" without an arm definition`);
            }
            const isOptionalCycle = type.type === "option" && type.innerType.type === "reference" && type.innerType.name === name;
            const typeReference = isOptionalCycle ? `Option<Box<${name}>>` : (0, types_1.determineTypeReference)(type);
            const fullyQualifiedTypeReference = isOptionalCycle
                ? `Option::<Box<${name}>>`
                : (0, types_1.determineFullyQualifiedTypeReference)(type);
            const mustBeBoxed = fullyQualifiedTypeReference.startsWith("ScSpecType");
            if (mustBeBoxed) {
                subTypes.push(`    ${caseIdentifier}(Box<${typeReference}>)`);
            }
            else {
                subTypes.push(`    ${caseIdentifier}(${typeReference})`);
            }
            subWriters.push(`            ${name}::${caseIdentifier}(value) => {${fieldName}.to_xdr_buffered(write_stream); value.to_xdr_buffered(write_stream)},`);
            if (mustBeBoxed) {
                subReaders.push(`            ${simpleFieldName} => Ok(${name}::${caseIdentifier}(Box::new(${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?))),`);
            }
            else {
                subReaders.push(`            ${simpleFieldName} => Ok(${name}::${caseIdentifier}(${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?)),`);
            }
        }
        else {
            subTypes.push(`    ${caseIdentifier}`);
            subWriters.push(`            ${name}::${caseIdentifier} => ${fieldName}.to_xdr_buffered(write_stream),`);
            subReaders.push(`            ${simpleFieldName} => Ok(${name}::${caseIdentifier}),`);
        }
    });
    const patternNotExaustive = resolvedSwitchType.type === "uint" ||
        resolvedSwitchType.type === "int" ||
        (resolvedSwitchType.type === "bool" && unionDefinition.switches.length < 2) ||
        (resolvedSwitchType.type === "enum" && unionDefinition.switches.length < resolvedSwitchType.noOfCases);
    if (unionDefinition.defaultArm || patternNotExaustive) {
        subTypes.push(`    Default(${(0, types_1.determineTypeReference)(unionDefinition.switchOn)})`);
        subWriters.push(`            ${name}::Default(code) => code.to_xdr_buffered(write_stream),`);
        defaultReader = `${name}::Default(code)`;
    }
    let dependencies = {};
    Object.values(unionDefinition.arms).forEach((arm) => {
        dependencies = Object.assign(Object.assign({}, dependencies), (0, types_1.determineDependencies)(arm));
    }, {});
    dependencies = Object.assign(Object.assign({}, dependencies), (0, types_1.determineDependencies)(unionDefinition.switchOn));
    const typeDefinition = `pub enum ${name} {\n${subTypes.join(",\n")}\n}`;
    const typeImplementation = `
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        match self {
${subWriters.join("\n")}
        }
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        match ${(0, types_1.determineTypeReference)(unionDefinition.switchOn)}::from_xdr_buffered(read_stream)? {
${subReaders.join("\n")}${defaultReader ? `\n            code => Ok(${defaultReader}),` : ""}
        }
    }`;
    return { type: "union", typeDefinition, typeImplementation, referredTypes: dependencies };
}
exports.processUnion = processUnion;
//# sourceMappingURL=union.js.map