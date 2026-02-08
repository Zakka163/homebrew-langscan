class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.6.0"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.6.0/langscan-macos-amd64"
      sha256 "ff06a0367ae9894b611967f5a28b5caf52967452d4bb7cc85ee1a7b70bc84f9c"
    elsif Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.6.0/langscan-macos-arm64"
      sha256 "8a3738cfef51633230554d9d8a3db8d5a0c0205824477fe9f408856b1e173379"
    end
  elsif OS.linux?
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.6.0/langscan-linux-amd64"
    sha256 "ce617a0cf1a729a0ec23a4c6d60c8cb48f9c6a9d0d34020183c78be90c43b94b"
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
