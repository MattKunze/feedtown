{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;

  packages = [
    pkgs.just
    pkgs.postgresql
  ];
}
