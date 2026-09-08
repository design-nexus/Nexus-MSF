use crate::model::*;

pub fn snapshot() -> Snapshot {
    Snapshot {
        version: "6.4.0-demo".into(),
        ruby: "3.2".into(),
        db_ok: true,
        workspace: "demo".into(),
        workspaces: vec!["default".into(), "demo".into()],
        hosts: vec![
            Host {
                address: "10.0.0.5".into(),
                name: "dc1.lab".into(),
                os: "Windows Server 2019".into(),
                purpose: "server".into(),
                info: "domain controller".into(),
                services: vec![
                    svc(135, "msrpc"),
                    svc(139, "netbios-ssn"),
                    svc(445, "smb"),
                    svc(3389, "rdp"),
                ],
                vulns: vec![Vuln {
                    name: "MS17-010".into(),
                    refs: vec!["CVE-2017-0144".into()],
                    info: "SMBv1".into(),
                }],
            },
            Host {
                address: "10.0.0.12".into(),
                name: "web1.lab".into(),
                os: "Ubuntu Linux 22.04".into(),
                purpose: "server".into(),
                info: String::new(),
                services: vec![svc(22, "ssh"), svc(80, "http"), svc(443, "https")],
                vulns: vec![],
            },
            Host {
                address: "10.0.0.20".into(),
                name: "files.lab".into(),
                os: "Windows 10".into(),
                purpose: "client".into(),
                info: String::new(),
                services: vec![svc(445, "smb")],
                vulns: vec![],
            },
        ],
        sessions: vec![Session {
            id: "1".into(),
            kind: "meterpreter".into(),
            via_exploit: "exploit/windows/smb/psexec".into(),
            via_payload: "windows/x64/meterpreter/reverse_tcp".into(),
            tunnel_local: "10.0.0.2:4444".into(),
            tunnel_peer: "10.0.0.20:55331".into(),
            info: r"NT AUTHORITY\SYSTEM @ FILES".into(),
            workspace: "demo".into(),
            session_host: "10.0.0.20".into(),
            platform: "windows".into(),
            arch: "x64".into(),
            routes: "10.0.1.0/24".into(),
        }],
        jobs: vec![Job {
            id: "0".into(),
            name: "Exploit: multi/handler".into(),
        }],
        creds: vec![Cred {
            host: "10.0.0.20".into(),
            service: "smb".into(),
            user: "Administrator".into(),
            pass: "••••demo".into(),
            kind: "password".into(),
            realm: "LAB".into(),
        }],
        loot: vec![Loot {
            host: "10.0.0.20".into(),
            ltype: "host.windows.gather.env".into(),
            name: "env.txt".into(),
            info: "environment".into(),
            path: "/tmp/demo-env.txt".into(),
        }],
        notes: vec![Note {
            host: "10.0.0.5".into(),
            ntype: "smb.fingerprint".into(),
            data: "Windows Server 2019".into(),
        }],
        module_stats: ModuleStats {
            exploits: 2400,
            auxiliary: 1200,
            post: 400,
            payloads: 600,
            encoders: 50,
            nops: 10,
        },
    }
}

fn svc(port: u16, name: &str) -> Service {
    Service {
        port,
        proto: "tcp".into(),
        name: name.into(),
        state: "open".into(),
        info: String::new(),
    }
}

pub fn modules() -> Vec<ModuleMeta> {
    vec![
        ModuleMeta {
            kind: "exploit".into(),
            fullname: "windows/smb/ms17_010_eternalblue".into(),
            rank: "great".into(),
            refs: vec!["CVE-2017-0144".into()],
            rport: Some(445),
            platforms: vec!["windows".into()],
            description: "MS17-010 EternalBlue SMB Remote Windows Kernel Pool Corruption".into(),
        },
        ModuleMeta {
            kind: "exploit".into(),
            fullname: "windows/smb/psexec".into(),
            rank: "excellent".into(),
            refs: vec![],
            rport: Some(445),
            platforms: vec!["windows".into()],
            description: "Authenticated SMB code execution".into(),
        },
        ModuleMeta {
            kind: "auxiliary".into(),
            fullname: "scanner/ssh/ssh_version".into(),
            rank: "normal".into(),
            refs: vec![],
            rport: Some(22),
            platforms: vec![],
            description: "SSH version scanner".into(),
        },
        ModuleMeta {
            kind: "auxiliary".into(),
            fullname: "scanner/http/http_version".into(),
            rank: "normal".into(),
            refs: vec![],
            rport: Some(80),
            platforms: vec![],
            description: "HTTP version scanner".into(),
        },
        ModuleMeta {
            kind: "post".into(),
            fullname: "windows/gather/enum_logged_on_users".into(),
            rank: "normal".into(),
            refs: vec![],
            rport: None,
            platforms: vec!["windows".into()],
            description: "Logged on users".into(),
        },
        ModuleMeta {
            kind: "payload".into(),
            fullname: "windows/x64/meterpreter/reverse_tcp".into(),
            rank: "normal".into(),
            refs: vec![],
            rport: None,
            platforms: vec!["windows".into()],
            description: "Windows Meterpreter (Reflective Injection), Reverse TCP Stager".into(),
        },
    ]
}

pub fn module_names(kind: ModuleKind) -> Vec<String> {
    modules()
        .into_iter()
        .filter(|m| m.kind == kind.as_str())
        .map(|m| m.fullname)
        .collect()
}

pub fn module_info(kind: &str, name: &str) -> ModuleInfo {
    let meta = modules()
        .into_iter()
        .find(|m| m.kind == kind && m.fullname == name);
    let Some(m) = meta else {
        return ModuleInfo {
            kind: kind.into(),
            fullname: name.into(),
            name: name.into(),
            rank: "normal".into(),
            description: String::new(),
            refs: vec![],
            options: default_opts(),
        };
    };
    ModuleInfo {
        kind: m.kind,
        fullname: m.fullname.clone(),
        name: m.fullname,
        rank: m.rank,
        description: m.description,
        refs: m.refs,
        options: default_opts(),
    }
}

fn default_opts() -> Vec<ModuleOption> {
    vec![
        ModuleOption {
            name: "RHOSTS".into(),
            required: true,
            desc: "Target hosts".into(),
            opt_type: "address".into(),
            ..Default::default()
        },
        ModuleOption {
            name: "RPORT".into(),
            required: true,
            desc: "Target port".into(),
            default: "445".into(),
            value: "445".into(),
            opt_type: "port".into(),
            ..Default::default()
        },
        ModuleOption {
            name: "LHOST".into(),
            required: false,
            desc: "Listener address".into(),
            default: "10.0.0.2".into(),
            value: "10.0.0.2".into(),
            opt_type: "address".into(),
            ..Default::default()
        },
        ModuleOption {
            name: "LPORT".into(),
            required: false,
            desc: "Listener port".into(),
            default: "4444".into(),
            value: "4444".into(),
            opt_type: "port".into(),
            ..Default::default()
        },
    ]
}
