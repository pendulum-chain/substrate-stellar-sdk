import { XdrType } from "../types/types";
export declare function initializeOutputPath(outputPath: string): void;
export declare function generateXdrDefinition(types: Record<string, XdrType>, constants: Record<string, number>, outputPath: string): void;
