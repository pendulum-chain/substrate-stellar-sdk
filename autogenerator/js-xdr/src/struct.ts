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

  structDefinition.forEach((entry) => {
    const [key, type] = entry;
    let stringifiedKey = snakeCase(key);
    if (stringifiedKey === "type") stringifiedKey += "_";
    const isOptionalCycle =
      type.type === "option" && type.innerType.type === "reference" && type.innerType.name === name;

    const typeReference = isOptionalCycle ? `Option<Box<${name}>>` : determineTypeReference(type);
    const fullyQualifiedTypeReference = isOptionalCycle
      ? `Option::<Box<${name}>>`
      : determineFullyQualifiedTypeReference(type);

    subTypes.push(`    pub ${stringifiedKey}: ${typeReference}`);
    subWriters.push(`        self.${stringifiedKey}.to_xdr_buffered(write_stream);`);
    subReaders.push(`            ${stringifiedKey}: ${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?,`);
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
