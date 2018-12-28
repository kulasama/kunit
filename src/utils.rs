use oci::config::Spec;

const CONFIG: &'static str="config.json";

pub fn load_spec() -> file::Result<()>{
    let data = file::get_text(CONFIG)?;
    Ok(())
}
