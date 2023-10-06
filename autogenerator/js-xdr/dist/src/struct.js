"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.processStruct = void 0;
const snake_case_1 = require("snake-case");
const types_1 = require("../types/types");
function processStruct(name, structDefinition) {
    const subTypes = [];
    const subReaders = [];
    const subWriters = [];
    let dependencies = {};
    let isFirstProperty = true;
    structDefinition.forEach((entry) => {
        const [key, type] = entry;
        let stringifiedKey = (0, snake_case_1.snakeCase)(key);
        if (stringifiedKey === "type")
            stringifiedKey += "_";
        const isOptionalCycle = type.type === "option" && type.innerType.type === "reference" && type.innerType.name === name;
        const mustBeBoxed = name.startsWith("ScSpecType");
        const typeReference = isOptionalCycle ? `Option<Box<${name}>>` : (0, types_1.determineTypeReference)(type);
        const fullyQualifiedTypeReference = isOptionalCycle
            ? `Option::<Box<${name}>>`
            : (0, types_1.determineFullyQualifiedTypeReference)(type);
        //here, if subType is cyclical insert a box. How do we realize??
        //introduce Box struct references a non-primitive type that is not wrapped into a Vec
        if (mustBeBoxed && isFirstProperty) {
            subTypes.push(`    pub ${stringifiedKey}: Box<${typeReference}>`);
        }
        else {
            subTypes.push(`    pub ${stringifiedKey}: ${typeReference}`);
        }
        subWriters.push(`        self.${stringifiedKey}.to_xdr_buffered(write_stream);`);
        if (mustBeBoxed && isFirstProperty) {
            subReaders.push(`            ${stringifiedKey}: Box::new(${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?),`);
            isFirstProperty = false;
        }
        else {
            subReaders.push(`            ${stringifiedKey}: ${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?,`);
        }
        dependencies = Object.assign(Object.assign({}, dependencies), (0, types_1.determineDependencies)(type));
    });
    const typeDefinition = `pub struct ${name} {\n${subTypes.join(",\n")}\n}`;
    const typeImplementation = `
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
${subWriters.join("\n")}
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        Ok(${name} {
${subReaders.join("\n")}
        })
    }`;
    return { type: "struct", typeDefinition, typeImplementation, referredTypes: dependencies };
}
exports.processStruct = processStruct;
//# sourceMappingURL=struct.js.map