pub enum LinuxCapabilityType{
    CAP_NET_RAW,
    CAP_NET_BIND_SERVICE,
    CAP_AUDIT_READ,
    CAP_AUDIT_WRITE,
    CAP_DAC_OVERRIDE,
    CAP_SETFCAP,
    CAP_SETPCAP,
    CAP_SETGID,
    CAP_SETUID,
    CAP_MKNOD,
    CAP_CHOWN,
    CAP_FOWNER,
    CAP_FSETID,
    CAP_KILL,
    CAP_SYS_CHROOT,
    CAP_NET_BROADCAST,
    CAP_SYS_MODULE,
    CAP_SYS_RAWIO,
    CAP_SYS_PACCT,
    CAP_SYS_ADMIN,
    CAP_SYS_NICE,
    CAP_SYS_RESOURCE,
    CAP_SYS_TIME,
    CAP_SYS_TTY_CONFIG,
    CAP_AUDIT_CONTROL,
    CAP_MAC_OVERRIDE,
    CAP_MAC_ADMIN,
    CAP_NET_ADMIN,
    CAP_SYSLOG,
    CAP_DAC_READ_SEARCH,
    CAP_LINUX_IMMUTABLE,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_SYS_PTRACE,
    CAP_SYS_BOOT,
    CAP_LEASE,
    CAP_WAKE_ALARM,
    CAP_BLOCK_SUSPEND
}
pub struct Box{
    pub Height: u64,
    pub Width: u64,
}

pub struct User{
    pub UID:u32,
    pub GID:u32,
    pub AdditionalGids:Vec<u32>,
    pub Username:String,
}

pub struct POSIXRlimit{
    pub Type:String,
    pub Hard:u64,
    pub Soft:u64,
}
pub struct LinuxCapabilities{
    pub Bounding:Vec<LinuxCapabilityType>,
    pub Effective:Vec<LinuxCapabilityType>,
    pub Inheritable:Vec<LinuxCapabilityType>,
    pub Permitted:Vec<LinuxCapabilityType>,
    pub Ambient:Vec<LinuxCapabilityType>,
}

pub struct Process {
    pub Terminal: bool,
    pub ConsoleSize: Box,
    pub User: User,
    pub Args: Vec<String>,
    pub Env: Vec<String>,
    pub Cmd: String,
    pub Capabilities: LinuxCapabilities,
    pub Rlimits: Vec<POSIXRlimit>,
    pub NoNewPrivileges: bool,
    pub ApparmorProfile: String,
    pub OOMScoreAdj: i64,
    pub SelinuxLabel: String,
}






}
#[derive(Serialize, Deserialize, Debug)]
pub struct Spec{
    pub Version:String,
}