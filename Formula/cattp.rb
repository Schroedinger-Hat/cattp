class Cattp < Formula
    desc "A CLI tool to understand HTTP codes and cats."
    homepage "https://github.com/Schrodinger-Hat/cattp"
    url "https://github.com/Schrodinger-Hat/cattp/raw/main/cattp.tar.gz"
    sha256 "1da1110fa423cb8b554eece8d4576ac28c8e4572"
    version "1.0.1"
    license "MIT"
  
    def install
        # Move the extracted files to the appropriate location
        bin.install "cattp"

        # Make the executable executable
        chmod "+x", bin/"cattp"
    end
end