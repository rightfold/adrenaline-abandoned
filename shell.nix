{pkgs ? import ./nix/pkgs.nix {}}:
pkgs.stdenv.mkDerivation {
    name = "adrenaline";
    buildInputs = [
        pkgs.cargo
    ];
    phases = ["installPhase"];
    installPhase = ''
        {
            echo 'This derivation is not for building, just for use with '
            echo 'nix-shell. See README for instructions.'
        } 2>&1
        false
    '';
}
