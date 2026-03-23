try {
  module.exports = require('./titan-evm-core.android-arm64.node');
} catch (e) {
  console.error("Titan EVM binary missing or incompatible architecture.", e);
  process.exit(1);
}
