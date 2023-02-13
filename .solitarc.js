const path = require("path");
const programDir = path.join(__dirname, "programs/cardinal-creator-standard");
const idlDir = path.join(__dirname, "sdk/idl");
const sdkDir = path.join(__dirname, "sdk", "generated");
const binaryInstallDir = path.join(__dirname, "..", "..", "target", "solita");

module.exports = {
  idlGenerator: "shank",
  programName: "cardinal_creator_standard",
  programId: "ccsxqYAg64wuLEh45KabyPvkKjrEfDPsDZUQrGn7mf3",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
