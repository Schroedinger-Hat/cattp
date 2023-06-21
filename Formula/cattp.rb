class Cattp < Formula
    desc "A CLI tool to understand HTTP codes and cats."
    homepage "https://github.com/Schrodinger-Hat/cattp"
    url "https://github.com/Schrodinger-Hat/cattp/raw/main/cattp.tar.gz"
    sha256 "36dca26af9f913000ecb879810f990a19d43516de9c749b57236772ce8496ea4"
    version "1.0.4"
    license "MIT"
  
    def install
        # Move the extracted files to the appropriate location
        bin.install "cattp"

        # Make the executable executable
        chmod "+x", bin/"cattp"
    end
end