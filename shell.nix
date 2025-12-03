{ pkgs ? import <nixpkgs> {} }:

let
  multimarkdown7 = pkgs.stdenv.mkDerivation rec {
    pname = "multimarkdown";
    version = "7.0.0-alpha.2";

    src = pkgs.fetchFromGitHub {
      owner = "fletcher";
      repo = "MultiMarkdown-7";
      rev = "12d7bd9db288181810a78c121ed7a30130b76c0a"; # develop branch (HEAD)
      sha256 = "sha256-+KJSkyGZ4w6GnwfGCPBp8aNfp+vqZHggvUSZyQAOm0A=";
    };

    nativeBuildInputs = [ pkgs.cmake ];

    # Don't use the Makefile, use CMake directly
    dontUseCmakeConfigure = false;

    cmakeFlags = [
      "-DCMAKE_BUILD_TYPE=Release"
    ];

    # The default install target doesn't install the multimarkdown executable
    # so we need to do it manually
    installPhase = ''
      runHook preInstall

      mkdir -p $out/bin
      # Find and copy the multimarkdown executable
      find . -name multimarkdown -type f -executable -exec cp {} $out/bin/ \;

      runHook postInstall
    '';

    meta = with pkgs.lib; {
      description = "MultiMarkdown version 7";
      homepage = "https://github.com/fletcher/MultiMarkdown-7";
      license = licenses.mit;
      platforms = platforms.unix;
    };
  };
in
pkgs.mkShell {
  buildInputs = [
    multimarkdown7      # Convert markdown to HTML (version 7)
  ] ++ (with pkgs; [
    static-web-server   # Static webserver for testing
    wrangler            # Cloudflare CLI

    # Rust
    rustc
    cargo
    clippy
    rustfmt
    pkg-config
    rust-script

    # Fast linker for improved build times
    lld_21
    clang
  ]);

  shellHook = ''
    echo "Development environment activated"
  '';
}

