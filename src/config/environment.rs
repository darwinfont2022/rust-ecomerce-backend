use config::Config;
use crate::config::setting::Setting;

pub fn read_setting() -> Setting {
    let setting = Config::builder()
        .add_source(config::File::with_name("e_config.json"))
        .build()
        .expect("Could not read configuration file.");

    let rsp_setting = setting.try_deserialize::<Setting>().expect("Failed to deserialize configuration file.");

    rsp_setting
}