class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.3.1"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.1/langscan-macos-amd64"
      sha256 "780cdc61103517e777a9655ba301d2c2b6b65908667b7433be231caf53cfea2c"
    elsif Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.1/langscan-macos-arm64"
      sha256 "0dd55b8a959c9d5eda914df954d4398275104558086a6ee2456de92622c8ea97"
    end
  elsif OS.linux?
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.1/langscan-linux-amd64"
    sha256 "577ed0a88f9ace497bbfc87d23a5c4440e2b8dc858eb28cfa0f77b664a392e59"
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
