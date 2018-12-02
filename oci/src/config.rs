
use std::collections::HashMap;

// doc link: http://man7.org/linux/man-pages/man7/capabilities.7.html
#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxCapabilityType{
    CAP_AUDIT_CONTROL,
    CAP_AUDIT_READ,
    CAP_AUDIT_WRITE,
    CAP_BLOCK_SUSPEND,
    CAP_CHOWN,
    CAP_DAC_OVERRIDE,
    CAP_DAC_READ_SEARCH,
    CAP_FOWNER,
    CAP_FSETID,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_KILL,
    CAP_LEASE,
    CAP_LINUX_IMMUTABLE,
    CAP_MAC_ADMIN,
    CAP_MAC_OVERRIDE,
    CAP_MKNOD,
    CAP_NET_ADMIN,
    CAP_NET_BIND_SERVICE,
    CAP_NET_BROADCAST,
    CAP_NET_RAW,
    CAP_SETGID,
    CAP_SETFCAP,
    CAP_SETPCAP,
    CAP_SETUID,
    CAP_SYS_ADMIN,
    CAP_SYS_BOOT,
    CAP_SYS_CHROOT,
    CAP_SYS_MODULE,
    CAP_SYS_NICE,
    CAP_SYS_PACCT,
    CAP_SYS_RAWIO,
    CAP_SYS_RESOURCE,
    CAP_SYS_TIME,
    CAP_SYS_TTY_CONFIG,
    CAP_SYSLOG,
    CAP_WAKE_ALARM,
}






#[derive(Serialize, Deserialize, Debug)]
pub struct Box{
    pub Height: u64,
    pub Width: u64,
}
















#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub UID:u32,
    pub GID:u32,
    pub AdditionalGids:Vec<u32>,
    pub Username:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct POSIXRlimit{
    pub Type:String,
    pub Hard:u64,
    pub Soft:u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxCapabilities{
    pub Bounding:Vec<LinuxCapabilityType>,
    pub Effective:Vec<LinuxCapabilityType>,
    pub Inheritable:Vec<LinuxCapabilityType>,
    pub Permitted:Vec<LinuxCapabilityType>,
    pub Ambient:Vec<LinuxCapabilityType>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Root{
    pub Path:String,
    pub Readonly:bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount{
    pub Destination:String,
    pub Type:String,
    pub Source:String,
    pub Options:Vec<String>,
}



pub struct LinuxIDMapping{
    pub ContainerID:u32,
    pub HostID:u32,
    pub Size:u32,
}


pub struct LinuxDeviceCgroup{
    pub Allow:bool,
    pub Type:String,
    pub Major:i64,
    pub Minor:i64,
    pub Access:String,
}

pub struct LinuxMemory{
    pub Limit:i64,
    pub Reservation:i64,
    pub Swap:i64,
    pub Kernel:i64,
    pub KernelTCP:i64,
    pub Swappiness:u64,
    pub DisableOOMKiller:bool,
}

pub struct LinuxCPU{
    pub Shares:u64,
    pub Quota:i64,
    pub Period:u64,
    pub RealtimeRuntime:i64,
    pub RealtimePeriod:u64,
    pub Cpus:String,
    pub Mems:String,
}

pub struct LinuxPids{
    pub Limit:i64,
}

pub struct LinuxWeightDevice{
    pub Major:i64,
    pub Minor:i64,
    pub Weight:u16,
    pub LeafWeight:u16,
}

pub struct LinuxThrottleDevice{
    pub Major:i64,
    pub Minor:i64,
    pub Rate:u64,
}
pub struct LinuxBlockIO{
    pub Weight:u16,
    pub LeafWeight:u16,
    pub WeightDevice:Vec<LinuxWeightDevice>,
    pub ThrottleReadBpsDevice:Vec<LinuxThrottleDevice>,
    pub ThrottleWriteBpsDevice:Vec<LinuxThrottleDevice>,
    pub ThrottleReadIOPSDevice:Vec<LinuxThrottleDevice>,
    pub ThrottleWriteIOPSDevice:Vec<LinuxThrottleDevice>,
}

pub struct LinuxHugepageLimit{
    pub Pagesize:String,
    pub Limit:u64,
}

pub struct LinuxInterfacePriority{
    pub Name:String,
    pub Priority:u32,
}

pub struct LinuxNetwork{
    pub ClassID:u32,
    pub Priorities:Vec<LinuxInterfacePriority>
}

pub struct LinuxRdma{
    pub HcaHandles:u32,
    pub HcaObjects:u32,
}
pub struct LinuxResources{
    pub Devices:Vec<LinuxDeviceCgroup>,
    pub Memory:LinuxMemory,
    pub CPU:LinuxCPU,
    pub Pids:LinuxPids,
    pub BlockIO:LinuxBlockIO,
    pub HugepageLimits:Vec<LinuxHugepageLimit>,
    pub Network:LinuxNetwork,
    pub Rdma:HashMap<String,LinuxRdma>,
}

pub enum LinuxNamespaceType{
    Pidnamespace = 0x20000000,
    NetworkNamespace = 0x40000000,
    MountNamespace = 0x00020000,
    IPCNamespace = 0x08000000,
    UTSNamespace = 0x04000000,
    UserNamespace = 0x10000000,
    CgroupNamespace = 0x02000000,
}

pub struct LinuxNamespace{
    pub Type:LinuxNamespaceType,
    pub Path:String,
}

pub struct LinuxDevice{
    pub Path:String,
    pub Type:String,
    pub Major:i64,
    pub Minor:i64,
    pub FileMode:u32,
    pub UID:u32,
    pub GID:u32,
}

pub enum LinuxSeccompAction{
    SCMP_ACT_KILL = 0x00000000,
    SCMP_ACT_TRAP = 0x00030000,
    SCMP_ACT_ERRNO = 0x00050001,
    SCMP_ACT_TRACE = 0x7ff00001,
    SCMP_ACT_ALLOW = 0x7fff0000,
}

type Arch = String;

pub enum LinuxSeccompOperator {
    SCMP_CMP_NE = 1, /* not equal */
    SCMP_CMP_LT = 2, /* less than */
    SCMP_CMP_LE = 3, /* less than or equal */
    SCMP_CMP_EQ = 4, /* equal */
    SCMP_CMP_GE = 5, /* greater than or equal */
    SCMP_CMP_GT = 6, /* greater than */
    SCMP_CMP_MASKED_EQ = 7, /* masked equality */
}

pub struct LinuxSeccompArg{
    pub Index:u64,
    pub Value:u64,
    pub ValueTwo:u64,
    pub Op:LinuxSeccompOperator,
}
pub struct LinuxSyscall{
    pub Names:Vec<String>,
    pub Action:LinuxSeccompAction,
    pub Args:Vec<LinuxSeccompArg>,
}

pub struct LinuxSeccomp{
    pub DefautltAction:LinuxSeccompAction,
    pub Architectures:Vec<Arch>,
    pub Syscalls:Vec<LinuxSyscall>,
}

pub struct LinuxIntelRdt{
    pub ClosID:String,
    pub L3CacheSchema:String,
    pub MemBwSchema:String,
}

struct Linux{
    pub UIDMappings:Vec<LinuxIDMapping>,
    pub GIDMappings:Vec<LinuxIDMapping>,
    pub Sysctl:HashMap<String,String>,
    pub Resources:LinuxResources,
    pub CgroupPath:String,
    pub Namesapces:Vec<LinuxNamespace>,
    pub Devices:Vec<LinuxDevice>,
    pub Seccomp:LinuxSeccomp,
    pub RootfsPropagation:String,
    pub MaskedPaths:Vec<String>,
    pub ReadonlyPaths:Vec<String>,
    pub MountLabel:String,
    pub IntelRdt:LinuxIntelRdt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook{
    pub Path:String,
    pub Args:Vec<String>,
    pub Env:Vec<String>,
    pub Timeout:i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hooks{
    pub Prestart:Vec<Hook>,
    pub Poststart:Vec<Hook>,
    pub Poststop:Vec<Hook>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec{
    pub Version:String,
    pub Process:Process,
    pub Root:Root,
    pub Hostname:String,
    pub Mounts:Vec<Mount>,
    pub Hooks:Hooks,
    pub Annotations:HashMap<String,String>,
}











