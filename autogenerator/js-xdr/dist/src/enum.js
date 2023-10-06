"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.processEnum = void 0;
const change_case_1 = require("change-case");
function processEnum(name, enumDefinition) {
    const subTypes = [];
    const subReaders = [];
    Object.entries(enumDefinition).forEach(([key, constant]) => {
        const stringifiedKey = `    ${(0, change_case_1.pascalCase)(key)} = ${constant}`;
        subTypes.push(stringifiedKey);
        subReaders.push(`            ${constant} => Ok(${name}::${(0, change_case_1.pascalCase)(key)}),`);
    });
    const typeDefinition = `pub enum ${name} {\n${subTypes.join(",\n")}\n}`;
    const typeImplementation = `
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        let value = *self as i32;
        value.to_xdr_buffered(write_stream);
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        let enum_value = i32::from_xdr_buffered(read_stream)?;
        match enum_value {
${subReaders.join("\n")}
            _ => Err(DecodeError::InvalidEnumDiscriminator {at_position: read_stream.get_position()})
        }
    }`;
    return { type: "enum", typeDefinition, typeImplementation, noOfCases: Object.entries(enumDefinition).length };
}
exports.processEnum = processEnum;
//# sourceMappingURL=enum.js.map