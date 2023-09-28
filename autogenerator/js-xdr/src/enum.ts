import { pascalCase } from "change-case";
import { EnumType } from "../types/types";

export type EnumDefinition = Record<string, number>;

export function processEnum(name: string, enumDefinition: EnumDefinition): EnumType {
  const subTypes: string[] = [];
  const subReaders: string[] = [];

  Object.entries(enumDefinition).forEach(([key, constant]) => {
    const stringifiedKey = `    ${pascalCase(key)} = ${constant}`;
    subTypes.push(stringifiedKey);
    subReaders.push(`            ${constant} => Ok(${name}::${pascalCase(key)}),`);
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
