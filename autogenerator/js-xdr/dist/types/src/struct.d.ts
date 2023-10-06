import { ReferableXdrType, StructType } from "../types/types";
export type StructDefinition = Array<[string, ReferableXdrType]>;
export declare function processStruct(name: string, structDefinition: StructDefinition): StructType;
