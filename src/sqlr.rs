use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;
use iot_sqlx::configuration::{get_configuration, Settings, MysqlSettings};

#[derive(Debug)]
struct PropertyConfig {
    server_ip: String,
    server_port: i32,
    terminal_id: String,
    station_id: String,
    info_obj_addr: String,
    protocol_name: String,
    property_name: String,
    equipment_type: String,
    equipment_id: String,
    factor: Option<String>,
    constant: Option<String>,
    expression: Option<String>,
    result_type: Option<String>,
    cab: Option<String>,
    stack: Option<String>,
    cluster: Option<String>,
    pack: Option<String>,
    cell: Option<String>,
    description: Option<String>,
    comment: Option<String>,
    unit: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let configuration: Settings = get_configuration().expect("Failed to read configuration");
    let mysqlConf: MysqlSettings = configuration.mysql;
    let connection_str = format!("{}:{}@{}/{}", mysqlConf.username, mysqlConf.password, mysqlConf.hostname, mysqlConf.database);

    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(mysqlConf.max_connections)
        .connect(&connection_str).await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row = sqlx::query(r#"
    select
      t1.server_ip,
        t1.server_port,
        t1.terminal_id,
        t2.* from terminal_info as t1
      inner join property_config as t2
    on t1.station_id=t2.station_id"#)
        .map(|row: MySqlRow| PropertyConfig {
            server_ip: row.get("server_ip"),
            server_port: row.get("server_port"),
            terminal_id: row.get("terminal_id"),
            station_id: row.get("station_id"),
            info_obj_addr: row.get("info_obj_addr"),
            protocol_name: row.get("protocol_name"),
            property_name: row.get("property_name"),
            equipment_type: row.get("equipment_type"),
            equipment_id: row.get("equipment_id"),
            factor: row.get("factor"),
            constant: row.get("constant"),
            expression: row.get("expression"),
            result_type: row.get("result_type"),
            cab: row.get("cab"),
            stack: row.get("stack"),
            cluster: row.get("cluster"),
            pack: row.get("pack"),
            cell: row.get("cell"),
            description: row.get("description"),
            comment: row.get("comment"),
            unit: row.get("unit"),
        })
        .fetch_all(&pool).await?;

    for i in row {
        println!("{:?}", i);
    }

    Ok(())
}