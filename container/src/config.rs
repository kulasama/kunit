use std::collections::HashMap;
use time::Duration;

pub enum NamespaceType{
    NEWNET,
    NEWPID,
    NEWNS,
    NEWUTS,
    NEWIPC,
    NEWUSER,
    NEWCGROUP,
}

pub struct Namespace{
    pub Type:NamespaceType,
    pub Path:String,
}

type Namespaces = Vec<Namespace>;

pub struct Command{
    pub Path:String,
    pub Args:Vec<String>,
    pub Env:Vec<String>,
    pub Dir:String,
    pub Timeout:Duration,
}

pub struct Mount{
    pub Source:String,
    pub Destination:String,
    pub Device:String,
    pub Flags:i32,
    pub PropagationFlags:Vec<i32>,
    pub Data:String,
    pub Relabel:String,
    pub Extensions:Vec<i32>,
    pub PremountCmds:Vec<Command>,
    pub PostmountCmds:Vec<Command>,
}

pub struct Device{
    pub Type:i32,
    pub Path:String,
    pub Major:i64,
    pub Minor:i64,
    pub Permissions:String,
    pub FileMode:i32,
    pub Uid:u32,
    pub Gid:u32,
    pub Allow:bool,
}

pub struct Capabilities{
    pub Bounding:Vec<String>,
    pub Effective:Vec<String>,
    pub Inheritable:Vec<String>,
    pub Permitted:Vec<String>,
    pub Ambient:Vec<String>,
}

pub struct Network{
    pub Type:String,
    pub Name:String,
    pub Bridge:String,
    pub MacAddress:String,
    pub Address:String,
    pub Gateway:String,
    pub IPv6Address:String,
    pub IPv6Gateway:String,
    pub Mtu:i32,
    pub TxQueueLen:i32,
    pub HostInterfaceName:String,
    pub HairpinMode:bool,
}

pub struct Route{
    pub Destination:String,
    pub Source:String,
    pub Gateway:String,
    pub InterfaceName:String,
}

pub struct ThrottleDevice{
    pub Major:i64,
    pub Minor:i64,
    pub Rate:u64,
}

pub enum FreezerState{
    Undefined,
    Frozen,
    Thawedin
}

pub struct HugepageLimit{
    pub Pagesize:String,
    pub Limit:u64,
}

pub struct IfPrioMap{
    pub Interface:String,
    pub Priority:i64,
}

pub struct Cgroup{
    pub Name:String,
    pub Parent:String,
    pub Path:String,
    pub ScopePrefix:String,
    pub Paths:HashMap<String,String>,
    pub AllowAllDevices:bool,
    pub AllowedDevices:Vec<Device>,
    pub DeniedDevices:Vec<Device>,
    pub Devices:Vec<Device>,
    pub Memory:i64,
    pub MemoryReservation:i64,
    pub MemorySwap:i64,
    pub KernelMemory:i64,
    pub KernelMemoryTCP:i64,
    pub CpuShares:u64,
    pub CpuQuota:i64,
    pub CpuPeriod:u64,
    pub CpuRtRuntime:i64,
    pub CpuRtPeriod:u64,
    pub CpusetCpus:String,
    pub CpusetMems:String,
    pub PidsLimit:i64,
    pub BlkioWeight:u16,
    pub BlkioLeafWeight:u16,
    pub BlkioWeightDevice:Vec<WeightDevice>,
    pub BlkioThrottleReadBpsDevice:Vec<ThrottleDevice>,
    pub BlkioThrottleWriteBpsDevice:Vec<ThrottleDevice>,
    pub BlkioThrottleReadIOPSDevice:Vec<ThrottleDevice>,
    pub BlkioThrottleWriteIOPSDevice:Vec<ThrottleDevice>,
    pub Freezer:FreezerState,
    pub HugetlbLimit:Vec<HugepageLimit>,
    pub OomKillDisable:bool,
    pub MemroySwappiness:u64,
    pub NetPrioIfpriomap:Vec<IfPrioMap>,
    pub NetClsClassid:u32,
}

pub struct WeightDevice{
    pub Major:i64,
    pub Minor:i64,
    pub Weight:u16,
    pub LeafWeight:u16,
}

pub struct Rlimit{
    pub Type:i32,
    pub Hard:u64,
    pub Soft:u64,
}

pub struct IDMap{
    pub ContainerID:i32,
    pub HostID:i32,
    pub Size:i32,
}

pub enum Operator{
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    MaskEqualTo,
}

pub enum Action{
    Kill,
    Errno,
    Trap,
    Allow,
    Trace
}

pub struct Arg{
    pub Index:u32,
    pub Value:u64,
    pub ValueTwo:u64,
    pub Op:Operator,
}

pub struct Syscall{
    pub Name:String,
    pub Action:Action,
    pub Args:Vec<Arg>,
}

pub struct Seccomp{
    pub DefaultAction:Action,
    pub Architectures:Vec<String>,
    pub Syscalls:Vec<Syscall>,
}


pub struct IntelRdt{
    pub LeCacheSchema:String,
    pub MemBwSchema:String,
}


pub trait Hook{
    fn Run(&self,oci::state::State);
}

pub struct Hooks{
    pub Prestate:Vec<Box<Hook>>,
    pub Poststart:Vec<Box<Hook>>,
    pub Poststop:Vec<Box<Hook>>,
}

pub struct Config{
    pub NoPivotRoot:bool,
    pub ParentDeathSignal:i32,
    pub Rootfs:String,
    pub Readonlyfs:bool,
    pub RootPropagation:i32,
    pub Mounts:Vec<Mount>,
    pub Devices:Vec<Device>,
    pub MountLabel:String,
    pub Hostname:String,
    pub Namespaces:Namespaces,
    pub Capabilities:Capabilities,
    pub Networks:Vec<Network>,
    pub Routes:Vec<Route>,
    pub Cgroup:Cgroup,
    pub AppArmorProfile:String,
    pub ProcessLabel:String,
    pub Rlimit:Vec<Rlimit>,
    pub OomScoreAdj:i32,
    pub UidMappings:Vec<IDMap>,
    pub GidMappings:Vec<IDMap>,
    pub MaskPaths:Vec<String>,
    pub ReadonlyPaths:Vec<String>,
    pub Sysctl:HashMap<String,String>,
    pub Seccomp:Seccomp,
    pub NoNewPrivileges:bool,
    pub Hooks:Hooks,
    pub Version:String,
    pub Labels:Vec<String>,
    pub NoNewKeyring:bool,
    pub IntelRdt:IntelRdt,
    pub RootlessEUID:bool,
    pub RootlessCgroups:bool,
}