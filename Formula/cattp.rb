class Cattp < Formula
    desc "A CLI tool to understand HTTP codes and cats."
    homepage "https://github.com/Schrodinger-Hat/cattp"
    url "https://github.com/Schrodinger-Hat/cattp/raw/main/cattp.tar.gz"
    sha256 "fb759ae362bbfc4f41090cf4c97855226c67ff6a63369629e5ab417131387279"
    version "1.0.3"
    license "MIT"
  
    def install
        # Move the extracted files to the appropriate location
        bin.install "cattp"

        # Make the executable executable
        chmod "+x", bin/"cattp"
    end
end