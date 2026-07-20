{ pkgs }:
pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.eva-toolchain.normal
    pkgs.zed-editor
  ];
}