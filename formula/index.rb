class PrettyGitUi < Formula
  desc "Terminal UI for git operations"
  homepage "https://github.com/mmrakt/pretty-git-ui"
  version "0.2.7"
  url "https://github.com/mmrakt/pretty-git-ui/archive/refs/tags/v#{version}.tar.gz"
  sha256 "9f6a0bce3e568a4bc05bd0b0a59a6fd434d24389c2a38ecd8ba11a7f0b2741a5"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "pretty-git-ui version #{version}", shell_output("#{bin}/pretty-git-ui --version")
  end
end
