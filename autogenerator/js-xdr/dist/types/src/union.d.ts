import { BoolType, IntType, ReferableXdrType, ReferenceType, UIntType, UnionType, VoidType, XdrType } from "../types/types";
export type UnionDefinition = {
    switchOn: IntType | UIntType | BoolType | ReferenceType;
    switchName: string;
    switches: Array<[number | string, string | VoidType]>;
    arms: Record<string, ReferableXdrType>;
    defaultArm?: VoidType;
};
export declare function processUnion(name: string, unionDefinition: UnionDefinition, resolvedSwitchType: XdrType): UnionType;
