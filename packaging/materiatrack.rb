# Homebrew formula for MateriaTrack
# Final Fantasy-themed CLI time tracker

class Materiatrack < Formula
  desc "Mystical Final Fantasy-themed CLI time tracker based on Zeit"
  homepage "https://github.com/ind4skylivey/matteria-track"
  url "https://github.com/ind4skylivey/matteria-track/archive/refs/tags/v1.0.3.tar.gz"
  sha256 "9958f84456b9cc9673d73fa1758866030f49ff277b6f7ca18e43c04199fe8096"
  license "MIT"
  head "https://github.com/ind4skylivey/matteria-track.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    man1.install "man/materiatrack.1" if File.exist?("man/materiatrack.1")

    bash_completion.install "completions/materiatrack.bash" if File.exist?("completions/materiatrack.bash")
    zsh_completion.install "completions/_materiatrack" if File.exist?("completions/_materiatrack")
    fish_completion.install "completions/materiatrack.fish" if File.exist?("completions/materiatrack.fish")

    bin.install_symlink "materiatrack" => "mtrack"
  end

  def caveats
    <<~EOS
      💎 MateriaTrack has been installed!

      Quick start:
        mtrack track -p "Project" -t "Task"  # Start tracking
        mtrack finish                        # Stop tracking
        mtrack list                          # Show entries
        mtrack stats                         # View statistics

      Configuration file: #{etc}/materiatrack/config.toml
      Database location: ~/Library/Application Support/materiatrack/

      "Master your time, master your destiny"
    EOS
  end

  test do
    assert_match "materiatrack", shell_output("#{bin}/materiatrack --version")

    system bin/"materiatrack", "project", "add", "TestProject"
    system bin/"materiatrack", "task", "add", "TestTask", "-p", "TestProject"

    output = shell_output("#{bin}/materiatrack project list")
    assert_match "TestProject", output
  end
end
