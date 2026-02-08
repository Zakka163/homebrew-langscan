class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.4.0"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.4.0/langscan-macos-amd64"
      sha256 "4335cbc7c09882d3becbed3934c9e00321e95cd6db8cd3b21645097f16253a2d"
    elsif Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.4.0/langscan-macos-arm64"
      sha256 "e8fd1a7029f0ffe79d7c3789e8794f7b5cd911c6715d49c9cf4e34ae679a4e22"
    end
  elsif OS.linux?
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.4.0/langscan-linux-amd64"
    sha256 "1974ca93ed99ef02a7fba6625934b160fd767d9055f7a9cc76db380e52dfd870"
  end

  def install
    if OS.mac?
      if Hardware::CPU.intel?
        bin.install "langscan-macos-amd64" => "langscan"
      elsif Hardware::CPU.arm?
        bin.install "langscan-macos-arm64" => "langscan"
      end
    elsif OS.linux?
      bin.install "langscan-linux-amd64" => "langscan"
    end
  end

  test do
    system "#{bin}/langscan", "--v"
  end
end
