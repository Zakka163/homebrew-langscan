class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.1.0/langscan-macos-arm64"
      sha256 "ISI_SHA256_ARM"
    else
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.1.0/langscan-macos-amd64"
      sha256 "ISI_SHA256_AMD"
    end
  end

  on_linux do
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.1.0/langscan-linux-amd64"
    sha256 "ISI_SHA256_LINUX"
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
