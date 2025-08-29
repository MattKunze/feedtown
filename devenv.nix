{ pkgs, lib, config, inputs, ... }:

{
  languages.deno.enable = true;
  languages.rust.enable = true;

  packages = [
    pkgs.just
    pkgs.postgresql
  ];
}
