class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.3.0"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.0/langscan-macos-amd64"
      sha256 "609c382e6095a2e7b3dbf0354b6aa17a16308b8ef445777705978628fb87a87d"
    elsif Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.0/langscan-macos-arm64"
      sha256 "c323e0c91a2b47fa09d61d2dbf1a72bcf55aabcb3124a289969064f8e30893b7"
    end
  elsif OS.linux?
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.0/langscan-linux-amd64"
    sha256 "5dbcce626bbc45bc8be0a3766541402a3fef71775a7b52488e704121ecaef2af"
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
