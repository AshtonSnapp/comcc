{
	description = "Component-C compiler development environment";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
	};

	outputs = { self, nixpkgs }: let
		systems = [
			"x86_64-linux"
			"aarch64-linux"
			"x86_64-darwin"
			"aarch64-darwin"
		];
		
		forAllSystems = nixpkgs.lib.genAttrs systems;
	in {
		devShells = forAllSystems (system: let
			pkgs = import nixpkgs { inherit system; };
			llvm = pkgs.llvmPackages_23;
		in {
			default = pkgs.mkShell {
				packages = with pkgs; [
					rustc
					cargo
					rustfmt
					clippy
					rust-analyzer
					llvm.llvm
					llvm.llvm.dev
					llvm.clang
					llvm.lld

					pkg-config
					cmake
				];

				LLVM_SYS_231_PREFIX = "${llvm.llvm.dev}";

				shellHook = ''
					echo "Component-C Compiler Development Environment"
					echo "LLVM: $(llvm-config --version)"
					echo "Rust: $(rustc --version)"
					alias lla='ls -lA'
				'';
			};
		});
	};
}
