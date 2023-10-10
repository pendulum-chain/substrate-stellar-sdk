import { pascalCase } from "pascal-case";
import {
  BoolType,
  determineDependencies,
  determineFullyQualifiedTypeReference,
  determineTypeReference,
  IntType,
  ReferableXdrType,
  ReferenceType,
  UIntType,
  UnionType,
  VoidType,
  XdrType,
} from "../types/types";

export type UnionDefinition = {
  switchOn: IntType | UIntType | BoolType | ReferenceType;
  switchName: string;
  switches: Array<[number | string, string | VoidType]>;
  arms: Record<string, ReferableXdrType>;
  defaultArm?: VoidType;
};

export function processUnion(name: string, unionDefinition: UnionDefinition, resolvedSwitchType: XdrType): UnionType {
  const subTypes: string[] = [];
  const subReaders: string[] = [];
  const subWriters: string[] = [];
  let defaultReader: undefined | string;

  unionDefinition.switches.forEach((switchSpec) => {
    const armOrVoid = switchSpec[1];
    const switchValue = switchSpec[0];
    const caseIdentifier = typeof switchValue === "string" ? pascalCase(switchValue) : `V${switchValue}`;

    const fieldName =
      typeof switchValue !== "string"
        ? `(${switchValue} as ${determineTypeReference(unionDefinition.switchOn)})`
        : `${determineTypeReference(unionDefinition.switchOn)}::${caseIdentifier}`;

    const simpleFieldName =
      typeof switchValue !== "string"
        ? `${switchValue}`
        : `${determineTypeReference(unionDefinition.switchOn)}::${caseIdentifier}`;

    if (typeof armOrVoid === "string") {
      const type: ReferableXdrType = unionDefinition.arms[armOrVoid];
      if (type === undefined) {
        throw new Error(`Union definition "${name}" has a switch "${caseIdentifier}" without an arm definition`);
      }

      const isOptionalCycle =
        type.type === "option" && type.innerType.type === "reference" && type.innerType.name === name;

      const typeReference = isOptionalCycle ? `Option<Box<${name}>>` : determineTypeReference(type);
      const fullyQualifiedTypeReference = isOptionalCycle
        ? `Option::<Box<${name}>>`
        : determineFullyQualifiedTypeReference(type);

      const mustBeBoxed = fullyQualifiedTypeReference.startsWith("ScSpecType");
      if (mustBeBoxed) {
        subTypes.push(`    ${caseIdentifier}(Box<${typeReference}>)`);
      } else {
        subTypes.push(`    ${caseIdentifier}(${typeReference})`);
      }


      subWriters.push(
        `            ${name}::${caseIdentifier}(value) => {${fieldName}.to_xdr_buffered(write_stream); value.to_xdr_buffered(write_stream)},`
      );
      if (mustBeBoxed) {
        subReaders.push(
          `            ${simpleFieldName} => Ok(${name}::${caseIdentifier}(Box::new(${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?))),`
        );
      } else {
        subReaders.push(
          `            ${simpleFieldName} => Ok(${name}::${caseIdentifier}(${fullyQualifiedTypeReference}::from_xdr_buffered(read_stream)?)),`
        );
      }

    } else {
      subTypes.push(`    ${caseIdentifier}`);
      subWriters.push(`            ${name}::${caseIdentifier} => ${fieldName}.to_xdr_buffered(write_stream),`);
      subReaders.push(`            ${simpleFieldName} => Ok(${name}::${caseIdentifier}),`);
    }
  });

  const patternNotExaustive =
    resolvedSwitchType.type === "uint" ||
    resolvedSwitchType.type === "int" ||
    (resolvedSwitchType.type === "bool" && unionDefinition.switches.length < 2) ||
    (resolvedSwitchType.type === "enum" && unionDefinition.switches.length < resolvedSwitchType.noOfCases);

  if (unionDefinition.defaultArm || patternNotExaustive) {
    subTypes.push(`    Default(${determineTypeReference(unionDefinition.switchOn)})`);
    subWriters.push(`            ${name}::Default(code) => code.to_xdr_buffered(write_stream),`);
    defaultReader = `${name}::Default(code)`;
  }

  let dependencies: Record<string, true> = {};
  Object.values<ReferableXdrType>(unionDefinition.arms).forEach((arm) => {
    dependencies = { ...dependencies, ...determineDependencies(arm) };
  }, {});
  dependencies = { ...dependencies, ...determineDependencies(unionDefinition.switchOn) };

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
        match ${determineTypeReference(unionDefinition.switchOn)}::from_xdr_buffered(read_stream)? {
${subReaders.join("\n")}${defaultReader ? `\n            code => Ok(${defaultReader}),` : ""}
        }
    }`;

  return { type: "union", typeDefinition, typeImplementation, referredTypes: dependencies };
}
