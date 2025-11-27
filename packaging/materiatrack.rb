# Homebrew formula for MateriaTrack
# Final Fantasy-themed CLI time tracker

class Materiatrack < Formula
  desc "Mystical Final Fantasy-themed CLI time tracker based on Zeit"
  homepage "https://github.com/ind4skylivey/matteria-track"
  url "https://github.com/ind4skylivey/matteria-track/archive/v1.0.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"
  head "https://github.com/ind4skylivey/matteria-track.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    man1.install "man/materiatrack.1" if File.exist?("man/materiatrack.1")

    generate_completions_from_executable(bin/"materiatrack", "completions")

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
