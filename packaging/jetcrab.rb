class JetCrab < Formula
  desc "JavaScript runtime in Rust with Node.js compatibility"
  homepage "https://github.com/JetCrabCollab/jetcrab"
  url "https://github.com/JetCrabCollab/jetcrab/releases/download/v0.4.0/jetcrab-macos-x86_64.tar.gz"
  sha256 "placeholder-sha256"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/JetCrabCollab/jetcrab/releases/download/v0.4.0/jetcrab-macos-aarch64.tar.gz"
    sha256 "placeholder-sha256-arm"
  end

  def install
    bin.install "jetcrab"
    man1.install "man/jetcrab.1" if File.exist?("man/jetcrab.1")
  end

  test do
    system "#{bin}/jetcrab", "--version"
  end
end
