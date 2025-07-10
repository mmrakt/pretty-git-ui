class PrettyGitUi < Formula
  desc "Terminal UI for git operations"
  homepage "https://github.com/mmrakt/pretty-git-ui"
  version "0.1.1"
  url "https://github.com/mmrakt/pretty-git-ui/archive/refs/tags/v#{version}.tar.gz"
  sha256 "ea002bfd275fb23b1d6c5985fd6c607c5bbcf8ba94f71f6fd5667fb9f8814901"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "pretty-git-ui version #{version}", shell_output("#{bin}/pretty-git-ui --version")
  end
end
