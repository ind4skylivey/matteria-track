%global crate materiatrack

Name:           materiatrack
Version:        1.0.0
Release:        1%{?dist}
Summary:        Mystical Final Fantasy-themed CLI time tracker

License:        MIT
URL:            https://github.com/ind4skylivey/matteria-track
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  sqlite-devel

Requires:       sqlite
Recommends:     gnupg2
Suggests:       libnotify

%description
MateriaTrack is a mystical, Final Fantasy-themed power user time tracking CLI
based on Zeit. Track your time with style, earn achievements, and master your
productivity destiny.

Features:
- 6 Materia-themed color schemes
- Git commit integration
- Obsidian note sync
- 20+ achievements to unlock
- GPG-encrypted database option
- DWM/Polybar statusbar integration

%prep
%autosetup -n matteria-track-%{version}

%build
export CARGO_HOME="$PWD/.cargo"
cargo build --release --locked

%install
install -D -m 755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

ln -s %{name} %{buildroot}%{_bindir}/mtrack

install -D -m 644 man/%{name}.1 %{buildroot}%{_mandir}/man1/%{name}.1 || true

install -D -m 644 completions/%{name}.bash \
    %{buildroot}%{_datadir}/bash-completion/completions/%{name} || true
install -D -m 644 completions/_%{name} \
    %{buildroot}%{_datadir}/zsh/site-functions/_%{name} || true
install -D -m 644 completions/%{name}.fish \
    %{buildroot}%{_datadir}/fish/vendor_completions.d/%{name}.fish || true

install -D -m 644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

%check
cargo test --release --locked

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_bindir}/mtrack
%{_mandir}/man1/%{name}.1*
%{_datadir}/bash-completion/completions/%{name}
%{_datadir}/zsh/site-functions/_%{name}
%{_datadir}/fish/vendor_completions.d/%{name}.fish

%changelog
* %(date "+%a %b %d %Y") ind4skylivey <your.email@example.com> - 1.0.0-1
- Initial release
- Core time tracking engine
- Git/Obsidian integrations
- Security features (GPG encryption, audit logging)
- 6 Materia themes
- 20+ achievements
- Desktop notifications
- Fuzzy finder for projects/tasks
