class Langscan < Formula
  desc "Scan installed programming languages"
  homepage "https://github.com/Zakka163/homebrew-langscan"
  version "0.3.2"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.2/langscan-macos-amd64"
      sha256 "b9e39c196264fd25b2c2132f2eb34418ded3007911edf290c315d3130ff04ca2"
    elsif Hardware::CPU.arm?
      url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.2/langscan-macos-arm64"
      sha256 "4085d1cd094d5122679afdddf426a5b69a3df86c8095d9c836f76c5c3524dd6f"
    end
  elsif OS.linux?
    url "https://github.com/Zakka163/homebrew-langscan/releases/download/v0.3.2/langscan-linux-amd64"
    sha256 "33087830fe86d86165d706c0e6b8107a7f1d083910e25d67d282c7241897b708"
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
