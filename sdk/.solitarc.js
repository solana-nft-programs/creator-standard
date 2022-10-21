const path = require("path");
const programDir = path.join(__dirname, "..", "program");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, "..", "..", "target", "solita");

module.exports = {
  idlGenerator: "anchor",
  programName: "cardinal_creator_standard",
  programId: "creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
