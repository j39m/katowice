Name:       bubblejail
Version:    0.7.0
Release:    1%{?dist}
Summary:    bubblewrap-based sandboxing utility

License:    GPL-3.0
URL:        https://github.com/igo95862/%{name}
Source0:    https://github.com/igo95862/%{name}/releases/download/%{version}/%{name}-%{version}.tar.gz

%global     min_python_version 3.9
%global     min_bubblewrap_version 0.5.0

BuildArch:  noarch

BuildRequires:  python3 >= %{min_python_version}
BuildRequires:  python3-jinja2 meson scdoc

Requires:   python3 >= %{min_python_version}
Requires:   python3-pyxdg python3-tomli python3-tomli-w
Requires:   bubblewrap >= %{min_bubblewrap_version}
Requires:   xdg-dbus-proxy python3-pyqt6 libseccomp

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
# rpmlint warns that `%{_libdir}` ought not be used in a noarch package.
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


%changelog
* Mon Feb 20 2023 j39m - 0.7.0-1
- Initial scratch work
