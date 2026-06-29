#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const LANGUAGES = (process.env.SDKWORK_CATALOG_SDK_LANGUAGES ?? "typescript")
  .split(",")
  .map((value) => value.trim())
  .filter(Boolean);

const PACKAGE_NAMES = {
  typescript: "@sdkwork/catalog-app-sdk",
};

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(scriptDir, "..");
const catalogRoot = path.resolve(sdkRoot, "../..");
const generatorBin = path.resolve(catalogRoot, "../sdkwork-sdk-generator/bin/sdkgen.js");
const defaultInput = path.resolve(sdkRoot, "openapi/sdkwork-catalog-app-api.openapi.json");
const authorityInput = path.resolve(
  catalogRoot,
  "apis/app-api/catalog/catalog-app-api.openapi.json",
);

run(process.argv.slice(2));

function run(argv) {
  const args = parseArgs(argv);
  syncAuthorityOpenApi(args.input ?? defaultInput);
  const input = args.input ? resolveCatalogPath(args.input) : defaultInput;
  ensureOpenApi(input);

  if (!existsSync(generatorBin)) {
    fail(`SDK generator not found: ${generatorBin}`);
  }

  for (const language of args.languages) {
    generateLanguage({
      language,
      input,
      baseUrl: args.baseUrl,
      sdkName: "sdkwork-catalog-app-sdk",
      sdkType: "app",
      apiPrefix: "/app/v3/api",
    });
  }
}

function syncAuthorityOpenApi(targetInput) {
  if (!existsSync(authorityInput)) {
    return;
  }
  const payload = readFileSync(authorityInput, "utf8");
  writeFileSync(targetInput, payload, "utf8");
  if (targetInput !== authorityInput) {
    writeFileSync(
      path.resolve(sdkRoot, "openapi/sdkwork-catalog-app-api.sdkgen.json"),
      payload,
      "utf8",
    );
  }
}

function generateLanguage({ language, input, baseUrl, sdkName, sdkType, apiPrefix }) {
  const outputPath = path.join(sdkRoot, `${sdkName}-${language}`, "generated", "server-openapi");
  mkdirSync(outputPath, { recursive: true });

  const commandArgs = [
    "generate",
    "--input",
    input,
    "--output",
    outputPath,
    "--name",
    sdkName,
    "--type",
    sdkType,
    "--language",
    language,
    "--base-url",
    baseUrl,
    "--api-prefix",
    apiPrefix,
    "--fixed-sdk-version",
    "0.1.0",
    "--sdk-root",
    sdkRoot,
    "--sdk-name",
    sdkName,
    "--package-name",
    PACKAGE_NAMES[language] ?? `${sdkName}-${language}`,
    "--standard-profile",
    "sdkwork-v3",
  ];

  const result = spawnSync("node", [generatorBin, ...commandArgs], {
    cwd: sdkRoot,
    stdio: "inherit",
  });
  if (result.status !== 0) {
    fail(`generator failed for ${language}`);
  }

  writeFileSync(
    path.join(outputPath, "source-openapi.json"),
    `${JSON.stringify(JSON.parse(readFileSync(input, "utf8")), null, 2)}\n`,
    "utf8",
  );
  console.log(`Generated ${sdkName} (${language})`);
}

function ensureOpenApi(input) {
  if (existsSync(input)) {
    return;
  }
  if (existsSync(authorityInput)) {
    syncAuthorityOpenApi(input);
    return;
  }
  fail(`OpenAPI input not found: ${input}`);
}

function parseArgs(argv) {
  const parsed = {
    input: null,
    languages: [...LANGUAGES],
    baseUrl: "http://localhost:8080",
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--input") {
      parsed.input = argv[index + 1] ?? "";
      index += 1;
      continue;
    }
    if (arg === "--languages") {
      parsed.languages = (argv[index + 1] ?? "")
        .split(",")
        .map((value) => value.trim())
        .filter(Boolean);
      index += 1;
      continue;
    }
    if (arg === "--base-url") {
      parsed.baseUrl = argv[index + 1] ?? parsed.baseUrl;
      index += 1;
    }
  }

  return parsed;
}

function resolveCatalogPath(value) {
  return path.isAbsolute(value) ? value : path.resolve(catalogRoot, value);
}

function fail(message) {
  console.error(message);
  process.exit(1);
}
