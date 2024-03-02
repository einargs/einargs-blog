{
  description = "Blog built with Rust as the frontend";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
  };
  nixConfig = {
    bash-prompt = ''\[\033[1;32m\][\[\e]0;\u@\h: \w\a\]dev-shell:\w]\$\[\033[0m\] '';
  };

  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      /*tailwindcss = pkgs.nodePackages.tailwindcss;
      tailwind-watch = pkgs.writeShellApplication {
        name = "tailwind-watch";

        runtimeInputs = [ tailwindcss ];
        text = ''
          tailwindcss -i ./input.css -o ./style/output.css --watch
        '';
      };*/
  in {

    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      buildInputs = with pkgs; [
        openssl
        openssl.dev
        rustup
        nodejs_20
        # Useful tool for serving the generated files statically while
        # developing in order to test how they'll do without the router.
        miniserve
      ];
      # To setup you'll need to use rustup to install nightly, then
      # add the wasm-unknown-unknown target.

      # Then do cargo install trunk cargo-leptos

      # To develop, use cargo leptos watch and tailwind-watch
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
      src = [
        ./flake.nix
        ./flake.lock
      ];
      shellHook = ''
        alias tailwind-watch="npx tailwindcss -i ./style/input.css -o ./style/output.css --watch"
        alias trunk="~/.cargo/bin/trunk"
        alias firebase="npx firebase-tools"
      '';
    };
  };
}
