let

  fetchNixpkgs = {rev, sha256}: builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
    inherit sha256;
  };

  nixpkgs = fetchNixpkgs {
    rev = "7db611f2af869bac6e31ba814a5593c52d54ec19";
    sha256 = "0yp97ayg3bbi2bm2sgvjhrrmc73hqpv4cymm7gb49mmqjwg5fzws";
  };

  pkgs = import nixpkgs { };

in pkgs.mkShell rec {
  buildInputs = with pkgs; [
    pandoc
  ];
}
