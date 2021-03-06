use oci::config::Spec;
use config::NamespaceType;
use config::Config;
use std::fs::File;
use std::net::TcpStream;
use std::collections::HashMap;
use time::Tm;

pub enum CtAct{
    CT_ACT_CREATE,
    CT_ACT_RUN,
    CT_ACT_RESTORE,
}

pub struct notifySocket{
    pub socket:TcpStream,
    pub host:String,
    pub soktetPath:String,
}

pub struct CriuPageServerInfo{
    pub Address:String,
    pub Port:i32,
}

pub struct VethPairName{
    pub ContainerInterfaceName:String,
    pub HostInterfaceName:String,
}

pub enum cgMode{
    CRIU_CG_MODE_SOFT,
    CRIU_CG_MODE_FULL,
    CRIU_CG_MODE_STRICT,
    CRIU_CG_MODE_DEFAULT,
}
pub struct CriuOpts{
    pub ImageDirectory:String,
    pub WorkDirectory:String,
    pub ParentImage:String,
    pub LeaveRunning:bool,
    pub TcpEstablished:bool,
    pub ExternalUnixConnections:bool,
    pub ShellJob:bool,
    pub FileLocks:bool,
    pub PreDump:bool,
    pub PageServer:CriuPageServerInfo,
    pub VethPairs:Vec<VethPairName>,
    pub ManageCgroupsMode:cgMode,
    pub AutoDedup:bool,
    pub LazyPages:bool,
    pub StatusFd:String,
}

pub enum Status{
    Created,
    Running,
    Pausing,
    Paused,
    Stopped,
}

pub struct State{
    pub ID:String,
    pub InitProcessPid:i32,
    pub InitProcessStartTime:u64,
    pub Crated:Tm,
    pub config:Config,
    pub Rootless:bool,
    pub CgroupPaths:HashMap<String,String>,
    pub NamespacePaths:HashMap<NamespaceType,String>,
    pub ExternalDescriptors:Vec<String>,
    pub IntelRdtPath:String,
}
pub trait Container{
    fn ID() -> String;
    fn Status() -> Status;
    fn OCIState() -> oci::state::State;
    fn Config() -> Config;
}



pub struct Runner<T:Container>{
    pub init:bool,
    pub enableSubreaper:bool,
    pub shouldDestory:bool,
    pub detach:bool,
    pub listenFDs:Vec<File>,
    pub preserveFDs:i64,
    pub pidFile:String,
    pub consoleSocket:String,
    pub container:T,
    pub action:CtAct,
    pub notifySocket:notifySocket,
    pub criuOpts:CriuOpts,
}
fn startContainer(spec:Spec){
    println!("t");
}