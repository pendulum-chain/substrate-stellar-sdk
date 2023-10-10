import { snakeCase } from "snake-case";
import {
  determineDependencies,
  determineFullyQualifiedTypeReference,
  determineTypeReference,
  ReferableXdrType,
  StructType,
} from "../types/types";

export type StructDefinition = Array<[string, ReferableXdrType]>;

export function processStruct(name: string, structDefinition: StructDefinition): StructType {
  const subTypes: string[] = [];
  const subReaders: string[] = [];
  const subWriters: string[] = [];
  let dependencies: Record<string, true> = {};

  let isFirstProperty = true;
  structDefinition.forEach((entry) => {
    const [key, type] = entry;
    let stringifiedKey = snakeCase(key);
    if (stringifiedKey === "type") stringifiedKey += "_";
    const isOptionalCycle =
      type.type === "option" && type.innerType.type === "reference" && type.innerType.name === name;

    const mustBeBoxed = name.startsWith("ScSpecType");

    const typeReference = isOptionalCycle ? `Option<Box<${name}>>` : determineTypeReference(type);
    const fullyQualifiedTypeReference = isOptionalCycle
      ? `Option::<Box<${name}>>`
      : determineFullyQualifiedTypeReference(type);

    //here, if subType is cyclical insert a box. How do we realize??
    //introduce Box struct references a non-primitive type that is not wrapped into a Vec
    if (mustBeBoxed && isFirstProperty) {
      subTypes.push(`    pub ${stringifiedKey}: Box<${typeReference}>`);
    } else {
      subTypes.push(`    pub ${stringifiedKey}: ${typeReference}`);
    }

    subWriters.push(`        self.${stringifiedKey}.to_xdr_buffered(write_stream);`);

    if (mustBeBoxed && isFirstProperty) {
      subReaders.push(`            ${stringifiedKey}: Box::new(${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?),`);
      isFirstProperty = false;
    } else {
      subReaders.push(`            ${stringifiedKey}: ${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?,`);
    }

    dependencies = { ...dependencies, ...determineDependencies(type) };
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
