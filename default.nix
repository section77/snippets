let

  # 2019-06-13T19:02:54+02:00
  nixpkgs-mozilla = (import "${
    (import <nixpkgs> { }).fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      rev = "200cf0640fd8fdff0e1a342db98c9e31e6f13cd7";
      sha256 = "1am353ims43ylvay263alchzy3y87r1khnwr0x2fp35qr347bvxi";
    }
  }/rust-overlay.nix");

  pkgs = import <nixpkgs> { overlays = [nixpkgs-mozilla]; };

  rust = (pkgs.rustChannelOf {
    channel = "nightly";
    date = "2019-07-08";
  });

  nightlyRustPlatform = pkgs.makeRustPlatform { inherit (rust) rustc cargo; };

in nightlyRustPlatform.buildRustPackage rec {
  name = "snippets";

  src = pkgs.nix-gitignore.gitignoreSource [] ./.;

  cargoSha256 = "sha256:1ng54jg08pb13w25xk7h4jn9jg6mvmmzl1z2wjc3wka66r5dq5hs";

  # https://github.com/NixOS/nixpkgs/issues/61618
  preConfigure = ''
    export HOME=`mktemp -d`
  '';

  # fix: 'can't find crate for `std`'
  RUSTFLAGS =
  "-L ${rust.rust-std}/lib/rustlib/${pkgs.stdenv.targetPlatform.config}/lib/";

  doCheck = false;
}
