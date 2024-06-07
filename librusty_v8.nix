{
  rust,
  stdenv,
  fetchurl,
}: let
  arch = rust.toRustTarget stdenv.hostPlatform;
  fetch_librusty_v8 = args:
    fetchurl {
      name = "librusty_v8-${args.version}";
      url = "https://github.com/denoland/rusty_v8/releases/download/v${args.version}/librusty_v8_release_${arch}.a.gz";
      sha256 = args.shas.${stdenv.hostPlatform.system};
      meta = {inherit (args) version;};
    };
in
  fetch_librusty_v8 {
    version = "0.93.0";
    shas = {
      x86_64-linux = "sha256-kvME/WThZ76i7NNyTmZxLdWhpKOBVgosrt3/m8k/BMM=";
      aarch64-linux = "sha256-gUrgXun7Ei0UPUgv6mi1pQujVF6yjeirSWUiVsSjdpc=";
      x86_64-darwin = "sha256-fOgRFw6voxLvBDczW2XCpdkD2VKFAupyH9ENBVRBRzA=";
      aarch64-darwin = "sha256-GCEL+CkBTJTz6NTSsGnPo/HK5dzIjGhQkLC2dNpnqvk=";
    };
  }
