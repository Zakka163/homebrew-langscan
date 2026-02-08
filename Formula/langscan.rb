class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.1.0"

  on_macos do
   if Hardware::CPU.intel?
      url "https://github.com/Zakka163/langscan/releases/download/v0.1.0/langscan-macos-amd64"
      sha256 "64090c35b0649ae73787fbea159fc86ba033769a92f80ccbc8d2369699da0d71"
    else
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.1.0/langscan-macos-amd64"
      sha256 "64090c35b0649ae73787fbea159fc86ba033769a92f80ccbc8d2369699da0d71"
    end
  end

  on_linux do
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.1.0/langscan-linux-amd64"
    sha256 "1cf4a54700dd4db23bdb3eb06f73ceaf4e03827e3a34d49db2b394bfb7f20671"
  end

  def install
    bin.install "langscan-macos-arm64" => "langscan" if OS.mac? && Hardware::CPU.arm?
    bin.install "langscan-macos-amd64" => "langscan" if OS.mac? && Hardware::CPU.intel?
    bin.install "langscan-linux-amd64" => "langscan" if OS.linux?
  end

  test do
    system "#{bin}/langscan"
  end
end
