{
  stdenv,
  fetchurl,
}:
let
  arch = stdenv.hostPlatform.rust.rustcTarget;
  fetch_librusty_v8 =
    args:
    fetchurl {
      name = "librusty_v8-${args.version}";
      url = "https://github.com/denoland/rusty_v8/releases/download/v${args.version}/librusty_v8_release_${arch}.a.gz";
      sha256 = args.shas.${stdenv.hostPlatform.system};
      meta = { inherit (args) version; };
    };
in
fetch_librusty_v8 {
  version = "130.0.2";
  shas = {
    x86_64-linux = "sha256-ew2WZhdsHfffRQtif076AWAlFohwPo/RbmW/6D3LzkU=";
    aarch64-linux = "sha256-p9+tHmKIM5wBABubHIAstpwfzO19ypPzOuaV4b6loCU=";
    x86_64-darwin = "sha256-zNC0DAkMbbFM1M+t6rgKtN0QAm4ONEbCi6Sxivhf8dk=";
    aarch64-darwin = "sha256-aWZ/4Q4Wttx37xOdBmTCPGP+eYGhr4CM1UkYq8pC7Qs=";
  };
}
