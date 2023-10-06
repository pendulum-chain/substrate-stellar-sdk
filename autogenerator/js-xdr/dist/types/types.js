"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.determineDependencies = exports.determineFullyQualifiedTypeReference = exports.determineTypeReference = void 0;
function lengthToString(length) {
    if (typeof length === "number")
        return String(length);
    return length.name;
}
function determineTypeReference(type) {
    switch (type.type) {
        case "uhyper":
            return "u64";
        case "hyper":
            return "i64";
        case "uint":
            return "u32";
        case "int":
            return "i32";
        case "limitedVarArray":
            return `LimitedVarArray<${determineTypeReference(type.innerType)}, ${lengthToString(type.maxLength)}>`;
        case "unlimitedVarArray":
            return `UnlimitedVarArray<${determineTypeReference(type.innerType)}>`;
        case "array":
            return `[${determineTypeReference(type.innerType)}; ${lengthToString(type.length)}]`;
        case "limitedVarOpaque":
            return `LimitedVarOpaque<${lengthToString(type.maxLength)}>`;
        case "unlimitedVarOpaque":
            return `UnlimitedVarOpaque`;
        case "opaque":
            return `[u8; ${lengthToString(type.length)}]`;
        case "limitedString":
            return `LimitedString<${lengthToString(type.maxLength)}>`;
        case "unlimitedString":
            return `UnlimitedString`;
        case "void":
            throw new Error("No need to reference void");
        case "bool":
            return "bool";
        case "option":
            return `Option<${determineTypeReference(type.innerType)}>`;
        case "reference":
            return type.name;
    }
}
exports.determineTypeReference = determineTypeReference;
function determineFullyQualifiedTypeReference(type) {
    switch (type.type) {
        case "uhyper":
            return "u64";
        case "hyper":
            return "i64";
        case "uint":
            return "u32";
        case "int":
            return "i32";
        case "limitedVarArray":
            return `LimitedVarArray::<${determineFullyQualifiedTypeReference(type.innerType)}, ${lengthToString(type.maxLength)}>`;
        case "unlimitedVarArray":
            return `UnlimitedVarArray::<${determineFullyQualifiedTypeReference(type.innerType)}>`;
        case "array":
            return `<[${determineFullyQualifiedTypeReference(type.innerType)}; ${lengthToString(type.length)}]>`;
        case "limitedVarOpaque":
            return `LimitedVarOpaque::<${lengthToString(type.maxLength)}>`;
        case "unlimitedVarOpaque":
            return `UnlimitedVarOpaque`;
        case "opaque":
            return `<[u8; ${lengthToString(type.length)}]>`;
        case "limitedString":
            return `LimitedString::<${lengthToString(type.maxLength)}>`;
        case "unlimitedString":
            return `UnlimitedString`;
        case "void":
            throw new Error("No need to reference void");
        case "bool":
            return "bool";
        case "option":
            return `Option::<${determineFullyQualifiedTypeReference(type.innerType)}>`;
        case "reference":
            return type.name;
    }
}
exports.determineFullyQualifiedTypeReference = determineFullyQualifiedTypeReference;
function determineDependencies(type) {
    switch (type.type) {
        case "uhyper":
        case "hyper":
        case "uint":
        case "int":
        case "limitedVarOpaque":
        case "unlimitedVarOpaque":
        case "opaque":
        case "limitedString":
        case "unlimitedString":
        case "void":
        case "bool":
        case "enum":
            return {};
        case "limitedVarArray":
        case "unlimitedVarArray":
        case "array":
        case "option":
            return determineDependencies(type.innerType);
        case "reference":
            return { [type.name]: true };
        case "struct":
        case "union":
            return type.referredTypes;
    }
}
exports.determineDependencies = determineDependencies;
//# sourceMappingURL=types.js.map