{ pkgs }:
let
  baseVersion = "2025-12-27";
  baseToolchain = pkgs.fenix.toolchainOf {
    date = baseVersion;
    sha256 = "siHAq8pCOyiCtgQlfRjHwMkaWt8+VEoaztjozbHuFmA=";
  };
  baseCommit = "7b5cde7370b82fecf700071dda98c3fbc7d41fae";
  lastCommit = "b121bbe7d5274132e9b089a0c013a5cc2ff27f4d";
  patches = pkgs.fetchpatch {
    url = "https://github.com/Tazdevil971/rust/compare/${baseCommit}..${lastCommit}.diff";
    hash = "sha256-RB7R7XGCio/gelgfYpnmrtYMadfwPXipH86vGfs5hi4=";
  };
  eva-rust-src = pkgs.applyPatches {
    src = baseToolchain.rust-src;
    patches = [ patches ];
    patchFlags = [
      "-p1"
      "-d"
      "lib/rustlib/src/rust"
    ];
  };
in
{
  normal = pkgs.fenix.combine (
    with baseToolchain;
    [
      eva-rust-src
      rustc
      rustc-dev
      llvm-tools
      rust-std

      cargo
      clippy
      miri
      rustfmt
      rust-analyzer
    ]
  );

  minimal = pkgs.fenix.combine (
    with baseToolchain;
    [
      eva-rust-src
      rustc
      llvm-tools
      cargo
    ]
  );
}
