Name:       bubblejail
Version:    0.7.0
Release:    1%{?dist}
Summary:    bubblewrap-based sandboxing utility

License:    GPL-3.0-or-later
URL:        https://github.com/igo95862/%{name}
Source0:    https://github.com/igo95862/%{name}/releases/download/%{version}/%{name}-%{version}.tar.gz

%global     min_python_version 3.9
%global     min_bubblewrap_version 0.5.0

%global     common_requires python3 >= %{min_python_version}, python3-tomli, python3-tomli-w, python3-pyxdg, libseccomp

BuildArch:  noarch

BuildRequires:  %{common_requires}
BuildRequires:  meson
BuildRequires:  python3-jinja2
BuildRequires:  scdoc

Requires:   %{common_requires}
Requires:   bubblewrap >= %{min_bubblewrap_version}
Requires:   python3-pyqt6
Requires:   xdg-dbus-proxy

%description
Bubblewrap-based sandboxing for desktop applications

%prep
%autosetup -c

%build
%meson
%meson_build

%install
%meson_install

%files
# rpmlint warns that `_libdir` ought not be used in a noarch package.
%{_libdir}/%{name}/
%{_datadir}/bash-completion/completions/%{name}
%{_datadir}/icons/hicolor/48x48/apps/%{name}-config.png
%{_datadir}/icons/hicolor/scalable/apps/%{name}-config.svg
%{_datadir}/applications/%{name}-config.desktop
%{_datadir}/%{name}/profiles/
%{_datadir}/fish/vendor_completions.d/bubblejail.fish
%{_bindir}/%{name}
%{_bindir}/%{name}-config
%doc %{_mandir}/man1/%{name}.1.gz
%doc %{_mandir}/man5/%{name}.services.5.gz


# TODO: When picking up the next release,
# * Deal with lack of `/etc/bubblejail/` - seems to be unconditionally
#   expected
# * Re-isolate build dependencies, as underlying issues are patched
%changelog
* Mon Feb 20 2023 j39m - 0.7.0-1
- Initial scratch work
