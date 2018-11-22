
pub enum NamespaceType{
    mount = 0x00020000,
    cgroup = 0x02000000,
    uts = 0x04000000,
    ipc = 0x08000000,
    user = 0x10000000,
    pid = 0x20000000,
    network = 0x40000000,
}
pub struct Namespace{
    #[serde(rename = "type")]
    pub typ: NamespaceType,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String
}