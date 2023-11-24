# Docs: https://devenv.sh/basics/
{ pkgs, ... }: {
  name = "tk test";

  # Extend main devenv:
  imports = [ ./devenv.nix ];

  languages = {
    # Docs: https://devenv.sh/languages/
    java.enable = true;
    javascript.enable = true;
    python.enable = true;
  };

  packages = with pkgs; [
    # Search for packages: https://search.nixos.org/packages?channel=unstable&query=cowsay
    # (note: this searches on unstable channel, be aware your nixpkgs flake input might be on a release channel)

    go-task
    gnumake
    deno
    sbt
    go-task
    cargo-make
    jbang

    #handled by `languages` above:
    # python3
    # nodejs
    # adoptopenjdk-jre-bin
  ];

  env = {
    NODE_HOME = "${pkgs.nodejs}";
    # JAVA_HOME = "${pkgs.adoptopenjdk-jre-bin.home}"; - seems to be handled by languages.java
    JBANG_HOME = "${pkgs.jbang}/bin";
  };
}
