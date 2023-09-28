import { DefinitionFactory, XdrType } from "../types/types";
import { processEnum } from "./enum";
import { processStruct } from "./struct";
import { processUnion, UnionDefinition } from "./union";
import { initializeOutputPath, generateXdrDefinition } from "./writer";

if (process.env.DESTINATION === undefined) {
  throw new Error(`The environment variable "DESTINATION" is not defined`);
}

const outputPath: string = process.env.DESTINATION;
initializeOutputPath(outputPath);

export function config(definitionFactory: DefinitionFactory) {
  const constants: Record<string, number> = {};
  const types: Record<string, XdrType> = {};

  const unions: Array<{ name: string; unionDefinition: UnionDefinition }> = [];

  definitionFactory({
    typedef: (name, type) => {
      types[name] = type;
    },

    enum: (name, enumDefinition) => {
      types[name] = processEnum(name, enumDefinition);
    },

    struct: (name, structDefinition) => {
      types[name] = processStruct(name, structDefinition);
    },

    union: (name, unionDefinition) => {
      // postpone construction of union type because we need to make sure
      // that all enums are already defined
      unions.push({ name, unionDefinition });
    },

    const: (name, value) => {
      constants[name] = value;
    },

    lookup: (name) => ({ type: "reference", name }),
    option: (innerType) => ({ type: "option", innerType }),

    varOpaque: (maxLength?) =>
      maxLength === undefined || maxLength === 0x7fffffff
        ? { type: "unlimitedVarOpaque" }
        : { type: "limitedVarOpaque", maxLength },

    string: (maxLength?) =>
      maxLength === undefined || maxLength === 0x7fffffff
        ? { type: "unlimitedString" }
        : { type: "limitedString", maxLength },

    varArray: (innerType, maxLength?) =>
      maxLength === undefined || maxLength === 0x7fffffff
        ? { type: "unlimitedVarArray", innerType }
        : { type: "limitedVarArray", maxLength, innerType },

    opaque: (length) => ({ type: "opaque", length }),
    array: (innerType, length) => ({ type: "array", length, innerType }),

    bool: () => ({ type: "bool" }),
    void: () => ({ type: "void" }),
    int: () => ({ type: "int" }),
    uint: () => ({ type: "uint" }),
    hyper: () => ({ type: "hyper" }),
    uhyper: () => ({ type: "uhyper" }),
  });

  unions.forEach(({ name, unionDefinition }) => {
    const resolvedSwitchType =
      unionDefinition.switchOn.type === "reference" ? types[unionDefinition.switchOn.name] : unionDefinition.switchOn;

    types[name] = processUnion(name, unionDefinition, resolvedSwitchType);
  });

  generateXdrDefinition(types, constants, outputPath);
}
