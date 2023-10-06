import { EnumDefinition } from "../src/enum";
import { StructDefinition } from "../src/struct";
import { UnionDefinition } from "../src/union";
export interface UHyperType {
    type: "uhyper";
}
export interface HyperType {
    type: "hyper";
}
export interface UIntType {
    type: "uint";
}
export interface IntType {
    type: "int";
}
export interface LimitedVarArrayType {
    type: "limitedVarArray";
    maxLength: number | ReferenceType;
    innerType: ReferableXdrType;
}
export interface UnlimitedVarArrayType {
    type: "unlimitedVarArray";
    innerType: ReferableXdrType;
}
export interface ArrayType {
    type: "array";
    length: number | ReferenceType;
    innerType: ReferableXdrType;
}
export interface LimitedVarOpaqueType {
    type: "limitedVarOpaque";
    maxLength: number | ReferenceType;
}
export interface UnlimitedVarOpaqueType {
    type: "unlimitedVarOpaque";
}
export interface OpaqueType {
    type: "opaque";
    length: number | ReferenceType;
}
export interface LimitedStringType {
    type: "limitedString";
    maxLength: number | ReferenceType;
}
export interface UnlimitedStringType {
    type: "unlimitedString";
}
export interface VoidType {
    type: "void";
}
export interface BoolType {
    type: "bool";
}
export interface OptionType {
    type: "option";
    innerType: ReferableXdrType;
}
export interface ReferenceType {
    type: "reference";
    name: string;
}
export interface EnumType {
    type: "enum";
    typeDefinition: string;
    typeImplementation: string;
    noOfCases: number;
}
export interface StructType {
    type: "struct";
    typeDefinition: string;
    typeImplementation: string;
    referredTypes: Record<string, true>;
}
export interface UnionType {
    type: "union";
    typeDefinition: string;
    typeImplementation: string;
    referredTypes: Record<string, true>;
}
export type ReferableXdrType = UHyperType | HyperType | UIntType | IntType | LimitedVarArrayType | UnlimitedVarArrayType | ArrayType | LimitedVarOpaqueType | UnlimitedVarOpaqueType | OpaqueType | LimitedStringType | UnlimitedStringType | VoidType | BoolType | OptionType | ReferenceType;
export type ComplexXdrType = EnumType | StructType | UnionType;
export type XdrType = ReferableXdrType | ComplexXdrType;
export declare function determineTypeReference(type: ReferableXdrType): string;
export declare function determineFullyQualifiedTypeReference(type: ReferableXdrType): string;
export declare function determineDependencies(type: XdrType): Record<string, true>;
export type XdrValue = number | string;
export type DefinitionFactory = (definition: {
    typedef: (name: string, type: XdrType) => void;
    enum: (name: string, enumDefinition: EnumDefinition) => void;
    struct: (name: string, structDefinition: StructDefinition) => void;
    union: (name: string, unionDefinition: UnionDefinition) => void;
    const: (name: string, value: number) => void;
    lookup: (name: string) => ReferenceType;
    option: (innerType: ReferableXdrType) => OptionType;
    opaque: (length: number | ReferenceType) => OpaqueType;
    varOpaque: (maxLength?: number | ReferenceType) => LimitedVarOpaqueType | UnlimitedVarOpaqueType;
    bool: () => BoolType;
    void: () => VoidType;
    string: (maxLength?: number | ReferenceType) => LimitedStringType | UnlimitedStringType;
    array: (innerType: ReferableXdrType, length: number | ReferenceType) => ArrayType;
    varArray: (innerType: ReferableXdrType, maxLength?: number | ReferenceType) => UnlimitedVarArrayType | LimitedVarArrayType;
    int: () => IntType;
    uint: () => UIntType;
    hyper: () => HyperType;
    uhyper: () => UHyperType;
}) => void;
