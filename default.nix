let

  fetchNixpkgs = {rev, sha256}: builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
    inherit sha256;
  };

  nixpkgs = fetchNixpkgs {
    # Tue Sep 4 22:16:31 2018 +0200
    rev = "fb89213cf650a4a847ee7481ae767a2f2ebdabf2";
    sha256 = "09k96aqflx40wwms3dngjkhrx0zppj6ffrzv3b0ssmj12ql69k9y";
  };

  pkgs = import nixpkgs { };

in {

  manpage = pkgs.stdenv.mkDerivation rec {
    name = "free-manpage";

    src = ./MANUAL.org;

    phases = [ "buildPhase" ];

    buildPhase = ''
      mkdir -p $out
      ${pkgs.pandoc}/bin/pandoc -s -o $out/free.1 $src
    '';
  };


  manifest = pkgs.stdenv.mkDerivation rec {
    name = "free-manifest";

    src = ./Cargo.toml;

    version = builtins.readFile (pkgs.runCommand "version" {} ''
      ${pkgs.perl}/bin/perl -ne 'print "$1" if /version\s*=\s*"(.+)"/' ${src} > $out
    '');

    manifest = pkgs.writeText name ''
      name: free
      version: ${version}
      comment: free for FreeBSD
      www: https://github.com/j-keck/free
      maintainer: jhypenkeck@gmail.com
      prefix: /usr/local
      origin: sysutils
      desc: |-
        Display amount of free and used memory in the system
      files: {
        /usr/local/bin/free: {uname: root, gname: wheel, perm: 0555},
        /usr/local/man/man1/free.1: {uname: root, gname: wheel, perm: 0444}
      }
    '';

    phases = [ "buildPhase" ];

    buildPhase = ''
      mkdir -p $out
      cp $manifest $out/manifest
    '';
  };
}
