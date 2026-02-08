class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.5.1"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.5.1/langscan-macos-amd64"
      sha256 "055717c59af3bbfdb2d2aee54150867132638887d9dca2d59d67dff82b083a4a"
    elsif Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.5.1/langscan-macos-arm64"
      sha256 "cecbe65c29d7821e1db40b906c16b406d05b0a9f6ae73ca6e2128fe99c05ef74"
    end
  elsif OS.linux?
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.5.1/langscan-linux-amd64"
    sha256 "86c08310a6186f4102bd7f347227bf283541242e7c7abd906cae18faf6062338"
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
