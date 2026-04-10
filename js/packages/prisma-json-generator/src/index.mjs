import fs from "node:fs/promises";
import path from "node:path";

import pkg from "@prisma/generator-helper";
const { generatorHandler } = pkg;

const onGenerate = async ({ generator, dmmf }) => {
  const { output } = generator;
  if (!output) {
    throw new Error("Output is required");
  }
  const outputPath = output.fromEnvVar
    ? process.env[output.fromEnvVar]
    : output.value;
  if (!outputPath) {
    throw new Error("Output path is required");
  }
  if (!outputPath.endsWith(".json")) {
    throw new Error("Output path must end with `.json`");
  }

  const { datamodel } = dmmf;

  await fs.mkdir(path.dirname(outputPath), { recursive: true });
  await fs.writeFile(outputPath, JSON.stringify(datamodel, null, 2));
};

const main = () => {
  generatorHandler({ onGenerate });
};

export default main;
