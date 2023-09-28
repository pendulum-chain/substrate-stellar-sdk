import { constantCase } from "change-case";

import { writeFileSync, mkdirSync } from "fs";
import { join } from "path";

import { determineDependencies, determineTypeReference, XdrType } from "../types/types";

export function initializeOutputPath(outputPath: string) {
  mkdirSync(outputPath, { recursive: true });
}

const ROOT_MAIN_TYPES =
  "TransactionEnvelope, TransactionResult, TransactionMeta, TrustLineFlags, EnvelopeType, TransactionSignaturePayload, Curve25519Secret";

function determineMainTypes(types: Record<string, XdrType>) {
  const remaining = ROOT_MAIN_TYPES.split(",").map((name) => name.trim());
  const mainTypes = new Set<string>();

  while (true) {
    const typeName = remaining.pop();
    if (typeName === undefined) return mainTypes;
    mainTypes.add(typeName);

    Object.keys(determineDependencies(types[typeName])).forEach((key) => {
      if (!mainTypes.has(key) && remaining.indexOf(key) === -1) {
        remaining.push(key);
      }
    });
  }

  return mainTypes;
}

export function generateXdrDefinition(
  types: Record<string, XdrType>,
  constants: Record<string, number>,
  outputPath: string
) {
  const mainTypes = determineMainTypes(types);

  let result =
    `//! Autogenerated XDR types\n//!\n` +
    `// This code has been automatically generated on ${new Date().toISOString().slice(0, 10)}\n` +
    `// using the code in folder \`/autogenerator\`\n` +
    "// Do not edit this file manually!\n\n" +
    "#[allow(unused_imports)]\nuse sp_std::{prelude::*, boxed::Box};\n#[allow(unused_imports)]\nuse core::convert::AsRef;\n#[allow(unused_imports)]\nuse super::xdr_codec::XdrCodec;\n";
  result += "#[allow(unused_imports)]\nuse super::streams::{ReadStream, DecodeError, WriteStream};\n";
  result +=
    "#[allow(unused_imports)]\nuse super::compound_types::{LimitedVarOpaque, LimitedString, LimitedVarArray, UnlimitedVarOpaque, UnlimitedString, UnlimitedVarArray};\n\n";

  result +=
    Object.entries(constants)
      .map(([constant, value]) => {
        const comment = `/// Autogenerated definition for constant ${constant}\n`;
        return `${comment}#[allow(dead_code)]\npub const ${constantCase(constant)}: i32 = ${value};\n`;
      })
      .join("") + "\n";

  Object.keys(types).forEach((typeName) => {
    const typeDefinition = types[typeName];

    const comment = `/// Autogenerated definition for type ${typeName}\n`;
    const typePrefix = `${mainTypes.has(typeName) ? "" : '#[cfg(feature = "all-types")]\n'}`;
    if (typeDefinition.type !== "enum" && typeDefinition.type !== "struct" && typeDefinition.type !== "union") {
      result += `${comment}#[allow(dead_code)]\n${typePrefix}pub type ${typeName} = ${determineTypeReference(
        typeDefinition
      )};\n\n`;
    } else {
      const derive =
        typeDefinition.type === "enum" ? "Debug, Copy, Clone, Eq, PartialEq" : "Debug, Clone, Eq, PartialEq";
      result += `${comment}#[allow(dead_code)]\n${typePrefix}#[derive(${derive})]\n${typeDefinition.typeDefinition}\n\n`;
      result += `${typePrefix}impl XdrCodec for ${typeName} {${typeDefinition.typeImplementation}\n}\n\n`;
    }
  });

  const mainFileName = process.env.MAIN_FILE_NAME;
  if (!mainFileName) {
    throw new Error('Environment variable "MAIN_FILE_NAME" not specified');
  }

  writeFileSync(join(outputPath, mainFileName), result);
}