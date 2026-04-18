class IneditCli < Formula
  desc "Terminal-based INI/conf file editor built in Rust"
  homepage "https://github.com/filoucrackeur/inedit-cli"
  url "https://github.com/filoucrackeur/inedit-cli/releases/download/v0.1.0/inedit-cli-macos-arm64.tar.gz"
  version "0.1.0"
  license "MIT"
  sha256 "REPLACEME"

  bottle :unneeded

  depends_on "rust" => :build

  on_intel do
    url "https://github.com/filoucrackeur/inedit-cli/releases/download/v0.1.0/inedit-cli-macos-x64.tar.gz"
    sha256 "REPLACEME"
  end

  def install
    system "cargo", "build", "--release", "--manifest-path=inedit-cli/Cargo.toml"
    bin.install "inedit-cli/target/release/inedit-cli"
  end

  test do
    system "#{bin}/inedit-cli", "--version"
  end
end
