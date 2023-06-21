class Cattp < Formula
    desc "A CLI tool to understand HTTP codes and cats."
    homepage "https://github.com/Schrodinger-Hat/cattp"
    url "https://github.com/Schrodinger-Hat/cattp/raw/main/cattp.tar.gz"
    sha256 "2236b760aec8489009d2c2b1fc03a4642fdaa89224973148b249d085abf2f875"
    version "1.0.2"
    license "MIT"
  
    def install
        # Move the extracted files to the appropriate location
        bin.install "cattp"

        # Make the executable executable
        chmod "+x", bin/"cattp"
    end
end