"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.config = void 0;
const enum_1 = require("./enum");
const struct_1 = require("./struct");
const union_1 = require("./union");
const writer_1 = require("./writer");
if (process.env.DESTINATION === undefined) {
    throw new Error(`The environment variable "DESTINATION" is not defined`);
}
const outputPath = process.env.DESTINATION;
(0, writer_1.initializeOutputPath)(outputPath);
function config(definitionFactory) {
    const constants = {};
    const types = {};
    const unions = [];
    definitionFactory({
        typedef: (name, type) => {
            types[name] = type;
        },
        enum: (name, enumDefinition) => {
            types[name] = (0, enum_1.processEnum)(name, enumDefinition);
        },
        struct: (name, structDefinition) => {
            types[name] = (0, struct_1.processStruct)(name, structDefinition);
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
        varOpaque: (maxLength) => maxLength === undefined || maxLength === 0x7fffffff
            ? { type: "unlimitedVarOpaque" }
            : { type: "limitedVarOpaque", maxLength },
        string: (maxLength) => maxLength === undefined || maxLength === 0x7fffffff
            ? { type: "unlimitedString" }
            : { type: "limitedString", maxLength },
        varArray: (innerType, maxLength) => maxLength === undefined || maxLength === 0x7fffffff
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
        const resolvedSwitchType = unionDefinition.switchOn.type === "reference" ? types[unionDefinition.switchOn.name] : unionDefinition.switchOn;
        types[name] = (0, union_1.processUnion)(name, unionDefinition, resolvedSwitchType);
    });
    (0, writer_1.generateXdrDefinition)(types, constants, outputPath);
}
exports.config = config;
//# sourceMappingURL=index.js.map