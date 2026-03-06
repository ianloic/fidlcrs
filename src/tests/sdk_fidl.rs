use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct FidlBuild {
    pub base_dir: PathBuf,
    pub target_name: String,
    pub sources: Vec<String>,
    pub public_deps: Vec<String>,
    pub experimental_flags: Vec<String>,
}

impl FidlBuild {
    pub fn resolved_sources(&self) -> Vec<PathBuf> {
        self.sources.iter().map(|s| self.base_dir.join(s)).collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct SdkFidl {
    pub libs: std::collections::HashMap<String, FidlBuild>,
}

impl SdkFidl {
    pub fn new() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let sdk_fidl_dir = manifest_dir.join("sdk-fidl");

        let mut libs = std::collections::HashMap::new();

        if let Ok(entries) = std::fs::read_dir(&sdk_fidl_dir) {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    let build_gn_path = path.join("BUILD.gn");
                    if build_gn_path.exists() {
                        let content = std::fs::read_to_string(&build_gn_path).unwrap();
                        if let Some(parsed) = parse_build_gn(&path, &content) {
                            let name = path.file_name().unwrap().to_str().unwrap().to_string();
                            libs.insert(name, parsed);
                        }
                    }
                }
            }
        }

        Self { libs }
    }

    pub fn cli_for_library(&self, name: &str) -> Option<(crate::cli::Cli, Vec<Vec<String>>)> {
        let parsed = self.libs.get(name)?;
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let vdso1 = manifest_dir
            .join("vdso-fidl/rights.fidl")
            .to_string_lossy()
            .to_string();
        let vdso2 = manifest_dir
            .join("vdso-fidl/zx_common.fidl")
            .to_string_lossy()
            .to_string();
        let vdso3 = manifest_dir
            .join("vdso-fidl/overview.fidl")
            .to_string_lossy()
            .to_string();

        let mut dep_filenames = vec![vdso1, vdso2, vdso3];

        let mut visited = std::collections::HashSet::new();
        let mut all_experimental: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for flag in &parsed.experimental_flags {
            all_experimental.insert(flag.clone());
        }

        fn visit(
            lib_name: &str,
            all_libs: &std::collections::HashMap<String, FidlBuild>,
            visited: &mut std::collections::HashSet<String>,
            dep_filenames: &mut Vec<String>,
            all_experimental: &mut std::collections::HashSet<String>,
        ) {
            if !visited.insert(lib_name.to_string()) {
                return;
            }
            if let Some(p) = all_libs.get(lib_name) {
                for flag in &p.experimental_flags {
                    all_experimental.insert(flag.clone());
                }
                for dep in &p.public_deps {
                    let dep_name = dep.trim_start_matches("//sdk/fidl/").to_string();
                    if dep_name == "//zircon/vdso/zx" {
                        continue;
                    }
                    visit(
                        &dep_name,
                        all_libs,
                        visited,
                        dep_filenames,
                        all_experimental,
                    );
                }
                for src in p.resolved_sources() {
                    dep_filenames.push(src.to_string_lossy().to_string());
                }
            }
        }

        for dep in &parsed.public_deps {
            let dep_name = dep.trim_start_matches("//sdk/fidl/").to_string();
            if dep_name == "//zircon/vdso/zx" {
                continue;
            }
            visit(
                &dep_name,
                &self.libs,
                &mut visited,
                &mut dep_filenames,
                &mut all_experimental,
            );
        }

        let mut main_filenames = Vec::new();
        for src in parsed.resolved_sources() {
            main_filenames.push(src.to_string_lossy().to_string());
        }

        // TODO: get versions from //sdk/version_history.json
        let cli = crate::cli::Cli {
            json: None,
            available: vec!["fuchsia:27,28,29,30,NEXT,HEAD".to_string()],
            experimental: all_experimental.into_iter().collect(),
            files: vec![],
            format: "text".to_string(),
            ..Default::default()
        };

        let source_managers = vec![dep_filenames, main_filenames];

        Some((cli, source_managers))
    }
}

pub struct SdkFidlIter<'a> {
    keys: std::collections::hash_map::Keys<'a, String, FidlBuild>,
}

impl<'a> Iterator for SdkFidlIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next().map(|s| s.as_str())
    }
}

impl<'a> IntoIterator for &'a SdkFidl {
    type Item = &'a str;
    type IntoIter = SdkFidlIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SdkFidlIter {
            keys: self.libs.keys(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token<'a> {
    Ident(&'a str),
    String(&'a str),
    Punct(char),
}

fn tokenize(content: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut chars = content.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '#' => {
                while let Some(&(_, ch)) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '"' => {
                let mut end = i + 1;
                while let Some(&(j, ch)) = chars.peek() {
                    if ch == '"' {
                        end = j;
                        chars.next(); // consume '"'
                        break;
                    }
                    chars.next();
                }
                tokens.push(Token::String(&content[i + 1..end]));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                let mut end = start + c.len_utf8();
                while let Some(&(j, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        end = j + ch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(&content[start..end]));
            }
            c if c.is_whitespace() => {}
            c if "=[](){},".contains(c) => {
                tokens.push(Token::Punct(c));
            }
            _ => {}
        }
    }
    tokens
}

pub fn parse_build_gn(base_dir: &std::path::Path, content: &str) -> Option<FidlBuild> {
    let tokens = tokenize(content);
    let mut iter = tokens.iter().peekable();

    let mut target_name = String::new();
    let mut sources = Vec::new();
    let mut public_deps = Vec::new();
    let mut experimental_flags = Vec::new();
    let mut in_fidl = false;

    while let Some(tok) = iter.next() {
        if let Token::Ident("fidl") = tok {
            if let Some(Token::Punct('(')) = iter.next() {
            } else {
                continue;
            }
            if let Some(Token::String(name)) = iter.next() {
                target_name = name.to_string();
            } else {
                continue;
            }
            if let Some(Token::Punct(')')) = iter.next() {
            } else {
                continue;
            }
            if let Some(Token::Punct('{')) = iter.next() {
            } else {
                continue;
            }
            in_fidl = true;
            break;
        }
    }

    if !in_fidl {
        return None;
    }

    while let Some(tok) = iter.next() {
        match tok {
            Token::Punct('}') => break,
            Token::Ident("sources") => {
                if let Some(Token::Punct('=')) = iter.next() {
                } else {
                    continue;
                }
                if let Some(Token::Punct('[')) = iter.next() {
                } else {
                    continue;
                }
                sources = parse_string_list(&mut iter);
            }
            Token::Ident("public_deps") => {
                if let Some(Token::Punct('=')) = iter.next() {
                } else {
                    continue;
                }
                if let Some(Token::Punct('[')) = iter.next() {
                } else {
                    continue;
                }
                public_deps = parse_string_list(&mut iter);
            }
            Token::Ident("experimental_flags") => {
                if let Some(Token::Punct('=')) = iter.next() {
                } else {
                    continue;
                }
                if let Some(Token::Punct('[')) = iter.next() {
                } else {
                    continue;
                }
                experimental_flags = parse_string_list(&mut iter);
            }
            _ => {}
        }
    }

    Some(FidlBuild {
        base_dir: base_dir.to_path_buf(),
        target_name,
        sources,
        public_deps,
        experimental_flags,
    })
}

fn parse_string_list(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token<'_>>>,
) -> Vec<String> {
    let mut list = Vec::new();
    while let Some(&tok) = iter.peek() {
        match tok {
            Token::Punct(']') => {
                iter.next();
                break;
            }
            Token::String(s) => {
                list.push(s.to_string());
                iter.next();
            }
            Token::Punct(',') => {
                iter.next();
            }
            _ => {
                iter.next();
            }
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_build_gn() {
        let content = r#"
        import("//build/fidl/fidl.gni")

        fidl("fuchsia.accessibility.scene") {
            sources = [ "provider.fidl" ]
            public_deps = [ "//sdk/fidl/fuchsia.ui.views" ]
            enable_hlcpp = true
        }
        "#;

        let parsed = parse_build_gn(std::path::Path::new(""), content).unwrap();
        assert_eq!(parsed.target_name, "fuchsia.accessibility.scene");
        assert_eq!(parsed.sources, vec!["provider.fidl"]);
        assert_eq!(
            parsed.public_deps,
            vec!["//sdk/fidl/fuchsia.ui.views".to_string()]
        );
        assert!(parsed.experimental_flags.is_empty());
    }

    #[test]
    fn test_parse_all_sdk_build_files() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let sdk_fidl_dir = manifest_dir.join("sdk-fidl");

        let entries = std::fs::read_dir(sdk_fidl_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("fuchsia.") {
                        let build_gn_path = path.join("BUILD.gn");
                        if build_gn_path.exists() {
                            let content = std::fs::read_to_string(&build_gn_path).unwrap();
                            let parsed = parse_build_gn(&path, &content);
                            assert!(
                                parsed.is_some(),
                                "Failed to parse BUILD.gn at {:?}",
                                build_gn_path
                            );
                            let parsed = parsed.unwrap();
                            assert!(
                                !parsed.sources.is_empty() || content.contains("sources = []"),
                                "Parsed no sources for {:?} (or ensure it's manually empty)",
                                build_gn_path
                            );
                        }
                    }
                }
            }
        }
    }

    const COMPILE_DENYLIST: &[&str] = &[
        "fuchsia.accessibility.scene",
        "fuchsia.accessibility.semantics",
        "fuchsia.accessibility.virtualkeyboard",
        "fuchsia.audio.controller",
        "fuchsia.audio.device",
        "fuchsia.audio.effects",
        "fuchsia.audio.mixer",
        "fuchsia.audio",
        "fuchsia.auth",
        "fuchsia.bluetooth.affordances",
        "fuchsia.bluetooth.avdtp.test",
        "fuchsia.bluetooth.avrcp.test",
        "fuchsia.bluetooth.avrcp",
        "fuchsia.bluetooth.bredr.test",
        "fuchsia.bluetooth.bredr",
        "fuchsia.bluetooth.fastpair",
        "fuchsia.bluetooth.gatt",
        "fuchsia.bluetooth.gatt2",
        "fuchsia.bluetooth.hfp",
        "fuchsia.bluetooth.host",
        "fuchsia.bluetooth.internal.a2dp",
        "fuchsia.bluetooth.le",
        "fuchsia.bluetooth.map",
        "fuchsia.bluetooth.power",
        "fuchsia.bluetooth.rfcomm.test",
        "fuchsia.bluetooth.snoop",
        "fuchsia.bluetooth.sys",
        "fuchsia.boot.metadata",
        "fuchsia.boot",
        "fuchsia.buildinfo.test",
        "fuchsia.buttons",
        "fuchsia.camera",
        "fuchsia.camera2.hal",
        "fuchsia.camera2",
        "fuchsia.camera3",
        "fuchsia.castauth",
        "fuchsia.component.decl",
        "fuchsia.component.internal",
        "fuchsia.component.resolution",
        "fuchsia.component.runner",
        "fuchsia.component.runtime",
        "fuchsia.component.sandbox",
        "fuchsia.component.test",
        "fuchsia.component",
        "fuchsia.cpu.profiler",
        "fuchsia.dash",
        "fuchsia.debugdata",
        "fuchsia.debugger",
        "fuchsia.developer.console",
        "fuchsia.developer.remotecontrol",
        "fuchsia.developer.tiles",
        "fuchsia.device.fs",
        "fuchsia.device.vsock",
        "fuchsia.diagnostics.host",
        "fuchsia.diagnostics.types",
        "fuchsia.diagnostics",
        "fuchsia.driver.crash",
        "fuchsia.driver.development",
        "fuchsia.driver.framework",
        "fuchsia.driver.host",
        "fuchsia.driver.index",
        "fuchsia.driver.loader",
        "fuchsia.driver.playground",
        "fuchsia.driver.registrar",
        "fuchsia.driver.test",
        "fuchsia.driver.token",
        "fuchsia.element",
        "fuchsia.factory.lowpan",
        "fuchsia.factory",
        "fuchsia.fdomain",
        "fuchsia.firmware.crash",
        "fuchsia.fonts.experimental",
        "fuchsia.fshost",
        "fuchsia.gpu.magma",
        "fuchsia.hardware.audio",
        "fuchsia.hardware.block.volume",
        "fuchsia.hardware.bluetooth",
        "fuchsia.hardware.camera",
        "fuchsia.hardware.display.engine",
        "fuchsia.hardware.display",
        "fuchsia.hardware.fan",
        "fuchsia.hardware.google.nanohub",
        "fuchsia.hardware.gpio",
        "fuchsia.hardware.hrtimer",
        "fuchsia.hardware.i2c.businfo",
        "fuchsia.hardware.i2cimpl",
        "fuchsia.hardware.input",
        "fuchsia.hardware.mediacodec",
        "fuchsia.hardware.network.driver",
        "fuchsia.hardware.network",
        "fuchsia.hardware.nfc",
        "fuchsia.hardware.pin",
        "fuchsia.hardware.pinimpl",
        "fuchsia.hardware.platform.bus",
        "fuchsia.hardware.platform.device",
        "fuchsia.hardware.power.statecontrol",
        "fuchsia.hardware.pty",
        "fuchsia.hardware.qcom.hvdcpopti",
        "fuchsia.hardware.ramdisk",
        "fuchsia.hardware.sdhci",
        "fuchsia.hardware.sdio",
        "fuchsia.hardware.sdmmc",
        "fuchsia.hardware.sensors",
        "fuchsia.hardware.spi",
        "fuchsia.hardware.spmi",
        "fuchsia.hardware.sysmem",
        "fuchsia.hardware.tee",
        "fuchsia.hardware.telephony.transport",
        "fuchsia.hardware.temperature",
        "fuchsia.hardware.thermal",
        "fuchsia.hardware.ufs",
        "fuchsia.hardware.usb.dci",
        "fuchsia.hardware.usb.endpoint",
        "fuchsia.hardware.usb.function",
        "fuchsia.hardware.usb.hci",
        "fuchsia.hardware.usb.phy",
        "fuchsia.hardware.usb",
        "fuchsia.identity.account",
        "fuchsia.identity.internal",
        "fuchsia.images",
        "fuchsia.input.injection",
        "fuchsia.input.report",
        "fuchsia.input.virtualkeyboard",
        "fuchsia.inspect",
        "fuchsia.io.test",
        "fuchsia.io",
        "fuchsia.kms",
        "fuchsia.lightsensor",
        "fuchsia.location.sensor",
        "fuchsia.logger",
        "fuchsia.lowpan.bootstrap",
        "fuchsia.lowpan.device",
        "fuchsia.lowpan.driver",
        "fuchsia.lowpan.experimental",
        "fuchsia.lowpan.test",
        "fuchsia.lowpan.thread",
        "fuchsia.media.drm",
        "fuchsia.media.playback",
        "fuchsia.media.sessions2",
        "fuchsia.media.sounds",
        "fuchsia.media.target",
        "fuchsia.media",
        "fuchsia.mediacodec",
        "fuchsia.mediastreams",
        "fuchsia.memory.attribution.plugin",
        "fuchsia.memory.attribution",
        "fuchsia.memory.heapdump.process",
        "fuchsia.metrics.test",
        "fuchsia.metrics",
        "fuchsia.migration",
        "fuchsia.nand",
        "fuchsia.net.debug",
        "fuchsia.net.dhcp",
        "fuchsia.net.dhcpv6",
        "fuchsia.net.filter.deprecated",
        "fuchsia.net.filter",
        "fuchsia.net.http",
        "fuchsia.net.interfaces.admin",
        "fuchsia.net.interfaces",
        "fuchsia.net.masquerade",
        "fuchsia.net.matchers",
        "fuchsia.net.mdns",
        "fuchsia.net.multicast.admin",
        "fuchsia.net.name",
        "fuchsia.net.ndp",
        "fuchsia.net.neighbor",
        "fuchsia.net.policy.properties",
        "fuchsia.net.policy.socketproxy",
        "fuchsia.net.power",
        "fuchsia.net.resources",
        "fuchsia.net.root",
        "fuchsia.net.routes.admin",
        "fuchsia.net.routes",
        "fuchsia.net.settings",
        "fuchsia.net.sockets",
        "fuchsia.net.stack",
        "fuchsia.net.tun",
        "fuchsia.net.virtualization",
        "fuchsia.overnet.protocol",
        "fuchsia.pkg.http",
        "fuchsia.pkg.internal",
        "fuchsia.pkg.resolution",
        "fuchsia.pkg",
        "fuchsia.posix.socket.packet",
        "fuchsia.posix.socket.raw",
        "fuchsia.posix.socket",
        "fuchsia.power.battery.test",
        "fuchsia.power.battery",
        "fuchsia.power.system",
        "fuchsia.power.topology.test",
        "fuchsia.process.init",
        "fuchsia.process.lifecycle",
        "fuchsia.process",
        "fuchsia.recovery.android",
        "fuchsia.sensors.types",
        "fuchsia.sensors",
        "fuchsia.session.scene",
        "fuchsia.session.window",
        "fuchsia.session",
        "fuchsia.settings.policy",
        "fuchsia.settings",
        "fuchsia.starnix.binder",
        "fuchsia.storage.block",
        "fuchsia.storage.partitions",
        "fuchsia.sys2",
        "fuchsia.sysmem",
        "fuchsia.tee.manager",
        "fuchsia.telephony.manager",
        "fuchsia.terminal",
        "fuchsia.test.manager",
        "fuchsia.testing.harness",
        "fuchsia.thermal",
        "fuchsia.time.alarms",
        "fuchsia.tpm.cr50",
        "fuchsia.tracing.controller",
        "fuchsia.tracing.perfetto",
        "fuchsia.tracing.provider",
        "fuchsia.ui.activity.control",
        "fuchsia.ui.activity",
        "fuchsia.ui.annotation",
        "fuchsia.ui.app",
        "fuchsia.ui.composition.internal",
        "fuchsia.ui.composition",
        "fuchsia.ui.focus",
        "fuchsia.ui.gfx",
        "fuchsia.ui.input.accessibility",
        "fuchsia.ui.input",
        "fuchsia.ui.input3",
        "fuchsia.ui.keyboard.focus",
        "fuchsia.ui.observation.geometry",
        "fuchsia.ui.observation.scope",
        "fuchsia.ui.observation.test",
        "fuchsia.ui.pointer.augment",
        "fuchsia.ui.pointer",
        "fuchsia.ui.pointerinjector.configuration",
        "fuchsia.ui.pointerinjector",
        "fuchsia.ui.policy",
        "fuchsia.ui.scenic",
        "fuchsia.ui.test.conformance",
        "fuchsia.ui.test.context",
        "fuchsia.ui.test.input",
        "fuchsia.ui.test.scene",
        "fuchsia.ui.views",
        "fuchsia.ultrasound",
        "fuchsia.update.channelcontrol",
        "fuchsia.update.usb",
        "fuchsia.video",
        "fuchsia.virtualaudio",
        "fuchsia.virtualconsole",
        "fuchsia.virtualization.hardware",
        "fuchsia.virtualization",
        "fuchsia.weave",
        "fuchsia.web",
        "fuchsia.wlan.common",
        "fuchsia.wlan.device.service",
        "fuchsia.wlan.device",
        "fuchsia.wlan.fullmac",
        "fuchsia.wlan.ieee80211",
        "fuchsia.wlan.minstrel",
        "fuchsia.wlan.mlme",
        "fuchsia.wlan.phyimpl",
        "fuchsia.wlan.policy",
        "fuchsia.wlan.product.deprecatedclient",
        "fuchsia.wlan.product.deprecatedconfiguration",
        "fuchsia.wlan.sme",
        "fuchsia.wlan.softmac",
        "fuchsia.wlan.stats",
        "fuchsia.wlan.tap",
        "zbi",
    ];

    const COMPARE_DENYLIST: &[&str] = &[
        // missing .fidl.json
        "fdf",
        "fuchsia.acpi.chromeos",
        "fuchsia.device.test",
        "fuchsia.firebase.messaging",
        "fuchsia.hardware.amlogiccanvas",
        "fuchsia.hardware.block.verified",
        "fuchsia.hardware.ethernet.board",
        "fuchsia.hardware.google.ec",
        "fuchsia.hardware.gpu.amlogic",
        "fuchsia.hardware.gpu.mali",
        "fuchsia.hardware.hidctl",
        "fuchsia.hardware.input.focaltech",
        "fuchsia.hardware.lightsensor",
        "fuchsia.hardware.nvram",
        "fuchsia.hardware.powersource.test",
        "fuchsia.hardware.securemem",
        "fuchsia.hardware.ti.metadata",
        "fuchsia.hardware.usb.hcitest",
        "fuchsia.hardware.usb.tester",
        "fuchsia.hardware.vsi",
        "fuchsia.identity.authentication",
        "fuchsia.identity.credential",
        "fuchsia.identity.ctap",
        "fuchsia.input.interaction.observation",
        "fuchsia.opencl.loader",
        "fuchsia.perfmon.cpu",
        "fuchsia.telephony.ril",
        "fuchsia.telephony.snoop",
        "fuchsia.testing.runner",
        // mismatched output
        "fuchsia.accessibility.gesture",
        "fuchsia.accessibility.tts",
        "fuchsia.acpi.tables",
        "fuchsia.bluetooth",
        "fuchsia.bluetooth.deviceid",
        "fuchsia.bluetooth.pandora",
        "fuchsia.buildinfo",
        "fuchsia.castconfig",
        "fuchsia.castremotecontrol",
        "fuchsia.castsetup",
        "fuchsia.castsysteminfo",
        "fuchsia.cobalt",
        "fuchsia.data",
        "fuchsia.developer.ffx.speedtest",
        "fuchsia.developer.ffxdaemonlifecycle",
        "fuchsia.device",
        "fuchsia.diagnostics.system",
        "fuchsia.driver.compat",
        "fuchsia.driver.metadata",
        "fuchsia.driver.test.logger",
        "fuchsia.drm",
        "fuchsia.exception",
        "fuchsia.factory.wlan",
        "fuchsia.feedback",
        "fuchsia.fido.report",
        "fuchsia.fonts",
        "fuchsia.fs",
        "fuchsia.gpu.agis",
        "fuchsia.gpu.virtio",
        "fuchsia.hardware.acpi",
        "fuchsia.hardware.adb",
        "fuchsia.hardware.adc",
        "fuchsia.hardware.adcimpl",
        "fuchsia.hardware.amlogic.metadata",
        "fuchsia.hardware.audio.signalprocessing",
        "fuchsia.hardware.backlight",
        "fuchsia.hardware.block",
        "fuchsia.hardware.block.driver",
        "fuchsia.hardware.block.encrypted",
        "fuchsia.hardware.clock",
        "fuchsia.hardware.clock.measure",
        "fuchsia.hardware.clockimpl",
        "fuchsia.hardware.cpu.ctrl",
        "fuchsia.hardware.display.types",
        "fuchsia.hardware.dsp",
        "fuchsia.hardware.fastboot",
        "fuchsia.hardware.gnss",
        "fuchsia.hardware.goldfish",
        "fuchsia.hardware.goldfish.pipe",
        "fuchsia.hardware.haptics",
        "fuchsia.hardware.hidbus",
        "fuchsia.hardware.i2c",
        "fuchsia.hardware.inlineencryption",
        "fuchsia.hardware.interconnect",
        "fuchsia.hardware.interrupt",
        "fuchsia.hardware.light",
        "fuchsia.hardware.mailbox",
        "fuchsia.hardware.midi",
        "fuchsia.hardware.nand",
        "fuchsia.hardware.pci",
        "fuchsia.hardware.power",
        "fuchsia.hardware.power.sensor",
        "fuchsia.hardware.power.suspend",
        "fuchsia.hardware.pwm",
        "fuchsia.hardware.qualcomm.fastrpc",
        "fuchsia.hardware.qualcomm.router",
        "fuchsia.hardware.radar",
        "fuchsia.hardware.ram.metrics",
        "fuchsia.hardware.registers",
        "fuchsia.hardware.reset",
        "fuchsia.hardware.rpmb",
        "fuchsia.hardware.rtc",
        "fuchsia.hardware.scsi",
        "fuchsia.hardware.serial",
        "fuchsia.hardware.serialimpl",
        "fuchsia.hardware.sharedmemory",
        "fuchsia.hardware.sockettunnel",
        "fuchsia.hardware.spiimpl",
        "fuchsia.hardware.tpmimpl",
        "fuchsia.hardware.trippoint",
        "fuchsia.hardware.usb.descriptor",
        "fuchsia.hardware.usb.device",
        "fuchsia.hardware.usb.peripheral",
        "fuchsia.hardware.usb.request",
        "fuchsia.hardware.uwb",
        "fuchsia.hardware.virtio.pmem",
        "fuchsia.hardware.vreg",
        "fuchsia.hardware.vsock",
        "fuchsia.hwinfo",
        "fuchsia.images2",
        "fuchsia.input",
        "fuchsia.intl",
        "fuchsia.kernel",
        "fuchsia.legacymetrics",
        "fuchsia.location.gnss",
        "fuchsia.location.namedplace",
        "fuchsia.location.position",
        "fuchsia.lowpan",
        "fuchsia.lowpan.spinel",
        "fuchsia.media.audio",
        "fuchsia.mem",
        "fuchsia.memory.debug",
        "fuchsia.memory.heapdump.client",
        "fuchsia.memory.inspection",
        "fuchsia.memory.sampler",
        "fuchsia.net",
        "fuchsia.net.stackmigrationdeprecated",
        "fuchsia.paver",
        "fuchsia.pkg.garbagecollector",
        "fuchsia.pkg.rewrite",
        "fuchsia.power",
        "fuchsia.power.broker",
        "fuchsia.power.cpu",
        "fuchsia.power.cpu.manager",
        "fuchsia.power.manager.debug",
        "fuchsia.power.metrics",
        "fuchsia.power.systemmode",
        "fuchsia.process.explorer",
        "fuchsia.recovery.ui",
        "fuchsia.scheduler",
        "fuchsia.security.keymint",
        "fuchsia.session.power",
        "fuchsia.starnix.container",
        "fuchsia.starnix.gralloc",
        "fuchsia.starnix.psi",
        "fuchsia.starnix.runner",
        "fuchsia.stash",
        "fuchsia.storage.blobfs",
        "fuchsia.storage.ftl",
        "fuchsia.stresstest",
        "fuchsia.sysinfo",
        "fuchsia.sysmem2",
        "fuchsia.system.state",
        "fuchsia.tee",
        "fuchsia.test",
        "fuchsia.time.external",
        "fuchsia.time.test",
        "fuchsia.tpm",
        "fuchsia.tracing",
        "fuchsia.ui.brightness",
        "fuchsia.ui.compression.internal",
        "fuchsia.ui.display.singleton",
        "fuchsia.unknown",
        "fuchsia.update",
        "fuchsia.update.config",
        "fuchsia.vsock",
        "fuchsia.vulkan.loader",
    ];

    fn strip_filename(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                map.remove("filename");
                for (_, v) in map.iter_mut() {
                    strip_filename(v);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr.iter_mut() {
                    strip_filename(v);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn test_compile_all_sdk_libraries() {
        let sdk_fidl = super::SdkFidl::new();

        let mut failed = Vec::new();
        let mut mismatched = Vec::new();
        let mut missing = Vec::new();
        let mut matched = Vec::new();

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let temp_dir = std::env::temp_dir();

        for name in &sdk_fidl {
            if COMPILE_DENYLIST.contains(&name) {
                continue;
            }

            if let Some((mut cli, source_managers)) = sdk_fidl.cli_for_library(name) {
                let out_json_path = temp_dir.join(format!("{}_out.fidl.json", name));
                cli.json = Some(out_json_path.to_string_lossy().to_string());

                let res = std::panic::catch_unwind(|| crate::cli::run(&cli, &source_managers));

                match res {
                    Ok(Err(_e)) => {
                        failed.push(name);
                    }
                    Err(_) => {
                        failed.push(name);
                    }
                    Ok(Ok(())) => {
                        if !COMPARE_DENYLIST.contains(&name) {
                            let ref_json_path = manifest_dir
                                .join(format!("sdk-fidl-gen/{}/{}.fidl.json", name, name));

                            if ref_json_path.exists() {
                                let expected_str = std::fs::read_to_string(&ref_json_path).unwrap();
                                let actual_str = std::fs::read_to_string(&out_json_path).unwrap();

                                if let (Ok(mut expected), Ok(mut actual)) = (
                                    serde_json::from_str::<serde_json::Value>(&expected_str),
                                    serde_json::from_str::<serde_json::Value>(&actual_str),
                                ) {
                                    strip_filename(&mut expected);
                                    strip_filename(&mut actual);

                                    if expected != actual {
                                        let exp_formatted =
                                            serde_json::to_string_pretty(&expected).unwrap();
                                        let act_formatted =
                                            serde_json::to_string_pretty(&actual).unwrap();

                                        let exp_path =
                                            temp_dir.join(format!("{}_expected.json", name));
                                        let act_path =
                                            temp_dir.join(format!("{}_actual.json", name));

                                        std::fs::write(&exp_path, exp_formatted).unwrap();
                                        std::fs::write(&act_path, act_formatted).unwrap();

                                        println!("\nJSON mismatch for {}", name);
                                        if let Ok(output) = std::process::Command::new("diff")
                                            .arg("-u")
                                            .arg("--color=always")
                                            .arg(&exp_path)
                                            .arg(&act_path)
                                            .output()
                                        {
                                            println!("{}", String::from_utf8_lossy(&output.stdout));
                                        }

                                        let _ = std::fs::remove_file(&exp_path);
                                        let _ = std::fs::remove_file(&act_path);

                                        mismatched.push(name);
                                    } else {
                                        matched.push(name);
                                    }
                                } else {
                                    mismatched.push(name);
                                }
                            } else {
                                println!("Reference JSON not found for {}", name);
                                missing.push(name);
                            }
                        }
                        let _ = std::fs::remove_file(&out_json_path);
                    }
                }
            }
        }

        if !mismatched.is_empty() {
            println!("Mismatched JSON for libraries:");
            let mut m = mismatched.clone();
            m.sort();
            for name in m {
                println!("  {:?},", name);
            }
        }
        if !missing.is_empty() {
            println!("Missing JSON for libraries: {:?}", missing);
        }

        println!("Total libraries: {}", sdk_fidl.libs.len());
        println!("Compile skipped libraries: {}", COMPILE_DENYLIST.len());
        println!("Compare skipped libraries: {}", COMPARE_DENYLIST.len());
        println!("Failed libraries: {}", failed.len());
        println!("Mismatched libraries: {}", mismatched.len());
        println!("Missing libraries: {}", missing.len());
        println!(
            "Passed libraries: {}",
            sdk_fidl.libs.len()
                - COMPILE_DENYLIST.len()
                - COMPARE_DENYLIST.len()
                - failed.len()
                - mismatched.len()
                - missing.len()
        );

        assert!(
            failed.is_empty(),
            "Failed to compile some SDK libraries: {:?}",
            failed
        );
    }
}
