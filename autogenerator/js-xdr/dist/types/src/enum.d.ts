import { EnumType } from "../types/types";
export type EnumDefinition = Record<string, number>;
export declare function processEnum(name: string, enumDefinition: EnumDefinition): EnumType;
